#!/usr/bin/env python3
"""Measure the selected facade same-block get/set fusion keeper."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


ROOT = Path(__file__).resolve().parents[2]
BASE_TOOL = ROOT / "tools/allocator/typed_object_exact_slot_direct_helper_measurement.py"


def read_kv(path: Path) -> dict[str, str]:
    values: dict[str, str] = {}
    for line in path.read_text(encoding="utf-8", errors="replace").splitlines():
        if not line or line.startswith("#") or "=" not in line:
            continue
        key, value = line.split("=", 1)
        values[key] = value
    return values


def require(values: dict[str, str], key: str, expected: str) -> None:
    actual = values.get(key)
    if actual != expected:
        raise SystemExit(f"{key} expected {expected!r}, got {actual!r}")


def require_key(values: dict[str, str], key: str) -> str:
    value = values.get(key)
    if value is None:
        raise SystemExit(f"missing {key}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--sample-count", type=int, default=1)
    args = parser.parse_args()

    if args.sample_count < 1:
        raise SystemExit("--sample-count must be positive")

    with tempfile.TemporaryDirectory(prefix="hakorune_facade_get_set_measure.") as tmp:
        base_report = Path(tmp) / "facade-get-set.out"
        subprocess.run(
            [
                "python3",
                str(BASE_TOOL),
                "--sample-count",
                str(args.sample_count),
                "--out",
                str(base_report),
            ],
            cwd=ROOT,
            check=True,
            stdout=subprocess.DEVNULL,
        )
        values = read_kv(base_report)

    require(values, "output_contract", "typed-object-exact-slot-direct-helper-measurement-v0")
    require(values, "input_contract", "typed-object-exact-slot-direct-helper-implementation-v0")
    require(values, "workload_id", "representative-object-lifecycle-small-block-v0")
    require(values, "typed_object_backend", "single_thread_exact")
    require(values, "array_slot_backend", "single_thread_exact")
    require(values, "winner_claim", "0")
    require(values, "replacement_active", "0")
    require(values, "hook_installed", "0")
    require(values, "global_allocator", "0")
    require(values, "summary", "ok")

    keeper_effect = require_key(values, "keeper_effect")
    keeper = "1" if keeper_effect == "accepted" else "0"

    lines = [
        "output_contract=selected-facade-same-block-get-set-measurement-v0",
        "input_contract=selected-facade-same-block-get-set-keeper-v0",
        "base_measurement_contract=typed-object-exact-slot-direct-helper-measurement-v0",
        "workload_id=representative-object-lifecycle-small-block-v0",
        "measurement_scope=object_lifecycle_exact_exe_after_selected_facade_get_set_fusion",
        f"sample_count={args.sample_count}",
        "typed_object_backend=single_thread_exact",
        "array_slot_backend=single_thread_exact",
        f"single_thread_exact_floor_body_elapsed_ns={require_key(values, 'single_thread_exact_floor_body_elapsed_ns')}",
        f"selected_facade_get_set_body_elapsed_ns={require_key(values, 'exact_slot_helper_body_elapsed_ns')}",
        f"body_elapsed_delta_ns={require_key(values, 'body_elapsed_delta_ns')}",
        f"selected_facade_get_set_body_ratio_pct={require_key(values, 'exact_slot_helper_body_ratio_pct')}",
        f"keeper_acceptance_min_improvement_pct={require_key(values, 'keeper_acceptance_min_improvement_pct')}",
        f"single_thread_exact_floor_external_elapsed_ms={require_key(values, 'single_thread_exact_floor_external_elapsed_ms')}",
        f"selected_facade_get_set_external_elapsed_ms={require_key(values, 'exact_slot_helper_external_elapsed_ms')}",
        f"keeper_effect={keeper_effect}",
        f"selected_facade_get_set_keeper={keeper}",
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
