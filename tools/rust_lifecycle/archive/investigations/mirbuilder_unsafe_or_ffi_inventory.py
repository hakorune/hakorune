#!/usr/bin/env python3
"""Inventory the UnsafeOrFFI decision."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from context_fact_extraction import require


ROOT = Path(__file__).resolve().parents[2]
TASK_ORDER = ROOT / "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md"
SUBSTRATE = ROOT / "docs/development/current/main/design/substrate-capability-ladder-ssot.md"
HAKO_PLAN = ROOT / "docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md"
OWNERSHIP_REFERENCE = ROOT / "docs/development/current/main/design/rust-to-hako-ownership-converter-reference.md"
ADAPTER_BOUNDARY = ROOT / "docs/development/current/main/design/rustc-semir-internal-adapter-boundary.md"
REFERENCE = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle/unsafe-or-ffi-v0.json"


def inventory_unsafe_or_ffi() -> dict[str, Any]:
    task_order = TASK_ORDER.read_text()
    substrate = SUBSTRATE.read_text()
    hako_plan = HAKO_PLAN.read_text()
    ownership_reference = OWNERSHIP_REFERENCE.read_text()
    adapter_boundary = ADAPTER_BOUNDARY.read_text()

    require("36. `UnsafeOrFFI`" in task_order, "UnsafeOrFFI row missing")
    require("Status: design stop." in task_order, "UnsafeOrFFI row is not marked as a design stop")
    require("restricted unsafe only" in substrate, "substrate ladder missing restricted unsafe rule")
    require("Do not replace this with a broad C-style unsafe surface." in substrate, "substrate ladder missing broad unsafe prohibition")
    require("unsafe / FFI" in hako_plan, "hako lifecycle plan missing unsafe/FFI compat boundary")
    require("CompatShim must be explicit and diagnostic-visible." in hako_plan, "hako lifecycle plan missing explicit compat shim rule")
    require("Unknown facts do not silently become CompatShim." in hako_plan, "hako lifecycle plan missing compat shim fail-fast rule")
    require("The converter is an emission surface. It is not the ownership policy owner." in ownership_reference, "ownership reference missing converter policy split")
    require("The adapter is a Rust facts producer. It is not a Hako policy owner." in adapter_boundary, "adapter boundary missing facts-producer rule")

    return {
        "schema_version": 0,
        "kind": "MirBuilderUnsafeOrFFIInventory",
        "subject": "MirBuilder unsafe / FFI boundary",
        "source": {
            "task_order": "docs/development/current/main/design/mirbuilder-rust-to-hako-converter-task-order-ssot.md",
            "substrate_capability_ladder": "docs/development/current/main/design/substrate-capability-ladder-ssot.md",
            "hako_plan_vocab": "docs/development/current/main/design/hako-lifecycle-plan-vocab-v0.md",
            "rust_to_hako_ownership_reference": "docs/development/current/main/design/rust-to-hako-ownership-converter-reference.md",
            "rustc_semir_boundary": "docs/development/current/main/design/rustc-semir-internal-adapter-boundary.md",
        },
        "current_contract": "inventory_only",
        "decision": [
            "keep UnsafeOrFFI parked until a restricted unsafe capability contract or explicit CompatShim row is named",
            "keep broad unsafe surface and FFI separate from the easy-tier converter",
            "do not select route or nightly rustc adapter",
        ],
        "supporting_evidence": [
            "Do not replace this with a broad C-style unsafe surface.",
            "CompatShim must be explicit and diagnostic-visible.",
            "Unknown facts do not silently become CompatShim.",
            "The converter is an emission surface. It is not the ownership policy owner.",
            "The adapter is a Rust facts producer. It is not a Hako policy owner.",
        ],
        "open_questions": [
            "Should the later hard tier use restricted unsafe capability modules or a CompatShim boundary?",
            "Which future families need explicit FFI rather than pure transport?",
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

    report = inventory_unsafe_or_ffi()
    if args.check_reference:
        expected = json.loads(REFERENCE.read_text())
        require(report == expected, "unsafe or ffi inventory differs from reference fixture")
    if args.emit_json:
        print(json.dumps(report, indent=2, sort_keys=True))
        return 0

    print("output_contract=rust-mirbuilder-unsafe-or-ffi-v0")
    print("unsafe_or_ffi_recorded=1")
    print("subject=MirBuilder unsafe / FFI boundary")
    print("route_selection=0")
    print("nightly_rustc_adapter=0")
    print("decision=inventory_only")
    print("summary=ok")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
