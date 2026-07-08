#!/usr/bin/env python3
"""Inventory ScalarKnown `.hako` adoption to Rust fast-path connection state."""

from __future__ import annotations

import argparse
import json
from pathlib import Path
from typing import Any

from shared_family_generator import sha256_file, stable_json, write_if_changed


ROOT = Path(__file__).resolve().parents[2]
FIXTURES = ROOT / "docs/development/current/main/design/fixtures/rust-lifecycle"
OUTPUT = FIXTURES / "mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory-v0.json"

TOKEN = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-ADOPTION-CONNECTION-INVENTORY-001"
NEXT_CARD = "MIRBUILDER-SCALAR-KNOWN-FASTPATH-HAKO-CONNECTION-DESIGN-CONSULTATION-001"

ROUTE_PLAN = ROOT / "src/mir/generic_method_route_plan.rs"
CONTRACT_RS = ROOT / "src/mir/generic_method_route_plan/scalar_known_typed_direct_closeout_contract.rs"
WRITE_ROUTES = ROOT / "src/mir/generic_method_route_plan/write_routes.rs"
ROUTE_FIXPOINT = ROOT / "src/mir/route_fixpoint.rs"
ROUTE_JSON = ROOT / "src/runner/mir_json_emit/route_json.rs"
C_SHIM_MATCH = ROOT / "lang/c-abi/shims/hako_llvmc_ffi_generic_method_match.inc"
HAKO_SOURCES = [
    ROOT / "lang/src/compiler/lib/write_push_surface_policy_classifier.hako",
    ROOT / "lang/src/compiler/lib/write_set_mapstore_i64_policy_classifier.hako",
    ROOT / "lang/src/compiler/lib/write_set_mapstore_any_policy_classifier.hako",
    ROOT / "lang/src/compiler/lib/generic_method_route_fact_token_formatter.hako",
]


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8")


def rust_external_reference_count(symbols: list[str]) -> int:
    count = 0
    for path in (ROOT / "src").rglob("*.rs"):
        if path == CONTRACT_RS:
            continue
        text = read(path)
        count += sum(text.count(symbol) for symbol in symbols)
    return count


def build_fixture() -> dict[str, Any]:
    route_plan = read(ROUTE_PLAN)
    write_routes = read(WRITE_ROUTES)
    contract = read(CONTRACT_RS)
    hako_texts = {rel(path): read(path) for path in HAKO_SOURCES}
    contract_symbols = [
        "ScalarKnownTypedDirectCloseoutContract",
        "accepted_scalar_known_contracts",
        "candidate_scalar_known_surfaces",
    ]
    external_refs = rust_external_reference_count(contract_symbols)

    rust_execution_path = [
        {
            "owner": "generic_method_route_plan",
            "path": rel(ROUTE_PLAN),
            "role": "route refresh entry",
            "evidence": "refresh_module_generic_method_routes",
        },
        {
            "owner": "write_routes",
            "path": rel(WRITE_ROUTES),
            "role": "Write route fast-path classifier",
            "evidence": "match_generic_set_route selects MapStoreI64/MapStoreAny",
        },
        {
            "owner": "route_fixpoint",
            "path": rel(ROUTE_FIXPOINT),
            "role": "route refresh iteration",
            "evidence": "refresh_module_generic_method_routes",
        },
        {
            "owner": "route_json",
            "path": rel(ROUTE_JSON),
            "role": "lowering_plan export",
            "evidence": "metadata.generic_method_routes",
        },
        {
            "owner": "c_shim_generic_method_match",
            "path": rel(C_SHIM_MATCH),
            "role": "backend route consumption",
            "evidence": "generic method route registry",
        },
    ]

    return {
        "schema_version": 0,
        "kind": "MirBuilderScalarKnownFastpathHakoAdoptionConnectionInventoryV1",
        "token": TOKEN,
        "source_files": {
            rel(ROUTE_PLAN): sha256_file(ROUTE_PLAN),
            rel(CONTRACT_RS): sha256_file(CONTRACT_RS),
            rel(WRITE_ROUTES): sha256_file(WRITE_ROUTES),
            rel(ROUTE_FIXPOINT): sha256_file(ROUTE_FIXPOINT),
            rel(ROUTE_JSON): sha256_file(ROUTE_JSON),
            rel(C_SHIM_MATCH): sha256_file(C_SHIM_MATCH),
            **{rel(path): sha256_file(path) for path in HAKO_SOURCES},
        },
        "rust_execution_path": rust_execution_path,
        "contract_inventory": {
            "module_declared_in_route_plan": "mod scalar_known_typed_direct_closeout_contract;" in route_plan,
            "contract_table_defined": "SCALAR_KNOWN_TYPED_DIRECT_CLOSEOUT_CONTRACTS" in contract,
            "external_rust_reference_count": external_refs,
            "fastpath_connected": external_refs > 0,
        },
        "hako_adoption_inventory": {
            "hako_policy_mirror_sources": list(hako_texts),
            "all_sources_mark_classifier_policy_mirror_only": all(
                "classifier_policy_mirror_only" in text for text in hako_texts.values()
            ),
            "all_sources_decline_route_selection_or_backend_lowering": all(
                "route selection" in text and "backend lowering" in text
                for text in hako_texts.values()
            ),
            "compiler_runtime_connection_found": False,
        },
        "summary": {
            "scalar_known_fastpath_hako_adoption_connection_inventory": 1,
            "rust_fastpath_owner_still_write_routes": 1,
            "contract_module_declared": 1,
            "contract_external_rust_reference_count": external_refs,
            "contract_fastpath_connected": 1 if external_refs > 0 else 0,
            "hako_policy_mirror_guard_only": 1,
            "hako_fastpath_runtime_connection": 0,
            "hako_adopted_as_runtime_authority": 0,
            "source_selfhost_claim": 0,
            "closeout_chain_pause_required": 1,
        },
        "decision": {
            "kind": "DesignConsultationRequired",
            "reason_token": "HakoAdoptionMirrorNotConnectedToRustFastpath",
            "selected_next_card": NEXT_CARD,
        },
        "pro_consultation_question": {
            "question": (
                "ScalarKnown `.hako` adoption is currently a guard-executed Rust-oracle "
                "mirror, while Rust fast-path truth remains write_routes/collection_read_routes/"
                "string_routes -> generic_method_routes -> lowering_plan/C shim. Should the next "
                "move be A) redefine HakoAdopted as executable mirror only and stop closeout "
                "claims, or B) choose one narrow surface such as SetSurfacePolicy/MapStoreI64 and "
                "connect a generated/compiled `.hako` artifact to the Rust fast-path decision point?"
            ),
            "must_answer": [
                "Which connection mechanism is allowed for the first real fast-path handoff?",
                "Which single surface is the minimum safe first connection?",
                "What claims remain forbidden until the Rust execution path consumes the `.hako` artifact?",
            ],
        },
        "claims": {
            "inventory_only": 1,
            "connection_design_consultation_required": 1,
            "rust_fastpath_rewired": 0,
            "hako_runtime_route_authority": 0,
            "hako_backend_lowering_authority": 0,
            "write_scalar_i64_routes_closeout": 0,
            "scalar_known_transport_axis_closeout": 0,
            "source_selfhost_claim": 0,
            "runtime_fallback": 0,
            "new_backend_route": 0,
            "new_abi": 0,
        },
    }


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true", help="Verify checked-in fixture.")
    args = parser.parse_args()

    output = stable_json(build_fixture())
    if args.check:
        if not OUTPUT.exists() or OUTPUT.read_text(encoding="utf-8") != output:
            raise SystemExit(f"{rel(OUTPUT)} is stale; rerun without --check")
        print("mirbuilder-scalar-known-fastpath-hako-adoption-connection-inventory unchanged")
        return 0

    changed = write_if_changed(OUTPUT, output)
    print(("updated=" if changed else "unchanged=") + rel(OUTPUT))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
