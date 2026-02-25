use crate::backend::vm::VMValue;
use crate::box_trait::{NyashBox, VoidBox};
use std::sync::Arc;

use super::common::{encode_out, tlv_encode_one};

fn vmvalue_to_box(value: VMValue) -> Box<dyn NyashBox> {
    match value {
        VMValue::Integer(i) => Box::new(crate::box_trait::IntegerBox::new(i)),
        VMValue::Float(f) => Box::new(crate::boxes::math_box::FloatBox::new(f)),
        VMValue::Bool(b) => Box::new(crate::box_trait::BoolBox::new(b)),
        VMValue::String(s) => Box::new(crate::box_trait::StringBox::new(s)),
        VMValue::BoxRef(b) => b.share_box(),
        _ => Box::new(VoidBox::new()),
    }
}

fn parse_index(value: VMValue) -> i64 {
    match value {
        VMValue::Integer(i) => i,
        other => other.to_string().parse::<i64>().unwrap_or(0),
    }
}

pub(super) fn dispatch_call_name(
    recv_arc: &Arc<dyn NyashBox>,
    method: &str,
    argv: &[VMValue],
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> Option<i32> {
    let arr = recv_arc
        .as_any()
        .downcast_ref::<crate::boxes::array::ArrayBox>()?;

    match method {
        "get" if !argv.is_empty() => {
            let idx = parse_index(argv[0].clone());
            let out = arr.get_index_i64(idx);
            let vmv = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vmv);
            Some(encode_out(out_ptr, out_len, &buf))
        }
        "set" if argv.len() >= 2 => {
            let idx = parse_index(argv[0].clone());
            let vb = vmvalue_to_box(argv[1].clone());
            let out = arr.set_index_i64(idx, vb);
            let vmv = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vmv);
            Some(encode_out(out_ptr, out_len, &buf))
        }
        _ => None,
    }
}

pub(super) fn dispatch_call_slot(
    recv_arc: &Arc<dyn NyashBox>,
    selector_id: u64,
    argv: &[VMValue],
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> Option<i32> {
    if !(100..=102).contains(&selector_id) {
        return None;
    }

    let arr = recv_arc
        .as_any()
        .downcast_ref::<crate::boxes::array::ArrayBox>()?;

    let code = match selector_id {
        100 if !argv.is_empty() => {
            let idx = parse_index(argv[0].clone());
            let out = arr.get_index_i64(idx);
            let vmv = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vmv);
            encode_out(out_ptr, out_len, &buf)
        }
        101 if argv.len() >= 2 => {
            let idx = parse_index(argv[0].clone());
            let vb = vmvalue_to_box(argv[1].clone());
            let out = arr.set_index_i64(idx, vb);
            let vmv = VMValue::from_nyash_box(out);
            let buf = tlv_encode_one(&vmv);
            encode_out(out_ptr, out_len, &buf)
        }
        102 => {
            let out = VMValue::Integer(arr.len() as i64);
            let buf = tlv_encode_one(&out);
            encode_out(out_ptr, out_len, &buf)
        }
        _ => -10,
    };

    Some(code)
}
