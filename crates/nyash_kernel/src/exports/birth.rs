// Birth helpers for core boxes.

#[export_name = "nyash.string.birth_h"]
pub extern "C" fn nyash_string_birth_h_export() -> i64 {
    // Create a new StringBox via unified plugin host; return runtime handle as i64
    if let Ok(host_g) = nyash_rust::runtime::get_global_plugin_host().read() {
        if let Ok(b) = host_g.create_box("StringBox", &[]) {
            let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = std::sync::Arc::from(b);
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            nyash_rust::runtime::global_hooks::gc_alloc(0);
            return h as i64;
        }
    }
    0
}

#[export_name = "nyash.integer.birth_h"]
pub extern "C" fn nyash_integer_birth_h_export() -> i64 {
    if let Ok(host_g) = nyash_rust::runtime::get_global_plugin_host().read() {
        if let Ok(b) = host_g.create_box("IntegerBox", &[]) {
            let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = std::sync::Arc::from(b);
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            nyash_rust::runtime::global_hooks::gc_alloc(0);
            return h as i64;
        }
    }
    0
}

// ConsoleBox birth shim for AOT/JIT handle-based creation
#[export_name = "nyash.console.birth_h"]
pub extern "C" fn nyash_console_birth_h_export() -> i64 {
    if let Ok(host_g) = nyash_rust::runtime::get_global_plugin_host().read() {
        if let Ok(b) = host_g.create_box("ConsoleBox", &[]) {
            let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> = std::sync::Arc::from(b);
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            nyash_rust::runtime::global_hooks::gc_alloc(0);
            return h as i64;
        }
    }
    0
}

// ArrayBox birth shim for AOT/JIT handle-based creation
#[export_name = "nyash.array.birth_h"]
pub extern "C" fn nyash_array_birth_h_export() -> i64 {
    let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
        std::sync::Arc::new(nyash_rust::boxes::array::ArrayBox::new());
    nyash_rust::runtime::global_hooks::gc_alloc(0);
    nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
}

// MapBox birth shim for AOT/JIT handle-based creation
#[export_name = "nyash.map.birth_h"]
pub extern "C" fn nyash_map_birth_h_export() -> i64 {
    let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
        std::sync::Arc::new(nyash_rust::boxes::map_box::MapBox::new());
    nyash_rust::runtime::global_hooks::gc_alloc(0);
    nyash_rust::runtime::host_handles::to_handle_arc(arc) as i64
}
