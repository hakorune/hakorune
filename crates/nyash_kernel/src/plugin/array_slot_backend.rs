//! Diagnostic backend seam for index-backed ArrayBox slot helpers.
//!
//! The default backend preserves the existing ArrayBox `RwLock<ArrayStorage>`
//! behavior. The single-thread backend is an exact-EXE perf lane for numeric
//! i64 slot helpers only; unsupported storage shapes fail fast instead of
//! silently falling back.

use std::cell::RefCell;
use std::collections::HashMap;
use std::sync::OnceLock;

use super::array_guard::valid_handle_idx;
use super::array_handle_cache::{array_get_index_encoded_i64, with_array_box};

const ARRAY_SLOT_STORE_ENV: &str = "HAKO_ARRAY_SLOT_STORE";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ArraySlotBackend {
    SafeRwLock,
    SingleThreadExact,
}

static BACKEND: OnceLock<ArraySlotBackend> = OnceLock::new();

thread_local! {
    static SINGLE_THREAD_I64_SLOTS: RefCell<HashMap<i64, Vec<i64>>> = RefCell::new(HashMap::new());
}

fn selected_backend() -> ArraySlotBackend {
    *BACKEND.get_or_init(
        || match std::env::var(ARRAY_SLOT_STORE_ENV).ok().as_deref() {
            None | Some("") | Some("safe_rwlock") => ArraySlotBackend::SafeRwLock,
            Some("single_thread_exact") => ArraySlotBackend::SingleThreadExact,
            Some(value) => panic!(
            "[freeze:contract][array-slot-store/backend] unsupported {ARRAY_SLOT_STORE_ENV}={value}"
        ),
        },
    )
}

fn safe_store_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    with_array_box(handle, |arr| {
        if arr.slot_store_i64_raw(idx, value_i64) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

fn safe_load_encoded_i64(handle: i64, idx: i64) -> i64 {
    array_get_index_encoded_i64(handle, idx).unwrap_or(0)
}

fn initialize_exact_i64_slots(handle: i64) -> Vec<i64> {
    with_array_box(handle, |arr| {
        let len = arr.len();
        let mut values = Vec::with_capacity(len);
        for idx in 0..len {
            let Some(value) = arr.slot_load_i64_raw(idx as i64) else {
                panic!(
                    "[freeze:contract][array-slot-store/single-thread-exact] \
                     non-i64 ArrayBox slot handle={handle} idx={idx}"
                );
            };
            values.push(value);
        }
        values
    })
    .unwrap_or_else(|| {
        panic!(
            "[freeze:contract][array-slot-store/single-thread-exact] invalid ArrayBox handle={handle}"
        )
    })
}

fn single_thread_slots_mut<R>(handle: i64, f: impl FnOnce(&mut Vec<i64>) -> R) -> R {
    SINGLE_THREAD_I64_SLOTS.with(|slots| {
        let mut slots = slots.borrow_mut();
        let values = slots
            .entry(handle)
            .or_insert_with(|| initialize_exact_i64_slots(handle));
        f(values)
    })
}

fn single_thread_store_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    single_thread_slots_mut(handle, |values| {
        let idx = idx as usize;
        if idx < values.len() {
            values[idx] = value_i64;
            1
        } else if idx == values.len() {
            values.push(value_i64);
            1
        } else {
            0
        }
    })
}

fn single_thread_load_encoded_i64(handle: i64, idx: i64) -> i64 {
    if !valid_handle_idx(handle, idx) {
        return 0;
    }
    single_thread_slots_mut(handle, |values| {
        values.get(idx as usize).copied().unwrap_or(0)
    })
}

#[inline(always)]
pub(super) fn store_i64(handle: i64, idx: i64, value_i64: i64) -> i64 {
    match selected_backend() {
        ArraySlotBackend::SafeRwLock => safe_store_i64(handle, idx, value_i64),
        ArraySlotBackend::SingleThreadExact => single_thread_store_i64(handle, idx, value_i64),
    }
}

#[inline(always)]
pub(super) fn load_encoded_i64(handle: i64, idx: i64) -> i64 {
    match selected_backend() {
        ArraySlotBackend::SafeRwLock => safe_load_encoded_i64(handle, idx),
        ArraySlotBackend::SingleThreadExact => single_thread_load_encoded_i64(handle, idx),
    }
}
