#!/usr/bin/env python3
"""Select the next easy-tier CoreContext pilot."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
READINESS = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-readiness-inventory-v0.json"
VOCABULARY = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-scalar-counter-vocabulary-v0.json"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-pilot-selection-v0.json"


def select_core_context_pilot() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    readiness = json.loads(READINESS.read_text())
    vocabulary = json.loads(VOCABULARY.read_text())

    require("29.5 `Record CoreContext readiness inventory`" in task_order, "CoreContext readiness inventory row missing")
    require(readiness.get("next_easy_tier_candidate") == "CoreContext", "readiness inventory does not name CoreContext")
    require("scalar counter field initialization" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing field initialization stop")
    require("increment / saturating_add" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing increment stop")
    require("ID constructor calls" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing ID constructor stop")
    require("struct-return construction" in json.dumps(vocabulary, sort_keys=True), "scalar-counter vocabulary missing struct-return stop")

    return {
        "schema_version": 0,
        "kind": "MirBuilderCoreContextPilotSelection",
        "subject": "hakorune_mir_builder::core_context::CoreContext",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "readiness_inventory": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-readiness-inventory-v0.json",
            "scalar_counter_vocabulary": "docs/development/current/main/design/fixtures/rust-lifecycle/core-context-scalar-counter-vocabulary-v0.json",
        },
        "current_contract": "selection_only",
        "selected_next_pilot": "CoreContext",
        "pilot_scope": "CoreContext_scalar_counter_only",
        "decision": [
            "select CoreContext as the next easy-tier family pilot",
            "keep the pilot bounded to the scalar-counter slice",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "core_context readiness inventory names CoreContext as the next easy-tier candidate",
            "core_context scalar-counter vocabulary is fixed in a machine-readable fixture",
            "route selection remains unopened",
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

    report = select_core_context_pilot()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "core-context pilot selection differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-core-context-pilot-selection-v0")
    print("selected_next_pilot=CoreContext")
    print("pilot_scope=CoreContext_scalar_counter_only")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=selection_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
