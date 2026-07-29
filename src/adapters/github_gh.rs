use std::{ffi::OsString, path::PathBuf, time::Duration};

use crate::{
    ports::GithubWorkSourceReader,
    support::path::{canonical_github_issue_url, redact_diagnostic},
    work_source::{MAX_SOURCE_BYTES, ObservedWorkSource, parse_github_issue_payload},
};

use super::{
    AdapterError, AdapterErrorKind,
    process::{ProcessSpec, run},
};

const ISSUE_FIELDS: &str = "number,title,body,state,updatedAt,url";
const HUMAN_CONFIG_ENVIRONMENT: &[&str] = &[
    "HOME",
    "XDG_CONFIG_HOME",
    "GH_CONFIG_DIR",
    "APPDATA",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "NO_PROXY",
    "SSL_CERT_FILE",
    "SSL_CERT_DIR",
];

#[derive(Clone, Debug)]
pub struct GithubGhAdapter {
    executable: OsString,
    timeout: Duration,
}

impl Default for GithubGhAdapter {
    fn default() -> Self {
        Self {
            executable: OsString::from("gh"),
            timeout: Duration::from_secs(15),
        }
    }
}

impl GithubGhAdapter {
    pub fn with_executable(executable: impl Into<OsString>) -> Self {
        Self {
            executable: executable.into(),
            ..Self::default()
        }
    }

    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    fn issue_view_args(issue_url: &str) -> Result<Vec<OsString>, AdapterError> {
        let (canonical, _) = canonical_github_issue_url(issue_url).ok_or_else(|| {
            AdapterError::new(
                AdapterErrorKind::InvalidInput,
                "github.issue.observe",
                "GitHub issue URL is unsupported",
                false,
                "use a canonical `https://github.com/<owner>/<repo>/issues/<number>` URL or a local export",
            )
        })?;
        Ok(vec![
            OsString::from("issue"),
            OsString::from("view"),
            OsString::from(canonical),
            OsString::from("--json"),
            OsString::from(ISSUE_FIELDS),
        ])
    }
}

impl GithubWorkSourceReader for GithubGhAdapter {
    type Error = AdapterError;

    fn observe_issue(&self, issue_url: &str) -> Result<ObservedWorkSource, Self::Error> {
        let args = Self::issue_view_args(issue_url)?;
        let cwd = PathBuf::from(".");
        let mut spec = ProcessSpec::new(self.executable.clone(), &cwd)
            .args(args)
            .timeout(self.timeout)
            .max_output(MAX_SOURCE_BYTES)
            .env("GH_PROMPT_DISABLED", "1")
            .env("GH_NO_UPDATE_NOTIFIER", "1")
            .env("GH_NO_EXTENSION_UPDATE_NOTIFIER", "1")
            .env("NO_COLOR", "1");
        for key in HUMAN_CONFIG_ENVIRONMENT {
            if let Some(value) = std::env::var_os(key) {
                spec = spec.env(key, value);
            }
        }
        let output = run(&spec, "github.issue.observe")?;
        if output.timed_out {
            return Err(AdapterError::new(
                AdapterErrorKind::Timeout,
                "github.issue.observe",
                "read-only GitHub issue observation timed out",
                true,
                "retry when `gh` and the network are responsive or use an exported local file",
            ));
        }
        if output.truncated() {
            return Err(AdapterError::new(
                AdapterErrorKind::OutputTooLarge,
                "github.issue.observe",
                "GitHub issue response exceeded the bounded output size",
                false,
                "use a bounded exported Markdown or JSON work source",
            ));
        }
        if !output.success() {
            return Err(AdapterError::new(
                AdapterErrorKind::Provider,
                "github.issue.observe",
                "read-only `gh issue view` did not complete successfully",
                true,
                "verify existing `gh` authentication and network access, then retry or use a local export",
            )
            .with_diagnostic(redact_diagnostic(&output.stderr, &cwd)));
        }
        parse_github_issue_payload(
            &output.stdout,
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        )
        .map_err(|error| {
            AdapterError::new(
                AdapterErrorKind::MalformedOutput,
                "github.issue.normalize",
                error.message(),
                false,
                error.correction(),
            )
            .with_context("sourceError", error.code())
        })
    }
}

#[cfg(test)]
mod tests {
    use super::{GithubGhAdapter, HUMAN_CONFIG_ENVIRONMENT, ISSUE_FIELDS};

    #[test]
    fn argv_is_fixed_and_read_only() {
        let args =
            GithubGhAdapter::issue_view_args("https://github.com/dornglut/werkstatt/issues/10")
                .unwrap();
        let rendered = args
            .iter()
            .map(|value| value.to_string_lossy().into_owned())
            .collect::<Vec<_>>();
        assert_eq!(
            rendered,
            [
                "issue",
                "view",
                "https://github.com/dornglut/werkstatt/issues/10",
                "--json",
                ISSUE_FIELDS,
            ]
        );
        assert!(!rendered.iter().any(|value| {
            matches!(
                value.as_str(),
                "create" | "edit" | "close" | "reopen" | "comment"
            )
        }));
    }

    #[test]
    fn human_config_environment_is_portable_and_excludes_tokens() {
        assert!(HUMAN_CONFIG_ENVIRONMENT.contains(&"APPDATA"));
        assert!(HUMAN_CONFIG_ENVIRONMENT.contains(&"HOME"));
        assert!(
            !HUMAN_CONFIG_ENVIRONMENT
                .iter()
                .any(|key| key.contains("TOKEN"))
        );
    }

    #[test]
    fn rejects_credentials_and_non_issue_urls() {
        assert!(GithubGhAdapter::issue_view_args("https://token@github.com/a/b/issues/1").is_err());
        assert!(GithubGhAdapter::issue_view_args("https://github.com/a/b/pull/1").is_err());
    }
}
