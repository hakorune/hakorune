// Box helper exports.

// box.from_i8_string(ptr) -> handle
// Helper: build a StringBox from i8* and return a handle for AOT marshalling
#[export_name = "nyash.box.from_i8_string"]
pub extern "C" fn nyash_box_from_i8_string(ptr: *const i8) -> i64 {
    use nyash_rust::{
        box_trait::{NyashBox, StringBox},
        runtime::host_handles as handles,
    };
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
    use nyash_rust::{
        box_trait::{NyashBox, StringBox},
        runtime::host_handles as handles,
    };
    use std::{
        collections::HashMap,
        ffi::CStr,
        sync::{
            atomic::{AtomicBool, AtomicU64, Ordering},
            Mutex, OnceLock,
        },
    };

    const CONST_CACHE_MAX_ENTRIES: usize = 4096;
    static CACHE: OnceLock<Mutex<HashMap<String, i64>>> = OnceLock::new();
    static CACHE_HITS: AtomicU64 = AtomicU64::new(0);
    static CACHE_MISSES: AtomicU64 = AtomicU64::new(0);
    static CACHE_CAP_WARNED: AtomicBool = AtomicBool::new(false);
    let cache = CACHE.get_or_init(|| Mutex::new(HashMap::new()));

    if ptr.is_null() {
        return 0;
    }
    let c = unsafe { CStr::from_ptr(ptr) };
    let s = match c.to_str() {
        Ok(v) => v.to_string(),
        Err(_) => return 0,
    };

    if let Ok(guard) = cache.lock() {
        if let Some(h) = guard.get(&s) {
            CACHE_HITS.fetch_add(1, Ordering::Relaxed);
            return *h;
        }
    }
    CACHE_MISSES.fetch_add(1, Ordering::Relaxed);

    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(s.clone()));
    nyash_rust::runtime::global_hooks::gc_alloc(s.len() as u64);
    let h = handles::to_handle_arc(arc) as i64;

    if let Ok(mut guard) = cache.lock() {
        if guard.len() < CONST_CACHE_MAX_ENTRIES {
            guard.insert(s, h);
        } else if !CACHE_CAP_WARNED.swap(true, Ordering::Relaxed) {
            let hits = CACHE_HITS.load(Ordering::Relaxed);
            let misses = CACHE_MISSES.load(Ordering::Relaxed);
            eprintln!(
                "[perf/const_cache] capped max_entries={} hits={} misses={} mode=passthrough",
                CONST_CACHE_MAX_ENTRIES, hits, misses
            );
        }
    }
    h
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
