use std::{fs, path::Path, process::Command};

use tempfile::tempdir;
use werkstatt::{
    adapters::{AdapterErrorKind, GitCliAdapter},
    domain::{GitOperation, PathChangeKind},
    ports::RepositoryReader,
};

fn git(repository: &Path, args: &[&str]) -> std::process::Output {
    Command::new("git")
        .args(args)
        .current_dir(repository)
        .env_clear()
        .env("PATH", std::env::var_os("PATH").unwrap_or_default())
        .env("GIT_CONFIG_NOSYSTEM", "1")
        .env("HOME", repository)
        .output()
        .unwrap()
}

fn git_ok(repository: &Path, args: &[&str]) -> String {
    let output = git(repository, args);
    assert!(
        output.status.success(),
        "git {:?} failed: {}",
        args,
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).unwrap().trim().to_owned()
}

fn repository() -> tempfile::TempDir {
    let directory = tempdir().unwrap();
    git_ok(directory.path(), &["init", "-b", "main"]);
    git_ok(directory.path(), &["config", "user.name", "Werkstatt Test"]);
    git_ok(
        directory.path(),
        &["config", "user.email", "werkstatt@example.invalid"],
    );
    fs::write(directory.path().join("tracked.txt"), "base\n").unwrap();
    git_ok(directory.path(), &["add", "tracked.txt"]);
    git_ok(directory.path(), &["commit", "-m", "base"]);
    directory
}

#[test]
fn observes_identity_remote_head_and_clean_state_without_private_paths() {
    let directory = repository();
    git_ok(
        directory.path(),
        &[
            "remote",
            "add",
            "origin",
            "https://token@example.com/dornglut/werkstatt.git?secret=yes",
        ],
    );
    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_eq!(observation.head().branch(), Some("main"));
    assert!(!observation.head().is_detached());
    assert!(observation.changes().is_empty());
    assert_eq!(observation.identity().observation_fingerprint().len(), 64);
    assert_eq!(
        observation.remotes()[0].identity(),
        "https://example.com/dornglut/werkstatt.git"
    );
    assert!(
        !observation
            .identity()
            .display()
            .contains(directory.path().to_str().unwrap())
    );
    assert!(
        observation
            .limitations()
            .iter()
            .any(|value| value.contains("not a process"))
    );
    assert!(observation.limitations().iter().any(|value| {
        value.contains("observation_fingerprint") && value.contains("not a durable")
    }));
}

#[test]
fn observes_modified_untracked_renamed_and_literal_metacharacter_paths() {
    let directory = repository();
    fs::write(directory.path().join("tracked.txt"), "changed\n").unwrap();
    fs::write(directory.path().join("odd;$(echo no).txt"), "literal\n").unwrap();
    git_ok(directory.path(), &["mv", "tracked.txt", "renamed.txt"]);

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert!(
        observation
            .changes()
            .iter()
            .any(|change| change.kind() == PathChangeKind::Renamed)
    );
    assert!(observation.changes().iter().any(|change| {
        change.kind() == PathChangeKind::Untracked && change.display() == "odd;$(echo no).txt"
    }));
    assert!(!directory.path().join("no").exists());
}

#[test]
fn retains_staged_and_unstaged_diff_sections_for_staged_only_changes() {
    let directory = repository();
    fs::write(directory.path().join("tracked.txt"), "staged\n").unwrap();
    git_ok(directory.path(), &["add", "tracked.txt"]);

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert!(
        observation
            .changes()
            .iter()
            .any(|change| change.kind() == PathChangeKind::Modified)
    );
    assert!(observation.diff().statistics().contains("## staged"));
    assert!(observation.diff().statistics().contains("tracked.txt"));
    assert!(observation.diff().statistics().contains("## unstaged"));
    assert!(observation.diff().text().contains("## staged"));
    assert!(observation.diff().text().contains("+staged"));
    assert!(observation.diff().text().contains("## unstaged"));
    assert!(!observation.diff().is_truncated());
}

#[test]
fn observes_revision_relation_detached_head_and_linked_worktree() {
    let directory = repository();
    let base = git_ok(directory.path(), &["rev-parse", "HEAD"]);
    fs::write(directory.path().join("second.txt"), "second\n").unwrap();
    git_ok(directory.path(), &["add", "second.txt"]);
    git_ok(directory.path(), &["commit", "-m", "second"]);
    let head = git_ok(directory.path(), &["rev-parse", "HEAD"]);

    let relation = GitCliAdapter::default()
        .relation(directory.path(), &base, Some(&head))
        .unwrap();
    assert!(relation.is_ancestor());
    assert_eq!(relation.merge_base(), Some(base.as_str()));

    let worktree_parent = tempdir().unwrap();
    let worktree = worktree_parent.path().join("checkout");
    git_ok(
        directory.path(),
        &[
            "worktree",
            "add",
            "--detach",
            worktree.to_str().unwrap(),
            &base,
        ],
    );
    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert!(observation.worktrees().len() >= 2);

    git_ok(directory.path(), &["checkout", "--detach", &head]);
    let detached = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert!(detached.head().is_detached());
}

#[test]
fn observes_wrong_base_and_head_movement() {
    let directory = repository();
    let base = git_ok(directory.path(), &["rev-parse", "HEAD"]);

    git_ok(directory.path(), &["checkout", "-b", "other"]);
    fs::write(directory.path().join("other.txt"), "other\n").unwrap();
    git_ok(directory.path(), &["add", "other.txt"]);
    git_ok(directory.path(), &["commit", "-m", "other"]);
    let other = git_ok(directory.path(), &["rev-parse", "HEAD"]);

    git_ok(directory.path(), &["checkout", "main"]);
    fs::write(directory.path().join("main.txt"), "main\n").unwrap();
    git_ok(directory.path(), &["add", "main.txt"]);
    git_ok(directory.path(), &["commit", "-m", "main"]);
    let first = GitCliAdapter::default().observe(directory.path()).unwrap();

    let relation = GitCliAdapter::default()
        .relation(directory.path(), &other, Some(first.head().revision()))
        .unwrap();
    assert!(!relation.is_ancestor());
    assert_eq!(relation.merge_base(), Some(base.as_str()));

    fs::write(directory.path().join("later.txt"), "later\n").unwrap();
    git_ok(directory.path(), &["add", "later.txt"]);
    git_ok(directory.path(), &["commit", "-m", "later"]);
    let second = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_ne!(first.head().revision(), second.head().revision());
}

#[test]
fn detects_active_merge_and_disables_external_diff() {
    let directory = repository();
    git_ok(directory.path(), &["checkout", "-b", "other"]);
    fs::write(directory.path().join("tracked.txt"), "other\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "other"]);
    git_ok(directory.path(), &["checkout", "main"]);
    fs::write(directory.path().join("tracked.txt"), "main\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "main"]);
    git_ok(
        directory.path(),
        &["config", "diff.external", "/definitely/not/available"],
    );
    let merge = git(directory.path(), &["merge", "other"]);
    assert!(!merge.status.success());

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_eq!(observation.operation(), GitOperation::Merge);
    assert!(
        observation
            .changes()
            .iter()
            .any(|change| change.kind() == PathChangeKind::Unmerged)
    );
}

#[test]
fn detects_active_rebase() {
    let directory = repository();
    git_ok(directory.path(), &["checkout", "-b", "feature"]);
    fs::write(directory.path().join("tracked.txt"), "feature\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "feature"]);
    git_ok(directory.path(), &["checkout", "main"]);
    fs::write(directory.path().join("tracked.txt"), "main\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "main"]);
    git_ok(directory.path(), &["checkout", "feature"]);
    let rebase = git(directory.path(), &["rebase", "main"]);
    assert!(!rebase.status.success());

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_eq!(observation.operation(), GitOperation::Rebase);
}

#[test]
fn detects_active_cherry_pick() {
    let directory = repository();
    git_ok(directory.path(), &["checkout", "-b", "source"]);
    fs::write(directory.path().join("tracked.txt"), "source\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "source"]);
    let source = git_ok(directory.path(), &["rev-parse", "HEAD"]);
    git_ok(directory.path(), &["checkout", "main"]);
    fs::write(directory.path().join("tracked.txt"), "main\n").unwrap();
    git_ok(directory.path(), &["commit", "-am", "main"]);
    let cherry_pick = git(directory.path(), &["cherry-pick", &source]);
    assert!(!cherry_pick.status.success());

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_eq!(observation.operation(), GitOperation::CherryPick);
}

#[test]
fn detects_active_bisect() {
    let directory = repository();
    let base = git_ok(directory.path(), &["rev-parse", "HEAD"]);
    for index in 1..=5 {
        let name = format!("commit-{index}.txt");
        fs::write(directory.path().join(&name), format!("{index}\n")).unwrap();
        git_ok(directory.path(), &["add", &name]);
        git_ok(
            directory.path(),
            &["commit", "-m", &format!("commit {index}")],
        );
    }
    let head = git_ok(directory.path(), &["rev-parse", "HEAD"]);
    git_ok(directory.path(), &["bisect", "start"]);
    git_ok(directory.path(), &["bisect", "bad", &head]);
    git_ok(directory.path(), &["bisect", "good", &base]);

    let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
    assert_eq!(observation.operation(), GitOperation::Bisect);
}

#[test]
fn distinguishes_unavailable_git_and_non_repository_without_path_leak() {
    let directory = tempdir().unwrap();
    let unavailable = GitCliAdapter::with_executable("werkstatt-git-does-not-exist")
        .observe(directory.path())
        .unwrap_err();
    assert_eq!(unavailable.kind(), AdapterErrorKind::Unavailable);

    let not_repository = GitCliAdapter::default()
        .observe(directory.path())
        .unwrap_err();
    assert_eq!(not_repository.kind(), AdapterErrorKind::NotRepository);
    assert!(
        !not_repository
            .to_string()
            .contains(directory.path().to_str().unwrap())
    );
}
