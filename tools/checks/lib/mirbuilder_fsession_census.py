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
    validate_s0a_s0b_route_map(data, sources, builder)
    validate_s0a_mixed_context_api_owners(data, sources)

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


def validate_s0a_s0b_route_map(
    data: dict[str, object],
    sources: dict[str, object],
    builder: Path,
) -> None:
    routes = data.get("function_state_routes")
    if not isinstance(routes, list):
        fail("missing S0b FunctionOwned route map")

    expected = {
        "current_block": ("builder.current_block", "function_state.current_block"),
        "type_ctx": ("builder.type_ctx", "function_state.type_ctx"),
        "variable_ctx": ("builder.variable_ctx", "function_state.variable_ctx"),
        "binding_ctx": ("builder.binding_ctx", "function_state.binding_ctx"),
        "resolved_binding_state": (
            "builder.resolved_binding_state",
            "function_state.resolved_binding_state",
        ),
        "scope.current_function": (
            "scope_ctx.current_function",
            "function_state.current_function",
        ),
        "scope.lexical_scope_stack": (
            "scope_ctx.lexical_scope_stack",
            "function_state.scope.lexical_scope_stack",
        ),
        "scope.loop_header_stack": (
            "scope_ctx.loop_header_stack",
            "function_state.scope.loop_header_stack",
        ),
        "scope.loop_exit_stack": (
            "scope_ctx.loop_exit_stack",
            "function_state.scope.loop_exit_stack",
        ),
        "scope.if_merge_stack": (
            "scope_ctx.if_merge_stack",
            "function_state.scope.if_merge_stack",
        ),
        "scope.function_param_names": (
            "scope_ctx.function_param_names",
            "function_state.scope.function_param_names",
        ),
        "scope.fastmem_region_stack": (
            "scope_ctx.fastmem_region_stack",
            "function_state.scope.fastmem_region_stack",
        ),
        "scope.entry_clear": (
            "scope_ctx.clear_for_function_entry",
            "function_state.scope",
        ),
        "compilation.reserved_value_ids": (
            "comp_ctx.reserved_value_ids",
            "function_state.compilation.reserved_value_ids",
        ),
        "compilation.fn_body_ast": (
            "comp_ctx.fn_body_ast",
            "function_state.compilation.fn_body_ast",
        ),
        "compilation.record_local_values": (
            "comp_ctx.record_local_values",
            "function_state.compilation.record_local_values",
        ),
        "value_origins.spans": (
            "metadata_ctx.value_origin_spans",
            "function_state.value_origins.value_origin_spans",
        ),
        "value_origins.callers": (
            "metadata_ctx.value_origin_callers",
            "function_state.value_origins.value_origin_callers",
        ),
        "pending_phis": ("builder.pending_phis", "function_state.pending_phis"),
        "local_ssa_map": ("builder.local_ssa_map", "function_state.local_ssa_map"),
        "schedule_mat_map": (
            "builder.schedule_mat_map",
            "function_state.schedule_mat_map",
        ),
        "pin_slot_names": ("builder.pin_slot_names", "function_state.pin_slot_names"),
        "frag_emit_session": (
            "builder.frag_emit_session",
            "function_state.frag_emit_session",
        ),
        "return_defer_active": (
            "builder.return_defer_active",
            "function_state.return_defer_active",
        ),
        "return_defer_slot": (
            "builder.return_defer_slot",
            "function_state.return_defer_slot",
        ),
        "return_defer_target": (
            "builder.return_defer_target",
            "function_state.return_defer_target",
        ),
        "return_deferred_emitted": (
            "builder.return_deferred_emitted",
            "function_state.return_deferred_emitted",
        ),
        "in_cleanup_block": (
            "builder.in_cleanup_block",
            "function_state.in_cleanup_block",
        ),
        "cleanup_allow_return": (
            "builder.cleanup_allow_return",
            "function_state.cleanup_allow_return",
        ),
        "cleanup_allow_throw": (
            "builder.cleanup_allow_throw",
            "function_state.cleanup_allow_throw",
        ),
        "suppress_pin_entry_copy_next": (
            "builder.suppress_pin_entry_copy_next",
            "function_state.suppress_pin_entry_copy_next",
        ),
        "in_unified_boxcall_fallback": (
            "builder.in_unified_boxcall_fallback",
            "function_state.in_unified_boxcall_fallback",
        ),
    }
    actual: dict[str, tuple[str, str]] = {}
    for row in routes:
        if not isinstance(row, dict):
            fail("S0b route row is not an object")
        selector = required_string(row, "selector", "S0b route")
        if selector in actual:
            fail(f"duplicate S0b route selector: {selector}")
        actual[selector] = (
            required_string(row, "old_storage", selector),
            required_string(row, "destination", selector),
        )
    if actual != expected:
        fail(
            "S0b route map drift "
            f"missing={sorted(expected.keys() - actual.keys())} "
            f"extra={sorted(actual.keys() - expected.keys())}"
        )

    source_paths = {key: ROOT / value for key, value in sources.items() if isinstance(value, str)}
    required_sources = {"type_context", "scope_context", "compilation_context", "metadata_context"}
    if required_sources - source_paths.keys():
        fail(f"S0b route sources missing: {sorted(required_sources - source_paths.keys())}")
    old_storage = {
        "type_context": {
            "value_types",
            "value_kinds",
            "value_origin_newbox",
            "string_literals",
            "map_value_types",
            "map_literal_value_types",
        },
        "scope_context": {
            "current_function",
            "lexical_scope_stack",
            "loop_header_stack",
            "loop_exit_stack",
            "if_merge_stack",
            "function_param_names",
            "fastmem_region_stack",
        },
        "compilation_context": {
            "reserved_value_ids",
            "fn_body_ast",
            "record_local_values",
        },
        "metadata_context": {"value_origin_spans", "value_origin_callers"},
    }
    struct_names = {
        "type_context": "TypeContext",
        "scope_context": "ScopeContext",
        "compilation_context": "CompilationContext",
        "metadata_context": "MetadataContext",
    }
    for source, fields in old_storage.items():
        actual_fields = struct_fields(source_paths[source], struct_names[source])
        if not fields <= actual_fields:
            fail(
                f"S0a old storage missing from {struct_names[source]}: "
                f"{sorted(fields - actual_fields)}"
            )

    builder_fields = struct_fields(builder, "MirBuilder")
    direct_builder = {
        old.split(".", 1)[1]
        for old, _destination in actual.values()
        if old.startswith("builder.")
    }
    if not direct_builder <= builder_fields:
        fail(f"S0a old Builder storage missing: {sorted(direct_builder - builder_fields)}")


def validate_s0a_mixed_context_api_owners(
    data: dict[str, object],
    sources: dict[str, object],
) -> None:
    owners = data.get("mixed_context_api_owners")
    if not isinstance(owners, list):
        fail("missing mixed-context API owner manifest")

    expected = {
        "scope.lexical_helpers": {
            "selector": "scope.lexical_scope_stack",
            "owner_source": "scope_context",
            "owner_type": "ScopeContext",
            "methods": {"push_lexical_scope", "pop_lexical_scope", "current_scope_mut"},
        },
        "scope.if_merge_helpers": {
            "selector": "scope.if_merge_stack",
            "owner_source": "scope_context",
            "owner_type": "ScopeContext",
            "methods": {"push_if_merge", "pop_if_merge"},
        },
        "scope.fastmem_helpers": {
            "selector": "scope.fastmem_region_stack",
            "owner_source": "scope_context",
            "owner_type": "ScopeContext",
            "methods": {"push_fastmem_region", "pop_fastmem_region", "current_fastmem_region"},
        },
        "scope.entry_clear": {
            "selector": "scope.entry_clear",
            "owner_source": "scope_context",
            "owner_type": "ScopeContext",
            "methods": {"clear_for_function_entry"},
            "function_owned_clears": {
                "lexical_scope_stack",
                "loop_header_stack",
                "loop_exit_stack",
                "if_merge_stack",
                "fastmem_region_stack",
            },
            "observation_clears": {"debug_scope_stack"},
            "s0b_policy": "split_mixed_clear_preserving_current_behavior",
        },
        "compilation.reservation_helpers": {
            "selector": "compilation.reserved_value_ids",
            "owner_source": "compilation_context",
            "owner_type": "CompilationContext",
            "methods": {
                "is_reserved_value_id",
                "reserve_value_id",
                "clear_reserved_value_ids",
            },
        },
        "compilation.fn_body_helpers": {
            "selector": "compilation.fn_body_ast",
            "owner_source": "compilation_context",
            "owner_type": "CompilationContext",
            "methods": {"set_fn_body_ast", "take_fn_body_ast", "clear_fn_body_ast"},
        },
        "compilation.record_local_helpers": {
            "selector": "compilation.record_local_values",
            "owner_source": "compilation_declarations",
            "owner_type": "CompilationContext",
            "methods": {
                "register_record_local_value",
                "record_local_value",
                "propagate_record_local_value",
                "propagate_record_local_value_from_phi",
                "clear_record_local_values",
            },
        },
        "metadata.origin_span_helpers": {
            "selector": "value_origins.spans",
            "owner_source": "metadata_context",
            "owner_type": "MetadataContext",
            "methods": {"record_value_span", "value_span"},
        },
        "metadata.origin_caller_helpers": {
            "selector": "value_origins.callers",
            "owner_source": "metadata_context",
            "owner_type": "MetadataContext",
            "methods": {"record_value_caller", "value_caller", "value_origin_callers"},
        },
    }
    rows: dict[str, dict[str, object]] = {}
    for row in owners:
        if not isinstance(row, dict):
            fail("mixed-context API owner row is not an object")
        identifier = required_string(row, "id", "mixed-context API owner")
        if identifier in rows:
            fail(f"duplicate mixed-context API owner: {identifier}")
        rows[identifier] = row
    if set(rows) != set(expected):
        fail(
            "mixed-context API owner manifest drift "
            f"missing={sorted(set(expected) - set(rows))} "
            f"extra={sorted(set(rows) - set(expected))}"
        )

    route_selectors = {
        required_string(row, "selector", "S0b route")
        for row in data["function_state_routes"]
        if isinstance(row, dict)
    }
    for identifier, contract in expected.items():
        row = rows[identifier]
        methods = row.get("methods")
        if not isinstance(methods, list) or not all(isinstance(value, str) for value in methods):
            fail(f"mixed-context API owner lists invalid: {identifier}")
        actual = {
            "selector": required_string(row, "selector", identifier),
            "owner_source": required_string(row, "owner_source", identifier),
            "owner_type": required_string(row, "owner_type", identifier),
            "methods": set(methods),
        }
        if identifier == "scope.entry_clear":
            owned = row.get("function_owned_clears")
            observation = row.get("observation_clears")
            if not isinstance(owned, list) or not isinstance(observation, list):
                fail("scope.entry_clear clear lists invalid")
            actual["function_owned_clears"] = set(owned)
            actual["observation_clears"] = set(observation)
            actual["s0b_policy"] = required_string(row, "s0b_policy", identifier)
        if actual != contract:
            fail(f"mixed-context API owner contract drift: {identifier}")
        if actual["selector"] not in route_selectors:
            fail(f"mixed-context API owner selector absent from Census routes: {identifier}")
        source_key = actual["owner_source"]
        source = sources.get(source_key)
        if not isinstance(source, str):
            fail(f"mixed-context API owner source missing: {identifier} -> {source_key}")
        source_text = (ROOT / source).read_text()
        if not re.search(
            rf"\bimpl(?:<[^>]+>)?\s+{re.escape(actual['owner_type'])}\b",
            source_text,
        ):
            fail(f"mixed-context API owner impl missing: {identifier}.{actual['owner_type']}")
        for method in actual["methods"]:
            matches = re.findall(rf"\bfn\s+{re.escape(method)}\s*\(", source_text)
            if len(matches) != 1:
                fail(f"mixed-context API method definition drift: {identifier}.{method}")

    lifecycle = (ROOT / required_string(sources, "lifecycle", "sources")).read_text()
    for field in ("value_origin_spans", "value_origin_callers"):
        if field in lifecycle:
            fail(f"metadata origin isolation changed before METAISO: {field}")
    clear_source = (ROOT / required_string(sources, "scope_context", "sources")).read_text()
    clear_body = re.search(
        r"fn\s+clear_for_function_entry\s*\([^)]*\)\s*\{(?P<body>.*?)\n    \}",
        clear_source,
        re.DOTALL,
    )
    if clear_body is None:
        fail("missing scope.entry_clear body")
    cleared = set(re.findall(r"self\.(\w+)\.clear\(\);", clear_body.group("body")))
    if cleared != expected["scope.entry_clear"]["function_owned_clears"] | expected["scope.entry_clear"]["observation_clears"]:
        fail(f"scope.entry_clear body drift: {sorted(cleared)}")


if __name__ == "__main__":
    main()
