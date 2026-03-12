// ---- Instance field helpers for LLVM lowering (handle-based) ----
use super::handle_helpers::with_instance_box;

// Exported as: nyash.instance.get_field_h(i64 handle, i8* name) -> i64
#[export_name = "nyash.instance.get_field_h"]
pub extern "C" fn nyash_instance_get_field_h(handle: i64, name: *const i8) -> i64 {
    if handle <= 0 || name.is_null() {
        return 0;
    }
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    let Ok(field) = name.to_str() else { return 0 };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        if let Some(shared) = inst.get_field(field) {
            let arc: std::sync::Arc<dyn nyash_rust::box_trait::NyashBox> =
                std::sync::Arc::from(shared);
            let h = nyash_rust::runtime::host_handles::to_handle_arc(arc) as u64;
            return h as i64;
        }
        0
    })
    .unwrap_or(0)
}

// Exported as: nyash.instance.set_field_h(i64 handle, i8* name, i64 val_h) -> i64
#[export_name = "nyash.instance.set_field_h"]
pub extern "C" fn nyash_instance_set_field_h(handle: i64, name: *const i8, val_h: i64) -> i64 {
    if handle <= 0 || name.is_null() {
        return 0;
    }
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    let Ok(field) = name.to_str() else { return 0 };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        if val_h > 0 {
            if let Some(val) = nyash_rust::runtime::host_handles::get(val_h as u64) {
                let shared: nyash_rust::box_trait::SharedNyashBox = std::sync::Arc::clone(&val);
                let _ = inst.set_field(field, shared);
            }
        }
        0
    })
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::box_trait::{IntegerBox, NyashBox};
    use nyash_rust::instance_v2::InstanceBox;
    use nyash_rust::runtime::host_handles;
    use std::ffi::CString;
    use std::sync::Arc;

    #[test]
    fn instance_get_field_invalid_input_returns_zero() {
        assert_eq!(nyash_instance_get_field_h(0, std::ptr::null()), 0);
        let name = CString::new("x").unwrap();
        assert_eq!(nyash_instance_get_field_h(0, name.as_ptr()), 0);
    }

    #[test]
    fn instance_set_field_invalid_input_returns_zero() {
        assert_eq!(nyash_instance_set_field_h(0, std::ptr::null(), 0), 0);
        let name = CString::new("x").unwrap();
        assert_eq!(nyash_instance_set_field_h(0, name.as_ptr(), 0), 0);
    }

    #[test]
    fn instance_field_ops_reject_invalid_utf8_name() {
        let bad = [0xFFu8, 0x00u8];
        let ptr = bad.as_ptr() as *const i8;
        assert_eq!(nyash_instance_get_field_h(1, ptr), 0);
        assert_eq!(nyash_instance_set_field_h(1, ptr, 1), 0);
    }

    #[test]
    fn instance_field_ops_return_zero_after_finalize() {
        let inst = InstanceBox::new("T".to_string(), vec!["x".to_string()], Default::default());
        let _ = inst.fini();
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;

        let value_arc: Arc<dyn NyashBox> = Arc::new(IntegerBox::new(7));
        let value_h = host_handles::to_handle_arc(value_arc) as i64;
        let name = CString::new("x").unwrap();

        assert_eq!(
            nyash_instance_set_field_h(inst_h, name.as_ptr(), value_h),
            0
        );
        assert_eq!(nyash_instance_get_field_h(inst_h, name.as_ptr()), 0);
    }
}
