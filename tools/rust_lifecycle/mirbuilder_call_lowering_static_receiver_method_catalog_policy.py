#!/usr/bin/env python3
"""Resolve CallLowering static receiver method catalog policy."""

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
EXTERN_POLICY = FIXTURES / "mirbuilder-call-lowering-extern-interface-registry-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-static-receiver-method-catalog-policy-v0.json"
SUBCLUSTER_ID = "StaticReceiverMethodCatalog"


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


def method_literals(expression: str) -> list[str]:
    return re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', expression)


def explicit_method_entries(body: str) -> list[dict[str, Any]]:
    entries = []
    pattern = re.compile(
        r'"(?P<box>[^"]+)"\s*=>\s*matches!\(\s*method\s*,\s*(?P<methods>[^)]*)\)',
        re.MULTILINE,
    )
    for match in pattern.finditer(body):
        entries.append({
            "box_name": match.group("box"),
            "catalog_kind": "explicit_method_names",
            "method_names": method_literals(match.group("methods")),
        })
    return entries


def delegated_catalog_entries(body: str) -> list[dict[str, Any]]:
    entries = []
    pattern = re.compile(
        r'"(?P<box>[^"]+)"\s*=>\s*(?P<resolver>crate::boxes(?:::|::[A-Za-z0-9_]+)+::from_name)\(method\)\.is_some\(\)',
        re.MULTILINE,
    )
    for match in pattern.finditer(body):
        entries.append({
            "box_name": match.group("box"),
            "catalog_kind": "delegated_catalog_resolver",
            "resolver": match.group("resolver"),
        })
    return entries


def build_policy() -> dict[str, Any]:
    decomposition = read_json(DECOMPOSITION)
    surfaces = [
        surface for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    if [surface["symbol"] for surface in surfaces] != ["has_method"]:
        raise SystemExit(f"unexpected static receiver surfaces: {surfaces}")

    surface = surfaces[0]
    body = function_body(read_source(surface["source_path"]), "has_method")
    explicit_entries = explicit_method_entries(body)
    delegated_entries = delegated_catalog_entries(body)

    entries = sorted(explicit_entries + delegated_entries, key=lambda item: item["box_name"])
    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringStaticReceiverMethodCatalogPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-STATIC-RECEIVER-METHOD-CATALOG-POLICY-001",
        "input_state": {
            "subcluster_decomposition": rel(DECOMPOSITION),
            "previous_policy": rel(EXTERN_POLICY),
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
                "registry_role": "static_receiver_method_catalog_predicate",
            }
            for surface in surfaces
        ],
        "catalog_descriptor": {
            "descriptor_id": "call_lowering_static_receiver_method_catalog_v1",
            "source_extraction": "rust_match_arms",
            "entry_count": len(entries),
            "explicit_entry_count": len(explicit_entries),
            "delegated_catalog_entry_count": len(delegated_entries),
            "conservative_unknown_box_policy": "RejectUnknownBoxes",
            "entries": entries,
        },
        "selected_policy": {
            "policy": "RegistryDescriptorFixture",
            "owner_edge": "mirbuilder::call_lowering_static_receiver_method_catalog",
            "registry_descriptor_selected": True,
            "projection_surface_selected": False,
            "delegated_catalogs_expanded": False,
            "reason_token": "StaticReceiverMethodsRequireDescriptorFixture",
        },
        "decision": {
            "kind": "SelectRegistryDescriptorPolicy",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-FEATURE-PREDICATES-PROJECTION-POLICY-001",
            "reason_token": "StaticReceiverMethodCatalogDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "projection_surface_selected": 0,
            "registry_descriptor_selected": 1,
            "delegated_catalogs_expanded": 0,
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
        print("mirbuilder-call-lowering-static-receiver-method-catalog-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
