#!/usr/bin/env python3
"""Resolve CallLowering extern interface registry policy."""

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
BUILTIN_POLICY = FIXTURES / "mirbuilder-call-lowering-builtin-global-function-registry-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-extern-interface-registry-policy-v0.json"
SUBCLUSTER_ID = "ExternInterfaceRegistry"


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


def starts_with_prefixes(source_path: str, symbol: str) -> list[str]:
    body = function_body(read_source(source_path), symbol)
    return re.findall(r'\.starts_with\("([^"\\]*(?:\\.[^"\\]*)*)"\)', body)


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    by_symbol = {surface["symbol"]: surface for surface in surfaces}
    if set(by_symbol) != {"is_env_interface", "is_extern_function"}:
        raise SystemExit(f"unexpected extern registry surfaces: {sorted(by_symbol)}")

    extern_prefixes = starts_with_prefixes(
        by_symbol["is_extern_function"]["source_path"],
        "is_extern_function",
    )
    env_interfaces = string_literals_in_function(
        by_symbol["is_env_interface"]["source_path"],
        "is_env_interface",
    )

    interface_entries = []
    for name in sorted(env_interfaces):
        interface_entries.append({
            "name": name,
            "interface_root": name.split(".", 1)[0],
            "is_env_interface": True,
        })

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringExternInterfaceRegistryPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-EXTERN-INTERFACE-REGISTRY-POLICY-001",
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "previous_policy": rel(BUILTIN_POLICY),
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
                    "extern_prefix_membership_predicate"
                    if surface["symbol"] == "is_extern_function"
                    else "env_interface_membership_predicate"
                ),
            }
            for surface in surfaces
        ],
        "registry_descriptor": {
            "descriptor_id": "call_lowering_extern_interface_registry_v1",
            "source_extraction": "rust_starts_with_and_matches_literals",
            "extern_prefixes": extern_prefixes,
            "extern_prefix_count": len(extern_prefixes),
            "env_interface_count": len(env_interfaces),
            "env_interfaces": interface_entries,
            "method_spec_surface_selected": False,
        },
        "selected_policy": {
            "policy": "RegistryDescriptorFixture",
            "owner_edge": "mirbuilder::call_lowering_extern_interface_registry",
            "registry_descriptor_selected": True,
            "projection_surface_selected": False,
            "reason_token": "ExternInterfaceNamesRequireDescriptorFixture",
        },
        "decision": {
            "kind": "SelectRegistryDescriptorPolicy",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001",
            "reason_token": "ExternInterfaceRegistryDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "registry_descriptor_selected": 1,
            "method_spec_surface_selected": 0,
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
        print("mirbuilder-call-lowering-extern-interface-registry-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
