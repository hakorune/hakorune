#!/usr/bin/env python3
"""Resolve CallLowering method-name canonicalization projection policy."""

from __future__ import annotations

import argparse
import json
import re
from pathlib import Path
from typing import Any

from shared_family_generator import stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
SUBCLUSTER_DECOMPOSITION = FIXTURES / "mirbuilder-call-lowering-policy-subcluster-decomposition-v0.json"
VALUE_RETURN_POLICY = FIXTURES / "mirbuilder-call-lowering-value-return-ast-scan-projection-policy-v0.json"
OUTPUT = FIXTURES / "mirbuilder-call-lowering-name-canonicalization-projection-policy-v0.json"
SUBCLUSTER_ID = "CallNameCanonicalizationHelpers"

FORMAT_RE = re.compile(r'format!\("(?P<format>[^"]+)",\s*box_name,\s*method_name,\s*arity\)')


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read_json(path: Path) -> dict[str, Any]:
    return json.loads(path.read_text(encoding="utf-8"))


def read_source(path: str) -> str:
    return (ROOT / path).read_text(encoding="utf-8")


def function_text(source_text: str, symbol: str) -> str:
    marker = f"pub fn {symbol}"
    start = source_text.find(marker)
    if start < 0:
        raise SystemExit(f"function marker not found: {marker}")
    return source_text[start:]


def callsites(symbol: str) -> list[str]:
    paths: list[str] = []
    for path in sorted((ROOT / "src/mir/builder").rglob("*.rs")):
        text = path.read_text(encoding="utf-8")
        if symbol in text and path.name != "function_lowering.rs":
            paths.append(rel(path))
    return paths


def build_policy() -> dict[str, Any]:
    decomposition = read_json(SUBCLUSTER_DECOMPOSITION)
    surfaces = [
        surface for surface in decomposition["source_surfaces"]
        if surface["subcluster_id"] == SUBCLUSTER_ID
    ]
    if len(surfaces) != 1 or surfaces[0]["symbol"] != "generate_method_function_name":
        raise SystemExit(f"unexpected CallNameCanonicalization surfaces: {surfaces}")

    surface = surfaces[0]
    source_text = read_source(surface["source_path"])
    surface_text = function_text(source_text, surface["symbol"])
    format_match = FORMAT_RE.search(surface_text)
    if not format_match:
        raise SystemExit("method-name format expression not found")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCallLoweringNameCanonicalizationProjectionPolicyV1",
        "token": "MIRBUILDER-CALL-LOWERING-NAME-CANONICALIZATION-PROJECTION-POLICY-001",
        "input_state": {
            "subcluster_decomposition": rel(SUBCLUSTER_DECOMPOSITION),
            "previous_policy": rel(VALUE_RETURN_POLICY),
            "selected_subcluster_id": SUBCLUSTER_ID,
            "source_count": len(surfaces),
            "current_blocker": "SOURCE-SELFHOST-WIDER-ROUTE-SELECTION-DESIGN-STOP-001",
        },
        "selection_axes": {
            "owner_edge_confidence": "FixtureMapped",
            "stable_deny_reason": "UnsupportedDirectShape",
            "shape_signature": "shape.name_canonicalization",
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
        },
        "name_canonicalization_descriptor": {
            "descriptor_kind": "MethodFunctionNameCanonicalizationV1",
            "source_extracted": True,
            "format": format_match.group("format"),
            "parts": ["box_name", ".", "method_name", "/", "arity"],
            "callsite_paths": callsites(surface["symbol"]),
        },
        "selected_policy": {
            "policy": "MaterializeNameCanonicalizationDescriptor",
            "owner_edge": "mirbuilder::call_lowering_name_canonicalization",
            "projection_surface_selected": False,
            "reason_token": "MethodFunctionNameIsSourceCanonicalizationDescriptor",
        },
        "decision": {
            "kind": "ReturnToProjectionPolicyClusterResolution",
            "selected_next_card": "MIRBUILDER-PROJECTION-POLICY-CLUSTER-PRIORITY-RESOLUTION-001",
            "reason_token": "CallLoweringSubclustersResolved",
        },
        "claims": {
            "manual_family_selection": 0,
            "source_extracted_descriptor": 1,
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
        print("mirbuilder-call-lowering-name-canonicalization-projection-policy unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
