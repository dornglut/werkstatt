use std::collections::BTreeMap;

use crate::{
    domain::{SourceIdentity, SourceKind, WorkFacts, WorkSourceObservation},
    support::sha256::digest_hex,
};

use super::{MAX_SOURCE_BYTES, SourceParseError};

pub(super) fn parse(
    bytes: &[u8],
    identity: SourceIdentity,
    observed_at_unix_seconds: u64,
    mutable: bool,
) -> Result<WorkSourceObservation, SourceParseError> {
    if bytes.len() > MAX_SOURCE_BYTES {
        return Err(SourceParseError::new(
            "source.too_large",
            "work source exceeds the bounded input size",
            "reduce or export the authoritative source below 131072 bytes",
        ));
    }
    let text = std::str::from_utf8(bytes).map_err(|_| {
        SourceParseError::new(
            "source.non_utf8",
            "Markdown work source is not valid UTF-8",
            "export the work source as UTF-8 Markdown or versioned JSON",
        )
    })?;
    let document = MarkdownDocument::parse(text)?;
    let facts = document.into_facts()?;
    WorkSourceObservation::new(
        identity,
        SourceKind::Markdown,
        digest_hex(bytes),
        observed_at_unix_seconds,
        mutable,
        facts,
        Vec::new(),
        bytes.len(),
    )
    .map_err(|error| {
        SourceParseError::new("source.invalid_contract", error.message, error.correction)
    })
}

#[derive(Debug)]
struct MarkdownDocument {
    title: String,
    metadata: BTreeMap<String, String>,
    sections: BTreeMap<String, Vec<String>>,
}

impl MarkdownDocument {
    fn parse(text: &str) -> Result<Self, SourceParseError> {
        let mut title = None;
        let mut metadata = BTreeMap::new();
        let mut headings = Vec::<(String, Vec<String>)>::new();
        let mut current: Option<(String, Vec<String>)> = None;

        for line in text.lines() {
            if let Some(value) = line.strip_prefix("# ") {
                if title.replace(value.trim().to_owned()).is_some() {
                    return Err(SourceParseError::new(
                        "source.ambiguous_title",
                        "Markdown contains more than one level-one title",
                        "retain one authoritative title",
                    ));
                }
                continue;
            }
            if let Some(value) = line.strip_prefix("## ") {
                if let Some(section) = current.take() {
                    headings.push(section);
                }
                current = Some((normalize_key(value), Vec::new()));
                continue;
            }
            if let Some((_, lines)) = current.as_mut() {
                lines.push(line.to_owned());
            } else if let Some((key, value)) = parse_metadata_line(line) {
                insert_material_value(&mut metadata, key, value)?;
            }
        }
        if let Some(section) = current {
            headings.push(section);
        }

        let title = title.ok_or_else(|| {
            SourceParseError::new(
                "source.missing_title",
                "Markdown work source has no level-one title",
                "add one `#` title to the authoritative work source",
            )
        })?;

        let mut sections = BTreeMap::new();
        for (name, lines) in headings {
            if sections.insert(name.clone(), lines).is_some() {
                return Err(SourceParseError::new(
                    "source.duplicate_section",
                    format!("material section `{name}` appears more than once"),
                    "merge duplicate material sections into one authoritative section",
                ));
            }
        }

        if let Some(contract_lines) = sections.get("work contract") {
            for (key, value) in parse_contract_table(contract_lines)? {
                insert_material_value(&mut metadata, key, value)?;
            }
        }
        if metadata.is_empty() {
            return Err(SourceParseError::new(
                "source.missing_contract",
                "Markdown source has neither leading work metadata nor a `## Work contract` table",
                "add the accepted leading `Field: value` metadata or work-contract table",
            ));
        }

        Ok(Self {
            title,
            metadata,
            sections,
        })
    }

    fn into_facts(self) -> Result<WorkFacts, SourceParseError> {
        let goal = self.section_scalar(&["goal", "objective"])?;
        let owner = self.metadata_value(&["owner", "owning domain", "owning role"])?;
        let work_class = self.metadata_value(&["work class", "kind"])?;
        let lifecycle_stage = self.metadata_value(&["lifecycle stage", "lifecycle"])?;
        let accepted_base =
            self.metadata_value(&["accepted base", "accepted design base", "base"])?;
        let scope = self.section_list(&["scope", "included scope", "executable contract"])?;
        let non_goals = self.section_list(&[
            "non goals",
            "explicit exclusions",
            "exclusions",
            "exclusions and stop conditions",
        ])?;
        let validation = self.section_list(&[
            "validation",
            "validation requirements",
            "acceptance and review",
        ])?;
        let stop_conditions =
            self.section_list(&["stop conditions", "exclusions and stop conditions"])?;
        let exit_gate = self
            .metadata_value_optional(&["exit gate"])
            .or_else(|| self.section_scalar_optional(&["exit gate"]))
            .ok_or_else(|| missing("exit gate"))?;
        let next_transition = self
            .metadata_value_optional(&[
                "next transition",
                "next authorized transition",
                "next gate",
            ])
            .or_else(|| {
                self.section_scalar_optional(&["next transition", "next authorized transition"])
            })
            .ok_or_else(|| missing("next authorized transition"))?;

        let recognized_sections = [
            "work contract",
            "goal",
            "objective",
            "scope",
            "included scope",
            "executable contract",
            "non goals",
            "explicit exclusions",
            "exclusions",
            "exclusions and stop conditions",
            "validation",
            "validation requirements",
            "acceptance and review",
            "stop conditions",
            "exit gate",
            "next transition",
            "next authorized transition",
        ];
        let recognized_metadata = [
            "owner",
            "owning domain",
            "owning role",
            "work class",
            "kind",
            "lifecycle stage",
            "lifecycle",
            "accepted base",
            "accepted design base",
            "base",
            "exit gate",
            "next transition",
            "next authorized transition",
            "next gate",
        ];
        let mut supplemental = self
            .sections
            .iter()
            .filter(|(name, _)| !recognized_sections.contains(&name.as_str()))
            .map(|(name, lines)| (name.clone(), normalized_lines(lines)))
            .filter(|(_, lines)| !lines.is_empty())
            .collect::<BTreeMap<_, _>>();
        for (name, value) in &self.metadata {
            if !recognized_metadata.contains(&name.as_str()) {
                supplemental.insert(format!("metadata.{name}"), vec![value.clone()]);
            }
        }

        WorkFacts::new(
            self.title,
            goal,
            owner,
            work_class,
            lifecycle_stage,
            accepted_base,
            scope,
            non_goals,
            validation,
            stop_conditions,
            exit_gate,
            next_transition,
            supplemental,
        )
        .map_err(|error| {
            SourceParseError::new("source.invalid_contract", error.message, error.correction)
        })
    }

    fn metadata_value(&self, names: &[&str]) -> Result<String, SourceParseError> {
        self.metadata_value_optional(names)
            .ok_or_else(|| missing(names[0]))
    }

    fn metadata_value_optional(&self, names: &[&str]) -> Option<String> {
        names
            .iter()
            .find_map(|name| self.metadata.get(*name).cloned())
    }

    fn section_scalar(&self, names: &[&str]) -> Result<String, SourceParseError> {
        self.section_scalar_optional(names)
            .ok_or_else(|| missing(names[0]))
    }

    fn section_scalar_optional(&self, names: &[&str]) -> Option<String> {
        names.iter().find_map(|name| {
            let values = normalized_lines(self.sections.get(*name)?);
            (!values.is_empty()).then(|| values.join("\n"))
        })
    }

    fn section_list(&self, names: &[&str]) -> Result<Vec<String>, SourceParseError> {
        names
            .iter()
            .find_map(|name| {
                self.sections
                    .get(*name)
                    .map(|lines| normalized_lines(lines))
            })
            .filter(|lines| !lines.is_empty())
            .ok_or_else(|| missing(names[0]))
    }
}

fn parse_metadata_line(line: &str) -> Option<(String, String)> {
    let line = line.trim();
    if line.is_empty()
        || line.starts_with("<!--")
        || line.starts_with('|')
        || line.starts_with(['-', '*'])
    {
        return None;
    }
    let (key, value) = line.split_once(':')?;
    let key = normalize_key(key);
    let value = normalize_scalar(value);
    (!key.is_empty() && !value.is_empty()).then_some((key, value))
}

fn insert_material_value(
    values: &mut BTreeMap<String, String>,
    key: String,
    value: String,
) -> Result<(), SourceParseError> {
    if values.insert(key.clone(), value).is_some() {
        return Err(SourceParseError::new(
            "source.duplicate_field",
            format!("work-contract field `{key}` appears more than once across accepted forms"),
            "retain one authoritative value for each material field",
        ));
    }
    Ok(())
}

fn parse_contract_table(lines: &[String]) -> Result<BTreeMap<String, String>, SourceParseError> {
    let rows = lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| line.starts_with('|'))
        .collect::<Vec<_>>();
    if rows.len() < 3 {
        return Err(SourceParseError::new(
            "source.malformed_table",
            "work-contract table is missing header, separator, or values",
            "use a two-column Markdown table with field and value rows",
        ));
    }
    let separator = rows[1]
        .trim_matches('|')
        .split('|')
        .map(str::trim)
        .collect::<Vec<_>>();
    if separator.len() != 2
        || separator.iter().any(|cell| {
            let marker = cell.trim_matches(':');
            marker.len() < 3 || !marker.bytes().all(|byte| byte == b'-')
        })
    {
        return Err(SourceParseError::new(
            "source.malformed_table",
            "work-contract table separator is malformed",
            "use a two-column Markdown separator such as `|---|---|`",
        ));
    }
    let mut table = BTreeMap::new();
    for row in rows.into_iter().skip(2) {
        let cells = row
            .trim_matches('|')
            .split('|')
            .map(|cell| cell.trim())
            .collect::<Vec<_>>();
        if cells.len() != 2 || cells[0].is_empty() {
            return Err(SourceParseError::new(
                "source.malformed_table",
                "work-contract table contains a malformed row",
                "keep exactly one non-empty field and value in every row",
            ));
        }
        let key = normalize_key(cells[0]);
        let value = normalize_scalar(cells[1]);
        if key.is_empty() || value.is_empty() {
            return Err(SourceParseError::new(
                "source.malformed_table",
                "work-contract table contains an empty normalized field or value",
                "keep exactly one non-empty field and value in every row",
            ));
        }
        if table.insert(key.clone(), value).is_some() {
            return Err(SourceParseError::new(
                "source.duplicate_field",
                format!("work-contract field `{key}` appears more than once"),
                "retain one authoritative value for each material field",
            ));
        }
    }
    Ok(table)
}

fn normalized_lines(lines: &[String]) -> Vec<String> {
    lines
        .iter()
        .map(|line| line.trim())
        .filter(|line| !line.is_empty())
        .filter(|line| !line.starts_with("<!--"))
        .map(|line| {
            line.strip_prefix("- ")
                .or_else(|| line.strip_prefix("* "))
                .unwrap_or(line)
                .trim()
                .to_owned()
        })
        .collect()
}

fn normalize_key(value: &str) -> String {
    value
        .trim()
        .trim_matches('`')
        .to_ascii_lowercase()
        .replace(['-', '_'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

fn normalize_scalar(value: &str) -> String {
    let value = value.trim();
    value
        .strip_prefix('`')
        .and_then(|inner| inner.strip_suffix('`'))
        .unwrap_or(value)
        .trim()
        .to_owned()
}

fn missing(name: &str) -> SourceParseError {
    SourceParseError::new(
        "source.missing_field",
        format!("material work-contract value `{name}` is missing"),
        "add the missing accepted value to the authoritative source",
    )
    .with_context(name)
}
