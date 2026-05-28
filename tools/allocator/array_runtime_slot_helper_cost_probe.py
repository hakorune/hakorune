#!/usr/bin/env python3
"""Sidecar probe for ArrayBox runtime slot helper cost."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


RUST_SOURCE = r'''
use std::sync::{Arc, OnceLock, RwLock};
use std::time::Instant;

#[derive(Clone)]
struct ArrayBoxLike {
    values: Arc<RwLock<Vec<i64>>>,
}

static ARRAY_CACHE: OnceLock<Vec<Arc<ArrayBoxLike>>> = OnceLock::new();

#[inline(always)]
fn valid_handle_idx(handle: i64, idx: i64) -> bool {
    handle > 0 && idx >= 0
}

#[inline(never)]
fn valid_handle_idx_probe(iterations: u64, handle: i64, idx: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        if valid_handle_idx(handle, idx) {
            acc = acc.wrapping_add(1);
        }
    }
    acc
}

#[inline(always)]
fn with_array_box<R>(handle: i64, f: impl FnOnce(&ArrayBoxLike) -> R) -> Option<R> {
    if handle <= 0 {
        return None;
    }
    let values = ARRAY_CACHE.get()?;
    let idx = (handle as usize).checked_sub(1)?;
    let array = values.get(idx)?;
    Some(f(array.as_ref()))
}

#[inline(never)]
fn handle_cache_with_array_box_probe(iterations: u64, handle: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        if let Some(len) = with_array_box(handle, |arr| arr.values.read().unwrap().len()) {
            acc = acc.wrapping_add(len as u64);
        }
    }
    acc
}

#[inline(never)]
fn storage_write_lock_probe(iterations: u64, handle: i64, idx: i64) -> u64 {
    let mut acc = 0u64;
    for i in 0..iterations {
        if let Some(ok) = with_array_box(handle, |arr| {
            let mut values = arr.values.write().unwrap();
            let idx = idx as usize;
            if idx < values.len() {
                values[idx] = i as i64;
                true
            } else {
                false
            }
        }) {
            if ok {
                acc = acc.wrapping_add(1);
            }
        }
    }
    acc
}

#[inline(never)]
fn inline_i64_store_probe(iterations: u64) -> u64 {
    let mut values = vec![0i64; 4];
    let mut acc = 0u64;
    for i in 0..iterations {
        values[1] = i as i64;
        acc = acc.wrapping_add(values[1] as u64);
        std::hint::black_box(&values);
    }
    acc
}

#[inline(never)]
fn array_slot_store_i64(handle: i64, idx: i64, value: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_array_box(handle, |arr| {
        let mut values = arr.values.write().unwrap();
        let idx = idx as usize;
        if idx < values.len() {
            values[idx] = value;
            1
        } else if idx == values.len() {
            values.push(value);
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

#[inline(never)]
fn array_runtime_set_idx_i64(handle: i64, idx: i64, value: i64) -> i64 {
    array_slot_store_i64(handle, idx, value)
}

#[inline(never)]
fn array_slot_store_i64_probe(iterations: u64, handle: i64, idx: i64) -> u64 {
    let mut acc = 0u64;
    for i in 0..iterations {
        acc = acc.wrapping_add(array_slot_store_i64(handle, idx, i as i64) as u64);
    }
    acc
}

#[inline(never)]
fn array_runtime_set_idx_i64_probe(iterations: u64, handle: i64, idx: i64) -> u64 {
    let mut acc = 0u64;
    for i in 0..iterations {
        acc = acc.wrapping_add(array_runtime_set_idx_i64(handle, idx, i as i64) as u64);
    }
    acc
}

fn measure_ns_per_op(iterations: u64, mut f: impl FnMut() -> u64) -> (u64, u64) {
    let start = Instant::now();
    let acc = f();
    let elapsed = start.elapsed().as_nanos() as u64;
    let per_op = if iterations == 0 { 0 } else { (elapsed + iterations - 1) / iterations };
    (per_op, acc)
}

fn main() {
    let iterations = std::env::args()
        .nth(1)
        .and_then(|s| s.parse::<u64>().ok())
        .unwrap_or(2_000_000);
    let handle = 1i64;
    let idx = 1i64;
    let _ = ARRAY_CACHE.set(vec![Arc::new(ArrayBoxLike {
        values: Arc::new(RwLock::new(vec![0, 0, 0, 0])),
    })]);

    let (valid, valid_acc) =
        measure_ns_per_op(iterations, || valid_handle_idx_probe(iterations, handle, idx));
    let (cache, cache_acc) =
        measure_ns_per_op(iterations, || handle_cache_with_array_box_probe(iterations, handle));
    let (lock, lock_acc) =
        measure_ns_per_op(iterations, || storage_write_lock_probe(iterations, handle, idx));
    let (inline_store, inline_acc) =
        measure_ns_per_op(iterations, || inline_i64_store_probe(iterations));
    let (slot_store, slot_acc) =
        measure_ns_per_op(iterations, || array_slot_store_i64_probe(iterations, handle, idx));
    let (facade, facade_acc) =
        measure_ns_per_op(iterations, || array_runtime_set_idx_i64_probe(iterations, handle, idx));

    println!("iterations={iterations}");
    println!("valid_handle_idx_ns_per_op={valid}");
    println!("handle_cache_with_array_box_ns_per_op={cache}");
    println!("array_storage_write_lock_ns_per_op={lock}");
    println!("inline_i64_store_ns_per_op={inline_store}");
    println!("array_slot_store_i64_ns_per_op={slot_store}");
    println!("array_runtime_set_idx_i64_ns_per_op={facade}");
    println!(
        "black_box_acc={}",
        valid_acc ^ cache_acc ^ lock_acc ^ inline_acc ^ slot_acc ^ facade_acc
    );
}
'''


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


def dominant_subowner(values: dict[str, int]) -> str:
    lock = positive(values, "array_storage_write_lock_ns_per_op")
    cache = positive(values, "handle_cache_with_array_box_ns_per_op")
    inline = positive(values, "inline_i64_store_ns_per_op")
    facade = positive(values, "array_runtime_set_idx_i64_ns_per_op")
    slot = positive(values, "array_slot_store_i64_ns_per_op")

    # Attribute the full helper cost conservatively. If lock/write dominates
    # the facade, runtime storage is the next seam. If facade exceeds slot by a
    # large amount, the exported boundary is suspicious.
    if lock * 100 >= facade * 40:
        return "array_storage_write_lock"
    if (facade - slot) * 100 >= facade * 25:
        return "facade_boundary"
    if cache * 100 >= facade * 40:
        return "handle_cache_lookup"
    if inline * 100 >= facade * 40:
        return "inline_i64_store"
    return "mixed"


def recommended_next(owner: str) -> str:
    if owner == "array_storage_write_lock":
        return "single_thread_array_store_backend"
    if owner == "facade_boundary":
        return "array_slot_direct_emit_or_inline_facade"
    if owner == "handle_cache_lookup":
        return "array_handle_cache_fast_lane"
    if owner == "inline_i64_store":
        return "slot_store_i64_raw_fast_lane"
    return "measurement_refresh"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--out", type=Path, required=True)
    parser.add_argument("--iterations", type=int, default=500_000)
    args = parser.parse_args()

    if args.iterations < 1:
        raise SystemExit("--iterations must be positive")

    with tempfile.TemporaryDirectory(prefix="hakorune_array_slot_probe.") as tmp:
        tmp_dir = Path(tmp)
        src = tmp_dir / "array_slot_probe.rs"
        exe = tmp_dir / "array_slot_probe"
        src.write_text(RUST_SOURCE, encoding="utf-8")
        subprocess.run(
            ["rustc", "-O", str(src), "-o", str(exe)],
            check=True,
            stdout=subprocess.DEVNULL,
        )
        raw = subprocess.check_output([str(exe), str(args.iterations)], text=True)

    values = parse_report(raw)
    owner = dominant_subowner(values)
    lines = [
        "output_contract=array-runtime-slot-helper-cost-probe-v0",
        "input_contract=large-owner-refresh-after-residence-zero-net-v0",
        f"iterations={positive(values, 'iterations')}",
        f"valid_handle_idx_ns_per_op={positive(values, 'valid_handle_idx_ns_per_op')}",
        "handle_cache_with_array_box_ns_per_op="
        f"{positive(values, 'handle_cache_with_array_box_ns_per_op')}",
        "array_storage_write_lock_ns_per_op="
        f"{positive(values, 'array_storage_write_lock_ns_per_op')}",
        f"inline_i64_store_ns_per_op={positive(values, 'inline_i64_store_ns_per_op')}",
        f"array_slot_store_i64_ns_per_op={positive(values, 'array_slot_store_i64_ns_per_op')}",
        "array_runtime_set_idx_i64_ns_per_op="
        f"{positive(values, 'array_runtime_set_idx_i64_ns_per_op')}",
        f"dominant_subowner={owner}",
        f"recommended_next={recommended_next(owner)}",
        "optimization_open=0",
        "winner_claim=0",
        "replacement_active=0",
        "hook_installed=0",
        "global_allocator=0",
        "summary=ok",
    ]
    args.out.parent.mkdir(parents=True, exist_ok=True)
    args.out.write_text("\n".join(lines) + "\n", encoding="utf-8")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
