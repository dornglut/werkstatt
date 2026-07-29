use std::{
    ffi::{OsStr, OsString},
    path::{Path, PathBuf},
};

use super::sha256::digest_hex;

const PRIVATE_DIAGNOSTIC_ENVIRONMENT: &[&str] = &[
    "HOME",
    "USERPROFILE",
    "XDG_CONFIG_HOME",
    "GH_CONFIG_DIR",
    "APPDATA",
    "LOCALAPPDATA",
    "HTTP_PROXY",
    "HTTPS_PROXY",
    "NO_PROXY",
    "SSL_CERT_FILE",
    "SSL_CERT_DIR",
];

pub(crate) fn path_key(path: &Path) -> String {
    digest_hex(&os_bytes(path.as_os_str()))
}

pub(crate) fn short_key(value: &str) -> &str {
    value.get(..12).unwrap_or(value)
}

pub(crate) fn safe_path_display(bytes: &[u8]) -> String {
    if let Ok(value) = std::str::from_utf8(bytes)
        && value.chars().all(|character| !character.is_control())
    {
        return value.replace('\\', "/");
    }
    let mut output = String::new();
    for byte in bytes {
        if byte.is_ascii_alphanumeric() || matches!(*byte, b'/' | b'.' | b'_' | b'-') {
            output.push(char::from(*byte));
        } else {
            use std::fmt::Write as _;
            let _ = write!(output, "%{byte:02X}");
        }
    }
    output
}

pub(crate) fn sanitize_remote(value: &str) -> String {
    let trimmed = value.trim();
    if trimmed.is_empty() {
        return "remote:unavailable".into();
    }
    let without_fragment = trimmed.split('#').next().unwrap_or(trimmed);
    let without_query = without_fragment
        .split('?')
        .next()
        .unwrap_or(without_fragment);

    if let Some((scheme, rest)) = without_query.split_once("://") {
        let (authority, path) = rest.split_once('/').unwrap_or((rest, ""));
        let host = authority
            .rsplit_once('@')
            .map_or(authority, |(_, host)| host);
        if host.is_empty() {
            return format!("remote:{}", digest_hex(without_query.as_bytes()));
        }
        let normalized_path = path.trim_start_matches('/');
        return if normalized_path.is_empty() {
            format!(
                "{}://{}",
                scheme.to_ascii_lowercase(),
                host.to_ascii_lowercase()
            )
        } else {
            format!(
                "{}://{}/{}",
                scheme.to_ascii_lowercase(),
                host.to_ascii_lowercase(),
                normalized_path
            )
        };
    }

    if let Some((authority, path)) = without_query.split_once(':')
        && authority.contains('@')
        && !path.is_empty()
    {
        let host = authority
            .rsplit_once('@')
            .map_or(authority, |(_, host)| host);
        return format!(
            "ssh://{}/{}",
            host.to_ascii_lowercase(),
            path.trim_start_matches('/')
        );
    }

    let path = Path::new(without_query);
    if path.is_absolute() || without_query.starts_with('.') {
        return format!("local:{}", path_key(path));
    }

    format!("remote:{}", digest_hex(without_query.as_bytes()))
}

pub(crate) fn canonical_github_issue_url(value: &str) -> Option<(String, String)> {
    let value = value.trim();
    if value.contains('@') || value.contains('?') || value.contains('#') {
        return None;
    }
    let path = value.strip_prefix("https://github.com/")?;
    let mut parts = path.trim_end_matches('/').split('/');
    let owner = parts.next()?;
    let repository = parts.next()?;
    if parts.next()? != "issues" {
        return None;
    }
    let number = parts.next()?;
    if parts.next().is_some()
        || owner.is_empty()
        || repository.is_empty()
        || number.parse::<u64>().ok()? == 0
        || !owner.chars().all(valid_slug_character)
        || !repository.chars().all(valid_slug_character)
    {
        return None;
    }
    let canonical = format!("https://github.com/{owner}/{repository}/issues/{number}");
    let opaque = format!("{owner}/{repository}#{number}");
    Some((canonical, opaque))
}

pub(crate) fn bytes_to_os_string(bytes: Vec<u8>) -> Option<OsString> {
    platform::bytes_to_os_string(bytes)
}

pub(crate) fn path_from_output(bytes: &[u8], cwd: &Path) -> Option<PathBuf> {
    let trimmed = trim_ascii(bytes);
    let value = bytes_to_os_string(trimmed.to_vec())?;
    let path = PathBuf::from(value);
    Some(if path.is_absolute() {
        path
    } else {
        cwd.join(path)
    })
}

pub(crate) fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|byte| !byte.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|byte| !byte.is_ascii_whitespace())
        .map_or(start, |index| index + 1);
    &bytes[start..end]
}

pub(crate) fn redact_diagnostic(bytes: &[u8], cwd: &Path) -> String {
    let mut value = String::from_utf8_lossy(bytes).into_owned();
    let absolute_cwd = cwd.canonicalize().ok().or_else(|| {
        if cwd.is_absolute() {
            Some(cwd.to_path_buf())
        } else {
            std::env::current_dir().ok().map(|base| base.join(cwd))
        }
    });
    if let Some(path) = absolute_cwd.as_deref().and_then(Path::to_str) {
        redact_literal(&mut value, path, "<repository>");
    }
    for key in PRIVATE_DIAGNOSTIC_ENVIRONMENT {
        if let Some(private) = std::env::var_os(key).and_then(|value| value.into_string().ok()) {
            redact_literal(&mut value, &private, "<private-environment>");
        }
    }
    value.lines().take(8).collect::<Vec<_>>().join("\n")
}

fn redact_literal(value: &mut String, private: &str, replacement: &str) {
    if private.len() > 3 {
        *value = value.replace(private, replacement);
    }
}

fn valid_slug_character(character: char) -> bool {
    character.is_ascii_alphanumeric() || matches!(character, '-' | '_' | '.')
}

fn os_bytes(value: &OsStr) -> Vec<u8> {
    platform::os_bytes(value)
}

#[cfg(unix)]
mod platform {
    use std::{
        ffi::{OsStr, OsString},
        os::unix::ffi::{OsStrExt, OsStringExt},
    };

    pub(super) fn os_bytes(value: &OsStr) -> Vec<u8> {
        value.as_bytes().to_vec()
    }

    pub(super) fn bytes_to_os_string(value: Vec<u8>) -> Option<OsString> {
        Some(OsString::from_vec(value))
    }
}

#[cfg(windows)]
mod platform {
    use std::{
        ffi::{OsStr, OsString},
        os::windows::ffi::OsStrExt,
    };

    pub(super) fn os_bytes(value: &OsStr) -> Vec<u8> {
        value.encode_wide().flat_map(u16::to_le_bytes).collect()
    }

    pub(super) fn bytes_to_os_string(value: Vec<u8>) -> Option<OsString> {
        String::from_utf8(value).ok().map(OsString::from)
    }
}

#[cfg(not(any(unix, windows)))]
mod platform {
    use std::ffi::{OsStr, OsString};

    pub(super) fn os_bytes(value: &OsStr) -> Vec<u8> {
        value.to_string_lossy().as_bytes().to_vec()
    }

    pub(super) fn bytes_to_os_string(value: Vec<u8>) -> Option<OsString> {
        String::from_utf8(value).ok().map(OsString::from)
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::{canonical_github_issue_url, redact_diagnostic, redact_literal, sanitize_remote};

    #[test]
    fn redacts_remote_credentials_and_query_material() {
        assert_eq!(
            sanitize_remote("https://token@example.com/org/repo.git?x=secret#fragment"),
            "https://example.com/org/repo.git"
        );
        assert_eq!(
            sanitize_remote("git@example.com:org/repo.git"),
            "ssh://example.com/org/repo.git"
        );
    }

    #[test]
    fn validates_supported_github_issue_urls() {
        assert_eq!(
            canonical_github_issue_url("https://github.com/dornglut/werkstatt/issues/10"),
            Some((
                "https://github.com/dornglut/werkstatt/issues/10".into(),
                "dornglut/werkstatt#10".into()
            ))
        );
        assert!(canonical_github_issue_url("https://token@github.com/a/b/issues/1").is_none());
    }

    #[test]
    fn redacts_absolute_working_directory_without_mangling_punctuation() {
        let current = std::env::current_dir().unwrap();
        let diagnostic = format!("failure in {}/config.json", current.display());
        let redacted = redact_diagnostic(diagnostic.as_bytes(), Path::new("."));
        assert!(!redacted.contains(current.to_str().unwrap()));
        assert!(redacted.contains("<repository>/config.json"));

        let mut punctuation = "error.json".to_owned();
        redact_literal(&mut punctuation, ".", "<repository>");
        assert_eq!(punctuation, "error.json");
    }

    #[test]
    fn redacts_known_private_environment_values_when_present() {
        let Some(home) = std::env::var_os("HOME").and_then(|value| value.into_string().ok()) else {
            return;
        };
        let diagnostic = format!("provider config at {home}/.config/gh");
        let redacted = redact_diagnostic(diagnostic.as_bytes(), Path::new("."));
        assert!(!redacted.contains(&home));
        assert!(redacted.contains("<private-environment>/.config/gh"));
    }
}
