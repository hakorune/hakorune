use super::borrowed_handle::maybe_borrow_string_handle_with_epoch;
use super::decode::int_arg_to_box;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    let arc: Arc<dyn NyashBox> = Arc::new(StringBox::new(value));
    handles::to_handle_arc(arc) as i64
}

#[inline(always)]
pub(crate) fn store_string_box_from_source(
    source_handle: i64,
    source_obj: Option<&Arc<dyn NyashBox>>,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = source_obj else {
        return int_arg_to_box(source_handle);
    };
    if obj.as_any().downcast_ref::<StringBox>().is_some()
        || obj
            .as_any()
            .downcast_ref::<crate::exports::string_view::StringViewBox>()
            .is_some()
    {
        return maybe_borrow_string_handle_with_epoch(obj.clone(), source_handle, source_drop_epoch);
    }
    int_arg_to_box(source_handle)
}
