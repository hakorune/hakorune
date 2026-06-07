#!/usr/bin/env python3
"""Emit FastMemory MIR-to-LLVM producer evidence from a MIR JSON file.

This tool is observation-only: it does not decide routes or rewrite MIR. It
first asks the existing Python LLVM producer to compile the MIR JSON, then emits
producer-neutral KV evidence from the verified FastMemory metadata that the
producer consumed successfully.
"""

from __future__ import annotations

import argparse
import sys
import tempfile
from pathlib import Path
from typing import Any

from fastmem_mir_to_llvm_producer_report_common import (
    ROOT,
    LLVM_BUILDER,
    branch_cfg_count,
    count_memops,
    count_plans,
    fastmem_access_plans,
    fastmem_free_head_non_empty_facts,
    fastmem_memops,
    fastmem_regions,
    function_has_fastmem_region,
    functions,
    int_flag,
    is_verified,
    load_json,
    metadata_facts,
    page_local_alloc_route_candidate,
    page_local_free_route_candidate,
    run_llvm_builder,
    string_value,
)
from fastmem_mir_to_llvm_producer_report_body import build_report_rows
from fastmem_mir_to_llvm_producer_report_tail_rows import build_tail_rows
from fastmem_route_profiles import (
    abandoned_reclaim_preflight_profile,
    abandoned_reclaim_producer_profile,
    fastmem_branch_cfg_lowering_preflight_profile,
    fastmem_branch_cfg_lowering_profile,
    fastmem_branch_cfg_preflight_profile,
    global_allocator_claim_preflight_profile,
    global_allocator_claim_producer_profile,
    hook_install_preflight_profile,
    hook_install_producer_profile,
    owner_slot_reuse_preflight_profile,
    owner_slot_reuse_producer_profile,
    page_local_alloc_route_cfg_producer_profile,
    page_local_alloc_route_cfg_preflight_profile,
    page_local_free_route_cfg_preflight_profile,
    page_local_free_route_cfg_producer_profile,
    terminal_ladder_refresh_preflight_profile,
    tls_backing_transfer_preflight_refresh_profile,
    product_activation_preflight_profile,
    product_activation_producer_profile,
    remote_owner_branch_route_body_preflight_profile,
    remote_owner_branch_routing_lowering_preflight_profile,
    remote_owner_branch_routing_lowering_profile,
    remote_owner_branch_routing_preflight_profile,
    same_remote_free_body_preflight_profile,
    same_remote_free_body_producer_profile,
    tls_backing_transfer_preflight_profile,
    tls_backing_transfer_producer_profile,
    winner_claim_preflight_profile,
    winner_claim_producer_profile,
)


def build_rows(
    mir: dict[str, Any], *, object_out: Path, profile: str
) -> list[tuple[str, str]]:
    return build_report_rows(mir, object_out=object_out, profile=profile)




def write_rows(rows: list[tuple[str, str]], out: Path | None) -> None:
    text = "".join(f"{key}={value}\n" for key, value in rows)
    if out is None:
        sys.stdout.write(text)
    else:
        out.parent.mkdir(parents=True, exist_ok=True)
        out.write_text(text, encoding="utf-8")


def parse_args(argv: list[str]) -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--mir-json", required=True, type=Path)
    parser.add_argument("--out", type=Path)
    parser.add_argument("--object-out", type=Path)
    parser.add_argument(
        "--profile",
        choices=(
            "layout-table",
            "owner-runtime",
            "local-free",
            "remote-free-preflight",
            "remote-free",
            "remote-free-retry-preflight",
            "remote-free-retry",
            "remote-free-drain-preflight",
            "remote-free-drain-exchange-selection",
            "remote-free-drain-exchange",
            "remote-free-drain-to-local-selection",
            "remote-free-drain-to-local",
            "remote-free-drain-local-list-mutation-preflight",
            "remote-free-drain-local-list-mutation-proof",
            "remote-free-drain-local-list-mutation-vocabulary-preflight",
            "remote-free-drain-local-list-mutation-verifier-preconditions",
            "remote-free-drain-local-list-mutation-lowering",
            "remote-owner-branch-routing-preflight",
            "remote-owner-branch-routing-lowering-preflight",
            "remote-owner-branch-routing-lowering",
            "remote-owner-branch-route-body-preflight",
            "fastmem-branch-cfg-preflight",
            "fastmem-branch-cfg-lowering-preflight",
            "fastmem-branch-cfg-lowering",
            "same-remote-free-body-preflight",
            "same-remote-free-body",
            "page-local-alloc-route-cfg-preflight",
            "page-local-alloc-route-cfg",
            "page-local-free-route-cfg-preflight",
            "page-local-free-route-cfg",
            "page-local-route-body-join-preflight",
            "page-local-route-body-join",
            "terminal-ladder-refresh-preflight",
            "tls-backing-transfer-preflight-refresh",
            "tls-backing-transfer-preflight",
            "tls-backing-transfer-producer-pilot",
            "owner-slot-reuse-preflight",
            "owner-slot-reuse-producer-pilot",
            "abandoned-reclaim-preflight",
            "abandoned-reclaim-producer-pilot",
            "product-activation-preflight",
            "product-activation-producer-pilot",
            "hook-install-preflight",
            "hook-install-producer-pilot",
            "global-allocator-claim-preflight",
            "global-allocator-claim-producer-pilot",
            "winner-claim-preflight",
            "winner-claim-producer-pilot",
        ),
        default="layout-table",
        help="evidence profile to emit after compiling the MIR JSON",
    )
    return parser.parse_args(argv)


def main(argv: list[str] | None = None) -> int:
    args = parse_args(sys.argv[1:] if argv is None else argv)
    mir_json = args.mir_json.resolve()
    mir = load_json(mir_json)

    if args.profile in {
        "remote-free-preflight",
        "remote-free-retry-preflight",
        "remote-free-drain-local-list-mutation-vocabulary-preflight",
        "remote-free-drain-local-list-mutation-verifier-preconditions",
    }:
        object_out = (
            args.object_out.resolve()
            if args.object_out is not None
            else Path("not_emitted_atomic_remote_head_cas_lowering_closed")
        )
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
        return 0

    if args.object_out is not None:
        object_out = args.object_out.resolve()
        object_out.parent.mkdir(parents=True, exist_ok=True)
        run_llvm_builder(mir_json, object_out)
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
        return 0

    with tempfile.TemporaryDirectory(prefix="hako_fastmem_llvm.") as tmp:
        object_out = Path(tmp) / "fastmem_pilot.o"
        run_llvm_builder(mir_json, object_out)
        rows = build_rows(mir, object_out=object_out, profile=args.profile)
        write_rows(rows, args.out)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
