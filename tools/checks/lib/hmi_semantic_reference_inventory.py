#!/usr/bin/env python3
"""Verify the behavior-neutral HMI-P0-I0 semantic-reference inventory."""

from __future__ import annotations

import json
from pathlib import Path
import re
import subprocess
import sys


SCHEMA = "HmiSemanticReferenceInventoryV1"
INSTRUCTION_KEYS = {
    "instruction_id",
    "semantic_owner",
    "dispatch_sites",
    "execution_family",
    "first_subset",
    "transport_op",
    "required_fields",
    "required_metadata",
    "input_value_classes",
    "output_value_classes",
    "lossiness",
    "loss_reasons",
    "caller_ids",
    "fixture_ids",
    "retirement_gate",
}
CALLER_KEYS = {
    "caller_id",
    "path",
    "symbol",
    "class",
    "constructor_occurrences",
    "retirement_condition",
}
FIXTURE_KEYS = {"fixture_id", "path", "purpose"}
TRANSPORT_KEYS = {
    "transport_id",
    "path",
    "symbol",
    "classification",
    "lossiness",
    "reason",
}
VALUE_KEYS = {"value_class_id", "status", "reason"}


def fail(message: str) -> None:
    raise SystemExit(f"[hmi-semantic-reference-inventory] {message}")


def read(path: Path) -> str:
    if not path.is_file():
        fail(f"missing source evidence: {path}")
    return path.read_text(errors="ignore")


def exact_keys(row: dict, expected: set[str], label: str) -> None:
    if set(row) != expected:
        fail(
            f"{label} schema drifted: "
            f"missing={sorted(expected - set(row))} extra={sorted(set(row) - expected)}"
        )


def unique_sorted(rows: list[dict], key: str, label: str) -> list[str]:
    ids = [row.get(key) for row in rows]
    if any(not isinstance(item, str) or not item for item in ids):
        fail(f"{label} contains an invalid ID")
    if ids != sorted(ids) or len(ids) != len(set(ids)):
        fail(f"{label} IDs must be unique and lexicographically sorted")
    return ids


def source_ref(root: Path, value: str, label: str) -> None:
    if "#" not in value:
        fail(f"{label} is not path#anchor: {value}")
    relative, anchor = value.split("#", 1)
    text = read(root / relative)
    if anchor not in text:
        fail(f"{label} anchor drifted: {value}")


def extract_kept_tags(root: Path) -> list[str]:
    text = read(root / "src/mir/contracts/backend_core_ops/vocabulary.rs")
    match = re.search(
        r"pub const MIR_INSTRUCTION_KEPT_TAGS[^=]*=\s*&\[(.*?)\];",
        text,
        re.S,
    )
    if not match:
        fail("cannot extract MIR_INSTRUCTION_KEPT_TAGS")
    tags = re.findall(r'"([A-Za-z][A-Za-z0-9_]*)"', match.group(1))
    if len(tags) != len(set(tags)):
        fail("Rust kept-tag vocabulary contains duplicates")
    return sorted(tags)


def extract_emitted_instruction_variants(root: Path) -> set[str]:
    text = read(root / "src/runner/mir_json_emit/emitters/mod.rs")
    return set(re.findall(r"\bI::([A-Z][A-Za-z0-9_]*)", text))


def extract_vm_value_variants(root: Path) -> list[str]:
    text = read(root / "src/backend/vm_types.rs")
    match = re.search(r"pub enum VMValue\s*\{(.*?)\n\}", text, re.S)
    if not match:
        fail("cannot extract VMValue variants")
    variants = re.findall(
        r"^\s*([A-Z][A-Za-z0-9_]*)\s*(?:\([^\n]*\)|,)",
        match.group(1),
        re.M,
    )
    return sorted(variants)


def production_constructor_files(root: Path) -> dict[str, int]:
    result = subprocess.run(
        ["git", "grep", "-l", "MirInterpreter::new", "--", "*.rs"],
        cwd=root,
        check=True,
        capture_output=True,
        text=True,
    )
    files: dict[str, int] = {}
    for relative in result.stdout.splitlines():
        name = Path(relative).name
        if relative.startswith("src/backend/mir_interpreter/"):
            continue
        if relative.startswith("src/tests/") or "/tests/" in relative:
            continue
        if name == "tests.rs" or name.endswith("_tests.rs"):
            continue
        text = read(root / relative)
        count = len(re.findall(r"\bMirInterpreter::new\s*\(", text))
        if count:
            files[relative] = count
    return files


def verify_instructions(root: Path, data: dict, fixture_ids: set[str], caller_ids: set[str], value_ids: set[str]) -> None:
    rows = data["instructions"]
    ids = unique_sorted(rows, "instruction_id", "instruction")
    kept = extract_kept_tags(root)
    if ids != kept:
        fail(
            f"kept instruction coverage drifted: "
            f"missing={sorted(set(kept) - set(ids))} stale={sorted(set(ids) - set(kept))}"
        )
    first_subset = data["first_subset"]
    if first_subset != sorted(first_subset) or len(first_subset) != len(set(first_subset)):
        fail("first_subset must be unique and sorted")
    selected = sorted(row["instruction_id"] for row in rows if row["first_subset"])
    if selected != first_subset:
        fail("first_subset root list and instruction flags disagree")

    transported: set[str] = set()
    for row in rows:
        exact_keys(row, INSTRUCTION_KEYS, f"instruction {row.get('instruction_id')}")
        if row["execution_family"] not in {"instruction", "phi", "terminator"}:
            fail(f"invalid execution family: {row['instruction_id']}")
        if row["lossiness"] not in {"conditional", "deferred", "lossless", "lossy", "not_transported"}:
            fail(f"invalid lossiness: {row['instruction_id']}")
        if row["transport_op"] is not None:
            if not isinstance(row["transport_op"], str) or not row["transport_op"]:
                fail(f"invalid transport op: {row['instruction_id']}")
            transported.add(row["instruction_id"])
        elif row["lossiness"] != "not_transported":
            fail(f"missing transport must be not_transported: {row['instruction_id']}")
        source_ref(root, row["semantic_owner"], f"semantic owner {row['instruction_id']}")
        if not row["dispatch_sites"] or len(row["dispatch_sites"]) != len(set(row["dispatch_sites"])):
            fail(f"dispatch sites must be nonempty and unique: {row['instruction_id']}")
        for site in row["dispatch_sites"]:
            source_ref(root, site, f"dispatch {row['instruction_id']}")
        for field in ("required_fields", "required_metadata", "loss_reasons"):
            if len(row[field]) != len(set(row[field])):
                fail(f"{field} must be unique: {row['instruction_id']}")
        for field, authority in (
            ("input_value_classes", value_ids),
            ("output_value_classes", value_ids),
            ("fixture_ids", fixture_ids),
            ("caller_ids", caller_ids),
        ):
            if len(row[field]) != len(set(row[field])) or not set(row[field]) <= authority:
                fail(f"invalid {field} foreign keys: {row['instruction_id']}")

    emitter_variants = extract_emitted_instruction_variants(root)
    if transported != emitter_variants:
        fail(
            f"MIR JSON emitter coverage drifted: "
            f"missing={sorted(emitter_variants - transported)} "
            f"stale={sorted(transported - emitter_variants)}"
        )


def verify_callers(root: Path, rows: list[dict]) -> set[str]:
    ids = set(unique_sorted(rows, "caller_id", "caller"))
    expected_constructor_files: dict[str, int] = {}
    for row in rows:
        exact_keys(row, CALLER_KEYS, f"caller {row.get('caller_id')}")
        if row["class"] not in {"product", "semantic_reference", "vm_only_compatibility"}:
            fail(f"invalid caller class: {row['caller_id']}")
        text = read(root / row["path"])
        if row["symbol"] not in text:
            fail(f"caller symbol drifted: {row['caller_id']}")
        count = row["constructor_occurrences"]
        if not isinstance(count, int) or count < 0:
            fail(f"invalid constructor count: {row['caller_id']}")
        if count:
            expected_constructor_files[row["path"]] = (
                expected_constructor_files.get(row["path"], 0) + count
            )
    actual = production_constructor_files(root)
    if actual != expected_constructor_files:
        fail(f"non-test MirInterpreter constructor callers drifted: expected={expected_constructor_files} actual={actual}")
    return ids


def verify_fixtures(root: Path, rows: list[dict]) -> set[str]:
    ids = set(unique_sorted(rows, "fixture_id", "fixture"))
    for row in rows:
        exact_keys(row, FIXTURE_KEYS, f"fixture {row.get('fixture_id')}")
        if not (root / row["path"]).exists() or not row["purpose"]:
            fail(f"fixture evidence drifted: {row['fixture_id']}")
    return ids


def verify_transports(root: Path, rows: list[dict]) -> None:
    unique_sorted(rows, "transport_id", "transport")
    for row in rows:
        exact_keys(row, TRANSPORT_KEYS, f"transport {row.get('transport_id')}")
        text = read(root / row["path"])
        if row["symbol"] not in text:
            fail(f"transport symbol drifted: {row['transport_id']}")
        if not row["classification"] or not row["lossiness"] or not row["reason"]:
            fail(f"incomplete transport row: {row['transport_id']}")


def verify_values(root: Path, rows: list[dict]) -> set[str]:
    ids = unique_sorted(rows, "value_class_id", "value class")
    variants = extract_vm_value_variants(root)
    if ids != variants:
        fail(f"VMValue variant coverage drifted: expected={variants} actual={ids}")
    for row in rows:
        exact_keys(row, VALUE_KEYS, f"value class {row.get('value_class_id')}")
        if row["status"] not in {"blocked_by_SSA-I1-O1", "deferred", "portable_S0"}:
            fail(f"invalid value-class status: {row['value_class_id']}")
        if not row["reason"]:
            fail(f"missing value-class reason: {row['value_class_id']}")
    return set(ids)


def render_report(data: dict) -> str:
    lines = [
        "# HMI Semantic Reference Inventory V1",
        "",
        f"Baseline: `{data['baseline']}`",
        "Production behavior delta: `0`",
        "",
        "## Summary",
        "",
        "| Inventory | Rows |",
        "| --- | ---: |",
        f"| Kept MIR instructions | {len(data['instructions'])} |",
        f"| Caller surfaces | {len(data['callers'])} |",
        f"| Fixture families | {len(data['fixtures'])} |",
        f"| Transports | {len(data['transports'])} |",
        f"| VMValue classes | {len(data['value_classes'])} |",
        "",
        "## Instruction coverage",
        "",
        "| Instruction | Family | First subset | V1 op | Lossiness |",
        "| --- | --- | --- | --- | --- |",
    ]
    for row in data["instructions"]:
        transport = row["transport_op"] or "—"
        first = "yes" if row["first_subset"] else "no"
        lines.append(
            f"| {row['instruction_id']} | {row['execution_family']} | "
            f"{first} | {transport} | {row['lossiness']} |"
        )

    lines.extend(
        [
            "",
            "## First-subset loss seams",
            "",
            "| Instruction | Required metadata | Loss reasons |",
            "| --- | --- | --- |",
        ]
    )
    for row in data["instructions"]:
        if not row["first_subset"]:
            continue
        metadata = ", ".join(row["required_metadata"]) or "—"
        reasons = "; ".join(row["loss_reasons"])
        lines.append(f"| {row['instruction_id']} | {metadata} | {reasons} |")

    lines.extend(
        [
            "",
            "## Caller classes",
            "",
            "| Caller | Class | Retirement condition |",
            "| --- | --- | --- |",
        ]
    )
    for row in data["callers"]:
        lines.append(
            f"| {row['caller_id']} | {row['class']} | {row['retirement_condition']} |"
        )

    lines.extend(
        [
            "",
            "## Transport lossiness",
            "",
            "| Transport | Classification | Lossiness | Reason |",
            "| --- | --- | --- | --- |",
        ]
    )
    for row in data["transports"]:
        lines.append(
            f"| {row['transport_id']} | {row['classification']} | "
            f"{row['lossiness']} | {row['reason']} |"
        )

    lines.extend(
        [
            "",
            "## VMValue classes",
            "",
            "| Class | Status | Reason |",
            "| --- | --- | --- |",
        ]
    )
    for row in data["value_classes"]:
        lines.append(
            f"| {row['value_class_id']} | {row['status']} | {row['reason']} |"
        )
    lines.extend(
        [
            "",
            "Generated from `hmi_semantic_reference_inventory_v1.json`; do not edit by hand.",
            "",
        ]
    )
    return "\n".join(lines)


def main() -> None:
    if len(sys.argv) not in {3, 4}:
        fail("usage: hmi_semantic_reference_inventory.py ROOT INVENTORY [REPORT]")
    root = Path(sys.argv[1]).resolve()
    inventory = Path(sys.argv[2]).resolve()
    data = json.loads(read(inventory))
    expected_root = {
        "schema",
        "schema_version",
        "baseline",
        "production_behavior_delta",
        "decision",
        "first_subset",
        "instructions",
        "callers",
        "fixtures",
        "transports",
        "value_classes",
    }
    if set(data) != expected_root:
        fail("top-level schema drifted")
    if data["schema"] != SCHEMA or data["schema_version"] != 1:
        fail("schema identity drifted")
    if data["production_behavior_delta"] != 0:
        fail("HMI-P0-I0 must remain behavior-neutral")
    if data["decision"] != "inventory_only_no_execution_owner_or_opcode_activation":
        fail("decision row drifted")

    caller_ids = verify_callers(root, data["callers"])
    fixture_ids = verify_fixtures(root, data["fixtures"])
    verify_transports(root, data["transports"])
    value_ids = verify_values(root, data["value_classes"])
    verify_instructions(root, data, fixture_ids, caller_ids, value_ids)

    report_text = render_report(data)
    if len(sys.argv) == 4:
        if sys.argv[3] == "--print-report":
            print(report_text, end="")
            return
        report = Path(sys.argv[3]).resolve()
        if read(report) != report_text:
            fail(f"normalized report drifted: {report}")

    checked_paths = [Path(__file__), inventory]
    if len(sys.argv) == 4:
        checked_paths.append(Path(sys.argv[3]).resolve())
    for path in checked_paths:
        lines = len(path.read_text().splitlines())
        if lines >= 800:
            fail(f"source/check file reached 800 lines: {path} lines={lines}")
    print(
        "[hmi-semantic-reference-inventory] ok "
        f"instructions={len(data['instructions'])} callers={len(data['callers'])} "
        f"fixtures={len(data['fixtures'])} transports={len(data['transports'])} "
        f"value_classes={len(data['value_classes'])} behavior_delta=0"
    )


if __name__ == "__main__":
    main()
