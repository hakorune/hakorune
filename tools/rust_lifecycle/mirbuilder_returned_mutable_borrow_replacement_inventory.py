#!/usr/bin/env python3
"""Inventory the returned mutable borrow replacement decision."""

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
    / "returned-mutable-borrow-replacement-decision-v0.json"
)


def inventory_returned_mutable_borrow_replacement(source_path: Path = SOURCE) -> dict[str, Any]:
    source = source_path.read_text()
    for fragment in [
        "pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>",
        "pub fn variable_map(&self) -> &BTreeMap<String, ValueId>",
        "pub fn snapshot(&self) -> BTreeMap<String, ValueId>",
        "pub fn restore(&mut self, snapshot: BTreeMap<String, ValueId>)",
    ]:
        require(fragment in source, f"missing VariableContext fragment: {fragment}")
    return {
        "schema_version": 0,
        "kind": "MirBuilderReturnedMutableBorrowReplacementDecision",
        "subject": "hakorune_mir_builder::variable_context::VariableContext",
        "source": {
            "crate": "hakorune_mir_builder",
            "module": "crate::variable_context",
            "source_path": "src/variable_context.rs",
        },
        "current_boundary": {
            "method": "VariableContext::variable_map_mut",
            "signature": "pub fn variable_map_mut(&mut self) -> &mut BTreeMap<String, ValueId>",
            "policy": "Deny(ReturnedMutableBorrow)",
        },
        "candidate_replacements": [
            "explicit mutation APIs",
            "bounded with-map operation",
            "ReplaceOwned-style ownership transfer",
        ],
        "current_supporting_methods": [
            "variable_map",
            "variable_map_mut",
            "snapshot",
            "restore",
        ],
        "reason": [
            "returned mutable alias is caller-owned",
            "replacement policy remains an explicit design decision",
            "converter/emitter must not choose the representation",
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

    report = inventory_returned_mutable_borrow_replacement(args.source)
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "returned mutable borrow replacement inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-returned-mutable-borrow-replacement-decision-v0")
    print("returned_mutable_borrow_replacement_recorded=1")
    print("subject=VariableContext")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=deferred")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
