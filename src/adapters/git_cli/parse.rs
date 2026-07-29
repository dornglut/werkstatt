use std::path::{Path, PathBuf};

use crate::{
    domain::{PathChange, PathChangeKind, WorktreeObservation},
    support::{
        path::{bytes_to_os_string, path_key, safe_path_display, trim_ascii},
        sha256::digest_hex,
    },
};

use super::{domain_adapter_error, malformed};
use crate::adapters::AdapterError;

pub(super) fn parse_status(bytes: &[u8]) -> Result<Vec<PathChange>, AdapterError> {
    let records = bytes.split(|byte| *byte == 0).collect::<Vec<_>>();
    let mut changes = Vec::new();
    let mut index = 0;
    while index < records.len() {
        let record = records[index];
        index += 1;
        if record.is_empty() || record.starts_with(b"# ") || record.starts_with(b"! ") {
            continue;
        }
        let (kind, path, original) = match record.first().copied() {
            Some(b'?') if record.starts_with(b"? ") => {
                (PathChangeKind::Untracked, &record[2..], None)
            }
            Some(b'1') if record.starts_with(b"1 ") => {
                let fields = split_fields(record, 9, "repository.status")?;
                (kind_from_xy(fields[1]), fields[8], None)
            }
            Some(b'2') if record.starts_with(b"2 ") => {
                let fields = split_fields(record, 10, "repository.status")?;
                let original = records.get(index).copied().ok_or_else(|| {
                    malformed("repository.status", "rename record has no original path")
                })?;
                index += 1;
                let kind = if fields[8].first() == Some(&b'C') {
                    PathChangeKind::Copied
                } else {
                    PathChangeKind::Renamed
                };
                (kind, fields[9], Some(original))
            }
            Some(b'u') if record.starts_with(b"u ") => {
                let fields = split_fields(record, 11, "repository.status")?;
                (PathChangeKind::Unmerged, fields[10], None)
            }
            _ => {
                return Err(malformed(
                    "repository.status",
                    "Git porcelain-v2 record is unsupported or malformed",
                ));
            }
        };
        changes.push(
            PathChange::new(
                kind,
                digest_hex(path),
                safe_path_display(path),
                original.map(safe_path_display),
            )
            .map_err(domain_adapter_error)?,
        );
    }
    Ok(changes)
}

fn split_fields<'a>(
    record: &'a [u8],
    count: usize,
    operation: &'static str,
) -> Result<Vec<&'a [u8]>, AdapterError> {
    let fields = record
        .splitn(count, |byte| *byte == b' ')
        .collect::<Vec<_>>();
    if fields.len() != count {
        Err(malformed(operation, "Git record has too few fields"))
    } else {
        Ok(fields)
    }
}

fn kind_from_xy(xy: &[u8]) -> PathChangeKind {
    if xy.contains(&b'D') {
        PathChangeKind::Deleted
    } else if xy.contains(&b'A') {
        PathChangeKind::Added
    } else {
        PathChangeKind::Modified
    }
}

pub(super) fn parse_worktrees(
    bytes: &[u8],
    current_root: &Path,
) -> Result<Vec<WorktreeObservation>, AdapterError> {
    let mut worktrees = Vec::new();
    let mut current_path: Option<PathBuf> = None;
    let mut head = None;
    let mut branch = None;
    let mut detached = false;

    for field in bytes.split(|byte| *byte == 0) {
        if field.is_empty() {
            if let Some(path) = current_path.take() {
                let identity = path_key(&path);
                worktrees.push(
                    WorktreeObservation::new(
                        identity,
                        head.take(),
                        if detached { None } else { branch.take() },
                        path == current_root,
                    )
                    .map_err(domain_adapter_error)?,
                );
            }
            head = None;
            branch = None;
            detached = false;
            continue;
        }
        if let Some(value) = field.strip_prefix(b"worktree ") {
            let os = bytes_to_os_string(value.to_vec()).ok_or_else(|| {
                malformed(
                    "repository.worktrees",
                    "worktree path could not be represented without loss",
                )
            })?;
            current_path = Some(PathBuf::from(os));
        } else if let Some(value) = field.strip_prefix(b"HEAD ") {
            head = Some(
                std::str::from_utf8(value)
                    .map_err(|_| malformed("repository.worktrees", "worktree head is not UTF-8"))?
                    .to_owned(),
            );
        } else if let Some(value) = field.strip_prefix(b"branch ") {
            branch = Some(
                std::str::from_utf8(value)
                    .map_err(|_| malformed("repository.worktrees", "worktree branch is not UTF-8"))?
                    .trim_start_matches("refs/heads/")
                    .to_owned(),
            );
        } else if field == b"detached" {
            detached = true;
        }
    }
    if let Some(path) = current_path {
        worktrees.push(
            WorktreeObservation::new(
                path_key(&path),
                head,
                if detached { None } else { branch },
                path == current_root,
            )
            .map_err(domain_adapter_error)?,
        );
    }
    Ok(worktrees)
}

pub(super) fn parse_revision(
    bytes: &[u8],
    operation: &'static str,
) -> Result<String, AdapterError> {
    let value = std::str::from_utf8(trim_ascii(bytes))
        .map_err(|_| malformed(operation, "Git revision is not UTF-8"))?;
    if !(7..=128).contains(&value.len()) || !value.bytes().all(|byte| byte.is_ascii_hexdigit()) {
        return Err(malformed(operation, "Git revision is malformed"));
    }
    Ok(value.to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use super::parse_status;
    use crate::domain::PathChangeKind;

    #[test]
    fn parses_porcelain_v2_without_shell_or_lossy_path_identity() {
        let bytes =
            b"1 .M N... 100644 100644 100644 abcdef abcdef src/lib.rs\0? odd name;$(x).txt\0";
        let changes = parse_status(bytes).unwrap();
        assert_eq!(changes.len(), 2);
        assert_eq!(changes[0].kind(), PathChangeKind::Modified);
        assert_eq!(changes[1].kind(), PathChangeKind::Untracked);
        assert_eq!(changes[1].display(), "odd name;$(x).txt");
        assert_ne!(changes[0].path_key(), changes[1].path_key());
    }
}
