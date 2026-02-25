// Environment and box creation exports.

use crate::user_box_registry::get_user_box_fields;

fn handle_to_env_string(handle: i64) -> Option<String> {
    use nyash_rust::box_trait::StringBox;
    use nyash_rust::runtime::host_handles as handles;

    if handle <= 0 {
        return None;
    }
    let obj = handles::get(handle as u64)?;
    if let Some(sb) = obj.as_any().downcast_ref::<StringBox>() {
        return Some(sb.value.clone());
    }
    Some(obj.to_string_box().value)
}

fn env_string_to_handle(value: &str) -> i64 {
    use nyash_rust::box_trait::{NyashBox, StringBox};
    use nyash_rust::runtime::host_handles as handles;

    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(StringBox::new(value.to_string()));
    handles::to_handle_arc(arc) as i64
}

// nyash.env.get(key_handle: i64) -> handle (StringBox|null)
#[export_name = "nyash.env.get"]
pub extern "C" fn nyash_env_get(key_handle: i64) -> i64 {
    let Some(key) = handle_to_env_string(key_handle) else {
        return 0;
    };
    match std::env::var(key) {
        Ok(v) => env_string_to_handle(&v),
        Err(_) => 0,
    }
}

// nyash.env.set(key_handle: i64, value_handle: i64) -> i64 (1=ok,0=fail)
#[export_name = "nyash.env.set"]
pub extern "C" fn nyash_env_set(key_handle: i64, value_handle: i64) -> i64 {
    let Some(key) = handle_to_env_string(key_handle) else {
        return 0;
    };
    let value = handle_to_env_string(value_handle).unwrap_or_else(|| value_handle.to_string());
    unsafe {
        std::env::set_var(key, value);
    }
    1
}

// Legacy aliases for builders that still reference env.get/env.set directly.
#[export_name = "env.get"]
pub extern "C" fn env_get_legacy_alias(key_handle: i64) -> i64 {
    nyash_env_get(key_handle)
}

#[export_name = "env.set"]
pub extern "C" fn env_set_legacy_alias(key_handle: i64, value_handle: i64) -> i64 {
    nyash_env_set(key_handle, value_handle)
}

// Build ArrayBox from process argv (excluding program name)
// Exported as: nyash.env.argv_get() -> i64 (ArrayBox handle)
#[export_name = "nyash.env.argv_get"]
pub extern "C" fn nyash_env_argv_get() -> i64 {
    use nyash_rust::{
        box_trait::{NyashBox, StringBox},
        boxes::array::ArrayBox,
        runtime::host_handles as handles,
    };
    let arr = ArrayBox::new();
    // Skip argv[0] (program name), collect the rest
    for (i, a) in std::env::args().enumerate() {
        if i == 0 {
            continue;
        }
        let sb: Box<dyn NyashBox> = Box::new(StringBox::new(a));
        let _ = arr.push(sb);
    }
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(arr);
    handles::to_handle_arc(arc) as i64
}

// env.box.new(type_name: *const i8) -> handle (i64)
// Minimal shim for Core-13 pure AOT: constructs Box via registry by name (no args)
#[export_name = "nyash.env.box.new"]
pub extern "C" fn nyash_env_box_new(type_name: *const i8) -> i64 {
    use nyash_rust::{
        box_trait::NyashBox,
        runtime::{box_registry::get_global_registry, host_handles as handles},
    };
    use std::ffi::CStr;
    if type_name.is_null() {
        return 0;
    }
    let cstr = unsafe { CStr::from_ptr(type_name) };
    let ty = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };
    // Core-first special cases: construct built-in boxes directly
    if ty == "MapBox" {
        use nyash_rust::boxes::map_box::MapBox;
        let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(MapBox::new());
        return handles::to_handle_arc(arc) as i64;
    }
    if ty == "ArrayBox" {
        use nyash_rust::boxes::array::ArrayBox;
        let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(ArrayBox::new());
        let h = handles::to_handle_arc(arc) as i64;
        if std::env::var("NYASH_CLI_VERBOSE").ok().as_deref() == Some("1") {
            eprintln!("nyrt: env.box.new ArrayBox -> handle={}", h);
        }
        return h;
    }
    let reg = get_global_registry();
    match reg.create_box(ty, &[]) {
        Ok(b) => {
            let arc: std::sync::Arc<dyn NyashBox> = b.into();
            handles::to_handle_arc(arc) as i64
        }
        Err(_) => 0,
    }
}

// env.box.new_i64x(type_name: *const i8, argc: i64, a1: i64, a2: i64, a3: i64, a4: i64) -> handle (i64)
// Minimal shim: construct args from handles or wrap i64 as IntegerBox
#[export_name = "nyash.env.box.new_i64x"]
pub extern "C" fn nyash_env_box_new_i64x(
    type_name: *const i8,
    argc: i64,
    a1: i64,
    a2: i64,
    a3: i64,
    a4: i64,
) -> i64 {
    use nyash_rust::{
        box_trait::{IntegerBox, NyashBox},
        runtime::{box_registry::get_global_registry, host_handles as handles},
    };
    use std::ffi::CStr;
    if type_name.is_null() {
        return 0;
    }
    let cstr = unsafe { CStr::from_ptr(type_name) };
    let ty = match cstr.to_str() {
        Ok(s) => s,
        Err(_) => return 0,
    };

    if ty == "MapBox" {
        use nyash_rust::boxes::map_box::MapBox;
        let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(MapBox::new());
        return handles::to_handle_arc(arc) as i64;
    }
    if ty == "ArrayBox" {
        use nyash_rust::boxes::array::ArrayBox;
        let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::new(ArrayBox::new());
        return handles::to_handle_arc(arc) as i64;
    }

    // Build args vec from provided i64 words
    let mut argv: Vec<Box<dyn NyashBox>> = Vec::new();
    let push_val = |dst: &mut Vec<Box<dyn NyashBox>>, v: i64| {
        if v > 0 {
            if let Some(obj) = handles::get(v as u64) {
                dst.push(obj.share_box());
                return;
            }
        }
        dst.push(Box::new(IntegerBox::new(v)));
    };
    if argc >= 1 {
        push_val(&mut argv, a1);
    }
    if argc >= 2 {
        push_val(&mut argv, a2);
    }
    if argc >= 3 {
        push_val(&mut argv, a3);
    }
    if argc >= 4 {
        push_val(&mut argv, a4);
    }

    // Phase 285LLVM-1.1: Check if this is a user-defined box in the field registry
    if let Some(fields) = get_user_box_fields(ty) {
        // Create InstanceBox with the registered fields
        use nyash_rust::instance_v2::InstanceBox;
        use std::collections::HashMap as StdHashMap;
        use std::sync::Arc;

        eprintln!("[DEBUG] Creating user box '{}' with fields: {:?}", ty, fields);
        let instance = InstanceBox::from_declaration(
            ty.to_string(),
            fields.clone(),
            StdHashMap::new(),
        );
        let boxed: Box<dyn NyashBox> = Box::new(instance);
        let arc: Arc<dyn NyashBox> = Arc::from(boxed);
        let handle = handles::to_handle_arc(arc) as i64;
        return handle;
    }

    let reg = get_global_registry();
    match reg.create_box(ty, &argv) {
        Ok(b) => {
            let arc: std::sync::Arc<dyn NyashBox> = b.into();
            handles::to_handle_arc(arc) as i64
        }
        Err(e) => {
            // Phase 285LLVM-1.1: Improved error message
            eprintln!("[nyrt_error] Failed to create box '{}': {}", ty, e);
            eprintln!("[nyrt_hint] User-defined boxes must be registered via nyrt_register_user_box_decl()");
            eprintln!("[nyrt_hint] Check MIR JSON user_box_decls or box declaration metadata");
            0
        }
    }
}
