#!/usr/bin/env python3
"""Normalize native-fusion provider repeated measurement before LD_PRELOAD."""

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


def require_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"{label}: missing {key}")
    try:
        return int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-report", type=Path, required=True)
    parser.add_argument("--measurement-report", type=Path, required=True)
    parser.add_argument("--out", type=Path)
    args = parser.parse_args()

    build = read_kv(args.build_report)
    measurement = read_kv(args.measurement_report)

    require(build, "output_contract", "hakorune-provider-package-hako-derived-build-v0", "build")
    require(build, "hako_semantic_provider_codegen", MODE, "build")
    require(build, "hako_provider_object_lifecycle_entrypoint_verified", "1", "build")
    require(build, "summary", "ok", "build")

    require(
        measurement,
        "output_contract",
        "hakorune-provider-explicit-repeated-measurement-v0",
        "measurement",
    )
    require(measurement, "provider_call_executed", "1", "measurement")
    require(measurement, "provider_alloc_executed", "1", "measurement")
    require(measurement, "provider_free_executed", "1", "measurement")
    require(measurement, "provider_active", "0", "measurement")
    require(measurement, "replacement_active", "0", "measurement")
    require(measurement, "hook_installed", "0", "measurement")
    require(measurement, "global_allocator", "0", "measurement")
    require(measurement, "winner_claim", "0", "measurement")
    require(measurement, "summary", "ok", "measurement")

    sample_median_ns = require_int(measurement, "sample_elapsed_median_ns", "measurement")
    operation_repeat = require_int(measurement, "operation_repeat", "measurement")
    per_op_median_ns = sample_median_ns // operation_repeat

    lines = [
        "output_contract=hako-mimalloc-provider-package-native-fusion-explicit-measurement-v0",
        "input_contract=hako-mimalloc-provider-package-native-fusion-pilot-v0",
        "selected_entrypoint=object_lifecycle_small_alloc_release_v0",
        f"hako_semantic_provider_codegen={MODE}",
        "measurement_profile=provider-native-fusion-explicit-repeated-v0",
        f"sample_count={measurement['sample_count']}",
        f"warmup_count={measurement['warmup_count']}",
        f"operation_repeat={measurement['operation_repeat']}",
        f"request_size={measurement['request_size']}",
        f"request_align={measurement['request_align']}",
        f"sample_elapsed_min_ns={measurement['sample_elapsed_min_ns']}",
        f"sample_elapsed_median_ns={sample_median_ns}",
        f"sample_elapsed_max_ns={measurement['sample_elapsed_max_ns']}",
        f"provider_per_operation_median_ns={per_op_median_ns}",
        f"allocation_count={measurement['allocation_count']}",
        f"free_count={measurement['free_count']}",
        f"requested_bytes={measurement['requested_bytes']}",
        "provider_explicit_measurement_ready=1",
        "ld_preload_decision_ready=1",
        "provider_call_executed=1",
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
