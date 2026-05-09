// hako.atomic slot exports for pure-first EXE lowering.
//
// This is a narrow allocator-substrate seam. It owns fixed i64 atomic slots
// only; generic atomics, pointer atomics, and allocator policy remain above it.

use std::sync::atomic::{AtomicI64, Ordering};

const HAKO_OK: i64 = 0;
const HAKO_VALIDATION: i64 = 6;

static HAKO_ATOMIC_SLOT0: AtomicI64 = AtomicI64::new(0);
static HAKO_ATOMIC_SLOT1: AtomicI64 = AtomicI64::new(0);
static HAKO_ATOMIC_SLOT2: AtomicI64 = AtomicI64::new(0);
static HAKO_ATOMIC_SLOT3: AtomicI64 = AtomicI64::new(0);

fn atomic_slot(slot: i64) -> Option<&'static AtomicI64> {
    match slot {
        0 => Some(&HAKO_ATOMIC_SLOT0),
        1 => Some(&HAKO_ATOMIC_SLOT1),
        2 => Some(&HAKO_ATOMIC_SLOT2),
        3 => Some(&HAKO_ATOMIC_SLOT3),
        _ => None,
    }
}

#[no_mangle]
pub extern "C" fn hako_atomic_slot_cas_i64(slot: i64, expected: i64, desired: i64) -> i64 {
    let Some(cell) = atomic_slot(slot) else {
        return HAKO_VALIDATION;
    };
    match cell.compare_exchange(expected, desired, Ordering::SeqCst, Ordering::SeqCst) {
        Ok(previous) | Err(previous) => previous,
    }
}

#[no_mangle]
pub extern "C" fn hako_atomic_slot_load_i64(slot: i64) -> i64 {
    let Some(cell) = atomic_slot(slot) else {
        return HAKO_VALIDATION;
    };
    cell.load(Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn hako_atomic_slot_store_i64(slot: i64, value: i64) -> i64 {
    let Some(cell) = atomic_slot(slot) else {
        return HAKO_VALIDATION;
    };
    cell.store(value, Ordering::SeqCst);
    HAKO_OK
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slot_cas_returns_previous_value_for_success_and_failure() {
        assert_eq!(hako_atomic_slot_cas_i64(0, 0, 0), 0);
        assert_eq!(hako_atomic_slot_cas_i64(0, 0, 4096), 0);
        assert_eq!(hako_atomic_slot_load_i64(0), 4096);
        assert_eq!(hako_atomic_slot_cas_i64(0, 0, 1), 4096);
        assert_eq!(hako_atomic_slot_cas_i64(0, 4096, 0), 4096);
        assert_eq!(hako_atomic_slot_load_i64(0), 0);
        assert_eq!(hako_atomic_slot_store_i64(0, 7), HAKO_OK);
        assert_eq!(hako_atomic_slot_load_i64(0), 7);
        assert_eq!(hako_atomic_slot_store_i64(0, 0), HAKO_OK);
        assert_eq!(hako_atomic_slot_cas_i64(4, 0, 1), HAKO_VALIDATION);
        assert_eq!(hako_atomic_slot_load_i64(4), HAKO_VALIDATION);
        assert_eq!(hako_atomic_slot_store_i64(4, 0), HAKO_VALIDATION);
    }
}
