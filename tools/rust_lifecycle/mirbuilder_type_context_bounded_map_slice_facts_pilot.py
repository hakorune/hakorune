#!/usr/bin/env python3
"""Inventory the TypeContext bounded map-slice facts pilot boundary."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
READINESS = ROOT / "docs/development/current/main/phases/phase-296x/296x-1548-TYPE-CONTEXT-BOUNDED-MAP-SLICE-READINESS-001.md"
BOUNDED = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/type-context-bounded-map-slice-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/type-context-bounded-map-slice-facts-pilot-v0.json"


def inventory_type_context_facts_pilot() -> dict[str, Any]:
    readiness = READINESS.read_text()
    bounded = json.loads(BOUNDED.read_text())

    require("TypeContext is broader than the scalar-counter CoreContext slice" in readiness, "TypeContext readiness missing design stop context")
    require("HashMap" in json.dumps(bounded, sort_keys=True), "bounded TypeContext slice missing HashMap evidence")
    require("TypeContextSnapshot stores all mirrored fields" in json.dumps(bounded, sort_keys=True), "bounded TypeContext slice missing snapshot evidence")

    return {
        "schema_version": 0,
        "kind": "MirBuilderTypeContextBoundedMapSliceFactsPilot",
        "subject": "hakorune_mir_builder::type_context::TypeContext",
        "source": {
            "readiness_inventory": "docs/development/current/main/phases/phase-296x/296x-1548-TYPE-CONTEXT-BOUNDED-MAP-SLICE-READINESS-001.md",
            "bounded_map_slice": "docs/development/current/main/design/fixtures/rust-lifecycle/type-context-bounded-map-slice-v0.json",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep TypeContext bounded map slice facts pilot parked until a bounded facts extractor is named",
            "keep the pilot separate from route selection and nightly rustc adapter work",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "TypeContext bounded map slice readiness is fixed in a machine-readable inventory",
            "TypeContext source shape includes snapshot struct behavior and HashMap fields",
            "route selection remains unopened",
        ],
        "next_design_stop": [
            "non-String keys",
            "HashMap rather than BTreeMap",
            "Option/default behavior",
            "closure-shaped source paths",
            "snapshot struct behavior",
        ],
        "stop_line": [
            "do_not_select_route=1",
            "do_not_open_nightly_rustc_adapter=1",
            "do_not_claim_mirbuilder_wide_conversion=1",
            "do_not_add_runtime_fallback=1",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_type_context_facts_pilot()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "type-context facts pilot inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0
    print("output_contract=rust-mirbuilder-type-context-bounded-map-slice-facts-pilot-v0")
    print("type_context_bounded_map_slice_facts_pilot_recorded=1")
    print("subject=TypeContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
