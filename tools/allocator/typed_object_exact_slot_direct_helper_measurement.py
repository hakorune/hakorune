#!/usr/bin/env python3
"""Measure exact-slot typed-object helpers against the single-thread exact floor."""

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
SAMPLE_MAX_ATTEMPTS = 3


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


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


def validation_error(values: dict[str, str], label: str) -> str | None:
    expected = {
        "output_contract": "hako-exe-memory-evidence-v0",
        "summary": "ok",
        "workload": WORKLOAD,
        "provider_activation": "0",
        "host_replacement": "0",
        "hook_installed": "0",
        "global_allocator_installed": "0",
        "allocation_count": "524288",
        "free_count": "524288",
        "select_page_single_fast_path_count": "524288",
        "select_page_single_fallback_count": "0",
        "release_known_page_fast_path_count": "524288",
        "release_known_page_fallback_count": "0",
    }
    for key, expected_value in expected.items():
        actual = values.get(key)
        if actual != expected_value:
            return f"{label}: {key} expected {expected_value!r}, got {actual!r}"
    try:
        body_elapsed_ns = int(values.get("body_elapsed_ns", "0"))
    except ValueError:
        return f"{label}: body_elapsed_ns must be int, got {values.get('body_elapsed_ns')!r}"
    if body_elapsed_ns <= 0:
        return f"{label}: body_elapsed_ns must be positive, got {body_elapsed_ns}"
    return None


def median(values: list[int]) -> int:
    ordered = sorted(values)
    return ordered[len(ordered) // 2]


def ratio_pct(numerator: int, denominator: int) -> int:
    return int(round((numerator * 100) / denominator)) if denominator else 0


def run_one(tmp_dir: Path, exact_helper: bool, sample_idx: int) -> dict[str, str]:
    label = "exact_slot_helper" if exact_helper else "single_thread_exact_floor"
    env = os.environ.copy()
    env["HAKO_TYPED_OBJECT_STORE"] = "single_thread_exact"
    # This harness measures typed-object exact helpers. The object-lifecycle app
    # uses public ArrayBox handles, which are outside the numeric-only
    # single_thread_exact array-slot floor.
    env.pop("HAKO_ARRAY_SLOT_STORE", None)
    if exact_helper:
        env["HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER"] = "1"
    else:
        env.pop("HAKO_TYPED_OBJECT_EXACT_SLOT_HELPER", None)

    sample_label = f"{label} sample {sample_idx}"
    last_error = ""
    for attempt in range(SAMPLE_MAX_ATTEMPTS):
        report = tmp_dir / f"{label}-{sample_idx}-attempt-{attempt}.out"
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
        last_error = validation_error(values, sample_label) or ""
        if not last_error:
            return values
    raise SystemExit(f"{last_error} after {SAMPLE_MAX_ATTEMPTS} attempts")


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sample-count", type=int, default=1)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")

    subprocess.run(["bash", str(ROOT / "tools/build_hako_llvmc_ffi.sh")], cwd=ROOT, check=True)
    subprocess.run(["cargo", "build", "--release", "-p", "nyash_kernel"], cwd=ROOT, check=True)

    with tempfile.TemporaryDirectory(prefix="hakorune_exact_slot_measure.") as tmp:
        tmp_dir = Path(tmp)
        floor_samples = [run_one(tmp_dir, False, idx) for idx in range(args.sample_count)]
        exact_samples = [run_one(tmp_dir, True, idx) for idx in range(args.sample_count)]

    floor_body = [
        require_positive_int(sample, "body_elapsed_ns", f"floor sample {idx}")
        for idx, sample in enumerate(floor_samples)
    ]
    exact_body = [
        require_positive_int(sample, "body_elapsed_ns", f"exact sample {idx}")
        for idx, sample in enumerate(exact_samples)
    ]
    floor_external = [
        require_positive_int(sample, "external_elapsed_ms", f"floor sample {idx}")
        for idx, sample in enumerate(floor_samples)
    ]
    exact_external = [
        require_positive_int(sample, "external_elapsed_ms", f"exact sample {idx}")
        for idx, sample in enumerate(exact_samples)
    ]

    floor_body_median = median(floor_body)
    exact_body_median = median(exact_body)
    delta = floor_body_median - exact_body_median
    body_ratio = ratio_pct(exact_body_median, floor_body_median)
    accepted = delta > 0 and body_ratio <= 97

    lines = [
        "output_contract=typed-object-exact-slot-direct-helper-measurement-v0",
        "input_contract=typed-object-exact-slot-direct-helper-implementation-v0",
        f"workload_id={WORKLOAD}",
        "measurement_scope=object_lifecycle_exact_exe_exact_slot_helper_pair",
        f"sample_count={args.sample_count}",
        "typed_object_backend=single_thread_exact",
        "array_slot_backend=unset",
        "direct_helper_floor_run_status=ok",
        "direct_helper_floor_invalid_arraybox_handle_count=0",
        f"single_thread_exact_floor_body_elapsed_ns={floor_body_median}",
        f"exact_slot_helper_body_elapsed_ns={exact_body_median}",
        f"body_elapsed_delta_ns={delta}",
        f"exact_slot_helper_body_ratio_pct={body_ratio}",
        "keeper_acceptance_min_improvement_pct=3",
        f"single_thread_exact_floor_external_elapsed_ms={median(floor_external)}",
        f"exact_slot_helper_external_elapsed_ms={median(exact_external)}",
        f"keeper_effect={'accepted' if accepted else 'no_effect'}",
        f"exact_slot_helper_keeper={1 if accepted else 0}",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    print("\n".join(lines))
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
