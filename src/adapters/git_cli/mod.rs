mod parse;

use parse::{parse_revision, parse_status, parse_worktrees};

use std::{ffi::OsString, path::Path, time::Duration};

use crate::{
    domain::{
        DiffObservation, GitOperation, HeadObservation, RemoteObservation, RepositoryIdentity,
        RepositoryObservation, RevisionRelation,
    },
    ports::RepositoryReader,
    support::{
        path::{
            path_from_output, path_key, redact_diagnostic, sanitize_remote, short_key, trim_ascii,
        },
        sha256::digest_hex,
    },
};

use super::{
    AdapterError, AdapterErrorKind,
    process::{ProcessOutput, ProcessSpec, run},
};

#[derive(Clone, Debug)]
pub struct GitCliAdapter {
    executable: OsString,
    timeout: Duration,
    max_output: usize,
}

impl Default for GitCliAdapter {
    fn default() -> Self {
        Self {
            executable: OsString::from("git"),
            timeout: Duration::from_secs(10),
            max_output: 256 * 1024,
        }
    }
}

impl GitCliAdapter {
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

    pub fn with_max_output(mut self, max_output: usize) -> Self {
        self.max_output = max_output;
        self
    }

    fn raw<I, S>(
        &self,
        repository: &Path,
        args: I,
        operation: &'static str,
    ) -> Result<ProcessOutput, AdapterError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let mut fixed_args = vec![OsString::from("-c"), OsString::from("core.fsmonitor=false")];
        fixed_args.extend(args.into_iter().map(Into::into));
        let spec = ProcessSpec::new(self.executable.clone(), repository)
            .args(fixed_args)
            .timeout(self.timeout)
            .max_output(self.max_output)
            .env("GIT_OPTIONAL_LOCKS", "0")
            .env("GIT_TERMINAL_PROMPT", "0")
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("GIT_PAGER", "cat");
        run(&spec, operation)
    }

    fn required<I, S>(
        &self,
        repository: &Path,
        args: I,
        operation: &'static str,
    ) -> Result<ProcessOutput, AdapterError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let output = self.raw(repository, args, operation)?;
        if output.timed_out {
            return Err(AdapterError::new(
                AdapterErrorKind::Timeout,
                operation,
                "Git observation timed out",
                true,
                "retry when the local repository and Git executable are responsive",
            ));
        }
        if output.truncated() {
            return Err(AdapterError::new(
                AdapterErrorKind::OutputTooLarge,
                operation,
                "Git observation exceeded the bounded output size",
                false,
                "reduce the observed diff or inspect it with normal Git tools",
            ));
        }
        if !output.success() {
            let diagnostic = redact_diagnostic(&output.stderr, repository);
            let kind = if diagnostic.contains("not a git repository") {
                AdapterErrorKind::NotRepository
            } else {
                AdapterErrorKind::Provider
            };
            return Err(AdapterError::new(
                kind,
                operation,
                "Git read observation did not complete successfully",
                true,
                "verify the repository and requested revision, then retry",
            )
            .with_diagnostic(diagnostic));
        }
        Ok(output)
    }

    fn bounded<I, S>(
        &self,
        repository: &Path,
        args: I,
        operation: &'static str,
    ) -> Result<ProcessOutput, AdapterError>
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        let output = self.raw(repository, args, operation)?;
        if output.timed_out {
            return Err(AdapterError::new(
                AdapterErrorKind::Timeout,
                operation,
                "Git observation timed out",
                true,
                "retry when the local repository and Git executable are responsive",
            ));
        }
        if !output.success() {
            return Err(AdapterError::new(
                AdapterErrorKind::Provider,
                operation,
                "Git read observation did not complete successfully",
                true,
                "verify repository state and retry",
            )
            .with_diagnostic(redact_diagnostic(&output.stderr, repository)));
        }
        Ok(output)
    }

    fn resolve_commit(&self, repository: &Path, revision: &str) -> Result<String, AdapterError> {
        if revision.trim().is_empty() {
            return Err(AdapterError::new(
                AdapterErrorKind::InvalidInput,
                "repository.revision.resolve",
                "revision is empty",
                false,
                "supply an accepted Git revision",
            ));
        }
        let expression = format!("{revision}^{{commit}}");
        let output = self.required(
            repository,
            ["rev-parse", "--verify", "--end-of-options", &expression],
            "repository.revision.resolve",
        )?;
        parse_revision(&output.stdout, "repository.revision.resolve")
    }

    fn remotes(&self, repository: &Path) -> Result<Vec<RemoteObservation>, AdapterError> {
        let names = self.required(repository, ["remote"], "repository.remotes")?;
        let mut remotes = Vec::new();
        for name in String::from_utf8(names.stdout)
            .map_err(|_| malformed("repository.remotes", "remote names are not UTF-8"))?
            .lines()
            .map(str::trim)
            .filter(|value| !value.is_empty())
        {
            let output = self.required(
                repository,
                ["remote", "get-url", name],
                "repository.remote_url",
            )?;
            let url = std::str::from_utf8(trim_ascii(&output.stdout)).map_err(|_| {
                malformed(
                    "repository.remote_url",
                    "remote URL could not be normalized safely",
                )
            })?;
            remotes.push(
                RemoteObservation::new(name, sanitize_remote(url)).map_err(domain_adapter_error)?,
            );
        }
        remotes.sort_by(|left, right| left.name().cmp(right.name()));
        Ok(remotes)
    }

    fn operation(&self, repository: &Path) -> Result<GitOperation, AdapterError> {
        let candidates = [
            (GitOperation::Merge, "MERGE_HEAD"),
            (GitOperation::Rebase, "rebase-merge"),
            (GitOperation::Rebase, "rebase-apply"),
            (GitOperation::CherryPick, "CHERRY_PICK_HEAD"),
            (GitOperation::Revert, "REVERT_HEAD"),
            (GitOperation::Bisect, "BISECT_LOG"),
        ];
        for (operation, marker) in candidates {
            let output = self.required(
                repository,
                ["rev-parse", "--git-path", marker],
                "repository.operation",
            )?;
            let path = path_from_output(&output.stdout, repository).ok_or_else(|| {
                malformed(
                    "repository.operation",
                    "Git operation path could not be represented without loss",
                )
            })?;
            if path.exists() {
                return Ok(operation);
            }
        }
        Ok(GitOperation::None)
    }
}

impl RepositoryReader for GitCliAdapter {
    type Error = AdapterError;

    fn observe(&self, repository: &Path) -> Result<RepositoryObservation, Self::Error> {
        let root_output = self.required(
            repository,
            ["rev-parse", "--show-toplevel"],
            "repository.identity",
        )?;
        let root = path_from_output(&root_output.stdout, repository).ok_or_else(|| {
            malformed(
                "repository.identity",
                "repository root could not be represented without loss",
            )
        })?;
        let common_output = self.required(
            &root,
            ["rev-parse", "--path-format=absolute", "--git-common-dir"],
            "repository.common_directory",
        )?;
        let common_directory = path_from_output(&common_output.stdout, &root).ok_or_else(|| {
            malformed(
                "repository.common_directory",
                "Git common-directory identity could not be represented without loss",
            )
        })?;
        let head = self.resolve_commit(&root, "HEAD")?;
        let branch_output = self.raw(
            &root,
            ["symbolic-ref", "--quiet", "--short", "HEAD"],
            "repository.branch",
        )?;
        let branch = if branch_output.success() {
            Some(
                std::str::from_utf8(trim_ascii(&branch_output.stdout))
                    .map_err(|_| malformed("repository.branch", "branch name is not UTF-8"))?
                    .to_owned(),
            )
        } else if branch_output.status_code == Some(1) && !branch_output.timed_out {
            None
        } else {
            return Err(AdapterError::new(
                AdapterErrorKind::Provider,
                "repository.branch",
                "Git branch observation failed",
                true,
                "verify repository state and retry",
            ));
        };

        let remotes = self.remotes(&root)?;
        let roots_output = self.required(
            &root,
            ["rev-list", "--max-parents=0", "--all"],
            "repository.roots",
        )?;
        let mut roots = String::from_utf8(roots_output.stdout)
            .map_err(|_| malformed("repository.roots", "root revisions are not UTF-8"))?
            .lines()
            .map(str::trim)
            .filter(|value| !value.is_empty())
            .map(ToOwned::to_owned)
            .collect::<Vec<_>>();
        roots.sort();
        if roots.is_empty() {
            roots.push(head.clone());
        }
        let mut heuristic_material = roots.join("\n");
        for remote in &remotes {
            heuristic_material.push('\n');
            heuristic_material.push_str(remote.name());
            heuristic_material.push('=');
            heuristic_material.push_str(remote.identity());
        }
        let heuristic_key = digest_hex(heuristic_material.as_bytes());
        let identity = RepositoryIdentity::new(
            heuristic_key.clone(),
            path_key(&common_directory),
            format!("repository:{}", short_key(&heuristic_key)),
        )
        .map_err(domain_adapter_error)?;

        let status = self.required(
            &root,
            ["status", "--porcelain=v2", "-z", "--untracked-files=all"],
            "repository.status",
        )?;
        let changes = parse_status(&status.stdout)?;
        let worktrees = self.required(
            &root,
            ["worktree", "list", "--porcelain", "-z"],
            "repository.worktrees",
        )?;
        let worktrees = parse_worktrees(&worktrees.stdout, &root)?;
        let operation = self.operation(&root)?;

        let staged_statistics = self.bounded(
            &root,
            [
                "diff",
                "--cached",
                "--no-ext-diff",
                "--no-textconv",
                "--stat",
                "--",
            ],
            "repository.diff_statistics.staged",
        )?;
        let unstaged_statistics = self.bounded(
            &root,
            ["diff", "--no-ext-diff", "--no-textconv", "--stat", "--"],
            "repository.diff_statistics.unstaged",
        )?;
        let staged_diff = self.bounded(
            &root,
            [
                "diff",
                "--cached",
                "--no-ext-diff",
                "--no-textconv",
                "--unified=3",
                "--",
            ],
            "repository.diff.staged",
        )?;
        let unstaged_diff = self.bounded(
            &root,
            [
                "diff",
                "--no-ext-diff",
                "--no-textconv",
                "--unified=3",
                "--",
            ],
            "repository.diff.unstaged",
        )?;
        let (statistics, statistics_truncated) = combine_labeled_sections(
            self.max_output,
            &[
                ("## staged", &staged_statistics),
                ("## unstaged", &unstaged_statistics),
            ],
        );
        let (text, text_truncated) = combine_labeled_sections(
            self.max_output,
            &[("## staged", &staged_diff), ("## unstaged", &unstaged_diff)],
        );
        let diff_observation =
            DiffObservation::new(statistics, text, statistics_truncated || text_truncated);

        Ok(RepositoryObservation::new(
            identity,
            HeadObservation::new(head, branch).map_err(domain_adapter_error)?,
            remotes,
            changes,
            worktrees,
            operation,
            diff_observation,
            vec![
                "registered checkout observation is not a process, filesystem, network, or credential sandbox".into(),
                "read-only observation cannot prevent concurrent external writers".into(),
                "repository observation_fingerprint is a heuristic derived from observed root revisions and credential-safe remotes; it may change when those observations change and is not a durable repository identifier".into(),
            ],
        ))
    }

    fn relation(
        &self,
        repository: &Path,
        base: &str,
        head: Option<&str>,
    ) -> Result<RevisionRelation, Self::Error> {
        let base = self.resolve_commit(repository, base)?;
        let head = self.resolve_commit(repository, head.unwrap_or("HEAD"))?;
        let ancestor = self.raw(
            repository,
            ["merge-base", "--is-ancestor", &base, &head],
            "repository.ancestry",
        )?;
        let is_ancestor = match (ancestor.status_code, ancestor.timed_out) {
            (Some(0), false) => true,
            (Some(1), false) => false,
            _ => {
                return Err(AdapterError::new(
                    AdapterErrorKind::Provider,
                    "repository.ancestry",
                    "Git ancestry check failed",
                    true,
                    "verify both revisions belong to the observed repository",
                ));
            }
        };
        let merge_base_output = self.raw(
            repository,
            ["merge-base", &base, &head],
            "repository.merge_base",
        )?;
        let merge_base = if merge_base_output.success() {
            Some(parse_revision(
                &merge_base_output.stdout,
                "repository.merge_base",
            )?)
        } else if merge_base_output.status_code == Some(1) && !merge_base_output.timed_out {
            None
        } else {
            return Err(AdapterError::new(
                AdapterErrorKind::Provider,
                "repository.merge_base",
                "Git merge-base observation failed",
                true,
                "verify both revisions belong to the observed repository",
            ));
        };
        RevisionRelation::new(base, head, is_ancestor, merge_base).map_err(domain_adapter_error)
    }
}

fn combine_labeled_sections(limit: usize, sections: &[(&str, &ProcessOutput)]) -> (String, bool) {
    let mut bytes = Vec::with_capacity(limit.min(8192));
    let mut truncated = sections.iter().any(|(_, output)| output.truncated());
    for (label, output) in sections {
        append_bounded(&mut bytes, label.as_bytes(), limit, &mut truncated);
        append_bounded(&mut bytes, b"\n", limit, &mut truncated);
        append_bounded(&mut bytes, &output.stdout, limit, &mut truncated);
        if !bytes.ends_with(b"\n") {
            append_bounded(&mut bytes, b"\n", limit, &mut truncated);
        }
        append_bounded(&mut bytes, b"\n", limit, &mut truncated);
    }
    (String::from_utf8_lossy(&bytes).into_owned(), truncated)
}

fn append_bounded(target: &mut Vec<u8>, source: &[u8], limit: usize, truncated: &mut bool) {
    let remaining = limit.saturating_sub(target.len());
    let keep = remaining.min(source.len());
    target.extend_from_slice(&source[..keep]);
    if keep < source.len() {
        *truncated = true;
    }
}

fn malformed(operation: &'static str, message: impl Into<String>) -> AdapterError {
    AdapterError::new(
        AdapterErrorKind::MalformedOutput,
        operation,
        message,
        false,
        "upgrade or repair the local Git installation, then retry",
    )
}

fn domain_adapter_error(error: crate::domain::DomainError) -> AdapterError {
    AdapterError::new(
        AdapterErrorKind::MalformedOutput,
        error.operation,
        error.message,
        false,
        error.correction,
    )
}
