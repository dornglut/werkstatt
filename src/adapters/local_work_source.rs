use std::{
    fs,
    path::Path,
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    domain::SourceIdentity,
    ports::LocalWorkSourceReader,
    support::path::path_key,
    work_source::{MAX_SOURCE_BYTES, ObservedWorkSource, parse_json, parse_markdown},
};

use super::{AdapterError, AdapterErrorKind};

#[derive(Clone, Copy, Debug, Default)]
pub struct LocalWorkSourceAdapter;

impl LocalWorkSourceReader for LocalWorkSourceAdapter {
    type Error = AdapterError;

    fn observe_file(&self, source: &Path) -> Result<ObservedWorkSource, Self::Error> {
        let canonical = fs::canonicalize(source).map_err(|_| {
            AdapterError::new(
                AdapterErrorKind::Unavailable,
                "work_source.read",
                "local work source could not be resolved",
                false,
                "select an existing readable Markdown or JSON work source",
            )
        })?;
        let metadata = fs::metadata(&canonical).map_err(|_| {
            AdapterError::new(
                AdapterErrorKind::Unavailable,
                "work_source.read",
                "local work source could not be inspected",
                false,
                "select an existing readable Markdown or JSON work source",
            )
        })?;
        if !metadata.is_file() {
            return Err(AdapterError::new(
                AdapterErrorKind::InvalidInput,
                "work_source.read",
                "local work source is not a regular file",
                false,
                "select one bounded Markdown or JSON file",
            ));
        }
        if metadata.len() > MAX_SOURCE_BYTES as u64 {
            return Err(AdapterError::new(
                AdapterErrorKind::OutputTooLarge,
                "work_source.read",
                "local work source exceeds the bounded input size",
                false,
                "reduce or export the source below 131072 bytes",
            ));
        }
        let bytes = fs::read(&canonical).map_err(|_| {
            AdapterError::new(
                AdapterErrorKind::Io,
                "work_source.read",
                "local work source could not be read",
                true,
                "verify file permissions and retry",
            )
        })?;
        let identity =
            SourceIdentity::new("local_file", "work_document", path_key(&canonical), None)
                .map_err(|error| {
                    AdapterError::new(
                        AdapterErrorKind::InvalidInput,
                        "work_source.identity",
                        error.message,
                        false,
                        error.correction,
                    )
                })?;
        let observed_at = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();
        let is_json = canonical
            .extension()
            .and_then(|value| value.to_str())
            .is_some_and(|value| value.eq_ignore_ascii_case("json"))
            || bytes.iter().find(|byte| !byte.is_ascii_whitespace()) == Some(&b'{');
        let result = if is_json {
            parse_json(&bytes, identity, observed_at, false)
        } else {
            parse_markdown(&bytes, identity, observed_at, false)
        };
        result.map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::MalformedOutput,
                "work_source.normalize",
                error.message(),
                false,
                error.correction(),
            )
            .with_context("sourceError", error.code())
        })
    }
}
