#!/usr/bin/env python3
"""Materialize CallLowering pure-method catalog policy."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
UNIFIED_GATE_POLICY = FIXTURES / "mirbuilder-call-lowering-unified-call-mode-gate-projection-policy-v0.json"
FEATURE_POLICY = FIXTURES / "mirbuilder-call-lowering-feature-predicates-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-pure-method-catalog-policy-v0.json"
SUBCLUSTER_ID = "PureMethodCatalog"


MATCHES_ARM_RE = re.compile(
    r'\("(?P<box>[^"]+)",\s*m\)\s*=>\s*matches!\(m,\s*(?P<methods>[^)]*)\)',
    re.MULTILINE,
)
DIRECT_TRUE_ARM_RE = re.compile(
    r'\("(?P<box>[^"]+)",\s*"(?P<method>[^"]+)"\)\s*=>\s*true',
    re.MULTILINE,
)
QUOTED_RE = re.compile(r'"([^"]+)"')


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def pure_method_catalog(source_text: str) -> list[dict[str, Any]]:
    catalog: dict[str, set[str]] = {}
    for match in MATCHES_ARM_RE.finditer(source_text):
        catalog.setdefault(match.group("box"), set()).update(QUOTED_RE.findall(match.group("methods")))
    for match in DIRECT_TRUE_ARM_RE.finditer(source_text):
        catalog.setdefault(match.group("box"), set()).add(match.group("method"))
    return [
        {"box_name": box_name, "methods": sorted(methods), "method_count": len(methods)}
        for box_name, methods in sorted(catalog.items())
    ]


def build_policy() -> dict[str, Any]:
    feature_policy = read_json(FEATURE_POLICY)
    surfaces = [
        surface for surface in feature_policy["source_surfaces"]
        if surface["feature_subcluster_id"] == SUBCLUSTER_ID
    ]
    if len(surfaces) != 1 or surfaces[0]["symbol"] != "is_pure_method":
        raise SystemExit(f"unexpected PureMethodCatalog surfaces: {surfaces}")

    surface = surfaces[0]
    source_text = read_source(surface["source_path"])
    catalog = pure_method_catalog(source_text)
    if not catalog:
        raise SystemExit("pure method catalog extraction returned no entries")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringPureMethodCatalogPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-PURE-METHOD-CATALOG-POLICY-001",
        "input_state": {
            "feature_predicates_policy": rel(FEATURE_POLICY),
            "previous_policy": rel(UNIFIED_GATE_POLICY),
            "selected_feature_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.catalog_predicate",
            "borrow_axis": "NoBorrow",
            "type_transport_axis": "Known",
            "verifier_or_oracle_state": "Present",
        },
        "source_surface": {
            "source_id": surface["source_id"],
            "symbol": surface["symbol"],
            "source_path": surface["source_path"],
            "params": surface["params"],
            "return_type": surface["return_type"],
            "catalog_source": "match (box_name, method)",
        },
        "catalog_descriptor": {
            "descriptor_kind": "PureMethodCatalogDescriptorV1",
            "source_extracted": True,
            "entry_count": sum(entry["method_count"] for entry in catalog),
            "box_count": len(catalog),
            "entries": catalog,
        },
        "selected_policy": {
            "policy": "MaterializeCatalogDescriptor",
            "owner_edge": "mirbuilder::call_lowering_pure_method_catalog",
            "registry_descriptor_selected": True,
            "projection_surface_selected": False,
            "reason_token": "PureMethodPredicateIsSourceCatalog",
        },
        "decision": {
            "kind": "SelectCatalogDescriptor",
            "selected_next_card": "MIRBUILDER-CALL-LOWERING-VALUE-RETURN-AST-SCAN-PROJECTION-POLICY-001",
            "reason_token": "PureMethodCatalogDescriptorMaterialized",
        },
        "claims": {
            "manual_family_selection": 0,
            "source_extracted_catalog": 1,
            "projection_surface_selected": 0,
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
        print("mirbuilder-call-lowering-pure-method-catalog-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
