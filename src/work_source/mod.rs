mod json;
mod markdown;

use std::{error::Error, fmt};

use crate::{
    domain::{SourceIdentity, WorkSourceObservation},
    support::sha256::digest_hex,
};

pub const MAX_SOURCE_BYTES: usize = 131_072;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ObservedWorkSource {
    observation: WorkSourceObservation,
    payload: Vec<u8>,
}

impl ObservedWorkSource {
    fn new(observation: WorkSourceObservation, payload: Vec<u8>) -> Result<Self, SourceParseError> {
        if payload.len() > MAX_SOURCE_BYTES {
            return Err(SourceParseError::new(
                "source.too_large",
                "work-source payload exceeds the bounded input size",
                "reduce or export the authoritative source below 131072 bytes",
            ));
        }
        if payload.len() != observation.raw_size() || digest_hex(&payload) != observation.digest() {
            return Err(SourceParseError::new(
                "source.payload_mismatch",
                "work-source observation does not match its exact bounded payload",
                "re-observe the authoritative source in one bounded read",
            ));
        }
        Ok(Self {
            observation,
            payload,
        })
    }

    pub fn observation(&self) -> &WorkSourceObservation {
        &self.observation
    }

    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    pub fn into_parts(self) -> (WorkSourceObservation, Vec<u8>) {
        (self.observation, self.payload)
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SourceParseError {
    code: &'static str,
    message: String,
    correction: String,
    context: Option<String>,
}

impl SourceParseError {
    pub(crate) fn new(
        code: &'static str,
        message: impl Into<String>,
        correction: impl Into<String>,
    ) -> Self {
        Self {
            code,
            message: message.into(),
            correction: correction.into(),
            context: None,
        }
    }

    pub(crate) fn with_context(mut self, context: impl Into<String>) -> Self {
        self.context = Some(context.into());
        self
    }

    pub fn code(&self) -> &'static str {
        self.code
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn correction(&self) -> &str {
        &self.correction
    }

    pub fn context(&self) -> Option<&str> {
        self.context.as_deref()
    }
}

impl fmt::Display for SourceParseError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{}: {}; correction: {}",
            self.code, self.message, self.correction
        )
    }
}

impl Error for SourceParseError {}

pub fn parse_markdown(
    bytes: &[u8],
    identity: SourceIdentity,
    observed_at_unix_seconds: u64,
    mutable: bool,
) -> Result<ObservedWorkSource, SourceParseError> {
    let observation = markdown::parse(bytes, identity, observed_at_unix_seconds, mutable)?;
    ObservedWorkSource::new(observation, bytes.to_vec())
}

pub fn parse_json(
    bytes: &[u8],
    identity: SourceIdentity,
    observed_at_unix_seconds: u64,
    mutable: bool,
) -> Result<ObservedWorkSource, SourceParseError> {
    let observation = json::parse_work_source(bytes, identity, observed_at_unix_seconds, mutable)?;
    ObservedWorkSource::new(observation, bytes.to_vec())
}

pub fn parse_github_issue_payload(
    bytes: &[u8],
    observed_at_unix_seconds: u64,
) -> Result<ObservedWorkSource, SourceParseError> {
    let observation = json::parse_github_issue(bytes, observed_at_unix_seconds)?;
    ObservedWorkSource::new(observation, bytes.to_vec())
}

pub fn to_versioned_json(observation: &WorkSourceObservation) -> String {
    json::serialize(observation)
}
