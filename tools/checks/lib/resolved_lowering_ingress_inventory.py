#!/usr/bin/env python3
"""Validate the behavior-neutral B0-L1 Lower ingress inventory."""

from __future__ import annotations

import json
import sys
from pathlib import Path


MODULE_IDS = {
    "rust-source-default-bare-ast",
    "rust-source-imports-bare-ast",
    "direct-bare-ast-api",
    "program-v0-import-bundle",
    "repl-wrapper-source",
}
FUNCTION_IDS = {
    "free-static-function",
    "static-box-method-app",
    "static-box-method-script",
    "instance-constructor",
    "instance-method",
    "script-runtime-root",
    "main-callable-optional",
    "main-inline-wrapper",
    "lambda-closure-body",
    "repl-submission-main",
}
BODY_SEAM_IDS = {
    "raw-body-suffix-router",
    "function-body-program-wrapper",
}
ZERO_FIELDS = {
    "production_semantic_activation",
    "canonical_lower_route_count",
    "resolved_scope_consumer_count",
    "exact_source_site_transport_count",
    "route_selection_before_prepare_module",
    "legacy_retry_after_canonical_selection",
}


def fail(message: str) -> None:
    raise SystemExit(f"[resolved-lowering-ingress-inventory] ERROR: {message}")


def exact_ids(rows: list[dict], expected: set[str], label: str) -> None:
    actual = [row.get("id") for row in rows]
    if len(actual) != len(set(actual)):
        fail(f"{label} contains duplicate ids: {actual}")
    if set(actual) != expected:
        fail(
            f"{label} drifted: missing={sorted(expected - set(actual))} "
            f"extra={sorted(set(actual) - expected)}"
        )


def check_evidence(root: Path, rows: list[dict], label: str) -> None:
    for row in rows:
        evidence = row.get("evidence")
        if not isinstance(evidence, list) or not evidence:
            fail(f"{label}/{row.get('id')} has no evidence")
        for item in evidence:
            if not isinstance(item, str) or "#" not in item:
                fail(f"{label}/{row.get('id')} has malformed evidence: {item!r}")
            relative, anchor = item.split("#", 1)
            path = root / relative
            if not path.is_file():
                fail(f"{label}/{row.get('id')} evidence path is missing: {relative}")
            if anchor not in path.read_text(encoding="utf-8"):
                fail(f"{label}/{row.get('id')} evidence is stale: {item}")


def main() -> None:
    if len(sys.argv) != 3:
        fail("usage: resolved_lowering_ingress_inventory.py ROOT INVENTORY")
    root = Path(sys.argv[1]).resolve()
    inventory_path = Path(sys.argv[2]).resolve()
    data = json.loads(inventory_path.read_text(encoding="utf-8"))

    if data.get("schema") != "ResolvedLoweringIngressInventoryV1":
        fail("schema drifted")
    if data.get("decision") != "A-prime" or data.get("slice") != "B0-L1":
        fail("Decision A-prime / B0-L1 marker drifted")
    for field in ZERO_FIELDS:
        if data.get(field) != 0:
            fail(f"behavior-neutral field must remain zero: {field}={data.get(field)!r}")

    modules = data.get("module_ingresses")
    functions = data.get("function_families")
    seams = data.get("body_route_seams")
    if not all(isinstance(rows, list) for rows in (modules, functions, seams)):
        fail("inventory row arrays are missing")
    exact_ids(modules, MODULE_IDS, "module_ingresses")
    exact_ids(functions, FUNCTION_IDS, "function_families")
    exact_ids(seams, BODY_SEAM_IDS, "body_route_seams")

    if any(row.get("source_unit_provenance_preserved") is not False for row in modules):
        fail("a current bare-AST module ingress claims preserved source-unit provenance")
    if any(row.get("preflight_before_prepare_module") is not False for row in modules):
        fail("a current module ingress claims canonical preflight")
    if any(row.get("exact_source_root_transport") is not False for row in functions):
        fail("a current function family claims exact source-root transport")
    if any(row.get("located_transport") is not False for row in seams):
        fail("a current body seam claims located transport")
    if any(row.get("planner_connection_allowed_in_b0_l2") is not False for row in seams):
        fail("B0-L2 must not activate Planner transport")

    atomic = data.get("atomic_activation_contract", {})
    required = atomic.get("first_production_canonical_owner_requires", [])
    forbidden = atomic.get("forbidden_partial_states", [])
    if len(required) != 8 or len(set(required)) != 8:
        fail("atomic activation requirement inventory drifted")
    if len(forbidden) != 5 or len(set(forbidden)) != 5:
        fail("forbidden partial-state inventory drifted")
    if atomic.get("carrier_infrastructure_may_land_disconnected") is not True:
        fail("behavior-neutral carrier landing decision drifted")
    if data.get("selected_next_slice") != "B0-L2a-typed-source-unit-ingress":
        fail("B0-L1 must mechanically select B0-L2a")

    check_evidence(root, modules, "module_ingresses")
    check_evidence(root, functions, "function_families")
    check_evidence(root, seams, "body_route_seams")
    print("resolved_lowering_ingress_inventory=closed")
    print("resolved_lowering_module_ingresses=5")
    print("resolved_lowering_function_families=10")
    print("resolved_lowering_body_route_seams=2")
    print("resolved_lowering_production_activation=0")
    print("resolved_lowering_selected_next_slice=B0-L2a")


if __name__ == "__main__":
    main()
