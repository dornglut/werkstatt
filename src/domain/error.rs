use std::{collections::BTreeMap, fmt};

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    InvalidAuthority,
    InvalidContract,
    InvalidObservation,
    InvalidSource,
    InvalidTransition,
    TerminalExecution,
    EvidenceUnavailable,
}

impl ErrorCode {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::InvalidAuthority => "authority.invalid",
            Self::InvalidContract => "contract.invalid",
            Self::InvalidObservation => "observation.invalid",
            Self::InvalidSource => "source.invalid",
            Self::InvalidTransition => "transition.invalid",
            Self::TerminalExecution => "execution.terminal",
            Self::EvidenceUnavailable => "evidence.unavailable",
        }
    }
}

impl fmt::Display for ErrorCode {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SafeContext(BTreeMap<String, String>);

impl SafeContext {
    pub fn insert(&mut self, key: impl Into<String>, value: impl Into<String>) {
        self.0.insert(key.into(), value.into());
    }

    pub fn get(&self, key: &str) -> Option<&str> {
        self.0.get(key).map(String::as_str)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

#[derive(Debug, Error)]
#[error("{code} during {operation}: {message}; correction: {correction}")]
pub struct DomainError {
    pub code: ErrorCode,
    pub operation: &'static str,
    pub message: String,
    pub retryable: bool,
    pub correction: String,
    pub context: SafeContext,
}

impl DomainError {
    pub fn new(
        code: ErrorCode,
        operation: &'static str,
        message: impl Into<String>,
        retryable: bool,
        correction: impl Into<String>,
    ) -> Self {
        Self {
            code,
            operation,
            message: message.into(),
            retryable,
            correction: correction.into(),
            context: SafeContext::default(),
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.context.insert(key, value);
        self
    }
}
