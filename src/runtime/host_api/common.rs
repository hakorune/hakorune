use crate::backend::vm::VMValue;
use crate::box_trait::NyashBox;

pub(super) fn tlv_encode_one(val: &VMValue) -> Vec<u8> {
    use crate::runtime::plugin_ffi_common as tlv;

    let mut buf = tlv::encode_tlv_header(1);
    match val {
        VMValue::Integer(i) => tlv::encode::i64(&mut buf, *i),
        VMValue::Float(f) => tlv::encode::f64(&mut buf, *f),
        VMValue::Bool(b) => tlv::encode::bool(&mut buf, *b),
        VMValue::String(s) => tlv::encode::string(&mut buf, s),
        VMValue::BoxRef(b) => {
            let h = crate::runtime::host_handles::to_handle_arc(b.clone());
            tlv::encode::host_handle(&mut buf, h);
        }
        _ => tlv::encode::string(&mut buf, "void"),
    }
    buf
}

pub(super) fn vmvalue_from_tlv(tag: u8, payload: &[u8]) -> Option<VMValue> {
    use crate::runtime::plugin_ffi_common as tlv;

    match tag {
        1 => Some(VMValue::Bool(tlv::decode::bool(payload).unwrap_or(false))),
        2 => tlv::decode::i32(payload).map(|v| VMValue::Integer(v as i64)),
        3 => {
            if payload.len() == 8 {
                let mut b = [0u8; 8];
                b.copy_from_slice(payload);
                Some(VMValue::Integer(i64::from_le_bytes(b)))
            } else {
                None
            }
        }
        5 => tlv::decode::f64(payload).map(VMValue::Float),
        6 | 7 => Some(VMValue::String(tlv::decode::string(payload))),
        8 => {
            if let Some((type_id, instance_id)) = tlv::decode::plugin_handle(payload) {
                if let Some(arc) = plugin_box_from_handle(type_id, instance_id) {
                    return Some(VMValue::BoxRef(arc));
                }
            }
            None
        }
        9 => tlv::decode::u64(payload)
            .and_then(|h| crate::runtime::host_handles::get(h).map(VMValue::BoxRef)),
        _ => None,
    }
}

pub(super) unsafe fn slice_from_raw<'a>(ptr: *const u8, len: usize) -> &'a [u8] {
    std::slice::from_raw_parts(ptr, len)
}

unsafe fn slice_from_raw_mut<'a>(ptr: *mut u8, len: usize) -> &'a mut [u8] {
    std::slice::from_raw_parts_mut(ptr, len)
}

pub(super) fn encode_out(out_ptr: *mut u8, out_len: *mut usize, buf: &[u8]) -> i32 {
    unsafe {
        if out_ptr.is_null() || out_len.is_null() {
            return -2;
        }
        let cap = *out_len;
        if cap < buf.len() {
            return -3;
        }
        let out = slice_from_raw_mut(out_ptr, cap);
        out[..buf.len()].copy_from_slice(buf);
        *out_len = buf.len();
        0
    }
}

pub(super) fn parse_tlv_args(args_ptr: *const u8, args_len: usize) -> Vec<VMValue> {
    let mut argv: Vec<VMValue> = Vec::new();
    if args_ptr.is_null() || args_len < 4 {
        return argv;
    }

    let buf = unsafe { slice_from_raw(args_ptr, args_len) };
    let mut off = 4usize;
    while buf.len() >= off + 4 {
        let tag = buf[off];
        let sz = u16::from_le_bytes([buf[off + 2], buf[off + 3]]) as usize;
        if buf.len() < off + 4 + sz {
            break;
        }
        let payload = &buf[off + 4..off + 4 + sz];
        if let Some(v) = vmvalue_from_tlv(tag, payload) {
            argv.push(v);
        }
        off += 4 + sz;
    }
    argv
}

#[cfg(all(feature = "plugins", not(target_arch = "wasm32")))]
fn plugin_box_from_handle(type_id: u32, instance_id: u32) -> Option<std::sync::Arc<dyn NyashBox>> {
    let loader = crate::runtime::plugin_loader_v2::get_global_loader_v2();
    let loader = loader.read().ok()?;
    let bx = loader.construct_existing_instance(type_id, instance_id)?;
    Some(std::sync::Arc::from(bx))
}

#[cfg(any(not(feature = "plugins"), target_arch = "wasm32"))]
fn plugin_box_from_handle(
    _type_id: u32,
    _instance_id: u32,
) -> Option<std::sync::Arc<dyn NyashBox>> {
    None
}
