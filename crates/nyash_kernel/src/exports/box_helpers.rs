// Box helper exports.

use nyash_rust::{
    box_trait::{BoolBox, NyashBox, StringBox},
    boxes::FloatBox,
    runtime::host_handles as handles,
};
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicBool, AtomicU64, Ordering},
        Mutex, OnceLock,
    },
};

const CONST_CACHE_MAX_ENTRIES: usize = 4096;
static CONST_STRING_CACHE: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
static CONST_CACHE_HITS: AtomicU64 = AtomicU64::new(0);
static CONST_CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
static CONST_CACHE_CAP_WARNED: AtomicBool = AtomicBool::new(false);

#[inline(always)]
pub(crate) fn string_literal_handle_from_text(text: &str) -> i64 {
    let cache = CONST_STRING_CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if let Ok(guard) = cache.lock() {
        if let Some(h) = guard.get(text) {
            CONST_CACHE_HITS.fetch_add(1, Ordering::Relaxed);
            return *h;
        }
    }
    CONST_CACHE_MISSES.fetch_add(1, Ordering::Relaxed);

    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(text.to_owned()));
    nyash_rust::runtime::global_hooks::gc_alloc(text.len() as u64);
    let h = handles::to_handle_arc(arc) as i64;

    if let Ok(mut guard) = cache.lock() {
        if guard.len() < CONST_CACHE_MAX_ENTRIES {
            guard.insert(text.to_owned(), h);
        } else if !CONST_CACHE_CAP_WARNED.swap(true, Ordering::Relaxed) {
            let hits = CONST_CACHE_HITS.load(Ordering::Relaxed);
            let misses = CONST_CACHE_MISSES.load(Ordering::Relaxed);
            eprintln!(
                "[perf/const_cache] capped max_entries={} hits={} misses={} mode=passthrough",
                CONST_CACHE_MAX_ENTRIES, hits, misses
            );
        }
    }
    h
}

// box.from_i8_string(ptr) -> handle
// Helper: build a StringBox from i8* and return a handle for AOT marshalling
#[export_name = "nyash.box.from_i8_string"]
pub extern "C" fn nyash_box_from_i8_string(ptr: *const i8) -> i64 {
    use std::ffi::CStr;
    if ptr.is_null() {
        return 0;
    }
    let c = unsafe { CStr::from_ptr(ptr) };
    let s = match c.to_str() {
        Ok(v) => v.to_string(),
        Err(_) => return 0,
    };
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(s.clone()));
    nyash_rust::runtime::global_hooks::gc_alloc(s.len() as u64);
    let h = handles::to_handle_arc(arc) as i64;
    h
}

// box.from_i8_string_const(ptr) -> handle
// FAST-path helper: intern const strings and reuse the same handle.
// Used only from opt-in LLVM fast lowering.
#[export_name = "nyash.box.from_i8_string_const"]
pub extern "C" fn nyash_box_from_i8_string_const(ptr: *const i8) -> i64 {
    use std::ffi::CStr;
    if ptr.is_null() {
        return 0;
    }
    let c = unsafe { CStr::from_ptr(ptr) };
    let s = match c.to_str() {
        Ok(v) => v,
        Err(_) => return 0,
    };
    string_literal_handle_from_text(s)
}

// box.from_i64(val) -> handle
// Helper: build an IntegerBox and return a handle
#[export_name = "nyash.box.from_i64"]
pub extern "C" fn nyash_box_from_i64(val: i64) -> i64 {
    use nyash_rust::{
        box_trait::{IntegerBox, NyashBox},
        runtime::host_handles as handles,
    };
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(IntegerBox::new(val));
    nyash_rust::runtime::global_hooks::gc_alloc(8);
    let h = handles::to_handle_arc(arc) as i64;
    h
}

// box.from_bool(val) -> handle
// Helper: build a BoolBox and return a handle
#[export_name = "nyash.box.from_bool"]
pub extern "C" fn nyash_box_from_bool(val: i64) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(BoolBox::new(val != 0));
    nyash_rust::runtime::global_hooks::gc_alloc(1);
    handles::to_handle_arc(arc) as i64
}

// box.from_f64(val) -> handle
// Helper: build a FloatBox and return a handle
#[export_name = "nyash.box.from_f64"]
pub extern "C" fn nyash_box_from_f64(val: f64) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(FloatBox::new(val));
    nyash_rust::runtime::global_hooks::gc_alloc(8);
    handles::to_handle_arc(arc) as i64
}
