use std::collections::{BTreeMap, BTreeSet};

use crate::{
    domain::{SourceIdentity, SourceKind, WorkFacts, WorkSourceObservation},
    support::{
        json::{JsonValue, escape_string, parse},
        path::canonical_github_issue_url,
        sha256::digest_hex,
    },
};

use super::{MAX_SOURCE_BYTES, SourceParseError};

pub(super) fn parse_work_source(
    bytes: &[u8],
    identity: SourceIdentity,
    observed_at_unix_seconds: u64,
    mutable: bool,
) -> Result<WorkSourceObservation, SourceParseError> {
    validate_size(bytes)?;
    let value = parse(bytes).map_err(|error| {
        SourceParseError::new(
            "source.malformed_json",
            error.message,
            "provide valid schema-version-1 JSON",
        )
        .with_context(error.offset.to_string())
    })?;
    let object = value.as_object().ok_or_else(|| {
        SourceParseError::new(
            "source.malformed_json",
            "work-source JSON root must be an object",
            "wrap the versioned work contract in one JSON object",
        )
    })?;
    let version = required(object, "schemaVersion")?
        .as_u64()
        .ok_or_else(|| wrong_type("schemaVersion", "integer"))?;
    if version != 1 {
        return Err(SourceParseError::new(
            "source.unsupported_schema",
            format!("work-source JSON schema version {version} is unsupported"),
            "export schemaVersion 1 or upgrade Werkstatt through an accepted change",
        ));
    }
    let facts = facts_from_object(object)?;
    let known = known_keys();
    let unknown = object
        .keys()
        .filter(|key| !known.contains(key.as_str()))
        .cloned()
        .collect::<Vec<_>>();
    let limitations = if unknown.is_empty() {
        Vec::new()
    } else {
        vec![format!(
            "ignored additive JSON fields: {}",
            unknown.join(", ")
        )]
    };
    WorkSourceObservation::new(
        identity,
        SourceKind::Json,
        digest_hex(bytes),
        observed_at_unix_seconds,
        mutable,
        facts,
        limitations,
        bytes.len(),
    )
    .map_err(domain_error)
}

pub(super) fn parse_github_issue(
    bytes: &[u8],
    observed_at_unix_seconds: u64,
) -> Result<WorkSourceObservation, SourceParseError> {
    validate_size(bytes)?;
    let value = parse(bytes).map_err(|error| {
        SourceParseError::new(
            "github.malformed_json",
            error.message,
            "retry `gh issue view` or use the exported-file fallback",
        )
        .with_context(error.offset.to_string())
    })?;
    let object = value.as_object().ok_or_else(|| {
        SourceParseError::new(
            "github.malformed_json",
            "GitHub issue response root must be an object",
            "retry the bounded read-only `gh issue view` operation",
        )
    })?;
    let number = required(object, "number")?
        .as_u64()
        .ok_or_else(|| wrong_type("number", "integer"))?;
    let title = required_string(object, "title")?;
    let body = required_string(object, "body")?;
    let url = required_string(object, "url")?;
    let updated_at = required_string(object, "updatedAt")?;
    let state = required_string(object, "state")?;
    let (canonical_url, opaque_id) = canonical_github_issue_url(url).ok_or_else(|| {
        SourceParseError::new(
            "github.unsupported_url",
            "provider returned an unsupported GitHub issue URL",
            "use a canonical public github.com issue URL or an exported local file",
        )
    })?;
    if !opaque_id.ends_with(&format!("#{number}")) {
        return Err(SourceParseError::new(
            "github.identity_mismatch",
            "issue number and canonical URL disagree",
            "refresh the issue observation before deriving work facts",
        ));
    }

    let body_without_title = body
        .strip_prefix("# ")
        .and_then(|value| value.split_once('\n').map(|(_, remainder)| remainder))
        .unwrap_or(body);
    let synthetic = format!("# {title}\n\n{body_without_title}");
    let identity = SourceIdentity::new("github", "issue", opaque_id, Some(canonical_url))
        .map_err(domain_error)?;
    let mut observation = super::markdown::parse(
        synthetic.as_bytes(),
        identity,
        observed_at_unix_seconds,
        true,
    )?;
    let limitations = vec![
        format!(
            "mutable GitHub issue observation at provider update `{updated_at}`; not an immutable revision"
        ),
        format!("provider-reported issue state `{state}`"),
        "GitHub authentication and network availability remain human-owned; exported Markdown or JSON is the fallback".into(),
    ];
    observation = WorkSourceObservation::new(
        observation.identity().clone(),
        SourceKind::GithubIssue,
        digest_hex(bytes),
        observed_at_unix_seconds,
        true,
        observation.facts().clone(),
        limitations,
        bytes.len(),
    )
    .map_err(domain_error)?;
    Ok(observation)
}

pub(super) fn serialize(observation: &WorkSourceObservation) -> String {
    let facts = observation.facts();
    let mut fields = Vec::new();
    fields.push("\"schemaVersion\":1".to_owned());
    fields.push(format!("\"title\":{}", escape_string(facts.title())));
    fields.push(format!("\"goal\":{}", escape_string(facts.goal())));
    fields.push(format!("\"owner\":{}", escape_string(facts.owner())));
    fields.push(format!(
        "\"workClass\":{}",
        escape_string(facts.work_class())
    ));
    fields.push(format!(
        "\"lifecycleStage\":{}",
        escape_string(facts.lifecycle_stage())
    ));
    fields.push(format!(
        "\"acceptedBase\":{}",
        escape_string(facts.accepted_base())
    ));
    fields.push(format!("\"scope\":{}", string_array(facts.scope())));
    fields.push(format!("\"nonGoals\":{}", string_array(facts.non_goals())));
    fields.push(format!(
        "\"validation\":{}",
        string_array(facts.validation())
    ));
    fields.push(format!(
        "\"stopConditions\":{}",
        string_array(facts.stop_conditions())
    ));
    fields.push(format!("\"exitGate\":{}", escape_string(facts.exit_gate())));
    fields.push(format!(
        "\"nextTransition\":{}",
        escape_string(facts.next_transition())
    ));
    if !facts.supplemental().is_empty() {
        fields.push(format!(
            "\"supplemental\":{}",
            supplemental_object(facts.supplemental())
        ));
    }
    format!("{{{}}}", fields.join(","))
}

fn facts_from_object(object: &BTreeMap<String, JsonValue>) -> Result<WorkFacts, SourceParseError> {
    let supplemental = object
        .get("supplemental")
        .map(parse_supplemental)
        .transpose()?
        .unwrap_or_default();
    WorkFacts::new(
        required_string(object, "title")?,
        required_string(object, "goal")?,
        required_string(object, "owner")?,
        required_string(object, "workClass")?,
        required_string(object, "lifecycleStage")?,
        required_string(object, "acceptedBase")?,
        required_string_array(object, "scope")?,
        required_string_array(object, "nonGoals")?,
        required_string_array(object, "validation")?,
        required_string_array(object, "stopConditions")?,
        required_string(object, "exitGate")?,
        required_string(object, "nextTransition")?,
        supplemental,
    )
    .map_err(domain_error)
}

fn parse_supplemental(
    value: &JsonValue,
) -> Result<BTreeMap<String, Vec<String>>, SourceParseError> {
    let object = value
        .as_object()
        .ok_or_else(|| wrong_type("supplemental", "object"))?;
    object
        .iter()
        .map(|(key, value)| parse_string_array(value, key).map(|values| (key.clone(), values)))
        .collect()
}

fn required<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<&'a JsonValue, SourceParseError> {
    object.get(key).ok_or_else(|| {
        SourceParseError::new(
            "source.missing_field",
            format!("material JSON field `{key}` is missing"),
            "add the missing accepted value to the authoritative source",
        )
        .with_context(key)
    })
}

fn required_string<'a>(
    object: &'a BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<&'a str, SourceParseError> {
    let value = required(object, key)?
        .as_str()
        .ok_or_else(|| wrong_type(key, "string"))?;
    if value.trim().is_empty() {
        return Err(SourceParseError::new(
            "source.empty_field",
            format!("material JSON field `{key}` is empty"),
            "supply one accepted non-empty value",
        ));
    }
    Ok(value)
}

fn required_string_array(
    object: &BTreeMap<String, JsonValue>,
    key: &str,
) -> Result<Vec<String>, SourceParseError> {
    parse_string_array(required(object, key)?, key)
}

fn parse_string_array(value: &JsonValue, key: &str) -> Result<Vec<String>, SourceParseError> {
    let values = value
        .as_array()
        .ok_or_else(|| wrong_type(key, "array of strings"))?;
    let parsed = values
        .iter()
        .map(|value| {
            value
                .as_str()
                .filter(|value| !value.trim().is_empty())
                .map(ToOwned::to_owned)
                .ok_or_else(|| wrong_type(key, "non-empty string array"))
        })
        .collect::<Result<Vec<_>, _>>()?;
    if parsed.is_empty() {
        return Err(SourceParseError::new(
            "source.empty_field",
            format!("material JSON array `{key}` is empty"),
            "supply at least one accepted value",
        ));
    }
    Ok(parsed)
}

fn wrong_type(key: &str, expected: &str) -> SourceParseError {
    SourceParseError::new(
        "source.wrong_type",
        format!("JSON field `{key}` must be {expected}"),
        "export the accepted schema-version-1 work source",
    )
    .with_context(key)
}

fn validate_size(bytes: &[u8]) -> Result<(), SourceParseError> {
    if bytes.len() > MAX_SOURCE_BYTES {
        Err(SourceParseError::new(
            "source.too_large",
            "work source exceeds the bounded input size",
            "reduce or export the authoritative source below 131072 bytes",
        ))
    } else {
        Ok(())
    }
}

fn domain_error(error: crate::domain::DomainError) -> SourceParseError {
    SourceParseError::new("source.invalid_contract", error.message, error.correction)
}

fn known_keys() -> BTreeSet<&'static str> {
    [
        "schemaVersion",
        "title",
        "goal",
        "owner",
        "workClass",
        "lifecycleStage",
        "acceptedBase",
        "scope",
        "nonGoals",
        "validation",
        "stopConditions",
        "exitGate",
        "nextTransition",
        "supplemental",
    ]
    .into_iter()
    .collect()
}

fn supplemental_object(values: &BTreeMap<String, Vec<String>>) -> String {
    format!(
        "{{{}}}",
        values
            .iter()
            .map(|(key, values)| format!("{}:{}", escape_string(key), string_array(values)))
            .collect::<Vec<_>>()
            .join(",")
    )
}

fn string_array(values: &[String]) -> String {
    format!(
        "[{}]",
        values
            .iter()
            .map(|value| escape_string(value))
            .collect::<Vec<_>>()
            .join(",")
    )
}
