#!/usr/bin/env python3
"""Adapt native provider-package fusion build/smoke evidence into row-70 output."""

from __future__ import annotations

import argparse
from pathlib import Path


MODE = "object-lifecycle-small-alloc-release-v0"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str, label: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{label}: expected {key}={expected!r}, got {actual!r}")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-report", type=Path, required=True)
    parser.add_argument("--alloc-free-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    build = read_kv(args.build_report)
    smoke = read_kv(args.alloc_free_report)

    require(build, "output_contract", "hakorune-provider-package-hako-derived-build-v0", "build")
    require(build, "hako_semantic_provider_codegen", MODE, "build")
    require(build, "hako_provider_object_lifecycle_codegen", "1", "build")
    require(build, "hako_provider_object_lifecycle_entrypoint_verified", "1", "build")
    require(build, "shared_library_artifact_generated", "1", "build")
    require(build, "provider_call_executed", "0", "build")
    require(build, "summary", "ok", "build")

    require(smoke, "output_contract", "hakorune-provider-package-alloc-free-smoke-v0", "smoke")
    require(smoke, "provider_call_executed", "1", "smoke")
    require(smoke, "provider_alloc_executed", "1", "smoke")
    require(smoke, "provider_free_executed", "1", "smoke")
    require(smoke, "provider_owns_result", "1", "smoke")
    require(smoke, "provider_active", "0", "smoke")
    require(smoke, "replacement_active", "0", "smoke")
    require(smoke, "hook_installed", "0", "smoke")
    require(smoke, "global_allocator", "0", "smoke")
    require(smoke, "winner_claim", "0", "smoke")
    require(smoke, "summary", "ok", "smoke")

    lines = [
        "output_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0",
        "input_contract=hako-mimalloc-provider-package-native-fusion-selection-v0",
        "selected_entrypoint=object_lifecycle_small_alloc_release_v0",
        f"hako_semantic_provider_codegen={MODE}",
        f"hako_source_path={build['hako_source_path']}",
        f"hako_mir_json_path={build['hako_mir_json_path']}",
        f"manifest_path={build['manifest_path']}",
        f"artifact_path={build['artifact_path']}",
        f"artifact_sha256={build['artifact_sha256']}",
        f"contract_hash={build['contract_hash']}",
        "hako_entrypoint_mir_call_chain_verified=1",
        "provider_package_native_fusion_pilot_executed=1",
        "provider_call_executed=1",
        "provider_alloc_executed=1",
        "provider_free_executed=1",
        "provider_owns_result=1",
        "provider_active=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "ld_preload_shim_ready=0",
        "winner_claim=0",
        "next_row=HAKO-MIMALLOC-HAKMEM-LDPRELOAD-SHIM-DECISION-296X-001",
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
