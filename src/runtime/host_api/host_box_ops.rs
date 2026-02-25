use crate::backend::vm::VMValue;
use crate::box_trait::{NyashBox, VoidBox};
use crate::value::NyashValue;
use std::sync::Arc;

use super::common::{encode_out, tlv_encode_one};

fn vmvalue_to_box(value: VMValue) -> Box<dyn NyashBox> {
    match value {
        VMValue::Integer(i) => Box::new(crate::box_trait::IntegerBox::new(i)),
        VMValue::Float(f) => Box::new(crate::boxes::math_box::FloatBox::new(f)),
        VMValue::Bool(b) => Box::new(crate::box_trait::BoolBox::new(b)),
        VMValue::String(s) => Box::new(crate::box_trait::StringBox::new(s)),
        VMValue::BoxRef(b) => b.share_box(),
        VMValue::Future(future_box) => Box::new(future_box),
        VMValue::WeakBox(weak_box) => match weak_box.upgrade() {
            Some(strong_ref) => strong_ref.share_box(),
            None => Box::new(VoidBox::new()),
        },
        VMValue::Void => Box::new(VoidBox::new()),
    }
}

fn vmvalue_to_setfield_value(value: VMValue) -> Option<NyashValue> {
    match value {
        VMValue::Integer(i) => Some(NyashValue::Integer(i)),
        VMValue::Float(f) => Some(NyashValue::Float(f)),
        VMValue::Bool(b) => Some(NyashValue::Bool(b)),
        VMValue::String(s) => Some(NyashValue::String(s)),
        VMValue::BoxRef(_) => None,
        VMValue::Future(_) => None,
        VMValue::WeakBox(_) => None,
        VMValue::Void => None,
    }
}

fn nyash_value_to_vmvalue(value: NyashValue) -> VMValue {
    match value {
        NyashValue::Integer(i) => VMValue::Integer(i),
        NyashValue::Float(f) => VMValue::Float(f),
        NyashValue::Bool(b) => VMValue::Bool(b),
        NyashValue::String(s) => VMValue::String(s),
        NyashValue::Void | NyashValue::Null => VMValue::String(String::new()),
        NyashValue::Box(box_mutex) => match box_mutex.lock() {
            Ok(guard) => VMValue::BoxRef(Arc::from(guard.share_box())),
            Err(_) => VMValue::String(String::new()),
        },
        _ => VMValue::String(String::new()),
    }
}

fn field_name_from_arg(arg: &VMValue) -> String {
    match arg {
        VMValue::String(s) => s.clone(),
        other => other.to_string(),
    }
}

pub(super) fn dispatch_call_name(
    recv_arc: &Arc<dyn NyashBox>,
    method: &str,
    argv: &[VMValue],
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> Option<i32> {
    let inst = recv_arc
        .as_any()
        .downcast_ref::<crate::instance_v2::InstanceBox>()?;

    let code = match method {
        "getField" if !argv.is_empty() => {
            let field = field_name_from_arg(&argv[0]);
            let out = inst
                .get_field_unified(&field)
                .map(nyash_value_to_vmvalue)
                .unwrap_or_else(|| VMValue::String(String::new()));
            let buf = tlv_encode_one(&out);
            encode_out(out_ptr, out_len, &buf)
        }
        "setField" if argv.len() >= 2 => {
            let field = field_name_from_arg(&argv[0]);
            crate::runtime::global_hooks::gc_barrier(crate::runtime::gc::BarrierKind::Write);
            if let Some(value) = vmvalue_to_setfield_value(argv[1].clone()) {
                let _ = inst.set_field_unified(field, value);
            }
            let buf = tlv_encode_one(&VMValue::Bool(true));
            encode_out(out_ptr, out_len, &buf)
        }
        _ => return None,
    };

    Some(code)
}

pub(super) fn dispatch_call_slot(
    recv_arc: &Arc<dyn NyashBox>,
    selector_id: u64,
    argv: &[VMValue],
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> Option<i32> {
    if (1..=4).contains(&selector_id) {
        let inst = recv_arc
            .as_any()
            .downcast_ref::<crate::instance_v2::InstanceBox>()?;
        let code = match selector_id {
            1 if !argv.is_empty() => {
                let field = field_name_from_arg(&argv[0]);
                let out = inst
                    .get_field_unified(&field)
                    .map(nyash_value_to_vmvalue)
                    .unwrap_or_else(|| VMValue::String(String::new()));
                let buf = tlv_encode_one(&out);
                encode_out(out_ptr, out_len, &buf)
            }
            2 if argv.len() >= 2 => {
                let field = field_name_from_arg(&argv[0]);
                if let Some(value) = vmvalue_to_setfield_value(argv[1].clone()) {
                    let _ = inst.set_field_unified(field, value);
                }
                let buf = tlv_encode_one(&VMValue::Bool(true));
                encode_out(out_ptr, out_len, &buf)
            }
            3 if !argv.is_empty() => {
                let field = field_name_from_arg(&argv[0]);
                let has = inst.get_field_unified(&field).is_some();
                let buf = tlv_encode_one(&VMValue::Bool(has));
                encode_out(out_ptr, out_len, &buf)
            }
            4 => {
                let size = inst
                    .fields_ng
                    .lock()
                    .map(|map| map.len() as i64)
                    .unwrap_or(0);
                let buf = tlv_encode_one(&VMValue::Integer(size));
                encode_out(out_ptr, out_len, &buf)
            }
            _ => -10,
        };
        return Some(code);
    }

    if !(200..=204).contains(&selector_id) {
        return None;
    }

    let map = recv_arc
        .as_any()
        .downcast_ref::<crate::boxes::map_box::MapBox>()?;

    let code = match selector_id {
        200 | 201 => {
            let out = map.size();
            let vm_value = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vm_value);
            encode_out(out_ptr, out_len, &buf)
        }
        202 if !argv.is_empty() => {
            let key_box = vmvalue_to_box(argv[0].clone());
            let out = map.has(key_box);
            let vm_value = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vm_value);
            encode_out(out_ptr, out_len, &buf)
        }
        203 if !argv.is_empty() => {
            let key_box = vmvalue_to_box(argv[0].clone());
            let out = map.get(key_box);
            let vm_value = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vm_value);
            encode_out(out_ptr, out_len, &buf)
        }
        204 if argv.len() >= 2 => {
            let key_box = vmvalue_to_box(argv[0].clone());
            let value_box = vmvalue_to_box(argv[1].clone());
            let out = map.set(key_box, value_box);
            let vm_value = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vm_value);
            encode_out(out_ptr, out_len, &buf)
        }
        _ => -10,
    };

    Some(code)
}
