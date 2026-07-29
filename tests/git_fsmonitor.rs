#[cfg(unix)]
mod unix {
    use std::{fs, os::unix::fs::PermissionsExt, path::Path, process::Command};

    use tempfile::tempdir;
    use werkstatt::{adapters::GitCliAdapter, ports::RepositoryReader};

    fn git(repository: &Path, args: &[&str]) {
        let output = Command::new("git")
            .args(args)
            .current_dir(repository)
            .env_clear()
            .env("PATH", std::env::var_os("PATH").unwrap_or_default())
            .env("GIT_CONFIG_NOSYSTEM", "1")
            .env("HOME", repository)
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "git {:?} failed: {}",
            args,
            String::from_utf8_lossy(&output.stderr)
        );
    }

    #[test]
    fn repository_fsmonitor_hook_is_never_executed() {
        let directory = tempdir().unwrap();
        git(directory.path(), &["init", "-b", "main"]);
        git(directory.path(), &["config", "user.name", "Werkstatt Test"]);
        git(
            directory.path(),
            &["config", "user.email", "werkstatt@example.invalid"],
        );
        fs::write(directory.path().join("tracked.txt"), "base\n").unwrap();
        git(directory.path(), &["add", "tracked.txt"]);
        git(directory.path(), &["commit", "-m", "base"]);

        let hook_directory = tempdir().unwrap();
        let marker = hook_directory.path().join("fsmonitor-executed");
        let hook = hook_directory.path().join("fsmonitor-hook");
        fs::write(
            &hook,
            format!("#!/bin/sh\n: > \"{}\"\nprintf '0\\n'\n", marker.display()),
        )
        .unwrap();
        let mut permissions = fs::metadata(&hook).unwrap().permissions();
        permissions.set_mode(0o755);
        fs::set_permissions(&hook, permissions).unwrap();
        git(
            directory.path(),
            &["config", "core.fsmonitor", hook.to_str().unwrap()],
        );

        let observation = GitCliAdapter::default().observe(directory.path()).unwrap();
        assert!(observation.changes().is_empty());
        assert!(
            !marker.exists(),
            "repository-local fsmonitor hook executed during read observation"
        );
    }
}
