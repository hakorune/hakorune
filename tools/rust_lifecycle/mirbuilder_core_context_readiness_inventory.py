#!/usr/bin/env python3
"""Inventory the next easy-tier MirBuilder CoreContext readiness boundary.

This is intentionally read-only. It records what is present in
`crates/hakorune_mir_builder/src/core_context.rs` and what is still absent for
behavioral conversion:

- lifecycle facts
- Hako lifecycle plan
- behavior recipe
- oracle vectors
- derived artifact manifest
- route entry

The result is a design-consultation inventory, not a route selection or
generator.
"""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import extract_method_signatures, require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/core_context.rs"
ROUTE_MANIFEST = ROOT / "lang/generated/rust_derived/hakorune_mir_builder/family_routes.json"
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
GENERATED = ROOT / "lang/generated/rust_derived/hakorune_mir_builder"
REFERENCE = FIXTURES / "core-context-readiness-inventory-v0.json"


def _has_core_context_route_entry() -> bool:
    if not ROUTE_MANIFEST.exists():
        return False
    routes = json.loads(ROUTE_MANIFEST.read_text())
    for route in routes.get("routes", []):
        manifest = str(route.get("artifact_manifest", ""))
        if "core_context" in manifest:
            return True
    return False


def _missing_paths() -> list[str]:
    candidates = [
        FIXTURES / "core-context-plan-v0.json",
        FIXTURES / "core-context-oracle-v0.json",
        FIXTURES / "core-context-behavior-recipe-v0.json",
        FIXTURES / "core-context-derived-artifact-verifier-result-v0.json",
        GENERATED / "core_context.hako",
        GENERATED / "core_context.artifact.json",
        ROOT / "tools/checks/rust_lifecycle_core_context_derived_artifact_guard.sh",
        ROOT / "tools/checks/rust_lifecycle_core_context_derived_route_selection_guard.sh",
    ]
    return [str(path.relative_to(ROOT)) for path in candidates if not path.exists()]


def _source_inventory(source: str) -> dict[str, Any]:
    signatures = extract_method_signatures(source)
    required_methods = [
        "new",
        "next_value",
        "next_block",
        "next_binding",
        "next_temp_slot",
        "next_debug_join",
        "peek_next_value",
        "peek_next_block",
    ]
    for method in required_methods:
        require(method in signatures, f"missing method: {method}")
    require("BindingId::new(self.next_binding_id)" in source, "missing BindingId constructor call")
    require("self.next_binding_id = self.next_binding_id.saturating_add(1);" in source, "missing saturating_add counter bump")
    require("self.temp_slot_counter = self.temp_slot_counter.saturating_add(1);" in source, "missing temp slot bump")
    require("self.debug_join_counter = self.debug_join_counter.saturating_add(1);" in source, "missing debug join bump")
    require("ValueIdGenerator::new()" in source, "missing value generator initialization")
    require("BasicBlockIdGenerator::new()" in source, "missing block generator initialization")
    return {
        "methods": required_methods,
        "inventory_shape": [
            "scalar counter field initialization",
            "increment / saturating_add",
            "ID constructor calls",
            "struct-return construction",
        ],
    }


def inventory_core_context(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    require("pub struct CoreContext" in source, "missing CoreContext struct")
    require("impl Drop for CoreContext" not in source, "observable Drop detected")
    require("pub fn new() -> Self" in source, "missing CoreContext::new")
    require("pub fn next_binding(&mut self) -> BindingId" in source, "missing CoreContext::next_binding")
    require("pub fn next_temp_slot(&mut self) -> u32" in source, "missing CoreContext::next_temp_slot")
    require("pub fn next_debug_join(&mut self) -> u32" in source, "missing CoreContext::next_debug_join")

    source_inventory = _source_inventory(source)
    has_route_entry = _has_core_context_route_entry()
    missing = _missing_paths()

    return {
        "schema_version": 0,
        "kind": "MirBuilderCoreContextReadinessInventory",
        "subject": "hakorune_mir_builder::core_context::CoreContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::core_context",
            "source_path": "src/core_context.rs",
        },
        "present": {
            "source_methods": source_inventory["methods"],
            "inventory_shape": source_inventory["inventory_shape"],
            "lifecycle_facts_present": 1,
        },
        "missing": {
            "lifecycle_facts": 0,
            "hako_lifecycle_plan": 1,
            "behavior_recipe": 1,
            "oracle_vectors": 1,
            "derived_artifact_manifest": 1,
            "route_entry_present": 1 if has_route_entry else 0,
            "route_entry_missing": 0 if has_route_entry else 1,
            "generated_behavior": 1,
            "nightly_rustc_adapter": 0,
        },
        "missing_paths": missing,
        "next_easy_tier_candidate": "CoreContext",
        "next_design_stop": [
            "scalar counter field initialization",
            "increment / saturating_add",
            "ID constructor calls",
            "struct-return construction",
        ],
        "stop_line": [
            "do_not_select_route_in_same_row=1",
            "do_not_open_nightly_rustc_adapter=1",
            "do_not_claim_mirbuilder_wide_conversion=1",
            "do_not_add_runtime_fallback=1",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_core_context(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "readiness inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    for key, value in [
        ("output_contract", "rust-mirbuilder-core-context-readiness-inventory-v0"),
        ("core_context_readiness_recorded", "1"),
        ("subject", "CoreContext"),
        ("next_easy_tier_candidate", report["next_easy_tier_candidate"]),
        ("lifecycle_facts_present", str(report["present"]["lifecycle_facts_present"])),
        ("route_entry_present", str(report["missing"]["route_entry_present"])),
        ("route_entry_missing", str(report["missing"]["route_entry_missing"])),
        ("lifecycle_facts", str(report["missing"]["lifecycle_facts"])),
        ("hako_lifecycle_plan", str(report["missing"]["hako_lifecycle_plan"])),
        ("behavior_recipe", str(report["missing"]["behavior_recipe"])),
        ("oracle_vectors", str(report["missing"]["oracle_vectors"])),
        ("derived_artifact_manifest", str(report["missing"]["derived_artifact_manifest"])),
        ("generated_behavior", str(report["missing"]["generated_behavior"])),
    ]:
        print(f"{key}={value}")
    for missing_path in report["missing_paths"]:
        print(f"missing_path={missing_path}")
    for shape in report["next_design_stop"]:
        print(f"next_design_stop={shape}")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
