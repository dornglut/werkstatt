use std::{
    ffi::OsString,
    io::{self, Read},
    path::PathBuf,
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};

use super::{AdapterError, AdapterErrorKind};

#[derive(Clone, Debug)]
pub(crate) struct ProcessSpec {
    executable: OsString,
    args: Vec<OsString>,
    cwd: PathBuf,
    timeout: Duration,
    max_output: usize,
    environment: Vec<(OsString, OsString)>,
}

impl ProcessSpec {
    pub(crate) fn new(executable: impl Into<OsString>, cwd: impl Into<PathBuf>) -> Self {
        Self {
            executable: executable.into(),
            args: Vec::new(),
            cwd: cwd.into(),
            timeout: Duration::from_secs(10),
            max_output: 256 * 1024,
            environment: Vec::new(),
        }
    }

    pub(crate) fn args<I, S>(mut self, args: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<OsString>,
    {
        self.args = args.into_iter().map(Into::into).collect();
        self
    }

    pub(crate) fn timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    pub(crate) fn max_output(mut self, max_output: usize) -> Self {
        self.max_output = max_output;
        self
    }

    pub(crate) fn env(mut self, key: impl Into<OsString>, value: impl Into<OsString>) -> Self {
        self.environment.push((key.into(), value.into()));
        self
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProcessOutput {
    pub(crate) status_code: Option<i32>,
    pub(crate) stdout: Vec<u8>,
    pub(crate) stderr: Vec<u8>,
    pub(crate) stdout_truncated: bool,
    pub(crate) stderr_truncated: bool,
    pub(crate) timed_out: bool,
}

impl ProcessOutput {
    pub(crate) fn success(&self) -> bool {
        !self.timed_out && self.status_code == Some(0)
    }

    pub(crate) fn truncated(&self) -> bool {
        self.stdout_truncated || self.stderr_truncated
    }
}

pub(crate) fn run(
    spec: &ProcessSpec,
    operation: &'static str,
) -> Result<ProcessOutput, AdapterError> {
    let mut command = Command::new(&spec.executable);
    command
        .args(&spec.args)
        .current_dir(&spec.cwd)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .env_clear()
        .env("LC_ALL", "C")
        .env("LANG", "C");

    if let Some(path) = std::env::var_os("PATH") {
        command.env("PATH", path);
    }
    #[cfg(windows)]
    if let Some(system_root) = std::env::var_os("SystemRoot") {
        command.env("SystemRoot", system_root);
    }
    for (key, value) in &spec.environment {
        command.env(key, value);
    }

    let mut child = command
        .spawn()
        .map_err(|error| spawn_error(operation, error))?;
    let stdout = child.stdout.take().ok_or_else(|| {
        AdapterError::new(
            AdapterErrorKind::Io,
            operation,
            "child stdout pipe was unavailable",
            false,
            "retry after verifying the local executable",
        )
    })?;
    let stderr = child.stderr.take().ok_or_else(|| {
        AdapterError::new(
            AdapterErrorKind::Io,
            operation,
            "child stderr pipe was unavailable",
            false,
            "retry after verifying the local executable",
        )
    })?;

    let stdout_limit = spec.max_output;
    let stderr_limit = spec.max_output;
    let stdout_thread = thread::spawn(move || read_bounded(stdout, stdout_limit));
    let stderr_thread = thread::spawn(move || read_bounded(stderr, stderr_limit));

    let deadline = Instant::now() + spec.timeout;
    let (status_code, timed_out) = loop {
        match child.try_wait() {
            Ok(Some(status)) => break (status.code(), false),
            Ok(None) if Instant::now() < deadline => thread::sleep(Duration::from_millis(10)),
            Ok(None) => {
                let _ = child.kill();
                let status = child.wait().map_err(|error| {
                    AdapterError::new(
                        AdapterErrorKind::Io,
                        operation,
                        "timed-out process could not be reaped",
                        false,
                        "terminate the local executable and retry",
                    )
                    .with_diagnostic(error.to_string())
                })?;
                break (status.code(), true);
            }
            Err(error) => {
                let _ = child.kill();
                let _ = child.wait();
                return Err(AdapterError::new(
                    AdapterErrorKind::Io,
                    operation,
                    "process status could not be observed",
                    true,
                    "retry after verifying the local executable",
                )
                .with_diagnostic(error.to_string()));
            }
        }
    };

    let stdout = join_reader(stdout_thread, operation, "stdout")?;
    let stderr = join_reader(stderr_thread, operation, "stderr")?;
    Ok(ProcessOutput {
        status_code,
        stdout: stdout.bytes,
        stderr: stderr.bytes,
        stdout_truncated: stdout.truncated,
        stderr_truncated: stderr.truncated,
        timed_out,
    })
}

#[derive(Debug)]
struct BoundedRead {
    bytes: Vec<u8>,
    truncated: bool,
}

fn read_bounded(mut reader: impl Read, limit: usize) -> io::Result<BoundedRead> {
    let mut retained = Vec::with_capacity(limit.min(8192));
    let mut buffer = [0_u8; 8192];
    let mut truncated = false;
    loop {
        let count = reader.read(&mut buffer)?;
        if count == 0 {
            break;
        }
        let remaining = limit.saturating_sub(retained.len());
        let keep = remaining.min(count);
        retained.extend_from_slice(&buffer[..keep]);
        if keep < count {
            truncated = true;
        }
    }
    Ok(BoundedRead {
        bytes: retained,
        truncated,
    })
}

fn join_reader(
    handle: thread::JoinHandle<io::Result<BoundedRead>>,
    operation: &'static str,
    stream: &'static str,
) -> Result<BoundedRead, AdapterError> {
    match handle.join() {
        Ok(Ok(output)) => Ok(output),
        Ok(Err(error)) => Err(AdapterError::new(
            AdapterErrorKind::Io,
            operation,
            format!("process {stream} could not be read"),
            true,
            "retry after verifying the local executable",
        )
        .with_diagnostic(error.to_string())),
        Err(_) => Err(AdapterError::new(
            AdapterErrorKind::Io,
            operation,
            format!("process {stream} reader terminated unexpectedly"),
            false,
            "retry with developer diagnostics enabled",
        )),
    }
}

fn spawn_error(operation: &'static str, error: io::Error) -> AdapterError {
    let kind = if error.kind() == io::ErrorKind::NotFound {
        AdapterErrorKind::Unavailable
    } else {
        AdapterErrorKind::Io
    };
    AdapterError::new(
        kind,
        operation,
        "required local executable could not be started",
        false,
        "install or configure the executable, then retry",
    )
    .with_diagnostic(error.to_string())
}

#[cfg(test)]
mod tests {
    use std::{env, path::PathBuf, time::Duration};

    use super::{ProcessSpec, run};

    #[test]
    fn process_helper() {
        let Ok(mode) = env::var("WERKSTATT_PROCESS_HELPER") else {
            return;
        };
        match mode.as_str() {
            "large" => print!("{}", "x".repeat(32_768)),
            "sleep" => std::thread::sleep(Duration::from_secs(2)),
            "failure" => std::process::exit(7),
            _ => print!("{mode}"),
        }
    }

    fn helper(mode: &str) -> ProcessSpec {
        let executable = env::current_exe().unwrap();
        ProcessSpec::new(executable, PathBuf::from("."))
            .args([
                "--exact",
                "adapters::process::tests::process_helper",
                "--nocapture",
            ])
            .env("WERKSTATT_PROCESS_HELPER", mode)
    }

    #[test]
    fn bounds_output_without_pipe_deadlock() {
        let output = run(&helper("large").max_output(1024), "test.large").unwrap();
        assert!(output.success());
        assert_eq!(output.stdout.len(), 1024);
        assert!(output.stdout_truncated);
    }

    #[test]
    fn terminates_timed_out_process() {
        let output = run(
            &helper("sleep").timeout(Duration::from_millis(50)),
            "test.timeout",
        )
        .unwrap();
        assert!(output.timed_out);
        assert!(!output.success());
    }

    #[test]
    fn preserves_non_zero_status() {
        let output = run(&helper("failure"), "test.failure").unwrap();
        assert_eq!(output.status_code, Some(7));
        assert!(!output.success());
    }
}
