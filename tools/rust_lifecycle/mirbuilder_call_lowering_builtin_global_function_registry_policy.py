#!/usr/bin/env python3
"""Resolve CallLowering builtin global function registry policy."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
DECOMPOSITION = FIXTURES / "mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
DIAGNOSTIC_POLICY = FIXTURES / "mirbuilder-call-lowering-diagnostic-helpers-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-builtin-global-function-registry-policy-v0.json"
SUBCLUSTER_ID = "BuiltinGlobalFunctionRegistry"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def function_body(source: str, name: str) -> str:
    marker = f"pub fn {name}"
    start = source.find(marker)
    if start < 0:
        raise SystemExit(f"function not found: {name}")
    brace = source.find("{", start)
    if brace < 0:
        raise SystemExit(f"function body not found: {name}")

    depth = 0
    for index in range(brace, len(source)):
        char = source[index]
        if char == "{":
            depth += 1
        elif char == "}":
            depth -= 1
            if depth == 0:
                return source[brace:index + 1]
    raise SystemExit(f"function body did not close: {name}")


def string_literals_in_function(source_path: str, symbol: str) -> list[str]:
    body = function_body(read_source(source_path), symbol)
    return re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', body)


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    by_symbol = {surface["symbol"]: surface for surface in surfaces}
    if set(by_symbol) != {"is_builtin_function", "is_math_function"}:
        raise SystemExit(f"unexpected builtin registry surfaces: {sorted(by_symbol)}")

    builtin_names = string_literals_in_function(
        by_symbol["is_builtin_function"]["source_path"],
        "is_builtin_function",
    )
    math_names = string_literals_in_function(
        by_symbol["is_math_function"]["source_path"],
        "is_math_function",
    )
    builtin_set = set(builtin_names)
    math_set = set(math_names)

    registry_entries = []
    for name in sorted(builtin_set | math_set):
        categories = []
        if name in builtin_set:
            categories.append("builtin_global_function")
        if name in math_set:
            categories.append("math_special_function")
        registry_entries.append({
            "name": name,
            "categories": categories,
            "is_builtin_function": name in builtin_set,
            "is_math_function": name in math_set,
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringBuiltinGlobalFunctionRegistryPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-BUILTIN-GLOBAL-FUNCTION-REGISTRY-POLICY-001",
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "previous_policy": rel(DIAGNOSTIC_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.call_lowering",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surfaces": [
            {
                "source_id": surface["source_id"],
                "symbol": surface["symbol"],
                "source_path": surface["source_path"],
                "params": surface["params"],
                "return_type": surface["return_type"],
                "registry_role": (
                    "builtin_global_membership_predicate"
                    if surface["symbol"] == "is_builtin_function"
                    else "math_special_membership_predicate"
                ),
            }
            for surface in surfaces
        ],
        "registry_descriptor": {
            "descriptor_id": "call_lowering_builtin_global_function_registry_v1",
            "source_extraction": "rust_matches_string_literals",
            "builtin_function_count": len(builtin_names),
            "math_function_count": len(math_names),
            "shared_builtin_math_count": len(builtin_set & math_set),
            "entries": registry_entries,
        },
        "selected_policy": {
            "policy": "RegistryDescriptorFixture",
            "owner_edge": "mirbuilder::call_lowering_builtin_global_function_registry",
            "registry_descriptor_selected": True,
            "projection_surface_selected": False,
            "reason_token": "BuiltinGlobalFunctionNamesRequireDescriptorFixture",
        },
        "decision": {
            "kind": "SelectRegistryDescriptorPolicy",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001",
            "reason_token": "BuiltinGlobalRegistryDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "registry_descriptor_selected": 1,
            "ad_hoc_by_name_policy": 0,
            "runtime_or_projection_policy_by_name": 0,
            "hako_generation": 0,
            "hako_adopted_decision": 0,
            "native_seed_materialization": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
            "new_python_semantic_projector": 0,
            "runner_semantic_owner": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in policy fixture.")
    args = parser.parse_args()

    output = stable_json(build_policy())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-call-lowering-builtin-global-function-registry-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
