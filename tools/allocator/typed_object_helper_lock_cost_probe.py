#!/usr/bin/env python3
"""Sidecar probe for typed-object helper lock/global-slab cost."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path

from typed_object_helper_lock_cost_probe_source import RUST_SOURCE

def parse_report(text: str) -> dict[str, int]:
    values: dict[str, int] = {}
    for line in text.splitlines():
        if "=" not in line:
            continue
        key, value = line.split("=", 1)
        try:
            values[key] = int(value)
        except ValueError:
            continue
    return values


def positive(values: dict[str, int], key: str) -> int:
    value = values.get(key, 0)
    if value <= 0:
        raise SystemExit(f"{key} must be positive, got {value}")
    return value


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("--iterations", type=int, default=2_000_000)
    parser.add_argument("--field-dynamic-estimate", type=int, default=30_072_832)
    parser.add_argument("--perf-field-helper-pct", type=str, default="72.96")
    args = parser.parse_args()
    if args.iterations < 10_000:
        raise SystemExit("--iterations must be >= 10000")

    with tempfile.TemporaryDirectory(prefix="hakorune_typed_object_lock_probe.") as tmp:
        tmp_path = Path(tmp)
        source = tmp_path / "typed_object_lock_probe.rs"
        binary = tmp_path / "typed_object_lock_probe"
        source.write_text(RUST_SOURCE, encoding="utf-8")
        subprocess.run(
            ["rustc", "-O", str(source), "-o", str(binary)],
            check=True,
            text=True,
        )
        run = subprocess.run(
            [str(binary), str(args.iterations)],
            check=True,
            text=True,
            stdout=subprocess.PIPE,
        )

    values = parse_report(run.stdout)
    lock = positive(values, "lock_unlock_ns_per_op")
    read = positive(values, "mutex_vec_read_ns_per_op")
    write = positive(values, "mutex_vec_write_ns_per_op")
    enum_cost = positive(values, "typed_slot_match_ns_per_op")
    handle = positive(values, "handle_to_index_ns_per_op")
    slot = positive(values, "slot_normalize_ns_per_op")
    uget = positive(values, "u64_exact_get_ns_per_op")
    uset = positive(values, "u64_exact_set_ns_per_op")

    helper_avg = max(1, (read + write + uget + uset) // 4)
    lock_fraction = min(100, (lock * 100) // helper_avg)
    storage_lookup_fraction = min(100, ((read + write) * 50) // helper_avg)
    enum_fraction = min(100, (enum_cost * 100) // helper_avg)

    if lock_fraction >= 40:
        dominant = "lock_global_slab"
        recommended = "runtime_single_thread_fast_lane"
    elif enum_fraction >= 40:
        dominant = "typed_slot_value_repr"
        recommended = "typed_slot_repr_fast_lane"
    else:
        dominant = "mixed_helper_cost"
        recommended = "mir_scalar_residence_first"

    print("output_contract=typed-object-helper-lock-cost-probe-v0")
    print("input_contract=hako-mimalloc-field-array-runtime-boundary-probe-v0")
    print("workload_id=representative-object-lifecycle-small-block-v0")
    print(f"iterations={args.iterations}")
    print(f"field_dynamic_estimate={args.field_dynamic_estimate}")
    print(f"perf_field_helper_pct={args.perf_field_helper_pct}")
    for key in (
        "lock_unlock_ns_per_op",
        "mutex_vec_read_ns_per_op",
        "mutex_vec_write_ns_per_op",
        "handle_to_index_ns_per_op",
        "slot_normalize_ns_per_op",
        "typed_slot_match_ns_per_op",
        "u64_exact_get_ns_per_op",
        "u64_exact_set_ns_per_op",
        "hii_get_ns_per_op",
        "hii_set_ns_per_op",
    ):
        print(f"{key}={positive(values, key)}")
    print(f"lock_fraction_pct={lock_fraction}")
    print(f"storage_lookup_fraction_pct={storage_lookup_fraction}")
    print(f"enum_value_fraction_pct={enum_fraction}")
    print(f"dominant_helper_subowner={dominant}")
    print(f"recommended_next={recommended}")
    print("optimization_open=0")
    print("winner_claim=0")
    print("replacement_active=0")
    print("hook_installed=0")
    print("global_allocator=0")
    print("summary=ok")
    # Keep variables observed so mypy/linters don't tempt accidental removal.
    _ = (handle, slot)
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
