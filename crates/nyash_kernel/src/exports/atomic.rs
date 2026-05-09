// hako.atomic exports for pure-first EXE lowering.
//
// This is a narrow allocator-substrate seam. It owns fixed i64 atomic slots and
// the first native pointer store route only; generic atomics and allocator
// policy remain above it.

use std::ffi::c_void;
use std::sync::atomic::{AtomicI64, AtomicUsize, Ordering};

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

fn ptr_store_ordering(order: i64) -> Option<Ordering> {
    match order {
        0 => Some(Ordering::Relaxed),
        2 => Some(Ordering::Release),
        4 => Some(Ordering::SeqCst),
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
pub extern "C" fn hako_atomic_slot_fetch_add_i64(slot: i64, delta: i64) -> i64 {
    let Some(cell) = atomic_slot(slot) else {
        return HAKO_VALIDATION;
    };
    cell.fetch_add(delta, Ordering::SeqCst)
}

#[no_mangle]
pub extern "C" fn hako_atomic_slot_store_i64(slot: i64, value: i64) -> i64 {
    let Some(cell) = atomic_slot(slot) else {
        return HAKO_VALIDATION;
    };
    cell.store(value, Ordering::SeqCst);
    HAKO_OK
}

#[no_mangle]
pub extern "C" fn hako_atomic_ptr_store_ordered(
    cell_ptr: *mut c_void,
    value_ptr: *mut c_void,
    order: i64,
) -> i64 {
    if cell_ptr.is_null() {
        return HAKO_VALIDATION;
    }
    let Some(ordering) = ptr_store_ordering(order) else {
        return HAKO_VALIDATION;
    };
    unsafe {
        let cell = cell_ptr.cast::<AtomicUsize>();
        (*cell).store(value_ptr as usize, ordering);
    }
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
        assert_eq!(hako_atomic_slot_fetch_add_i64(0, 5), 0);
        assert_eq!(hako_atomic_slot_fetch_add_i64(0, 7), 5);
        assert_eq!(hako_atomic_slot_load_i64(0), 12);
        assert_eq!(hako_atomic_slot_store_i64(0, 0), HAKO_OK);
        assert_eq!(hako_atomic_slot_cas_i64(4, 0, 1), HAKO_VALIDATION);
        assert_eq!(hako_atomic_slot_load_i64(4), HAKO_VALIDATION);
        assert_eq!(hako_atomic_slot_store_i64(4, 0), HAKO_VALIDATION);
        assert_eq!(hako_atomic_slot_fetch_add_i64(4, 1), HAKO_VALIDATION);
    }

    #[test]
    fn ptr_store_ordered_writes_native_pointer_value() {
        let cell = AtomicUsize::new(0);
        let value = 0x1000usize as *mut c_void;

        assert_eq!(
            hako_atomic_ptr_store_ordered(
                (&cell as *const AtomicUsize).cast_mut().cast::<c_void>(),
                value,
                0,
            ),
            HAKO_OK
        );
        assert_eq!(cell.load(Ordering::SeqCst), value as usize);
        assert_eq!(
            hako_atomic_ptr_store_ordered(std::ptr::null_mut(), value, 0),
            HAKO_VALIDATION
        );
        assert_eq!(
            hako_atomic_ptr_store_ordered(
                (&cell as *const AtomicUsize).cast_mut().cast::<c_void>(),
                value,
                1,
            ),
            HAKO_VALIDATION
        );
    }
}
