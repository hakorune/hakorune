use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

pub(crate) fn decode_string_like_handle(handle: i64) -> Option<String> {
    if handle <= 0 {
        return None;
    }
    let object = handles::get(handle as u64)?;
    if let Some(string_box) = object.as_any().downcast_ref::<StringBox>() {
        return Some(string_box.value.clone());
    }
    Some(object.to_string_box().value)
}

pub(crate) fn ensure_test_ring0() {
    let _ = nyash_rust::runtime::ring0::ensure_global_ring0_initialized();
}

pub(crate) fn string_handle(value: &str) -> i64 {
    handles::to_handle_arc(Arc::new(StringBox::new(value.to_string()))) as i64
}

pub(crate) fn dispatch_stage1_module(receiver_name: &str, method: &str, source_text: &str) -> i64 {
    let receiver_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(receiver_name.to_string()))) as i64;
    let source_handle =
        handles::to_handle_arc(Arc::new(StringBox::new(source_text.to_string()))) as i64;
    crate::plugin::module_string_dispatch::try_dispatch(
        receiver_handle,
        method,
        1,
        source_handle,
        0,
    )
    .expect("stage1 direct dispatch")
}
