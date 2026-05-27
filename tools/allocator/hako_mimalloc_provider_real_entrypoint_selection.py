#!/usr/bin/env python3
"""Emit the selected real .hako mimalloc explicit provider entrypoint."""

from __future__ import annotations

import argparse
from pathlib import Path


SURFACE_FILE = "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"
SURFACE_OWNER = "HakoAllocObjectLifecycleFacade"
SELECTED_ALLOC = "objectLifecycleSmallAlloc"
SELECTED_RELEASE = "objectLifecycleReleaseBlock"
SELECTED_PAGE_ID = "objectLifecycleAllocPageId"
SELECTED_BLOCK_ID = "objectLifecycleAllocBlockId"

REJECTED = [
    (
        "production_facade_basic_alloc_release_v0",
        "HakoAllocProductionFacade",
        "Production facade is still mainly backed by the older HakoAllocHeap route, so it would hide the post-plateau integration gap.",
    ),
    (
        "ld_preload_malloc_free_v0",
        "malloc_free_symbol_family",
        "LD_PRELOAD-compatible malloc/free replacement needs explicit provider call evidence first.",
    ),
]


def require_text(path: Path, needles: list[str]) -> None:
    if not path.is_file():
        raise SystemExit(f"missing selected surface file: {path}")
    text = path.read_text(encoding="utf-8")
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(
            "selected surface is missing required entrypoint methods: "
            + ", ".join(missing)
        )


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    root = args.repo_root.resolve()
    require_text(
        root / SURFACE_FILE,
        [
            f"box {SURFACE_OWNER}",
            f"{SELECTED_ALLOC}(size)",
            f"{SELECTED_RELEASE}(page_id, block_id)",
            f"{SELECTED_PAGE_ID}()",
            f"{SELECTED_BLOCK_ID}()",
        ],
    )

    lines = [
        "output_contract=hako-mimalloc-provider-real-entrypoint-selection-v0",
        "input_contract=hako-mimalloc-port-feature-gap-inventory-v0",
        "selected_entrypoint=object_lifecycle_small_alloc_release_v0",
        f"selected_surface_owner={SURFACE_OWNER}",
        f"selected_surface_file={SURFACE_FILE}",
        f"selected_alloc_method={SELECTED_ALLOC}",
        f"selected_release_method={SELECTED_RELEASE}",
        f"selected_page_id_method={SELECTED_PAGE_ID}",
        f"selected_block_id_method={SELECTED_BLOCK_ID}",
        "selected_surface_scope=small_block_object_lifecycle",
        "selected_surface_reason=real_hako_mimalloc_facade_with_page_selection_reuse_release_and_observers",
        "provider_call_allowed=1",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "ld_preload_shim_ready=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT-296X-001",
    ]
    for index, (entrypoint, owner, reason) in enumerate(REJECTED):
        lines.append(f"rejected_{index}_entrypoint={entrypoint}")
        lines.append(f"rejected_{index}_surface_owner={owner}")
        lines.append(f"rejected_{index}_reason={reason}")
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
