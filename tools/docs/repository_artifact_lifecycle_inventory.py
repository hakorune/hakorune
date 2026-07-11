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
DESIGN_INDEX = ROOT / "docs/development/current/main/design/INDEX.md"
PHASE = ROOT / "docs/development/current/main/phases/phase-296x"
EXCLUDED_CARD_NAMES = {"README.md", "STATUS.md", "CARD-HYGIENE-RULE.md"}
MARKDOWN_TOKEN = re.compile(r"(?<![A-Za-z0-9_.-])([A-Za-z0-9_.-]+\.md)")
DESIGN_FILE_TOKEN = re.compile(
    r"(?<![A-Za-z0-9_.-])([A-Za-z0-9_.-]+\.(?:md|toml))"
)
PHASE_PATH_TOKEN = re.compile(
    r"docs/development/current/main/phases/(phase-[A-Za-z0-9-]+)/"
)
STATUS_LINE = re.compile(r"(?im)^(?:##\s*)?Status\s*:?\s*(.+)$")
CLOSED_WORDS = ("complete", "closed", "landed", "historical", "superseded")
ACTIVE_WORDS = ("active", "implementation", "design consultation")
DESIGN_ROLES = {
    "authority",
    "navigation",
    "supporting",
    "status-ledger",
    "superseded",
}
DESIGN_REGISTRY_BLOCK = re.compile(
    r"<!-- design-registry-v0:begin -->\s*```toml\s*(.*?)\s*```\s*"
    r"<!-- design-registry-v0:end -->",
    re.DOTALL,
)


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


def tracked_references(
    repository_paths: list[str], phase_names: set[str]
) -> tuple[dict[str, set[str]], dict[str, set[str]]]:
    references: dict[str, set[str]] = {}
    phase_references = {name: set() for name in phase_names}
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
        source_phase = next(
            (name for name in phase_names if f"/phases/{name}/" in f"/{relative}"),
            None,
        )
        for phase_name in PHASE_PATH_TOKEN.findall(text):
            if phase_name in phase_references and source_phase != phase_name:
                phase_references[phase_name].add(relative)
    return references, phase_references


def phase_status(phase_name: str) -> tuple[str, str | None]:
    phase_dir = ROOT / "docs/development/current/main/phases" / phase_name
    for filename in ("STATUS.md", "README.md"):
        path = phase_dir / filename
        if path.is_file():
            return card_status(path), path.relative_to(ROOT).as_posix()
    return "other_or_missing", None


def design_registry_inventory() -> dict[str, object]:
    design_dir = ROOT / "docs/development/current/main/design"
    direct_files = {path.name for path in design_dir.iterdir() if path.is_file()}
    markdown_files = {name for name in direct_files if name.endswith(".md")}
    seed_paths = (
        design_dir / "README.md",
        ROOT / "docs/development/current/main/DOCS_LAYOUT.md",
        ROOT / "AGENTS.md",
        ROOT / "docs/development/current/main/CURRENT_STATE.toml",
    )
    seed_counts: dict[str, int] = {}
    seed_union: set[str] = set()
    for path in seed_paths:
        names = set(DESIGN_FILE_TOKEN.findall(path.read_text(encoding="utf-8")))
        names &= direct_files
        seed_counts[path.relative_to(ROOT).as_posix()] = len(names)
        seed_union.update(names)
    status_counts = {"closed": 0, "active_like": 0, "other_or_missing": 0}
    for name in markdown_files:
        status_counts[card_status(design_dir / name)] += 1
    registry, violations = read_design_registry(direct_files)
    registered = {row["path"] for row in registry["documents"]}
    sidecars = {
        sidecar
        for row in registry["documents"]
        for sidecar in row.get("sidecars", [])
    }
    unregistered = sorted(direct_files - registered - sidecars)
    baseline = registry["unregistered_baseline"]
    if len(unregistered) > baseline:
        violations.append(
            f"unregistered design files grew: {len(unregistered)} > {baseline}"
        )
    if registry["mode"] == "strict" and unregistered:
        violations.append("strict design registry has unregistered files")
    return {
        "direct_files": len(direct_files),
        "markdown_files": len(markdown_files),
        "non_markdown_files": len(direct_files - markdown_files),
        "seed_reference_counts": seed_counts,
        "seed_union_count": len(seed_union),
        "unseeded_count": len(direct_files - seed_union),
        "status_counts": status_counts,
        "authority_registry_decided": True,
        "registry_mode": registry["mode"],
        "registered_count": len(registered),
        "owned_sidecar_count": len(sidecars),
        "unregistered_count": len(unregistered),
        "unregistered_baseline": baseline,
        "unregistered": unregistered,
        "violations": violations,
    }


def read_design_registry(
    direct_files: set[str],
) -> tuple[dict[str, object], list[str]]:
    violations: list[str] = []
    if not DESIGN_INDEX.is_file():
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            "design registry INDEX.md is missing"
        ]
    match = DESIGN_REGISTRY_BLOCK.search(DESIGN_INDEX.read_text(encoding="utf-8"))
    if not match:
        return {"mode": "warning", "unregistered_baseline": 0, "documents": []}, [
            "design registry typed block is missing"
        ]
    registry = tomllib.loads(match.group(1))
    if registry.get("schema_version") != 0:
        violations.append("design registry schema_version must be 0")
    if registry.get("mode") not in {"warning", "strict"}:
        violations.append("design registry mode must be warning or strict")
    rows = registry.get("documents", [])
    paths = [row.get("path", "") for row in rows]
    if len(paths) != len(set(paths)):
        violations.append("design registry contains duplicate paths")
    sidecar_owners: dict[str, str] = {}
    row_by_path = {row.get("path", ""): row for row in rows}
    for row in rows:
        path = row.get("path", "")
        role = row.get("role", "")
        if path not in direct_files:
            violations.append(f"registered design file is missing: {path}")
        if role not in DESIGN_ROLES:
            violations.append(f"invalid design role for {path}: {role}")
        if not row.get("owner"):
            violations.append(f"design row owner is missing: {path}")
        if not row.get("retire_when"):
            violations.append(f"design row retire_when is missing: {path}")
        if role == "superseded" and not row.get("superseded_by"):
            violations.append(f"superseded_by is required: {path}")
        for sidecar in row.get("sidecars", []):
            if sidecar not in direct_files:
                violations.append(f"design sidecar is missing: {path} -> {sidecar}")
            if sidecar in row_by_path:
                violations.append(f"design sidecar also has a document row: {sidecar}")
            previous_owner = sidecar_owners.setdefault(sidecar, path)
            if previous_owner != path:
                violations.append(
                    f"design sidecar has multiple owners: {sidecar}"
                )
    for path in paths:
        seen: set[str] = set()
        current = path
        while current in row_by_path:
            if current in seen:
                violations.append(f"design precedence cycle includes: {current}")
                break
            seen.add(current)
            current = row_by_path[current].get("precedence_parent", "")
    readme = (DESIGN_INDEX.parent / "README.md").read_text(encoding="utf-8")
    if "INDEX.md" not in readme or "navigation-only" not in readme:
        violations.append("design README must identify INDEX.md and navigation-only role")
    return registry, violations


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
    phase_prefix = "docs/development/current/main/phases/"
    phase_names = {
        parts[5]
        for relative in repository_paths
        if relative.startswith(phase_prefix)
        and len(parts := relative.split("/")) > 6
        and parts[5].startswith("phase-")
    }
    archived_phase_prefix = "docs/development/archive/phases/"
    archived_phase_names = {
        parts[4]
        for relative in repository_paths
        if relative.startswith(archived_phase_prefix)
        and len(parts := relative.split("/")) > 5
        and parts[4].startswith("phase-")
    }
    markdown_references, phase_references = tracked_references(
        repository_paths, phase_names
    )
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
    phase_rows: list[dict[str, object]] = []
    inactive_phase_candidates: list[str] = []
    for phase_name in sorted(phase_names):
        prefix = f"{phase_prefix}{phase_name}/"
        total_files = under(prefix)
        direct_files = direct(prefix)
        status, status_source = phase_status(phase_name)
        current_pointer = any(path.startswith(prefix) for path in current_paths)
        external_references = sorted(phase_references[phase_name])
        eligible = (
            status == "closed"
            and not current_pointer
            and not external_references
            and phase_name != "phase-296x"
        )
        if eligible:
            inactive_phase_candidates.append(phase_name)
        phase_rows.append(
            {
                "phase": phase_name,
                "direct_files": direct_files,
                "total_files": total_files,
                "status": status,
                "status_source": status_source,
                "current_pointer": current_pointer,
                "external_reference_count": len(external_references),
                "eligible_for_whole_phase_archive": eligible,
            }
        )
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
        "phase_directories": {
            "phase_count": len(phase_rows),
            "inactive_candidate_count": len(inactive_phase_candidates),
            "inactive_candidates": inactive_phase_candidates,
            "archived_phase_count": len(archived_phase_names),
            "archived_phase_files": under(archived_phase_prefix),
            "rows": phase_rows,
        },
        "design_registry": design_registry_inventory(),
    }


def serialized(value: dict[str, object]) -> str:
    return json.dumps(value, ensure_ascii=True, indent=2, sort_keys=True) + "\n"


def main() -> int:
    args = parse_args()
    output = args.output if args.output.is_absolute() else ROOT / args.output
    inventory = build_inventory()
    registry_violations = inventory["design_registry"]["violations"]
    if registry_violations:
        for violation in registry_violations:
            print(f"[repository-artifact-lifecycle] ERROR: {violation}", file=sys.stderr)
        return 1
    actual = serialized(inventory)
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
