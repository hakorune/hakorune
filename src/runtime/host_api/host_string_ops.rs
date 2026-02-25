use crate::backend::vm::VMValue;
use crate::box_trait::NyashBox;
use std::sync::Arc;

use super::common::{encode_out, tlv_encode_one};

pub(super) fn dispatch_call_slot(
    recv_arc: &Arc<dyn NyashBox>,
    selector_id: u64,
    _argv: &[VMValue],
    out_ptr: *mut u8,
    out_len: *mut usize,
) -> Option<i32> {
    if selector_id != 300 {
        return None;
    }

    let string_box = recv_arc
        .as_any()
        .downcast_ref::<crate::box_trait::StringBox>()?;
    let out = VMValue::Integer(string_box.value.len() as i64);
    let buf = tlv_encode_one(&out);
    Some(encode_out(out_ptr, out_len, &buf))
}
