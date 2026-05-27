#!/usr/bin/env python3
"""Adapt selected .hako mimalloc entrypoint execution into row-68 pilot evidence."""

from __future__ import annotations

import argparse
import json
from pathlib import Path


APP = "apps/mimalloc-facade-release-one-block-proof/main.hako"
SURFACE_FILE = "lang/src/hako_alloc/memory/object_lifecycle_facade_box.hako"


def require_contains(path: Path, needles: list[str], label: str) -> str:
    if not path.is_file():
        raise SystemExit(f"missing {label}: {path}")
    text = path.read_text(encoding="utf-8", errors="replace")
    missing = [needle for needle in needles if needle not in text]
    if missing:
        raise SystemExit(f"{label} missing required text: {', '.join(missing)}")
    return text


def iter_calls(function: dict) -> list[dict]:
    calls: list[dict] = []
    for block in function.get("blocks", []):
        for inst in block.get("instructions", []):
            if inst.get("op") != "mir_call":
                continue
            calls.append(inst.get("mir_call", {}).get("callee", {}))
    return calls


def require_function(functions: dict[str, dict], name: str) -> dict:
    fn = functions.get(name)
    if fn is None:
        raise SystemExit(f"missing MIR function: {name}")
    return fn


def require_method_call(function: dict, box_name: str, method_name: str) -> None:
    for callee in iter_calls(function):
        if (
            callee.get("type") == "Method"
            and callee.get("box_name") == box_name
            and callee.get("name") == method_name
        ):
            return
    raise SystemExit(
        f"missing method call in {function.get('name')}: {box_name}.{method_name}"
    )


def validate_mir(mir_json: Path) -> None:
    data = json.loads(mir_json.read_text(encoding="utf-8"))
    functions = {fn.get("name"): fn for fn in data.get("functions", [])}
    main = require_function(functions, "main")
    small_alloc = require_function(
        functions, "HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1"
    )
    release = require_function(
        functions, "HakoAllocObjectLifecycleFacade.objectLifecycleReleaseBlock/2"
    )

    for name in (
        "objectLifecycleAddPage",
        "objectLifecycleSmallAlloc",
        "objectLifecycleAllocPageId",
        "objectLifecycleAllocBlockId",
        "objectLifecycleReleaseBlock",
        "objectLifecycleReleasePageId",
        "objectLifecycleReleaseBlockId",
        "objectLifecycleReleaseReason",
        "objectLifecycleReleaseOk",
    ):
        require_method_call(main, "HakoAllocObjectLifecycleFacade", name)

    require_method_call(
        small_alloc, "HakoAllocObjectLifecyclePageQueue", "selectPage"
    )
    require_method_call(small_alloc, "HakoAllocPageModel", "acquire")
    require_method_call(
        release, "HakoAllocObjectLifecycleFacade", "objectLifecycleKnownPageIndexById"
    )
    require_method_call(release, "HakoAllocPageModel", "releaseLocal")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo-root", type=Path, default=Path("."))
    parser.add_argument("--mir-json", type=Path, required=True)
    parser.add_argument("--run-log", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    root = args.repo_root.resolve()
    app = root / APP
    surface = root / SURFACE_FILE
    require_contains(
        surface,
        [
            "box HakoAllocObjectLifecycleFacade",
            "objectLifecycleSmallAlloc(size)",
            "objectLifecycleReleaseBlock(page_id, block_id)",
        ],
        "selected surface",
    )
    require_contains(
        app,
        [
            "objectLifecycleSmallAlloc(8)",
            "objectLifecycleReleaseBlock(alloc_page, alloc_block)",
            'print("summary=ok")',
        ],
        "pilot app",
    )
    validate_mir(args.mir_json)
    require_contains(
        args.run_log,
        [
            "mimalloc-facade-release-one-block-proof",
            "alloc=90,0",
            "release=90,0,0",
            "release_counts=1,0",
            "summary=ok",
        ],
        "pilot run log",
    )

    lines = [
        "output_contract=hako-mimalloc-provider-real-entrypoint-pilot-v0",
        "input_contract=hako-mimalloc-provider-real-entrypoint-selection-v0",
        "selected_entrypoint=object_lifecycle_small_alloc_release_v0",
        "selected_surface_owner=HakoAllocObjectLifecycleFacade",
        f"selected_surface_file={SURFACE_FILE}",
        f"pilot_app={APP}",
        "provider_call_kind=hako_exact_exe_selected_entrypoint_pilot",
        "provider_call_executed=1",
        "hako_selected_entrypoint_executed=1",
        "alloc_method_called=objectLifecycleSmallAlloc",
        "release_method_called=objectLifecycleReleaseBlock",
        "alloc_observer_result=90,0",
        "release_observer_result=90,0,0",
        "release_counts=1,0",
        "mir_call_chain_verified=1",
        "exact_exe_run_verified=1",
        "provider_package_native_artifact_generated=0",
        "provider_package_native_fused_to_hako_entrypoint=0",
        "provider_package_native_fusion_required=1",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "ld_preload_shim_ready=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-PROVIDER-PACKAGE-NATIVE-FUSION-SELECTION-296X-001",
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
