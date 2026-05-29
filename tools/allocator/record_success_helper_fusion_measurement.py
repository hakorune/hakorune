#!/usr/bin/env python3
"""Measure the recordSuccess helper-fusion exact lane."""

from __future__ import annotations

import argparse
import os
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
APP = ROOT / "apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako"
RUNNER = ROOT / "tools/allocator/hako_exe_memory_runner.sh"
WORKLOAD = "representative-object-lifecycle-small-block-v0"


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
        raise SystemExit(f"{label}: {key} expected {expected!r}, got {actual!r}")


def require_positive_int(values: dict[str, str], key: str, label: str) -> int:
    text = values.get(key)
    if text is None:
        raise SystemExit(f"{label}: missing {key}")
    try:
        value = int(text)
    except ValueError as exc:
        raise SystemExit(f"{label}: {key} must be int, got {text!r}") from exc
    if value <= 0:
        raise SystemExit(f"{label}: {key} must be positive, got {value}")
    return value


def median(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def ratio_pct(numerator: int, denominator: int) -> int:
    return int(round((numerator * 100) / denominator)) if denominator else 0


def run_one(tmp_dir: Path, sample_idx: int) -> dict[str, str]:
    report = tmp_dir / f"record-success-helper-fusion-{sample_idx}.out"
    env = os.environ.copy()
    env["HAKO_TYPED_OBJECT_STORE"] = "single_thread_exact"
    env["HAKO_ARRAY_SLOT_STORE"] = "single_thread_exact"
    env["HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER"] = "1"
    subprocess.run(
        [
            "bash",
            str(RUNNER),
            "--app",
            str(APP),
            "--workload",
            WORKLOAD,
            "--runtime-config",
            "empty",
            "--operation-repeat",
            "1",
            "--out",
            str(report),
        ],
        cwd=ROOT,
        env=env,
        stdout=subprocess.DEVNULL,
        check=True,
    )
    values = read_kv(report)
    label = f"record_success_helper_fusion sample {sample_idx}"
    require(values, "output_contract", "hako-exe-memory-evidence-v0", label)
    require(values, "summary", "ok", label)
    require(values, "workload", WORKLOAD, label)
    require(values, "provider_activation", "0", label)
    require(values, "host_replacement", "0", label)
    require(values, "hook_installed", "0", label)
    require(values, "global_allocator_installed", "0", label)
    require(values, "allocation_count", "524288", label)
    require(values, "free_count", "524288", label)
    require(values, "select_page_single_fast_path_count", "524288", label)
    require(values, "select_page_single_fallback_count", "0", label)
    require(values, "release_known_page_fast_path_count", "524288", label)
    require(values, "release_known_page_fallback_count", "0", label)
    return values


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sample-count", type=int, default=1)
    parser.add_argument("--baseline-body-ns", type=int, default=110_000_000)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")
    if args.baseline_body_ns <= 0:
        raise SystemExit("--baseline-body-ns must be positive")

    subprocess.run(["bash", str(ROOT / "tools/build_hako_llvmc_ffi.sh")], cwd=ROOT, check=True)
    subprocess.run(["cargo", "build", "--release", "-p", "nyash_kernel"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_record_success_measure.") as tmp:
        tmp_dir = Path(tmp)
        samples = [run_one(tmp_dir, idx) for idx in range(args.sample_count)]

    body_values = [
        require_positive_int(sample, "body_elapsed_ns", f"sample {idx}")
        for idx, sample in enumerate(samples)
    ]
    external_values = [
        require_positive_int(sample, "external_elapsed_ms", f"sample {idx}")
        for idx, sample in enumerate(samples)
    ]
    body_median = median(body_values)
    external_median = median(external_values)
    delta = args.baseline_body_ns - body_median
    ratio = ratio_pct(body_median, args.baseline_body_ns)
    accepted = delta > 0 and ratio <= 97

    lines = [
        "output_contract=record-success-helper-fusion-measurement-v0",
        "input_contract=record-success-helper-fusion-implementation-v0",
        f"workload_id={WORKLOAD}",
        "measurement_scope=object_lifecycle_exact_exe_record_success_helper_fusion",
        f"sample_count={args.sample_count}",
        "typed_object_backend=single_thread_exact",
        "array_slot_backend=single_thread_exact",
        "baseline_row=296x-259",
        f"single_thread_exact_floor_body_elapsed_ns={args.baseline_body_ns}",
        f"record_success_helper_fusion_body_elapsed_ns={body_median}",
        f"body_elapsed_delta_ns={delta}",
        f"record_success_helper_fusion_body_ratio_pct={ratio}",
        "keeper_acceptance_min_improvement_pct=3",
        f"record_success_helper_fusion_external_elapsed_ms={external_median}",
        f"keeper_effect={'accepted' if accepted else 'no_effect'}",
        f"record_success_helper_fusion_keeper={1 if accepted else 0}",
        "next_diagnostic=post_record_success_helper_fusion_owner_refresh",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    for idx, value in enumerate(body_values):
        lines.append(f"sample_{idx}_body_elapsed_ns={value}")
    for idx, value in enumerate(external_values):
        lines.append(f"sample_{idx}_external_elapsed_ms={value}")

    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
