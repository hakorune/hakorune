#!/usr/bin/env python3
"""Inventory the NonAsciiOrderedKey decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
ORDERED_MAP_BOUNDARY = ROOT / "docs/development/current/main/design/ordered-map-box-boundary-ssot.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/non-ascii-ordered-key-v0.json"


def inventory_non_ascii_ordered_key() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    ordered_map_boundary = ORDERED_MAP_BOUNDARY.read_text()

    require("38. `NonAsciiOrderedKey`" in task_order, "NonAsciiOrderedKey row missing")
    require("Status: landed." in task_order, "NonAsciiOrderedKey row is not marked as landed")
    require("key_domain = String only" in ordered_map_boundary, "ordered-map boundary missing String-only key domain")
    require("ordering = deterministic lexical key order" in ordered_map_boundary, "ordered-map boundary missing lexical order rule")
    require("claim Rust BTreeMap parity beyond String-key deterministic iteration" in ordered_map_boundary, "ordered-map boundary missing BTreeMap parity stop line")

    return {
        "schema_version": 0,
        "kind": "MirBuilderNonAsciiOrderedKeyInventory",
        "subject": "MirBuilder non-ASCII ordered key boundary",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "ordered_map_boundary": "docs/development/current/main/design/ordered-map-box-boundary-ssot.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep NonAsciiOrderedKey parked until a dedicated key collation contract is named",
            "keep deterministic String-key ordering separate from non-ASCII collation questions",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "OrderedMapBox v0 uses String only keys.",
            "OrderedMapBox ordering is deterministic lexical key order.",
            "claim Rust BTreeMap parity beyond String-key deterministic iteration is forbidden.",
        ],
        "open_questions": [
            "Should non-ASCII keys follow Rust String byte ordering exactly or require an explicit collation contract?",
            "Should the converter defer all non-ASCII key questions until a later hard tier?",
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

    report = inventory_non_ascii_ordered_key()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "non-ascii ordered key inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-non-ascii-ordered-key-v0")
    print("non_ascii_ordered_key_recorded=1")
    print("subject=MirBuilder non-ASCII ordered key boundary")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
