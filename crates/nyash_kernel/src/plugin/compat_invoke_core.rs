use super::invoke_core::InvokeFn;

use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles;

pub(super) fn resolve_generic_fallback_route(
    fallback_invoke: InvokeFn,
) -> Option<(String, InvokeFn, Option<u32>)> {
    Some(("PluginBox".to_string(), fallback_invoke, None))
}

#[inline]
pub(super) fn encode_legacy_vm_args_range(
    dst: &mut Vec<u8>,
    start_pos: usize,
    end_pos_inclusive: usize,
) {
    if start_pos > end_pos_inclusive {
        return;
    }
    for pos in start_pos..=end_pos_inclusive {
        crate::encode::nyrt_encode_from_legacy_at(dst, pos);
    }
}

fn encode_box_handle(boxed: Box<dyn NyashBox>) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = boxed.into();
    host_handles::to_handle_arc(arc) as i64
}

fn decode_handle_to_string_like(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = host_handles::get(handle as u64)?;
    if let Some(string_box) = object.as_any().downcast_ref::<StringBox>() {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

fn decode_handle_to_box_or_integer(handle: i64) -> Box<dyn NyashBox> {
    if handle > 0 {
        if let Some(object) = host_handles::get(handle as u64) {
            return object.share_box();
        }
    }
    Box::new(nyash_rust::box_trait::IntegerBox::new(handle))
}

pub(super) fn try_handle_builtin_file_box_by_name(
    recv_handle: i64,
    method: &str,
    argc: i64,
    a1: i64,
    a2: i64,
) -> Option<i64> {
    use nyash_rust::boxes::array::ArrayBox;
    use nyash_rust::boxes::file::FileBox;

    if recv_handle <= 0 {
        return None;
    }
    let object = host_handles::get(recv_handle as u64)?;
    let file_box = object.as_any().downcast_ref::<FileBox>()?;

    match method {
        "open" => {
            if argc < 1 || argc > 2 {
                return Some(0);
            }
            let Some(path) = decode_handle_to_string_like(a1) else {
                return Some(0);
            };
            let mode = if argc >= 2 {
                match decode_handle_to_string_like(a2) {
                    Some(mode) => mode,
                    None => return Some(0),
                }
            } else {
                "r".to_string()
            };
            Some(if file_box.ny_open(&path, &mode).is_ok() {
                1
            } else {
                0
            })
        }
        "read" => {
            if argc != 0 {
                return Some(0);
            }
            file_box
                .ny_read_to_string()
                .ok()
                .map(|text| encode_box_handle(Box::new(StringBox::new(text))))
                .or(Some(0))
        }
        "readBytes" => {
            if argc != 0 {
                return Some(0);
            }
            match file_box.ny_read_bytes() {
                Ok(bytes) => {
                    let arr = ArrayBox::new();
                    for byte in bytes {
                        arr.push(Box::new(nyash_rust::box_trait::IntegerBox::new(byte as i64)));
                    }
                    Some(encode_box_handle(Box::new(arr)))
                }
                Err(_) => Some(0),
            }
        }
        "write" => {
            if argc != 1 {
                return Some(0);
            }
            let result = file_box.write(decode_handle_to_box_or_integer(a1));
            Some(encode_box_handle(result))
        }
        "writeBytes" => {
            if argc != 1 {
                return Some(0);
            }
            let result = file_box.writeBytes(decode_handle_to_box_or_integer(a1));
            Some(encode_box_handle(result))
        }
        "close" => {
            if argc != 0 {
                return Some(0);
            }
            let _ = file_box.ny_close();
            Some(0)
        }
        _ => None,
    }
}
