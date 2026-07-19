#!/usr/bin/env python3
"""Validate the behavior-neutral MirBuilder function-session census."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path


ROOT = Path(__file__).resolve().parents[3]
FIXTURE = ROOT / "tools/checks/fixtures/mirbuilder_fsession_census_v1.json"
FUNCTION_STATE = ROOT / "src/mir/builder/function_lowering_state.rs"
CLASSES = {
    "ModuleImmutable",
    "ModulePublication",
    "FunctionOwned",
    "ObservationBorrow",
    "LegacyCompatibility",
}


def fail(message: str) -> None:
    raise SystemExit(f"[mirbuilder-fsession-census] {message}")


def struct_fields(path: Path, struct_name: str) -> set[str]:
    lines = path.read_text().splitlines()
    start = next((i for i, line in enumerate(lines) if f"struct {struct_name}" in line), None)
    if start is None:
        fail(f"missing struct {struct_name}: {path.relative_to(ROOT)}")
    fields: set[str] = set()
    depth = 0
    opened = False
    for line in lines[start:]:
        if "{" in line:
            depth += line.count("{")
            opened = True
        if opened and depth == 1:
            match = re.match(r" {4}(?:pub(?:\([^)]*\))?\s+)?([A-Za-z_]\w*)\s*:", line)
            if match:
                fields.add(match.group(1))
        if "}" in line:
            depth -= line.count("}")
            if opened and depth == 0:
                break
    return fields


def required_string(row: dict[str, object], key: str, label: str) -> str:
    value = row.get(key)
    if not isinstance(value, str) or not value:
        fail(f"{label} missing non-empty {key}")
    return value


def main() -> None:
    data = json.loads(FIXTURE.read_text())
    if data.get("schema") != "MirBuilderFunctionSessionCensusV1":
        fail("unexpected schema")
    if data.get("decision") != "fsession_census_only":
        fail("census must remain behavior-neutral")
    if set(data.get("ownership_classes", [])) != CLASSES:
        fail("ownership class vocabulary drift")

    sources = data.get("sources")
    if not isinstance(sources, dict):
        fail("missing sources")
    source_text: dict[str, str] = {}
    for key, relative in sources.items():
        if not isinstance(relative, str):
            fail(f"source path is not a string: {key}")
        path = ROOT / relative
        if not path.is_file():
            fail(f"source path missing: {relative}")
        source_text[key] = path.read_text()

    surfaces = data.get("surfaces")
    if not isinstance(surfaces, list) or not surfaces:
        fail("missing surfaces")
    if len(surfaces) != 41:
        fail(f"expected 41 expanded snapshot surfaces, found {len(surfaces)}")
    ids: set[str] = set()
    paths: set[str] = set()
    roots: dict[str, set[str]] = {}
    snapshot_fields: set[str] = set()
    type_leaves: set[str] = set()
    scope_leaves: set[str] = set()
    for row in surfaces:
        if not isinstance(row, dict):
            fail("surface row is not an object")
        identifier = required_string(row, "id", "surface")
        path = required_string(row, "path", identifier)
        root = required_string(row, "root_field", identifier)
        snapshot = required_string(row, "snapshot_field", identifier)
        ownership = required_string(row, "class", identifier)
        if identifier in ids or path in paths:
            fail(f"duplicate surface id or path: {identifier} / {path}")
        if ownership not in CLASSES:
            fail(f"unknown ownership class for {identifier}: {ownership}")
        for key in ("prepare_anchor", "restore_anchor", "legacy_action", "box_context_action", "target", "note"):
            anchor = required_string(row, key, identifier)
            if key in {"prepare_anchor", "restore_anchor"} and anchor not in source_text["lifecycle"]:
                fail(f"stale {key} for {identifier}: {anchor}")
        ids.add(identifier)
        paths.add(path)
        roots.setdefault(root, set()).add(identifier)
        snapshot_fields.add(snapshot)
        if snapshot == "saved_type_ctx":
            type_leaves.add(required_string(row, "snapshot_leaf", identifier))
        if snapshot == "saved_scope_stacks":
            scope_leaves.add(required_string(row, "snapshot_leaf", identifier))

    lifecycle = ROOT / sources["lifecycle"]
    lifecycle_fields = struct_fields(lifecycle, "LoweringContext")
    expected_lifecycle = {"context_active"} | {field for field in lifecycle_fields if field.startswith("saved_")}
    if lifecycle_fields != expected_lifecycle:
        fail(f"unexpected LoweringContext field outside census grammar: {sorted(lifecycle_fields - expected_lifecycle)}")
    if snapshot_fields != expected_lifecycle:
        missing = sorted(expected_lifecycle - snapshot_fields)
        extra = sorted(snapshot_fields - expected_lifecycle)
        fail(f"LoweringContext coverage mismatch missing={missing} extra={extra}")

    expected_scope = struct_fields(lifecycle, "ScopeStacksSnapshot")
    if scope_leaves != expected_scope:
        fail(f"ScopeStacksSnapshot coverage mismatch missing={sorted(expected_scope - scope_leaves)} extra={sorted(scope_leaves - expected_scope)}")
    type_context = ROOT / sources["type_context"]
    expected_type = struct_fields(type_context, "TypeContextSnapshot")
    if type_leaves != expected_type:
        fail(f"TypeContextSnapshot coverage mismatch missing={sorted(expected_type - type_leaves)} extra={sorted(type_leaves - expected_type)}")
    for row in surfaces:
        if row["snapshot_field"] != "saved_type_ctx":
            continue
        leaf = row["snapshot_leaf"]
        clear_statement = f"self.type_ctx.{leaf}.clear();"
        action = row["box_context_action"]
        if action == "clear" and clear_statement not in source_text["lifecycle"]:
            fail(f"BoxCompilationContext clear missing for {leaf}")
        if action == "not_cleared" and clear_statement in source_text["lifecycle"]:
            fail(f"BoxCompilationContext handling changed for {leaf}; update census deliberately")

    builder = ROOT / sources["builder"]
    expected_builder = struct_fields(builder, "MirBuilder")
    builder_fields = data.get("builder_fields")
    if not isinstance(builder_fields, list):
        fail("missing builder_fields")
    manifest_builder: set[str] = set()
    for row in builder_fields:
        if not isinstance(row, dict):
            fail("builder field row is not an object")
        field = required_string(row, "field", "builder field")
        coverage = required_string(row, "coverage", field)
        if field in manifest_builder:
            fail(f"duplicate builder field: {field}")
        if coverage not in {"surface", "decomposed", "outside_session"}:
            fail(f"unknown builder coverage for {field}: {coverage}")
        listed_ids = row.get("surface_ids", [])
        if not isinstance(listed_ids, list) or not all(isinstance(item, str) for item in listed_ids):
            fail(f"invalid surface_ids for {field}")
        if coverage == "decomposed" and set(listed_ids) != roots.get(field, set()):
            fail(f"decomposed builder coverage mismatch for {field}")
        if coverage == "surface" and set(listed_ids) != roots.get(field, set()):
            fail(f"surface builder coverage mismatch for {field}")
        if coverage == "outside_session":
            ownership = required_string(row, "class", field)
            if ownership not in CLASSES:
                fail(f"unknown outside-session class for {field}: {ownership}")
        manifest_builder.add(field)
    if manifest_builder != expected_builder:
        fail(f"MirBuilder field coverage mismatch missing={sorted(expected_builder - manifest_builder)} extra={sorted(manifest_builder - expected_builder)}")

    gaps = data.get("uncovered_function_state", [])
    if not isinstance(gaps, list):
        fail("uncovered_function_state is not a list")
    if len(gaps) != 2:
        fail(f"expected two uncovered ValueId-keyed metadata surfaces, found {len(gaps)}")
    gap_ids: set[str] = set()
    for row in gaps:
        if not isinstance(row, dict):
            fail("uncovered state row is not an object")
        identifier = required_string(row, "id", "uncovered state")
        if identifier in ids or identifier in gap_ids:
            fail(f"duplicate uncovered state id: {identifier}")
        if required_string(row, "class", identifier) not in CLASSES:
            fail(f"unknown uncovered state class: {identifier}")
        required_string(row, "path", identifier)
        required_string(row, "current_handling", identifier)
        required_string(row, "target", identifier)
        metadata_field = required_string(row, "path", identifier).split(".")[-1]
        if metadata_field not in source_text["metadata_context"]:
            fail(f"uncovered metadata field no longer exists: {metadata_field}")
        gap_ids.add(identifier)
    expected_gaps = {"metadata.value_origin_spans", "metadata.value_origin_callers"}
    if gap_ids != expected_gaps:
        fail(f"uncovered metadata inventory mismatch missing={sorted(expected_gaps - gap_ids)} extra={sorted(gap_ids - expected_gaps)}")

    validate_s0a_function_state_vocabulary(builder)

    print(
        "[mirbuilder-fsession-census] ok "
        f"snapshot_surfaces={len(surfaces)} builder_fields={len(builder_fields)} uncovered={len(gaps)}"
    )


def validate_s0a_function_state_vocabulary(builder: Path) -> None:
    if not FUNCTION_STATE.is_file():
        fail("missing S0a function-state vocabulary")
    state_text = FUNCTION_STATE.read_text()
    state_code = "\n".join(
        line for line in state_text.splitlines() if not line.lstrip().startswith("//")
    )
    builder_text = builder.read_text()
    if "mod function_lowering_state;" not in builder_text:
        fail("MirBuilder does not declare the S0a function-state vocabulary")
    if "function_state" in struct_fields(builder, "MirBuilder"):
        fail("S0a must not install FunctionLoweringStateV1 in MirBuilder")
    if re.search(r"\b(?:Deref|DerefMut)\b", state_code):
        fail("S0a function-state vocabulary must not expose Deref compatibility")
    if "pub(crate)" in state_code or re.search(r"\bpub\s+struct\b", state_code):
        fail("S0a function-state vocabulary must stay builder-private")

    expected_fields = {
        "FunctionLoweringStateV1": {
            "current_function",
            "current_block",
            "variable_ctx",
            "type_ctx",
            "binding_ctx",
            "resolved_binding_state",
            "scope",
            "compilation",
            "value_origins",
            "pending_phis",
            "local_ssa_map",
            "schedule_mat_map",
            "pin_slot_names",
            "frag_emit_session",
            "return_defer_active",
            "return_defer_slot",
            "return_defer_target",
            "return_deferred_emitted",
            "in_cleanup_block",
            "cleanup_allow_return",
            "cleanup_allow_throw",
            "suppress_pin_entry_copy_next",
            "in_unified_boxcall_fallback",
        },
        "FunctionScopeStateV1": {
            "lexical_scope_stack",
            "loop_header_stack",
            "loop_exit_stack",
            "if_merge_stack",
            "function_param_names",
            "fastmem_region_stack",
        },
        "FunctionCompilationScratchV1": {
            "reserved_value_ids",
            "fn_body_ast",
            "record_local_values",
        },
        "FunctionValueOriginFactsV1": {
            "value_origin_spans",
            "value_origin_callers",
        },
    }
    for name, expected in expected_fields.items():
        actual = struct_fields(FUNCTION_STATE, name)
        if actual != expected:
            fail(
                f"S0a vocabulary partition mismatch for {name} "
                f"missing={sorted(expected - actual)} extra={sorted(actual - expected)}"
            )


if __name__ == "__main__":
    main()
