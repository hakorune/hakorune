#!/usr/bin/env python3
"""Inventory the CoreContext scalar-counter plan/oracle boundary."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
READINESS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-readiness-inventory-v0.json"
VOCABULARY = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-scalar-counter-vocabulary-v0.json"
PLAN = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-plan-v0.json"
ORACLE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-oracle-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-plan-oracle-inventory-v0.json"


def inventory_core_context_plan_oracle() -> dict[str, Any]:
    readiness = json.loads(READINESS.read_text())
    vocabulary = json.loads(VOCABULARY.read_text())
    plan = json.loads(PLAN.read_text())
    oracle = json.loads(ORACLE.read_text())

    require(readiness.get("next_easy_tier_candidate") == "CoreContext", "readiness inventory does not name CoreContext")
    require("scalar counter field initialization" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing required stop")
    require("increment / saturating_add" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing increment stop")
    require("ID constructor calls" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing ID constructor stop")
    require("struct-return construction" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing struct-return stop")
    require(plan.get("subject") == "hakorune_mir_builder::core_context::CoreContext", "CoreContext plan subject mismatch")
    require(oracle.get("subject") == "hakorune_mir_builder::core_context::CoreContext", "CoreContext oracle subject mismatch")
    require(plan.get("behavior", {}).get("nightly_rustc_adapter") is False, "nightly rustc adapter must remain disabled")
    require(oracle.get("promotion_scope", {}).get("mirbuilder_wide_claim") is False, "oracle must not claim MirBuilder-wide parity")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCoreContextPlanOracleInventory",
        "subject": "hakorune_mir_builder::core_context::CoreContext",
        "source": {
            "readiness_inventory": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-readiness-inventory-v0.json",
            "scalar_counter_vocabulary": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-scalar-counter-vocabulary-v0.json",
            "plan": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-plan-v0.json",
            "oracle": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-oracle-v0.json",
        },
        "current_contract": "scalar_counter_plan_oracle_named",
        "decision": [
            "name the CoreContext scalar-counter plan and oracle fixtures",
            "emit only next_binding / next_temp_slot / next_debug_join",
            "keep generator-object methods denied until transport is designed",
            "keep the plan/oracle question separate from route selection and nightly rustc adapter work",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "core-context-plan-v0.json exists and selects scalar-counter methods only",
            "core-context-oracle-v0.json exists and exercises scalar counters only",
            "scalar-counter vocabulary remains fixed in a machine-readable fixture",
        ],
        "named_paths": [
            "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-plan-v0.json",
            "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-oracle-v0.json",
        ],
        "denied_methods": [
            "CoreContext::next_value",
            "CoreContext::next_block",
            "CoreContext::peek_next_value",
            "CoreContext::peek_next_block",
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

    report = inventory_core_context_plan_oracle()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "core-context plan/oracle inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-core-context-plan-oracle-v0")
    print("core_context_plan_oracle_recorded=1")
    print("subject=CoreContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=scalar_counter_plan_oracle_named")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
