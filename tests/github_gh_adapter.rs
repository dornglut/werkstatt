#[cfg(unix)]
mod unix {
    use std::{fs, os::unix::fs::PermissionsExt, path::Path, time::Duration};

    use tempfile::tempdir;
    use werkstatt::{
        adapters::{AdapterErrorKind, GithubGhAdapter},
        domain::SourceKind,
        ports::GithubWorkSourceReader,
    };

    fn executable(path: &Path, body: &str) {
        fs::write(path, format!("#!/bin/sh\n{body}\n")).unwrap();
        let mut permissions = fs::metadata(path).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(path, permissions).unwrap();
    }

    fn issue_json() -> String {
        let body = "# W2B fixture\\n\\n## Work contract\\n\\n| Field | Value |\\n|---|---|\\n| Owner | dornglut/werkstatt |\\n| Work class | Product |\\n| Lifecycle stage | Implement |\\n| Accepted base | bd4f12f |\\n| Exit gate | green |\\n| Next transition | review |\\n\\n## Goal\\n\\nObserve.\\n\\n## Scope\\n\\n- Read only\\n\\n## Non-goals\\n\\n- No mutation\\n\\n## Validation\\n\\n- Validate\\n\\n## Stop conditions\\n\\n- Conflict\\n";
        format!(
            "{{\"number\":10,\"title\":\"W2B\",\"body\":\"{body}\",\"state\":\"OPEN\",\"updatedAt\":\"2026-07-29T10:00:00Z\",\"url\":\"https://github.com/dornglut/werkstatt/issues/10\"}}"
        )
    }

    #[test]
    fn observes_bounded_read_only_issue_json() {
        let directory = tempdir().unwrap();
        let fake = directory.path().join("gh-fake");
        let payload = issue_json();
        executable(&fake, &format!("printf '%s' '{payload}'"));
        let observed = GithubGhAdapter::with_executable(&fake)
            .observe_issue("https://github.com/dornglut/werkstatt/issues/10")
            .unwrap();
        assert_eq!(observed.observation().kind(), SourceKind::GithubIssue);
        assert!(observed.observation().is_mutable());
        assert_eq!(observed.payload(), payload.as_bytes());
    }

    #[test]
    fn reports_timeout_and_malformed_provider_output() {
        let directory = tempdir().unwrap();
        let slow = directory.path().join("gh-slow");
        executable(&slow, "sleep 2");
        let error = GithubGhAdapter::with_executable(&slow)
            .with_timeout(Duration::from_millis(50))
            .observe_issue("https://github.com/dornglut/werkstatt/issues/10")
            .unwrap_err();
        assert_eq!(error.kind(), AdapterErrorKind::Timeout);

        let malformed = directory.path().join("gh-malformed");
        executable(&malformed, "printf '%s' '{not-json}'");
        let error = GithubGhAdapter::with_executable(&malformed)
            .observe_issue("https://github.com/dornglut/werkstatt/issues/10")
            .unwrap_err();
        assert_eq!(error.kind(), AdapterErrorKind::MalformedOutput);
    }

    #[test]
    fn redacts_forwarded_human_paths_from_provider_diagnostics() {
        let Some(home) = std::env::var_os("HOME").and_then(|value| value.into_string().ok()) else {
            return;
        };
        let working_directory = std::env::current_dir().unwrap();
        let directory = tempdir().unwrap();
        let failing = directory.path().join("gh-private-diagnostic");
        executable(
            &failing,
            "printf '%s\\n' \"$HOME/.config/gh/hosts.yml\" \"$(pwd)/repo-state\" >&2\nexit 1",
        );

        let error = GithubGhAdapter::with_executable(&failing)
            .observe_issue("https://github.com/dornglut/werkstatt/issues/10")
            .unwrap_err();
        assert_eq!(error.kind(), AdapterErrorKind::Provider);
        let diagnostic = error.diagnostic().unwrap();
        assert!(!diagnostic.contains(&home));
        assert!(!diagnostic.contains(working_directory.to_str().unwrap()));
        assert!(diagnostic.contains("<private-environment>/.config/gh/hosts.yml"));
        assert!(diagnostic.contains("<repository>/repo-state"));
    }
}
