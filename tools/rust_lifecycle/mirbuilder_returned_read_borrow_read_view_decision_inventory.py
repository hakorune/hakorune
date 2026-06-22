#!/usr/bin/env python3
"""Inventory the VariableContext read-view decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
SOURCE = ROOT / "crates/hakorune_mir_builder/src/variable_context.rs"
REFERENCE = (
    ROOT
    / "docs/development/current/main/design/fixtures/rust-lifecycle/"
    / "returned-read-borrow-read-view-decision-v0.json"
)


def inventory_returned_read_borrow_read_view_decision(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    for fragment in [
        "pub fn variable_map(&self) -> &BTreeMap<String, ValueId>",
        "pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>",
        "pub fn snapshot(&self) -> BTreeMap<String, ValueId>",
        "pub fn restore(&mut self, snapshot: BTreeMap<String, ValueId>)",
    ]:
        require(fragment in source, f"missing VariableContext fragment: {fragment}")
    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnedReadBorrowReadViewDecision",
        "subject": "hakorune_mir_builder::variable_context::VariableContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::variable_context",
            "source_path": "src/variable_context.rs",
        },
        "current_contract": "NoReturnedAlias + OwnedReadSnapshotProjection",
        "decision": [
            "keep OwnedReadSnapshotProjection for bulk read consumers",
            "defer true read-view selection",
            "do not re-open variable_map() as a naked borrowed alias",
        ],
        "supporting_evidence": [
            "variable_map() returns &BTreeMap<String, ValueId>",
            "CarrierInfo::from_variable_map(...) consumes owned snapshot inputs",
            "read-only tests and observation remain the current non-owning consumers",
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
    parser.add_argument("--source", type=Path, default=SOURCE)
    parser.add_argument("--emit-json", action="store_true")
    parser.add_argument("--check-reference", action="store_true")
    args = parser.parse_args()

    report = inventory_returned_read_borrow_read_view_decision(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "returned read borrow read-view inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-returned-read-borrow-read-view-decision-v0")
    print("returned_read_borrow_read_view_decision_recorded=1")
    print("subject=VariableContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=defer_true_read_view")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
