#!/usr/bin/env python3
"""Build the deterministic repository artifact lifecycle inventory."""

from __future__ import annotations

import argparse
import json
import re
import subprocess
import sys
import tomllib
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
DEFAULT_OUTPUT = ROOT / "tools/checks/manifests/repository_artifact_lifecycle_v0.json"
PHASE = ROOT / "docs/development/current/main/phases/phase-296x"
EXCLUDED_CARD_NAMES = {"README.md", "STATUS.md", "CARD-HYGIENE-RULE.md"}
MARKDOWN_TOKEN = re.compile(r"(?<![A-Za-z0-9_.-])([A-Za-z0-9_.-]+\.md)")
STATUS_LINE = re.compile(r"(?im)^(?:##\s*)?Status\s*:?\s*(.+)$")
CLOSED_WORDS = ("complete", "closed", "landed", "historical", "superseded")
ACTIVE_WORDS = ("active", "implementation", "design consultation")


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser()
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--write", action="store_true")
    mode.add_argument("--check", action="store_true")
    parser.add_argument("--strict", action="store_true")
    parser.add_argument("--output", type=Path, default=DEFAULT_OUTPUT)
    return parser.parse_args()


def read_current_paths() -> set[str]:
    state_path = ROOT / "docs/development/current/main/CURRENT_STATE.toml"
    state = tomllib.loads(state_path.read_text(encoding="utf-8"))
    keys = (
        "active_phase",
        "phase_status",
        "method_anchor",
        "taskboard",
        "latest_workstream_card",
        "latest_card_path",
        "current_update_policy",
    )
    return {str(state[key]) for key in keys if state.get(key)}


def repository_files() -> list[str]:
    raw = subprocess.check_output(
        [
            "git",
            "-C",
            str(ROOT),
            "ls-files",
            "-z",
            "--cached",
            "--others",
            "--exclude-standard",
        ],
    )
    return sorted(
        encoded.decode("utf-8", errors="surrogateescape")
        for encoded in raw.split(b"\0")
        if encoded
    )


def tracked_markdown_references(repository_paths: list[str]) -> dict[str, set[str]]:
    references: dict[str, set[str]] = {}
    for relative in repository_paths:
        if relative == DEFAULT_OUTPUT.relative_to(ROOT).as_posix():
            continue
        path = ROOT / relative
        try:
            if not path.is_file() or path.stat().st_size > 2_000_000:
                continue
            data = path.read_bytes()
        except OSError:
            continue
        if b"\0" in data:
            continue
        text = data.decode("utf-8", errors="ignore")
        for token in MARKDOWN_TOKEN.findall(text):
            references.setdefault(token, set()).add(relative)
    return references


def card_status(path: Path) -> str:
    text = path.read_text(encoding="utf-8", errors="ignore")[:4000]
    match = STATUS_LINE.search(text)
    if not match:
        return "other_or_missing"
    status = match.group(1).strip().lower()
    if any(word in status for word in CLOSED_WORDS):
        return "closed"
    if any(word in status for word in ACTIVE_WORDS):
        return "active_like"
    return "other_or_missing"


def build_inventory() -> dict[str, object]:
    current_paths = read_current_paths()
    repository_paths = repository_files()
    markdown_references = tracked_markdown_references(repository_paths)
    cards = sorted(
        path for path in PHASE.glob("*.md") if path.name not in EXCLUDED_CARD_NAMES
    )

    candidates: list[str] = []
    referenced: list[str] = []
    review: list[str] = []
    status_counts = {"closed": 0, "active_like": 0, "other_or_missing": 0}
    for path in cards:
        relative = path.relative_to(ROOT).as_posix()
        status = card_status(path)
        status_counts[status] += 1
        referring_paths = markdown_references.get(path.name, set()) - {relative}
        is_referenced = bool(referring_paths) or relative in current_paths
        if is_referenced:
            referenced.append(relative)
        elif status == "closed":
            candidates.append(relative)
        else:
            review.append(relative)

    def under(prefix: str) -> int:
        return sum(path.startswith(prefix) for path in repository_paths)

    def direct(prefix: str) -> int:
        return sum(
            path.startswith(prefix) and "/" not in path[len(prefix) :]
            for path in repository_paths
        )

    counts = {
        "docs": under("docs/"),
        "tools": under("tools/"),
        "src": under("src/"),
        "phase_296x_direct": direct(
            "docs/development/current/main/phases/phase-296x/"
        ),
        "phase_296x_archive": under(
            "docs/development/current/main/phases/phase-296x/archive/"
        ),
        "design_direct": direct("docs/development/current/main/design/"),
        "main_direct": direct("docs/development/current/main/"),
        "checks": under("tools/checks/"),
        "private": under("docs/private/"),
    }
    return {
        "schema_version": 0,
        "inventory": "repository-artifact-lifecycle-v0",
        "counts": counts,
        "phase_296x": {
            "card_count": len(cards),
            "status_counts": status_counts,
            "externally_referenced_count": len(referenced),
            "archive_candidate_count": len(candidates),
            "review_count": len(review),
            "archive_candidates": candidates,
            "externally_referenced": referenced,
            "needs_review": review,
        },
    }


def serialized(value: dict[str, object]) -> str:
    return json.dumps(value, ensure_ascii=True, indent=2, sort_keys=True) + "\n"


def main() -> int:
    args = parse_args()
    output = args.output if args.output.is_absolute() else ROOT / args.output
    actual = serialized(build_inventory())
    if args.write:
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(actual, encoding="utf-8")
        print(f"[repository-artifact-lifecycle] wrote {output.relative_to(ROOT)}")
        return 0
    if not output.is_file():
        print(f"[repository-artifact-lifecycle] WARNING: missing {output}", file=sys.stderr)
        return 1 if args.strict else 0
    expected = output.read_text(encoding="utf-8")
    if actual == expected:
        print("[repository-artifact-lifecycle] inventory current")
        return 0
    message = "inventory drift; run tools/docs/repository_artifact_lifecycle_inventory.py --write"
    stream = sys.stderr
    print(f"[repository-artifact-lifecycle] {'ERROR' if args.strict else 'WARNING'}: {message}", file=stream)
    return 1 if args.strict else 0


if __name__ == "__main__":
    raise SystemExit(main())
