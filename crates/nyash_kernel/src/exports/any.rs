// Any box helpers.

use super::string::{string_is_empty_from_handle, string_len_from_handle};

#[export_name = "nyash.any.handle_live_h"]
pub extern "C" fn nyash_any_handle_live_h_export(handle: i64) -> i64 {
    use nyash_rust::runtime::host_handles as handles;
    if handle <= 0 {
        return 0;
    }
    if handles::get(handle as u64).is_some() {
        1
    } else {
        0
    }
}

// Any.length_h(handle) -> i64 (Array/String/Map)
#[export_name = "nyash.any.length_h"]
pub extern "C" fn nyash_any_length_h_export(handle: i64) -> i64 {
    use nyash_rust::runtime::host_handles as handles;
    if std::env::var("NYASH_JIT_TRACE_LEN").ok().as_deref() == Some("1") {
        let present = if handle > 0 {
            handles::get(handle as u64).is_some()
        } else {
            false
        };
        eprintln!(
            "[AOT-LEN_H] any.length_h handle={} present={}",
            handle, present
        );
    }
    if handle <= 0 {
        return 0;
    }
    if let Some(len) = string_len_from_handle(handle) {
        return len;
    }
    if let Some(obj) = handles::get(handle as u64) {
        if let Some(arr) = obj
            .as_any()
            .downcast_ref::<nyash_rust::boxes::array::ArrayBox>()
        {
            return arr.len() as i64;
        }
        if let Some(map) = obj
            .as_any()
            .downcast_ref::<nyash_rust::boxes::map_box::MapBox>()
        {
            return map.len() as i64;
        }
    }
    0
}

// Any.toString_h(handle) -> handle (StringBox)
//
// Universal display conversion for LLVM/JIT paths where method dispatch may not
// have plugin slots for builtin boxes. This should match the VM's expectation
// that `toString()` is always available (universal slot #0).
#[export_name = "nyash.any.toString_h"]
pub extern "C" fn nyash_any_to_string_h_export(handle: i64) -> i64 {
    use nyash_rust::{
        box_trait::{NyashBox, StringBox},
        runtime::host_handles as handles,
    };
    // Treat <=0 as the null/void handle in AOT paths.
    if handle <= 0 {
        let s = "null".to_string();
        let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(s.clone()));
        nyash_rust::runtime::global_hooks::gc_alloc(s.len() as u64);
        return handles::to_handle_arc(arc) as i64;
    }
    let obj = match handles::get(handle as u64) {
        Some(o) => o,
        None => return 0,
    };
    let s = obj.to_string_box().value;
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(s.clone()));
    nyash_rust::runtime::global_hooks::gc_alloc(s.len() as u64);
    handles::to_handle_arc(arc) as i64
}

// Any.is_empty_h(handle) -> i64 (0/1)
#[export_name = "nyash.any.is_empty_h"]
pub extern "C" fn nyash_any_is_empty_h_export(handle: i64) -> i64 {
    use nyash_rust::runtime::host_handles as handles;
    if handle <= 0 {
        return 1;
    }
    if let Some(is_empty) = string_is_empty_from_handle(handle) {
        return if is_empty { 1 } else { 0 };
    }
    if let Some(obj) = handles::get(handle as u64) {
        if let Some(arr) = obj
            .as_any()
            .downcast_ref::<nyash_rust::boxes::array::ArrayBox>()
        {
            return if arr.len() == 0 { 1 } else { 0 };
        }
        if let Some(map) = obj
            .as_any()
            .downcast_ref::<nyash_rust::boxes::map_box::MapBox>()
        {
            return if map.len() == 0 { 1 } else { 0 };
        }
    }
    1
}

// ---- Type introspection (Phase 274 P2) ----

/// Runtime type check for TypeOp implementation
/// Returns 1 if handle's runtime type matches type_name, 0 otherwise
#[export_name = "nyash.any.is_type_h"]
pub extern "C" fn nyash_any_is_type_h(handle: i64, type_name_ptr: *const i8) -> i64 {
    use nyash_rust::runtime::host_handles as handles;

    // Validate handle
    if handle <= 0 {
        return 0;
    }

    // Parse type_name from C string
    let type_name = unsafe {
        if type_name_ptr.is_null() {
            return 0;
        }
        match std::ffi::CStr::from_ptr(type_name_ptr).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        }
    };

    // Get object from handle registry
    let obj = match handles::get(handle as u64) {
        Some(o) => o,
        None => return 0,
    };

    // Compare type_name() with requested type
    let actual_type = obj.type_name();
    if actual_type == type_name {
        return 1;
    }

    // For InstanceBox, also check class_name field
    if let Some(inst) = obj
        .as_any()
        .downcast_ref::<nyash_rust::instance_v2::InstanceBox>()
    {
        if inst.class_name == type_name {
            return 1;
        }
    }

    // No match
    0
}
