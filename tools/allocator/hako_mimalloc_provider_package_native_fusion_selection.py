#!/usr/bin/env python3
"""Emit the native provider-package fusion strategy for the real .hako entrypoint."""

from __future__ import annotations

import argparse
from pathlib import Path


REQUIRED = {
    "row68_card": "docs/development/current/main/phases/phase-296x/296x-68-HAKO-MIMALLOC-PROVIDER-PACKAGE-REAL-ENTRYPOINT-PILOT.md",
    "cli_owner": "src/cli/provider_package_hako_derived_build.rs",
    "args_owner": "src/cli/args.rs",
    "surface": "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako",
}


def require_contains(root: Path, rel: str, needles: list[str]) -> None:
    path = root / rel
    if not path.is_file():
        raise SystemExit(f"missing required file: {rel}")
    text = path.read_text(encoding="utf-8", errors="replace")
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(f"{rel} missing required text: {', '.join(missing)}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    root = args.repo_root.resolve()
    require_contains(
        root,
        REQUIRED["row68_card"],
        [
            "output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0",
            "hako_selected_entrypoint_executed=1",
            "provider_package_native_fused_to_hako_entrypoint=0",
            "provider_package_native_fusion_required=1",
        ],
    )
    require_contains(
        root,
        REQUIRED["cli_owner"],
        [
            "provider_package_hako_derived_build",
            "ping-literal-v0",
            "alloc-free-owns-literal-v0",
            "hako_derived_wrapper_source",
        ],
    )
    require_contains(
        root,
        REQUIRED["args_owner"],
        ["provider-package-hako-semantic-codegen"],
    )
    require_contains(
        root,
        REQUIRED["surface"],
        ["objectLifecycleSmallAlloc(size)", "objectLifecycleReleaseBlock(page_id, block_id)"],
    )

    lines = [
        "output_contract=hako-mimalloc-provider-package-native-fusion-selection-v0",
        "input_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0",
        "selected_entrypoint=object_lifecycle_small_alloc_release_v0",
        "native_fusion_strategy=hako_derived_provider_semantic_mode_extension_v0",
        "strategy_owner=src/cli/provider_package_hako_derived_build.rs",
        "strategy_args_owner=src/cli/args.rs",
        "required_codegen_mode=object-lifecycle-small-alloc-release-v0",
        "required_fixture=apps/provider-package/hako-derived-mimalloc-real-entrypoint-fixture/main.hako",
        "required_surface_owner=HakoAllocObjectLifecycleFacade",
        "required_alloc_method=objectLifecycleSmallAlloc",
        "required_release_method=objectLifecycleReleaseBlock",
        "required_mir_call_chain_check=1",
        "required_provider_alloc_free_smoke=1",
        "provider_package_native_fusion_allowed=1",
        "provider_call_allowed=1",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "ld_preload_shim_ready=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-PILOT-296X-001",
        "summary=ok",
    ]
    report = "\n".join(lines) + "\n"
    if args.out is None:
        print(report, end="")
    else:
        args.out.parent.mkdir(parents=True, exist_ok=True)
        args.out.write_text(report, encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
