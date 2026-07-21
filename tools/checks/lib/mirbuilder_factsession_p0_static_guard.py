#!/usr/bin/env python3
"""Freeze FACTSESSION0-P0's syntax-only watched-site policy."""

from __future__ import annotations

import json
import subprocess
import sys
from collections import defaultdict
from pathlib import Path


TAG = "mirbuilder-factsession-p0-static"
TOOL_MANIFEST = "tools/checks/rust_source_topology/Cargo.toml"
FIXTURE = "tools/checks/fixtures/mirbuilder_factsession_p0_watched_sites_v1.json"
SELF = "tools/checks/lib/mirbuilder_factsession_p0_static_guard.py"
SCHEMA = "MirBuilderFactSessionP0WatchedSyntaxV1"
DISPOSITIONS = {
    "root_open",
    "function_open",
    "completion",
    "abort_drop",
    "publication",
    "synthetic_function",
    "test_adapter",
}
KEYS = (
    "path",
    "module_syntax_path",
    "enclosing_item_syntax_path",
    "expression_kind",
    "normalized_callee_syntax",
    "receiver_syntax",
    "direct_cfg_syntax",
    "inherited_cfg_syntax",
)


def fail(message: str) -> None:
    raise SystemExit(f"[{TAG}] ERROR: {message}")


def read_json(path: Path) -> object:
    try:
        return json.loads(path.read_text(encoding="utf-8"))
    except (OSError, json.JSONDecodeError) as error:
        fail(f"cannot read JSON {path}: {error}")


def topology(root: Path, path: str, module_syntax_path: str) -> list[dict[str, object]]:
    command = [
        "cargo",
        "run",
        "-q",
        "--manifest-path",
        TOOL_MANIFEST,
        "--",
        "single-file",
        path,
        "--module-syntax-path",
        module_syntax_path,
    ]
    result = subprocess.run(command, cwd=root, text=True, capture_output=True)
    if result.returncode != 0:
        fail(f"topology extraction failed path={path}: {result.stderr.strip()}")
    payload = json.loads(result.stdout)
    source = payload.get("source_file")
    if not isinstance(source, dict):
        fail(f"topology output has no source_file path={path}")
    calls = source.get("direct_call_sites")
    if not isinstance(calls, list) or not all(isinstance(call, dict) for call in calls):
        fail(f"topology output has invalid direct_call_sites path={path}")
    return calls


def exact_match(call: dict[str, object], entry: dict[str, object]) -> bool:
    return all(call.get(key) == entry[key] for key in KEYS)


def main() -> None:
    root = Path(sys.argv[1] if len(sys.argv) > 1 else ".").resolve()
    fixture_path = root / FIXTURE
    document = read_json(fixture_path)
    if not isinstance(document, dict) or document.get("schema") != SCHEMA:
        fail(f"fixture schema must be {SCHEMA}")
    entries = document.get("entries")
    if not isinstance(entries, list) or not entries:
        fail("fixture entries must be a non-empty list")

    grouped: dict[tuple[str, str], list[dict[str, object]]] = defaultdict(list)
    for index, entry in enumerate(entries):
        if not isinstance(entry, dict):
            fail(f"entry #{index} is not an object")
        if set(entry) != {*KEYS, "disposition"}:
            fail(f"entry #{index} keys drift: {sorted(entry)}")
        if not all(isinstance(entry[key], str) and entry[key] for key in KEYS[:5]):
            fail(f"entry #{index} has an empty syntax key")
        if entry["receiver_syntax"] is not None and not isinstance(entry["receiver_syntax"], str):
            fail(f"entry #{index} receiver_syntax must be string or null")
        if not all(isinstance(entry[key], list) for key in ("direct_cfg_syntax", "inherited_cfg_syntax")):
            fail(f"entry #{index} cfg syntax must be lists")
        if entry["disposition"] not in DISPOSITIONS:
            fail(f"entry #{index} has unknown disposition={entry['disposition']!r}")
        grouped[(entry["path"], entry["module_syntax_path"])].append(entry)

    used_sites: set[str] = set()
    seen_dispositions: set[str] = set()
    for (path, module_syntax_path), watched in sorted(grouped.items()):
        calls = topology(root, path, module_syntax_path)
        for entry in watched:
            matches = [call for call in calls if exact_match(call, entry)]
            if len(matches) != 1:
                fail(
                    "watched syntax must match exactly once "
                    f"path={path} callee={entry['normalized_callee_syntax']} "
                    f"enclosing={entry['enclosing_item_syntax_path']} matches={len(matches)}"
                )
            call = matches[0]
            site_id = call.get("call_site_id")
            if not isinstance(site_id, str) or not site_id:
                fail(f"watched syntax lacks call_site_id path={path}")
            if site_id in used_sites:
                fail(f"two policy rows classify one syntax site={site_id}")
            used_sites.add(site_id)
            resolution = call.get("resolution")
            if not isinstance(resolution, dict) or resolution.get("status") != "unresolved":
                fail(f"P0 must retain syntax-only unresolved observation site={site_id}")
            seen_dispositions.add(entry["disposition"])

    required = {
        "root_open",
        "function_open",
        "completion",
        "abort_drop",
        "publication",
        "synthetic_function",
        "test_adapter",
    }
    if seen_dispositions != required:
        fail(f"watched disposition coverage drift expected={sorted(required)} actual={sorted(seen_dispositions)}")

    guarded = [root / FIXTURE, root / SELF]
    oversized = [str(path.relative_to(root)) for path in guarded if len(path.read_text(encoding="utf-8").splitlines()) >= 800]
    if oversized:
        fail(f"source/check files reached 800 lines: {oversized}")
    print(f"[{TAG}] ok watched_sites={len(used_sites)} unclassified=0 semantic_resolution_claims=0")


if __name__ == "__main__":
    main()
