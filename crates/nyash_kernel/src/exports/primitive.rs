// Primitive box helpers.

// integer.get_h(handle) -> i64
// Extract IntegerBox value from a handle. Returns 0 if handle is invalid or not an IntegerBox.
#[export_name = "nyash.integer.get_h"]
pub extern "C" fn nyash_integer_get_h_export(h: i64) -> i64 {
    use nyash_rust::{box_trait::IntegerBox, runtime::host_handles as handles};
    if h <= 0 {
        return 0;
    }
    if let Some(obj) = handles::get(h as u64) {
        if let Some(ib) = obj.as_any().downcast_ref::<IntegerBox>() {
            return ib.value;
        }
    }
    0
}

// bool.get_h(handle) -> i64 (0/1)
#[export_name = "nyash.bool.get_h"]
pub extern "C" fn nyash_bool_get_h_export(h: i64) -> i64 {
    use nyash_rust::{box_trait::BoolBox, runtime::host_handles as handles};
    if h <= 0 {
        return 0;
    }
    if let Some(obj) = handles::get(h as u64) {
        if let Some(bb) = obj.as_any().downcast_ref::<BoolBox>() {
            return if bb.value { 1 } else { 0 };
        }
    }
    0
}

// float.get_bits_h(handle) -> i64 (f64 bits)
#[export_name = "nyash.float.get_bits_h"]
pub extern "C" fn nyash_float_get_bits_h_export(h: i64) -> i64 {
    use nyash_rust::{boxes::FloatBox, runtime::host_handles as handles};
    if h <= 0 {
        return 0;
    }
    if let Some(obj) = handles::get(h as u64) {
        if let Some(fb) = obj.as_any().downcast_ref::<FloatBox>() {
            return fb.value.to_bits() as i64;
        }
    }
    0
}

// Phase 275 C2: Float unbox helper for LLVM harness (Int+Float addition)
// Returns f64 value from Float handle
#[export_name = "nyash.float.unbox_to_f64"]
pub extern "C" fn nyash_float_unbox_to_f64(float_handle: i64) -> f64 {
    use nyash_rust::runtime::host_handles as handles;

    // Get the FloatBox from handle
    if float_handle <= 0 {
        return 0.0; // Invalid handle
    }

    if let Some(obj) = handles::get(float_handle as u64) {
        if let Some(fb) = obj.as_any().downcast_ref::<nyash_rust::FloatBox>() {
            return fb.value;
        }
    }

    0.0 // Not a FloatBox or handle invalid
}
