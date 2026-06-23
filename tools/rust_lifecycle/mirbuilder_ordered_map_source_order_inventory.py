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

    require("Use total text ordering in OrderedMapBox" in task_order, "landed ordering task missing")
    require("KeyAscending(RustStringOrdV1)" in task_order, "structured order fact missing")
    require("VmExeAotAccepted" in task_order, "comparator proof missing")

    return {
        "schema_version": 0,
        "kind": "MirBuilderOrderedMapSourceOrderInventory",
        "subject": "OrderedMapBox SourceOrdered String-key blocker",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
        },
        "current_contract": "comparator_capability_landed",
        "decision": [
            "RustStringOrdV1 is VM/EXE/AOT accepted",
            "OrderedMapBox uses TextOrder.compare_rust_string_v1",
            "do not silently downgrade SourceOrdered to insertion order",
            "do not add RegionObserver-specific key-order special cases",
        ],
        "supporting_evidence": [
            "RegionObserver prototype inserted b, a, args and observed insertion order.",
            "AOT implementation attempt using .hako string content comparison failed at OrderedMapBox.set/2 backend acceptance.",
            "MIR-only success is insufficient for the converter acceptance target.",
        ],
        "proof": {
            "comparator": "RustStringOrdV1",
            "status": "VmExeAotAccepted",
        },
        "next_task": "decide SlotMetadata / RefSlotKind owned output transport",
        "stop_line": [
            "source_ordered_read_fold_claim=0",
            "slot_metadata_output_transport_claim=0",
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
    print("comparator=RustStringOrdV1")
    print("comparator_proof=VmExeAotAccepted")
    print("slot_metadata_output_transport_claim=0")
    print("runtime_fallback=0")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
