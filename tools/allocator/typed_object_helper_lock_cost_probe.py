#!/usr/bin/env python3
"""Sidecar probe for typed-object helper lock/global-slab cost."""

from __future__ import annotations

import argparse
import subprocess
import tempfile
from pathlib import Path


RUST_SOURCE = r'''
use std::sync::{Mutex, OnceLock};
use std::time::Instant;
use std::convert::TryFrom;

#[derive(Clone, Copy)]
enum TypedSlotStorage {
    I64,
    Handle,
    U64,
}

#[derive(Clone, Copy)]
enum TypedSlotValue {
    I64(i64),
    Handle(i64),
    Unsigned(u128),
}

#[derive(Clone, Copy)]
struct TypedSlot {
    storage: TypedSlotStorage,
    value: TypedSlotValue,
}

impl TypedSlot {
    fn new(storage: TypedSlotStorage) -> Self {
        let value = match storage {
            TypedSlotStorage::I64 => TypedSlotValue::I64(0),
            TypedSlotStorage::Handle => TypedSlotValue::Handle(0),
            TypedSlotStorage::U64 => TypedSlotValue::Unsigned(0),
        };
        Self { storage, value }
    }

    #[inline(never)]
    fn set_legacy_i64(&mut self, value: i64) -> bool {
        self.value = match self.storage {
            TypedSlotStorage::Handle => TypedSlotValue::Handle(value),
            TypedSlotStorage::I64 => TypedSlotValue::I64(value),
            TypedSlotStorage::U64 => return false,
        };
        true
    }

    #[inline(never)]
    fn get_legacy_i64(&self) -> i64 {
        match self.value {
            TypedSlotValue::I64(value) | TypedSlotValue::Handle(value) => value,
            TypedSlotValue::Unsigned(_) => 0,
        }
    }

    #[inline(never)]
    fn set_exact_unsigned_u64(&mut self, value: u64) -> bool {
        match self.storage {
            TypedSlotStorage::U64 => {
                self.value = TypedSlotValue::Unsigned(value as u128);
                true
            }
            _ => false,
        }
    }

    #[inline(never)]
    fn get_exact_unsigned_u64(&self) -> u64 {
        match self.value {
            TypedSlotValue::Unsigned(value) => value as u64,
            _ => 0,
        }
    }
}

#[derive(Clone)]
struct TypedSlotObject {
    fields: Vec<TypedSlot>,
}

static TYPED_OBJECTS: OnceLock<Mutex<Vec<TypedSlotObject>>> = OnceLock::new();

#[inline(always)]
fn typed_objects() -> &'static Mutex<Vec<TypedSlotObject>> {
    TYPED_OBJECTS.get_or_init(|| Mutex::new(Vec::new()))
}

#[inline(never)]
fn handle_to_index(handle: i64) -> Option<usize> {
    if handle >= 0 {
        return None;
    }
    let idx = handle.checked_neg()?.checked_sub(1)?;
    usize::try_from(idx).ok()
}

#[inline(never)]
fn normalize_slot(slot: i64) -> Option<usize> {
    if slot < 0 || slot >= 4096 {
        return None;
    }
    usize::try_from(slot).ok()
}

#[inline(never)]
fn lock_unlock_probe(iterations: u64) -> u64 {
    let objects = typed_objects();
    let mut acc = 0u64;
    for _ in 0..iterations {
        let guard = objects.lock().unwrap();
        acc = acc.wrapping_add(guard.len() as u64);
        std::hint::black_box(&guard);
    }
    acc
}

#[inline(never)]
fn mutex_vec_read_probe(iterations: u64, handle: i64, slot: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        let Some(idx) = handle_to_index(handle) else { continue; };
        let Some(slot) = normalize_slot(slot) else { continue; };
        let objects = typed_objects().lock().unwrap();
        let Some(object) = objects.get(idx) else { continue; };
        let Some(field) = object.fields.get(slot) else { continue; };
        acc = acc.wrapping_add(field.get_legacy_i64() as u64);
        std::hint::black_box(field);
    }
    acc
}

#[inline(never)]
fn mutex_vec_write_probe(iterations: u64, handle: i64, slot: i64) -> u64 {
    let mut acc = 0u64;
    for i in 0..iterations {
        let Some(idx) = handle_to_index(handle) else { continue; };
        let Some(slot) = normalize_slot(slot) else { continue; };
        let mut objects = typed_objects().lock().unwrap();
        let Some(object) = objects.get_mut(idx) else { continue; };
        let Some(field) = object.fields.get_mut(slot) else { continue; };
        if field.set_legacy_i64(i as i64) {
            acc = acc.wrapping_add(1);
        }
        std::hint::black_box(field);
    }
    acc
}

#[inline(never)]
fn handle_to_index_probe(iterations: u64, handle: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        acc = acc.wrapping_add(handle_to_index(handle).unwrap_or(0) as u64);
    }
    acc
}

#[inline(never)]
fn slot_normalize_probe(iterations: u64, slot: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        acc = acc.wrapping_add(normalize_slot(slot).unwrap_or(0) as u64);
    }
    acc
}

#[inline(never)]
fn typed_slot_match_probe(iterations: u64, slot: usize) -> u64 {
    let objects = typed_objects().lock().unwrap();
    let field = objects[0].fields[slot];
    drop(objects);
    let mut acc = 0u64;
    for _ in 0..iterations {
        acc = acc.wrapping_add(field.get_legacy_i64() as u64);
        std::hint::black_box(field);
    }
    acc
}

#[inline(never)]
fn u64_exact_get_probe(iterations: u64, handle: i64, slot: i64) -> u64 {
    let mut acc = 0u64;
    for _ in 0..iterations {
        let Some(idx) = handle_to_index(handle) else { continue; };
        let Some(slot) = normalize_slot(slot) else { continue; };
        let objects = typed_objects().lock().unwrap();
        let Some(object) = objects.get(idx) else { continue; };
        let Some(field) = object.fields.get(slot) else { continue; };
        acc = acc.wrapping_add(field.get_exact_unsigned_u64());
        std::hint::black_box(field);
    }
    acc
}

#[inline(never)]
fn u64_exact_set_probe(iterations: u64, handle: i64, slot: i64) -> u64 {
    let mut acc = 0u64;
    for i in 0..iterations {
        let Some(idx) = handle_to_index(handle) else { continue; };
        let Some(slot) = normalize_slot(slot) else { continue; };
        let mut objects = typed_objects().lock().unwrap();
        let Some(object) = objects.get_mut(idx) else { continue; };
        let Some(field) = object.fields.get_mut(slot) else { continue; };
        if field.set_exact_unsigned_u64(i) {
            acc = acc.wrapping_add(1);
        }
        std::hint::black_box(field);
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
    let handle = -1i64;
    {
        let mut objects = typed_objects().lock().unwrap();
        objects.clear();
        objects.push(TypedSlotObject {
            fields: vec![
                TypedSlot::new(TypedSlotStorage::I64),
                TypedSlot::new(TypedSlotStorage::Handle),
                TypedSlot::new(TypedSlotStorage::U64),
            ],
        });
        objects[0].fields[0].set_legacy_i64(7);
        objects[0].fields[1].set_legacy_i64(11);
        objects[0].fields[2].set_exact_unsigned_u64(13);
    }

    let (lock, lock_acc) = measure_ns_per_op(iterations, || lock_unlock_probe(iterations));
    let (read, read_acc) = measure_ns_per_op(iterations, || mutex_vec_read_probe(iterations, handle, 0));
    let (write, write_acc) = measure_ns_per_op(iterations, || mutex_vec_write_probe(iterations, handle, 0));
    let (hidx, hidx_acc) = measure_ns_per_op(iterations, || handle_to_index_probe(iterations, handle));
    let (slot, slot_acc) = measure_ns_per_op(iterations, || slot_normalize_probe(iterations, 0));
    let (enm, enum_acc) = measure_ns_per_op(iterations, || typed_slot_match_probe(iterations, 0));
    let (uget, uget_acc) = measure_ns_per_op(iterations, || u64_exact_get_probe(iterations, handle, 2));
    let (uset, uset_acc) = measure_ns_per_op(iterations, || u64_exact_set_probe(iterations, handle, 2));

    println!("iterations={iterations}");
    println!("lock_unlock_ns_per_op={lock}");
    println!("mutex_vec_read_ns_per_op={read}");
    println!("mutex_vec_write_ns_per_op={write}");
    println!("handle_to_index_ns_per_op={hidx}");
    println!("slot_normalize_ns_per_op={slot}");
    println!("typed_slot_match_ns_per_op={enm}");
    println!("u64_exact_get_ns_per_op={uget}");
    println!("u64_exact_set_ns_per_op={uset}");
    println!("hii_get_ns_per_op={read}");
    println!("hii_set_ns_per_op={write}");
    println!("black_box_acc={}", lock_acc ^ read_acc ^ write_acc ^ hidx_acc ^ slot_acc ^ enum_acc ^ uget_acc ^ uset_acc);
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
