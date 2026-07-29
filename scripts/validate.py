#!/usr/bin/env python3
"""Read-only repository validation for dornglut/werkstatt."""

from __future__ import annotations

import re
import subprocess
import sys
from pathlib import Path
from urllib.parse import unquote

ROOT = Path(__file__).resolve().parents[1]
MAX_FILE_BYTES = 131_072
TEXT_SUFFIXES = {".md", ".py", ".rs", ".sql", ".txt", ".toml", ".yml", ".yaml"}
LINK_RE = re.compile(r"!?\[[^\]]*\]\(([^)]+)\)")
WORKFLOW_PATH = Path(".github/workflows/validate.yml")
EXPECTED_WORKFLOW = """name: Validate

on:
  pull_request:
  push:
    branches:
      - main

permissions:
  contents: read

jobs:
  validate:
    uses: dornglut/github-workflows/.github/workflows/reusable-python-repository-validate.yml@624cb41adeed21a6461eb838bc7330bd0a5079fd
"""


def relative(path: Path) -> str:
    return path.relative_to(ROOT).as_posix()


def fail(message: str, failures: list[str]) -> None:
    failures.append(message)


def repository_files() -> list[Path]:
    return sorted(
        path
        for path in ROOT.rglob("*")
        if path.is_file()
        and ".git" not in path.relative_to(ROOT).parts
        and "target" not in path.relative_to(ROOT).parts
    )


def validate_file_size(path: Path, failures: list[str]) -> None:
    size = path.stat().st_size
    if size > MAX_FILE_BYTES:
        fail(
            f"{relative(path)}: {size} bytes exceeds the {MAX_FILE_BYTES}-byte limit",
            failures,
        )


def validate_text(path: Path, failures: list[str]) -> str | None:
    data = path.read_bytes()
    path_text = relative(path)

    if b"\x00" in data:
        fail(f"{path_text}: contains a NUL byte", failures)
        return None

    try:
        text = data.decode("utf-8")
    except UnicodeDecodeError as error:
        fail(f"{path_text}: is not valid UTF-8: {error}", failures)
        return None

    if text and not text.endswith("\n"):
        fail(f"{path_text}: must end with a newline", failures)

    for line_number, line in enumerate(text.splitlines(), start=1):
        if line.endswith((" ", "\t")):
            fail(f"{path_text}:{line_number}: trailing whitespace", failures)
        if "\t" in line:
            fail(f"{path_text}:{line_number}: tab character", failures)

    return text


def validate_markdown_links(path: Path, text: str, failures: list[str]) -> None:
    for match in LINK_RE.finditer(text):
        raw_target = match.group(1).strip()
        target = raw_target.split(maxsplit=1)[0].strip("<>")

        if not target or target.startswith(("#", "http://", "https://", "mailto:")):
            continue

        target = unquote(target.split("#", 1)[0].split("?", 1)[0])
        if not target:
            continue

        resolved = (path.parent / target).resolve()
        try:
            resolved.relative_to(ROOT)
        except ValueError:
            fail(f"{relative(path)}: link escapes repository: {raw_target}", failures)
            continue

        if not resolved.exists():
            fail(f"{relative(path)}: broken relative link: {raw_target}", failures)


def validate_required_files(failures: list[str]) -> None:
    manifest = ROOT / "validation-required-files.txt"
    if not manifest.is_file():
        fail("validation-required-files.txt: missing", failures)
        return

    text = manifest.read_text(encoding="utf-8")
    for line_number, raw_line in enumerate(text.splitlines(), start=1):
        entry = raw_line.strip()
        if not entry or entry.startswith("#"):
            continue

        path = Path(entry)
        if path.is_absolute() or ".." in path.parts:
            fail(
                f"validation-required-files.txt:{line_number}: invalid path: {entry}",
                failures,
            )
            continue

        if not (ROOT / path).is_file():
            fail(f"{entry}: required file is missing", failures)


def validate_workflow(failures: list[str]) -> None:
    workflows = ROOT / ".github/workflows"
    actual = sorted(
        path.relative_to(ROOT)
        for path in workflows.glob("*")
        if path.is_file() and path.suffix.lower() in {".yml", ".yaml"}
    )

    if actual != [WORKFLOW_PATH]:
        found = ", ".join(path.as_posix() for path in actual) or "none"
        fail(
            ".github/workflows: expected only "
            f"{WORKFLOW_PATH.as_posix()}; found {found}",
            failures,
        )
        return

    workflow = ROOT / WORKFLOW_PATH
    text = workflow.read_text(encoding="utf-8")
    if text != EXPECTED_WORKFLOW:
        fail(
            f"{WORKFLOW_PATH.as_posix()}: must remain the exact accepted read-only caller",
            failures,
        )


def validate_cargo(failures: list[str]) -> None:
    commands = (
        ("cargo", "fmt", "--all", "--check"),
        ("cargo", "test", "--workspace", "--locked"),
        ("cargo", "clippy", "--workspace", "--all-targets", "--locked", "--", "-D", "warnings"),
    )
    for command in commands:
        try:
            result = subprocess.run(command, cwd=ROOT, check=False, text=True, capture_output=True)
        except OSError as error:
            fail(f"{' '.join(command)}: could not start: {error}", failures)
            continue
        if result.returncode:
            output = (result.stdout + result.stderr).strip()
            fail(f"{' '.join(command)}: failed ({result.returncode}): {output[:2000]}", failures)


def main() -> int:
    failures: list[str] = []

    for path in repository_files():
        validate_file_size(path, failures)
        if path.suffix.lower() not in TEXT_SUFFIXES and path.name != "LICENSE":
            continue

        text = validate_text(path, failures)
        if text is not None and path.suffix.lower() == ".md":
            validate_markdown_links(path, text, failures)

    validate_required_files(failures)
    validate_workflow(failures)
    validate_cargo(failures)

    if failures:
        print("Werkstatt repository validation failed:", file=sys.stderr)
        for failure in failures:
            print(f"- {failure}", file=sys.stderr)
        return 1

    print("Werkstatt repository validation passed.")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
