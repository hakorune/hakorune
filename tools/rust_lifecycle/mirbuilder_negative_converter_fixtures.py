#!/usr/bin/env python3
"""Reusable negative fixture corpus for MirBuilder converter fail-closed checks."""

from __future__ import annotations

import argparse
import copy
import json
import re
import subprocess
import sys
from pathlib import Path
from typing import Any, Callable


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
GENERATED = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
CORPUS = FIXTURES / "mirbuilder-negative-converter-fixtures-v0.json"


def _load_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text())


def _denied_map(facts: dict[str, Any], key: str) -> dict[str, Any]:
    rows = {row["id"]: row for row in facts.get(key, [])}
    return rows


def _replace_row(rows: list[dict[str, Any]], row_id: str, field: str, value: Any) -> None:
    for row in rows:
        if row.get("id") == row_id:
            row[field] = value
            return
    raise SystemExit(f"missing row: {row_id}")


def _deny_reason(exc: BaseException) -> str:
    reason = getattr(exc, "reason", None)
    if isinstance(reason, str):
        return reason
    match = re.search(r"Deny\(([^)]+)\)", str(exc))
    if match:
        return match.group(1)
    raise SystemExit(f"unable to read deny reason from: {exc!r}")


def _expect_deny(expected_reason: str, thunk: Callable[[], None]) -> None:
    try:
        thunk()
    except BaseException as exc:
        if _deny_reason(exc) != expected_reason:
            raise SystemExit(f"unexpected deny reason: {_deny_reason(exc)} != {expected_reason}") from exc
        return
    raise SystemExit(f"expected Deny({expected_reason})")


def _load_guard_output(script: str) -> str:
    result = subprocess.run(["bash", str(ROOT / script)], capture_output=True, text=True, check=False)
    if result.returncode != 0:
        raise SystemExit(result.stdout + result.stderr)
    return result.stdout


def _load_corpus() -> tuple[list[str], dict[str, str]]:
    corpus = _load_json(CORPUS)
    if corpus.get("kind") != "MirBuilderNegativeConverterFixtureCorpus":
        raise SystemExit("unexpected negative fixture corpus kind")
    cases = corpus.get("cases")
    if not isinstance(cases, list) or not cases:
        raise SystemExit("unexpected negative fixture corpus cases")

    ordered: list[str] = []
    statuses: dict[str, str] = {}
    seen: set[str] = set()
    for case in cases:
        if not isinstance(case, dict):
            raise SystemExit("unexpected negative fixture corpus entry")
        case_id = case.get("id")
        status = case.get("status")
        if not isinstance(case_id, str) or not case_id:
            raise SystemExit("negative fixture corpus entry missing id")
        if case_id in seen:
            raise SystemExit(f"duplicate negative fixture corpus case: {case_id}")
        if case_id not in CASE_RUNNERS:
            raise SystemExit(f"unknown negative fixture corpus case: {case_id}")
        if status not in {"green", "parked"}:
            raise SystemExit(f"unexpected negative fixture corpus status: {case_id}")
        ordered.append(case_id)
        statuses[case_id] = status
        seen.add(case_id)

    missing = sorted(set(CASE_RUNNERS) - seen)
    if missing:
        raise SystemExit(f"missing negative fixture corpus cases: {missing}")

    extra = sorted(seen - set(CASE_RUNNERS))
    if extra:
        raise SystemExit(f"unexpected negative fixture corpus cases: {extra}")

    return ordered, statuses


def _converter_case(
    *,
    name: str,
    facts_path: str,
    plan_path: str,
    method_id: str,
    field: str,
    operation: str,
    expected_reason: str,
    compiler: Callable[[dict[str, Any], dict[str, Any]], None],
) -> tuple[str, str]:
    facts = copy.deepcopy(_load_json(FIXTURES / facts_path))
    plan = copy.deepcopy(_load_json(FIXTURES / plan_path))
    _replace_row(facts[field], method_id, "operation", operation)
    _expect_deny(expected_reason, lambda: compiler(facts, plan))
    return name, expected_reason


def _binding_context_case() -> tuple[str, str]:
    from extract_binding_context_facts import SOURCE, extract_facts
    from mirbuilder_ordered_map_converter import compile_binding_context_methods

    facts = copy.deepcopy(extract_facts(SOURCE))
    plan = copy.deepcopy(_load_json(FIXTURES / "binding-context-plan-v0.json"))
    _replace_row(facts["body_facts"], "BindingContext::lookup", "operation", "UnexpectedMapGet")
    _expect_deny("UnsupportedResolvedCallTarget", lambda: compile_binding_context_methods(facts, plan))
    return "binding_context_unsupported_resolved_call_target", "UnsupportedResolvedCallTarget"


def _simple_map_case() -> tuple[str, str]:
    from mirbuilder_ordered_map_converter import compile_variable_context_simple_map_methods

    return _converter_case(
        name="variable_context_simple_map_unsupported_resolved_call_target",
        facts_path="variable-context-simple-map-facts-v0.json",
        plan_path="variable-context-simple-map-plan-v0.json",
        method_id="VariableContext::lookup",
        field="body_facts",
        operation="UnexpectedMapGet",
        expected_reason="UnsupportedResolvedCallTarget",
        compiler=lambda facts, plan: compile_variable_context_simple_map_methods(facts, plan),
    )


def _snapshot_restore_case() -> tuple[str, str]:
    from mirbuilder_ordered_map_converter import compile_variable_context_snapshot_restore_methods

    return _converter_case(
        name="variable_context_snapshot_restore_unsupported_resolved_call_target",
        facts_path="variable-context-snapshot-restore-facts-v0.json",
        plan_path="variable-context-snapshot-restore-plan-v0.json",
        method_id="VariableContext::snapshot",
        field="body_facts",
        operation="UnexpectedClone",
        expected_reason="UnsupportedResolvedCallTarget",
        compiler=lambda facts, plan: compile_variable_context_snapshot_restore_methods(facts, plan),
    )


def _structured_loop_carried_state_case() -> tuple[str, str]:
    from mirbuilder_structured_loop_converter import compile_structured_loop_without_carried_state_methods

    facts = copy.deepcopy(_load_json(FIXTURES / "structured-loop-without-carried-state-facts-v0.json"))
    plan = copy.deepcopy(_load_json(FIXTURES / "structured-loop-without-carried-state-plan-v0.json"))
    _replace_row(facts["body_facts"], "StructuredLoopPilot::copy_values", "loop_carried_state", True)
    _expect_deny(
        "LoopCarriedStateRequired",
        lambda: compile_structured_loop_without_carried_state_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.structured_loop_without_carried_state"],
        ),
    )
    return "structured_loop_carried_state_required", "LoopCarriedStateRequired"


def _structured_loop_unstructured_control_case() -> tuple[str, str]:
    from mirbuilder_structured_loop_converter import compile_structured_loop_without_carried_state_methods

    facts = copy.deepcopy(_load_json(FIXTURES / "structured-loop-without-carried-state-facts-v0.json"))
    plan = copy.deepcopy(_load_json(FIXTURES / "structured-loop-without-carried-state-plan-v0.json"))
    _replace_row(facts["body_facts"], "StructuredLoopPilot::copy_values", "break_count", 1)
    _expect_deny(
        "UnstructuredControlFlow",
        lambda: compile_structured_loop_without_carried_state_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.structured_loop_without_carried_state"],
        ),
    )
    return "structured_loop_unstructured_control_flow", "UnstructuredControlFlow"


def _single_scalar_loop_phi_case() -> tuple[str, str]:
    from mirbuilder_structured_loop_converter import compile_single_scalar_loop_carrier_methods

    facts = copy.deepcopy(_load_json(FIXTURES / "single-scalar-loop-carrier-facts-v0.json"))
    plan = copy.deepcopy(_load_json(FIXTURES / "single-scalar-loop-carrier-plan-v0.json"))
    _replace_row(facts["body_facts"], "SingleScalarLoopCarrierPilot::sum_values", "phi_required", True)
    _expect_deny(
        "PhiJoinRequired",
        lambda: compile_single_scalar_loop_carrier_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.single_scalar_loop_carrier"],
        ),
    )
    return "single_scalar_loop_phi_required", "PhiJoinRequired"


def _single_scalar_loop_carrier_escape_case() -> tuple[str, str]:
    from mirbuilder_structured_loop_converter import compile_single_scalar_loop_carrier_methods

    facts = copy.deepcopy(_load_json(FIXTURES / "single-scalar-loop-carrier-facts-v0.json"))
    plan = copy.deepcopy(_load_json(FIXTURES / "single-scalar-loop-carrier-plan-v0.json"))
    facts["body_facts"][0]["carrier"]["escapes"] = True
    _expect_deny(
        "CarrierSensitiveAlias",
        lambda: compile_single_scalar_loop_carrier_methods(
            facts,
            plan,
            **plan["direct_shape"]["control.single_scalar_loop_carrier"],
        ),
    )
    return "single_scalar_loop_carrier_escape", "CarrierSensitiveAlias"


def _returned_read_borrow_case() -> tuple[str, str]:
    for facts_name, subject in [
        ("variable-context-carrier-snapshot-facts-v0.json", "CarrierInfo.from_variable_map"),
        ("variable-context-explicit-carrier-snapshot-facts-v0.json", "CarrierInfo.with_explicit_carriers"),
    ]:
        facts = _load_json(FIXTURES / facts_name)
        method = facts["method_fact"]
        if method["input_snapshot"]["ownership"] != "OwnedReadSnapshotProjection":
            raise SystemExit(f"unexpected ownership in {facts_name}")
        if method["input_snapshot"]["access"] != "read" or method["input_snapshot"]["escapes"] is not False:
            raise SystemExit(f"unexpected snapshot access in {facts_name}")
        denied = _denied_map(facts, "denied_methods")
        if denied["VariableContext::variable_map"]["deny_reason"] != "ReturnedReadBorrow":
            raise SystemExit(f"ReturnedReadBorrow missing in {facts_name}")
        if subject not in facts["subject"]:
            raise SystemExit(f"unexpected subject in {facts_name}")
    return "returned_read_borrow", "ReturnedReadBorrow"


def _returned_mutable_borrow_case() -> tuple[str, str]:
    snapshot_restore = _load_json(FIXTURES / "variable-context-snapshot-restore-facts-v0.json")
    immutable_borrow = _load_json(FIXTURES / "variable-context-immutable-borrow-facts-v0.json")
    if _denied_map(snapshot_restore, "denied_methods")["VariableContext::variable_map_mut"]["deny_reason"] != "ReturnedMutableBorrow":
        raise SystemExit("ReturnedMutableBorrow missing in snapshot/restore fixture")
    if _denied_map(immutable_borrow, "denied_methods")["VariableContext::variable_map_mut"]["deny_reason"] != "ReturnedMutableBorrow":
        raise SystemExit("ReturnedMutableBorrow missing in immutable-borrow fixture")
    return "returned_mutable_borrow", "ReturnedMutableBorrow"


def _carrier_sensitive_alias_case() -> tuple[str, str]:
    return "carrier_sensitive_alias", "CarrierSensitiveAlias"


def _missing_requested_carrier_case() -> tuple[str, str]:
    oracle = _load_json(FIXTURES / "variable-context-explicit-carrier-snapshot-oracle-vectors-v0.json")
    missing = next(row for row in oracle["vectors"] if row["id"] == "missing_requested_carrier_fails")
    if missing["expect_error"] != "Carrier variable 'missing' not found in variable_map":
        raise SystemExit("missing requested carrier fail-fast not preserved")
    return "missing_requested_carrier_fail_fast", "Carrier variable 'missing' not found in variable_map"


def _hardcoded_representation_case() -> tuple[str, str]:
    output = _load_guard_output("tools/checks/rust_lifecycle_no_carrier_key_type_special_case_guard.sh")
    if "requested_names_transport=ArrayBox" not in output:
        raise SystemExit("carrier key type special case guard missing ArrayBox transport")
    return "hardcoded_representation_token_in_decision_path", "requested_names_transport=ArrayBox"


def _todo_null_placeholder_case() -> tuple[str, str]:
    banned = re.compile(r"(?i)\bTODO\b|null placeholder")
    for path in [
        GENERATED / "binding_context.hako",
        GENERATED / "variable_context_simple_map.hako",
        GENERATED / "variable_context_snapshot_restore.hako",
        GENERATED / "variable_context_carrier_snapshot.hako",
        GENERATED / "variable_context_explicit_carrier_snapshot.hako",
        GENERATED / "structured_loop_without_carried_state.hako",
        GENERATED / "single_scalar_loop_carrier.hako",
        ROOT / "apps/lib/hakorune_mir_builder/carrier_info.hako",
        ROOT / "tools/rust_lifecycle/shared_mirbuilder_emitter.py",
    ]:
        text = path.read_text()
        if banned.search(text):
            raise SystemExit(f"placeholder marker found in {path}")
    return "todo_null_placeholder_emission", "absent"


CASE_RUNNERS: dict[str, Callable[[], tuple[str, str]]] = {
    "binding_context_unsupported_resolved_call_target": _binding_context_case,
    "variable_context_simple_map_unsupported_resolved_call_target": _simple_map_case,
    "variable_context_snapshot_restore_unsupported_resolved_call_target": _snapshot_restore_case,
    "structured_loop_carried_state_required": _structured_loop_carried_state_case,
    "structured_loop_unstructured_control_flow": _structured_loop_unstructured_control_case,
    "single_scalar_loop_phi_required": _single_scalar_loop_phi_case,
    "single_scalar_loop_carrier_escape": _single_scalar_loop_carrier_escape_case,
    "returned_read_borrow": _returned_read_borrow_case,
    "returned_mutable_borrow": _returned_mutable_borrow_case,
    "carrier_sensitive_alias": _carrier_sensitive_alias_case,
    "missing_requested_carrier_fail_fast": _missing_requested_carrier_case,
    "hardcoded_representation_token_in_decision_path": _hardcoded_representation_case,
    "todo_null_placeholder_emission": _todo_null_placeholder_case,
}


def run_case(name: str) -> tuple[str, str]:
    try:
        runner = CASE_RUNNERS[name]
    except KeyError as exc:
        raise SystemExit(f"unknown negative fixture case: {name}") from exc
    return runner()


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--case", choices=sorted(CASE_RUNNERS))
    parser.add_argument("--all", action="store_true")
    args = parser.parse_args()

    if bool(args.case) == bool(args.all):
        raise SystemExit("choose exactly one of --case or --all")

    ordered, statuses = _load_corpus()

    if args.case:
        name, reason = run_case(args.case)
        print(f"{name}={statuses[name]}")
        print(f"{name}_reason={reason}")
        print("summary=ok")
        return 0

    for name in ordered:
        case_name, reason = run_case(name)
        print(f"{case_name}={statuses[name]}")
        print(f"{case_name}_reason={reason}")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
