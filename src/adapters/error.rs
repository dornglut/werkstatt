use std::{collections::BTreeMap, error::Error, fmt};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum AdapterErrorKind {
    InvalidInput,
    Unavailable,
    NotRepository,
    Timeout,
    OutputTooLarge,
    MalformedOutput,
    Provider,
    Io,
}

impl AdapterErrorKind {
    pub const fn code(self) -> &'static str {
        match self {
            Self::InvalidInput => "adapter.invalid_input",
            Self::Unavailable => "adapter.unavailable",
            Self::NotRepository => "repository.not_found",
            Self::Timeout => "adapter.timeout",
            Self::OutputTooLarge => "adapter.output_too_large",
            Self::MalformedOutput => "adapter.malformed_output",
            Self::Provider => "adapter.provider_failure",
            Self::Io => "adapter.io_failure",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AdapterError {
    kind: AdapterErrorKind,
    operation: &'static str,
    message: String,
    retryable: bool,
    correction: String,
    safe_context: BTreeMap<String, String>,
    diagnostic: Option<String>,
}

impl AdapterError {
    pub fn new(
        kind: AdapterErrorKind,
        operation: &'static str,
        message: impl Into<String>,
        retryable: bool,
        correction: impl Into<String>,
    ) -> Self {
        Self {
            kind,
            operation,
            message: message.into(),
            retryable,
            correction: correction.into(),
            safe_context: BTreeMap::new(),
            diagnostic: None,
        }
    }

    pub fn with_context(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.safe_context.insert(key.into(), value.into());
        self
    }

    pub(crate) fn with_diagnostic(mut self, diagnostic: impl Into<String>) -> Self {
        let diagnostic = diagnostic.into();
        if !diagnostic.trim().is_empty() {
            self.diagnostic = Some(diagnostic);
        }
        self
    }

    pub fn kind(&self) -> AdapterErrorKind {
        self.kind
    }

    pub fn code(&self) -> &'static str {
        self.kind.code()
    }

    pub fn operation(&self) -> &'static str {
        self.operation
    }

    pub fn message(&self) -> &str {
        &self.message
    }

    pub fn retryable(&self) -> bool {
        self.retryable
    }

    pub fn correction(&self) -> &str {
        &self.correction
    }

    pub fn safe_context(&self) -> &BTreeMap<String, String> {
        &self.safe_context
    }

    pub fn diagnostic(&self) -> Option<&str> {
        self.diagnostic.as_deref()
    }
}

impl fmt::Display for AdapterError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            formatter,
            "{} during {}: {}; correction: {}",
            self.code(),
            self.operation,
            self.message,
            self.correction
        )
    }
}

impl Error for AdapterError {}
