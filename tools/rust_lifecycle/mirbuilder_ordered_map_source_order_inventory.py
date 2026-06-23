#!/usr/bin/env python3
"""Record the SourceOrdered OrderedMapBox blocker for MirBuilder read-folds."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/ordered-map-source-order-v0.json"


def inventory_ordered_map_source_order() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()

    require("USE-TOTAL-TEXT-ORDERING-IN-ORDEREDMAPBOX-001" in task_order, "active blocker missing")
    require("KeyAscending(RustStringOrdV1)" in task_order, "structured order fact missing")
    require("UnsupportedOrderCapability" in task_order, "order capability deny missing")

    return {
        "schema_version": 0,
        "kind": "MirBuilderOrderedMapSourceOrderInventory",
        "subject": "OrderedMapBox SourceOrdered String-key blocker",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
        },
        "current_contract": "deny_source_ordered_read_fold",
        "decision": [
            "do not claim SourceOrdered read-fold until RustStringOrdV1 is VM/EXE/AOT accepted",
            "do not silently downgrade SourceOrdered to insertion order",
            "do not add RegionObserver-specific key-order special cases",
        ],
        "supporting_evidence": [
            "RegionObserver prototype inserted b, a, args and observed insertion order.",
            "AOT implementation attempt using .hako string content comparison failed at OrderedMapBox.set/2 backend acceptance.",
            "MIR-only success is insufficient for the converter acceptance target.",
        ],
        "deny": {
            "reason": "UnsupportedOrderCapability",
            "detail": "ComparatorUnavailable",
            "comparator": "RustStringOrdV1",
            "required_tiers": "VM,EXE,AOT",
        },
        "next_task": "define converter-side total text ordering capability",
        "stop_line": [
            "source_ordered_read_fold_claim=0",
            "runtime_fallback=0",
            "insertion_order_substitution=0",
            "region_observer_key_name_special_case=0",
        ],
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_ordered_map_source_order()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "ordered-map source-order inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-ordered-map-source-order-v0")
    print("source_ordered_read_fold_claim=0")
    print("deny_reason=UnsupportedOrderCapability")
    print("deny_detail=ComparatorUnavailable")
    print("deny_comparator=RustStringOrdV1")
    print("runtime_fallback=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
