#!/usr/bin/env python3
"""Emit the post-plateau .hako mimalloc port feature gap inventory."""

from __future__ import annotations

import argparse
from pathlib import Path


IMPLEMENTED = [
    ("size_class_policy", "lang/src/hako_alloc/memory/size_class_box.hako"),
    ("page_model", "lang/src/hako_alloc/memory/page_box.hako"),
    ("page_queue", "lang/src/hako_alloc/memory/page_queue_box.hako"),
    ("legacy_page_heap", "lang/src/hako_alloc/memory/page_heap_box.hako"),
    ("production_facade_basic_alloc_realloc_release", "lang/src/hako_alloc/memory/allocator_facade_box.hako"),
    ("page_map_release_realloc", "lang/src/hako_alloc/memory/page_map_box.hako"),
    ("aligned_small_policy_path", "lang/src/hako_alloc/memory/page_map_aligned_small_path_box.hako"),
    ("huge_page_model_and_routes", "lang/src/hako_alloc/memory/huge_page_model_box.hako"),
    ("remote_free_policy_and_page_port", "lang/src/hako_alloc/memory/remote_free_page_integration_box.hako"),
    ("page_source_purge_recommit_routes", "lang/src/hako_alloc/memory/osvm_fast_path_reuse_route_box.hako"),
    ("secure_free_list_policy", "lang/src/hako_alloc/memory/secure_free_list_policy_box.hako"),
    ("stats_surface", "lang/src/hako_alloc/memory/stats_box.hako"),
]

MISSING = [
    ("unified_production_allocator_api", "high", "Production facade still mainly uses legacy HakoAllocHeap; page-map, aligned, huge, OSVM, purge, secure-list, and remote-free seams are separate."),
    ("real_provider_explicit_entrypoint_selection", "high", "Provider package needs an explicit .hako mimalloc API surface before LD_PRELOAD or replacement work."),
    ("page_map_aligned_huge_osvm_facade_integration", "high", "Aligned, huge, page-source, purge/recommit, and unregister routes are proven but not one facade route."),
    ("segment_arena_reclaim_tls_unification", "medium", "Segment, arena, reclaim, worker identity, and TLS rows remain scalar/model proof surfaces."),
    ("secure_entropy_backed_free_list", "medium", "Secure free list has encode/decode policy, but no entropy-backed hardening claim."),
    ("mutable_runtime_options", "low", "Options are inventory/read-only, not mimalloc-style mutable runtime options."),
    ("ld_preload_compatible_shim", "later", "Hakmem-compatible malloc/free shim is intentionally after explicit provider API evidence."),
]


def require_files(root: Path, rows: list[tuple[str, str]]) -> None:
    missing = [path for _, path in rows if not (root / path).is_file()]
    if missing:
        raise SystemExit("missing required inventory source files: " + ", ".join(missing))


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    root = args.repo_root.resolve()
    require_files(root, IMPLEMENTED)

    lines = [
        "output_contract=hako-mimalloc-port-feature-gap-inventory-v0",
        "input_contract=hako-mimalloc-post-third-keeper-taxonomy-refresh-v0",
        "small_model_checkpoint_elapsed_median_ms=240",
        "small_model_remaining_gap_ms=236",
        "optimization_checkpoint=small_model_fast_path_plateau",
        "implemented_surface_count=12",
        "missing_feature_count=7",
        "primary_gap_kind=integration_surface_gap",
        "next_port_feature=real_provider_explicit_entrypoint_selection",
        "next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-SELECTION-296X-001",
        "ld_preload_shim_ready=0",
        "provider_entrypoint_selection_ready=1",
        "winner_claim=0",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
    ]
    for index, (name, path) in enumerate(IMPLEMENTED):
        lines.append(f"implemented_{index}_feature={name}")
        lines.append(f"implemented_{index}_source={path}")
    for index, (name, priority, note) in enumerate(MISSING):
        lines.append(f"missing_{index}_feature={name}")
        lines.append(f"missing_{index}_priority={priority}")
        lines.append(f"missing_{index}_note={note}")
    lines.append("summary=ok")

    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
