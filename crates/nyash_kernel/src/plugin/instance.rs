// ---- Instance field helpers for LLVM lowering (handle-based) ----
use super::handle_cache::with_instance_box;

fn instance_field_name(name: *const i8) -> Option<String> {
    if name.is_null() {
        return None;
    }
    let name = unsafe { std::ffi::CStr::from_ptr(name) };
    Some(name.to_str().ok()?.to_string())
}

fn instance_integer_field_value(
    inst: &nyash_rust::instance_v2::InstanceBox,
    field: &str,
) -> Option<i64> {
    if let Some(value) = inst.get_field_ng(field) {
        match value {
            nyash_rust::value::NyashValue::Integer(value) => return Some(value),
            nyash_rust::value::NyashValue::Bool(value) => return Some(if value { 1 } else { 0 }),
            nyash_rust::value::NyashValue::Box(shared) => {
                if let Ok(guard) = shared.lock() {
                    if let Some(int_box) = guard
                        .as_any()
                        .downcast_ref::<nyash_rust::box_trait::IntegerBox>()
                    {
                        return Some(int_box.value);
                    }
                }
            }
            _ => {}
        }
    }

    let shared = inst.get_field(field)?;
    let int_box = shared
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::IntegerBox>()?;
    Some(int_box.value)
}

fn instance_bool_field_value(
    inst: &nyash_rust::instance_v2::InstanceBox,
    field: &str,
) -> Option<bool> {
    if let Some(value) = inst.get_field_ng(field) {
        match value {
            nyash_rust::value::NyashValue::Bool(value) => return Some(value),
            nyash_rust::value::NyashValue::Box(shared) => {
                if let Ok(guard) = shared.lock() {
                    if let Some(bool_box) = guard
                        .as_any()
                        .downcast_ref::<nyash_rust::box_trait::BoolBox>()
                    {
                        return Some(bool_box.value);
                    }
                }
            }
            _ => {}
        }
    }

    let shared = inst.get_field(field)?;
    let bool_box = shared
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::BoolBox>()?;
    Some(bool_box.value)
}

fn instance_float_field_value(
    inst: &nyash_rust::instance_v2::InstanceBox,
    field: &str,
) -> Option<f64> {
    if let Some(value) = inst.get_field_ng(field) {
        match value {
            nyash_rust::value::NyashValue::Float(value) => return Some(value),
            nyash_rust::value::NyashValue::Box(shared) => {
                if let Ok(guard) = shared.lock() {
                    if let Some(float_box) =
                        guard.as_any().downcast_ref::<nyash_rust::boxes::FloatBox>()
                    {
                        return Some(float_box.value);
                    }
                }
            }
            _ => {}
        }
    }

    let shared = inst.get_field(field)?;
    let float_box = shared
        .as_any()
        .downcast_ref::<nyash_rust::boxes::FloatBox>()?;
    Some(float_box.value)
}

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

// Exported as: nyash.instance.get_i64_field_h(i64 handle, i8* name) -> i64
#[export_name = "nyash.instance.get_i64_field_h"]
pub extern "C" fn nyash_instance_get_i64_field_h(handle: i64, name: *const i8) -> i64 {
    if handle <= 0 {
        return 0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        instance_integer_field_value(inst, field.as_str()).unwrap_or(0)
    })
    .unwrap_or(0)
}

// Exported as: nyash.instance.get_bool_field_h(i64 handle, i8* name) -> i64 (0/1)
#[export_name = "nyash.instance.get_bool_field_h"]
pub extern "C" fn nyash_instance_get_bool_field_h(handle: i64, name: *const i8) -> i64 {
    if handle <= 0 {
        return 0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        if instance_bool_field_value(inst, field.as_str()).unwrap_or(false) {
            1
        } else {
            0
        }
    })
    .unwrap_or(0)
}

// Exported as: nyash.instance.get_float_field_h(i64 handle, i8* name) -> f64
#[export_name = "nyash.instance.get_float_field_h"]
pub extern "C" fn nyash_instance_get_float_field_h(handle: i64, name: *const i8) -> f64 {
    if handle <= 0 {
        return 0.0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0.0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0.0;
        }
        instance_float_field_value(inst, field.as_str()).unwrap_or(0.0)
    })
    .unwrap_or(0.0)
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

// Exported as: nyash.instance.set_i64_field_h(i64 handle, i8* name, i64 value) -> i64
#[export_name = "nyash.instance.set_i64_field_h"]
pub extern "C" fn nyash_instance_set_i64_field_h(handle: i64, name: *const i8, value: i64) -> i64 {
    if handle <= 0 {
        return 0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        let _ = inst.set_field_ng(field.clone(), nyash_rust::value::NyashValue::Integer(value));
        0
    })
    .unwrap_or(0)
}

// Exported as: nyash.instance.set_bool_field_h(i64 handle, i8* name, i64 value) -> i64
#[export_name = "nyash.instance.set_bool_field_h"]
pub extern "C" fn nyash_instance_set_bool_field_h(handle: i64, name: *const i8, value: i64) -> i64 {
    if handle <= 0 {
        return 0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        let _ = inst.set_field_ng(
            field.clone(),
            nyash_rust::value::NyashValue::Bool(value != 0),
        );
        0
    })
    .unwrap_or(0)
}

// Exported as: nyash.instance.set_float_field_h(i64 handle, i8* name, f64 value) -> i64
#[export_name = "nyash.instance.set_float_field_h"]
pub extern "C" fn nyash_instance_set_float_field_h(
    handle: i64,
    name: *const i8,
    value: f64,
) -> i64 {
    if handle <= 0 {
        return 0;
    }
    let Some(field) = instance_field_name(name) else {
        return 0;
    };
    with_instance_box(handle, |inst| {
        if inst.is_finalized() {
            return 0;
        }
        let _ = inst.set_field_ng(field.clone(), nyash_rust::value::NyashValue::Float(value));
        0
    })
    .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nyash_rust::instance_v2::InstanceBox;
    use nyash_rust::runtime::host_handles;
    use nyash_rust::{
        box_trait::{BoolBox, IntegerBox, NyashBox},
        boxes::FloatBox,
    };
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

    #[test]
    fn instance_i64_field_get_reads_internal_integer_storage() {
        let inst = InstanceBox::new("T".to_string(), vec!["x".to_string()], Default::default());
        let _ = inst.set_field_ng("x".to_string(), nyash_rust::value::NyashValue::Integer(42));
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("x").unwrap();

        assert_eq!(nyash_instance_get_i64_field_h(inst_h, name.as_ptr()), 42);
    }

    #[test]
    fn instance_i64_field_set_writes_internal_integer_storage() {
        let inst = InstanceBox::new("T".to_string(), vec!["x".to_string()], Default::default());
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("x").unwrap();

        assert_eq!(nyash_instance_set_i64_field_h(inst_h, name.as_ptr(), 77), 0);
        let stored = with_instance_box(inst_h, |inst| inst.get_field_ng("x")).flatten();
        assert!(matches!(
            stored,
            Some(nyash_rust::value::NyashValue::Integer(77))
        ));
    }

    #[test]
    fn instance_bool_field_get_reads_internal_bool_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["flag".to_string()],
            Default::default(),
        );
        let _ = inst.set_field_ng(
            "flag".to_string(),
            nyash_rust::value::NyashValue::Bool(true),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("flag").unwrap();

        assert_eq!(nyash_instance_get_bool_field_h(inst_h, name.as_ptr()), 1);
    }

    #[test]
    fn instance_bool_field_get_reads_internal_bool_box_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["flag".to_string()],
            Default::default(),
        );
        let boxed_flag = Arc::new(std::sync::Mutex::new(BoolBox::new(true)));
        let _ = inst.set_field_ng(
            "flag".to_string(),
            nyash_rust::value::NyashValue::Box(boxed_flag),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("flag").unwrap();

        assert_eq!(nyash_instance_get_bool_field_h(inst_h, name.as_ptr()), 1);
    }

    #[test]
    fn instance_bool_field_set_writes_internal_bool_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["flag".to_string()],
            Default::default(),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("flag").unwrap();

        assert_eq!(nyash_instance_set_bool_field_h(inst_h, name.as_ptr(), 7), 0);
        let stored = with_instance_box(inst_h, |inst| inst.get_field_ng("flag")).flatten();
        assert!(matches!(
            stored,
            Some(nyash_rust::value::NyashValue::Bool(true))
        ));
    }

    #[test]
    fn instance_float_field_get_reads_internal_float_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["value".to_string()],
            Default::default(),
        );
        let _ = inst.set_field_ng(
            "value".to_string(),
            nyash_rust::value::NyashValue::Float(3.5),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("value").unwrap();

        assert!(
            (nyash_instance_get_float_field_h(inst_h, name.as_ptr()) - 3.5).abs() < f64::EPSILON
        );
    }

    #[test]
    fn instance_float_field_get_reads_internal_float_box_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["value".to_string()],
            Default::default(),
        );
        let boxed_value = Arc::new(std::sync::Mutex::new(FloatBox::new(1.25)));
        let _ = inst.set_field_ng(
            "value".to_string(),
            nyash_rust::value::NyashValue::Box(boxed_value),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("value").unwrap();

        assert!(
            (nyash_instance_get_float_field_h(inst_h, name.as_ptr()) - 1.25).abs() < f64::EPSILON
        );
    }

    #[test]
    fn instance_float_field_set_writes_internal_float_storage() {
        let inst = InstanceBox::new(
            "T".to_string(),
            vec!["value".to_string()],
            Default::default(),
        );
        let inst_arc: Arc<dyn NyashBox> = Arc::new(inst);
        let inst_h = host_handles::to_handle_arc(inst_arc) as i64;
        let name = CString::new("value").unwrap();

        assert_eq!(
            nyash_instance_set_float_field_h(inst_h, name.as_ptr(), 6.25),
            0
        );
        let stored = with_instance_box(inst_h, |inst| inst.get_field_ng("value")).flatten();
        match stored {
            Some(nyash_rust::value::NyashValue::Float(value)) => {
                assert!((value - 6.25).abs() < f64::EPSILON);
            }
            other => panic!("expected Float storage, got {:?}", other),
        }
    }
}
