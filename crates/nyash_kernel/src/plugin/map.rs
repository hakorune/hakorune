// --- AOT ObjectModule dotted-name exports (Map) ---
// Provide dotted symbol names expected by ObjectBuilder lowering for MapBox operations.
use super::handle_helpers::with_map_box;
use super::value_codec::{
    any_arg_to_box, bool_box_to_i64, box_to_handle, int_arg_to_box, integer_box_to_i64,
};

// size: (handle) -> i64
#[export_name = "nyash.map.size_h"]
pub extern "C" fn nyash_map_size_h(handle: i64) -> i64 {
    if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[MAP] size_h(handle={})", handle);
    }
    with_map_box(handle, |map| {
        if let Some(size) = integer_box_to_i64(map.size().as_ref()) {
            if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
                eprintln!("[MAP] size_h => {}", size);
            }
            size
        } else {
            0
        }
    })
    .unwrap_or(0)
}

// get_h: (map_handle, key_i64) -> value_handle
#[export_name = "nyash.map.get_h"]
pub extern "C" fn nyash_map_get_h(handle: i64, key: i64) -> i64 {
    if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[MAP] get_h(handle={}, key={})", handle, key);
    }
    with_map_box(handle, |map| {
        let v = map.get(int_arg_to_box(key));
        let h = box_to_handle(v) as u64;
        if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
            eprintln!("[MAP] get_h => handle {}", h);
        }
        h as i64
    })
    .unwrap_or(0)
}

// get_hh: (map_handle, key_handle) -> value_handle
#[export_name = "nyash.map.get_hh"]
pub extern "C" fn nyash_map_get_hh(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let key_box = any_arg_to_box(key_any);
        let v = map.get(key_box);
        box_to_handle(v)
    })
    .unwrap_or(0)
}

// set_h: (map_handle, key_i64, val) -> i64 (ignored/0)
#[export_name = "nyash.map.set_h"]
pub extern "C" fn nyash_map_set_h(handle: i64, key: i64, val: i64) -> i64 {
    if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
        eprintln!("[MAP] set_h(handle={}, key={}, val={})", handle, key, val);
    }
    let _ = with_map_box(handle, |map| {
        let _ = map.set(int_arg_to_box(key), any_arg_to_box(val));
        if std::env::var("NYASH_LLVM_MAP_DEBUG").ok().as_deref() == Some("1") {
            let sz = integer_box_to_i64(map.size().as_ref()).unwrap_or(-1);
            eprintln!("[MAP] set_h done; size now {}", sz);
        }
    });
    0
}

// set_hh: (map_handle, key_any: handle or i64, val_any: handle or i64) -> i64
#[export_name = "nyash.map.set_hh"]
pub extern "C" fn nyash_map_set_hh(handle: i64, key_any: i64, val_any: i64) -> i64 {
    let _ = with_map_box(handle, |map| {
        let kbox = any_arg_to_box(key_any);
        let vbox = any_arg_to_box(val_any);
        let _ = map.set(kbox, vbox);
    });
    0
}

// has_hh: (map_handle, key_any: handle or i64) -> i64 (0/1)
#[export_name = "nyash.map.has_hh"]
pub extern "C" fn nyash_map_has_hh(handle: i64, key_any: i64) -> i64 {
    with_map_box(handle, |map| {
        let kbox = any_arg_to_box(key_any);
        let v = map.has(kbox);
        bool_box_to_i64(v.as_ref()).unwrap_or(0)
    })
    .unwrap_or(0)
}

// has_h: (map_handle, key_i64) -> i64 (0/1)
#[export_name = "nyash.map.has_h"]
pub extern "C" fn nyash_map_has_h(handle: i64, key: i64) -> i64 {
    with_map_box(handle, |map| {
        let v = map.has(int_arg_to_box(key));
        bool_box_to_i64(v.as_ref()).unwrap_or(0)
    })
    .unwrap_or(0)
}
