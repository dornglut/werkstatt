use std::collections::BTreeMap;

use thiserror::Error;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ErrorCode {
    InvalidTransition,
    TerminalExecution,
    EvidenceUnavailable,
    StorageConflict,
    StorageFormat,
    StorageBusy,
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct SafeContext(pub BTreeMap<String, String>);

#[derive(Debug, Error)]
#[error("{code:?} during {operation}: {message}; correction: {correction}")]
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
}
