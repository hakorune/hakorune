#!/usr/bin/env python3
"""Inventory direct transient type-map writers before FACT0 cutover."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


TYPE_WRITE = re.compile(
    r"\btype_ctx\s*\.\s*(?:value_types\s*\.\s*(?:insert|entry|clear|remove)|set_type)\s*\(",
    re.S,
)
IGNORED_RUST = re.compile(
    r"(?P<raw>r(?P<hash>#*)\".*?\"(?P=hash))"
    r"|(?P<string>(?:b|c)?\"(?:\\.|[^\"\\])*\")"
    r"|(?P<block>/\*.*?\*/)"
    r"|(?P<line>//[^\n]*)",
    re.S,
)
CFG_TEST_MODULE = re.compile(r"#\[cfg\(test\)\]\s*mod\s+\w+")


def fail(message: str) -> None:
    raise SystemExit(f"[mirbuilder-type-fact-producer-inventory] {message}")


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing required file: {path}")
    return path.read_text(encoding="utf-8")


def code_only(text: str) -> str:
    def blank(match: re.Match[str]) -> str:
        return "".join("\n" if char == "\n" else " " for char in match.group())

    return IGNORED_RUST.sub(blank, text)


def strip_cfg_test_modules(text: str) -> str:
    cursor = 0
    output: list[str] = []
    while True:
        match = CFG_TEST_MODULE.search(text, cursor)
        if match is None:
            output.append(text[cursor:])
            return "".join(output)
        start = match.start()
        output.append(text[cursor:start])
        brace = text.find("{", start)
        semicolon = text.find(";", start)
        if semicolon >= 0 and (brace < 0 or semicolon < brace):
            cursor = semicolon + 1
            continue
        if brace < 0:
            fail("cfg(test) module without body or declaration terminator")
        depth = 0
        end = brace
        for end in range(brace, len(text)):
            if text[end] == "{":
                depth += 1
            elif text[end] == "}":
                depth -= 1
                if depth == 0:
                    break
        else:
            fail("unterminated cfg(test) module")
        cursor = end + 1


def production_paths(root: Path) -> list[Path]:
    paths = []
    for path in (root / "src" / "mir" / "builder").rglob("*.rs"):
        relative = path.relative_to(root).as_posix()
        if "/tests/" in relative or path.name.endswith("_tests.rs") or path.name == "tests.rs":
            continue
        paths.append(path)
    return sorted(paths)


def writer_counts(root: Path) -> dict[str, int]:
    counts: dict[str, int] = {}
    for path in production_paths(root):
        text = strip_cfg_test_modules(code_only(read(path)))
        count = len(TYPE_WRITE.findall(text))
        if count:
            counts[path.relative_to(root).as_posix()] = count
    return counts


def require_anchor(root: Path, row: dict[str, object]) -> None:
    source_file = row.get("source_file")
    anchor = row.get("anchor")
    if not isinstance(source_file, str) or not isinstance(anchor, str):
        fail(f"invalid primary matrix row: {row!r}")
    if anchor not in read(root / source_file):
        fail(f"missing primary anchor {anchor!r} in {source_file}")


def load_fixture(root: Path) -> dict[str, object]:
    fixture = root / "tools/checks/fixtures/mirbuilder_type_fact_producer_matrix_v1.json"
    try:
        data = json.loads(read(fixture))
    except json.JSONDecodeError as error:
        fail(f"invalid fixture JSON: {error}")
    if data.get("schema_version") != 1:
        fail("fixture schema_version must be 1")
    return data


def check(root: Path) -> None:
    fixture = load_fixture(root)
    rows = fixture.get("write_inventory")
    controls = fixture.get("control_groups")
    matrix = fixture.get("primary_matrix")
    if not isinstance(rows, dict) or not isinstance(controls, list) or not isinstance(matrix, list):
        fail("fixture requires write_inventory object, control_groups, and primary_matrix arrays")

    expected = rows
    if not all(isinstance(path, str) and isinstance(count, int) for path, count in expected.items()):
        fail("write_inventory must map source_file strings to integer counts")

    actual = writer_counts(root)
    if actual != expected:
        fail(f"direct writer inventory drift: expected={expected} actual={actual}")

    classified: set[str] = set()
    for control in controls:
        if not isinstance(control, dict):
            fail(f"invalid control group: {control!r}")
        group_id = control.get("id")
        next_owner = control.get("next_owner")
        files = control.get("source_files")
        if not isinstance(group_id, str) or not isinstance(next_owner, str) or not isinstance(files, list):
            fail(f"invalid control group: {control!r}")
        for path in files:
            if not isinstance(path, str) or path not in expected:
                fail(f"control group {group_id} names unknown writer path: {path!r}")
            if path in classified:
                fail(f"writer path classified twice: {path}")
            classified.add(path)
    if classified != set(expected):
        fail(f"unclassified writer paths: {sorted(set(expected) - classified)}")

    ids: set[str] = set()
    for row in matrix:
        if not isinstance(row, dict):
            fail(f"invalid primary matrix row: {row!r}")
        row_id = row.get("id")
        timing = row.get("timing")
        if not isinstance(row_id, str) or not isinstance(timing, str):
            fail(f"invalid primary matrix row: {row!r}")
        if row_id in ids:
            fail(f"duplicate primary matrix id: {row_id}")
        ids.add(row_id)
        require_anchor(root, row)

    if ids != {"parameter", "copy", "phi", "call", "field_get"}:
        fail(f"primary matrix ids drift: {sorted(ids)}")

    print(
        "[mirbuilder-type-fact-producer-inventory] ok "
        f"writer_paths={len(actual)} writer_occurrences={sum(actual.values())} "
        f"primary_rows={len(matrix)}"
    )


def print_inventory(root: Path) -> None:
    print(json.dumps(writer_counts(root), indent=2, sort_keys=True))


def main(argv: list[str]) -> None:
    if len(argv) == 3 and argv[1] == "--inventory":
        print_inventory(Path(argv[2]).resolve())
        return
    if len(argv) != 2:
        fail("usage: mirbuilder_type_fact_producer_inventory.py ROOT | --inventory ROOT")
    check(Path(argv[1]).resolve())


if __name__ == "__main__":
    main(sys.argv)
