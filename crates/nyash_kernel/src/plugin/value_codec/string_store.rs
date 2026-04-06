use super::borrowed_handle::maybe_borrow_string_handle_with_epoch;
use super::decode::int_arg_to_box;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[inline(always)]
pub(crate) fn materialize_owned_string(value: String) -> i64 {
    crate::observe::record_birth_backend_materialize_owned(value.len());
    if crate::observe::bypass_gc_alloc_enabled() {
        crate::observe::record_birth_backend_gc_alloc_skipped();
    } else {
        crate::observe::record_birth_backend_gc_alloc(value.len());
        nyash_rust::runtime::global_hooks::gc_alloc(value.len() as u64);
    }
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
        crate::observe::record_birth_placement_store_from_source();
        return maybe_borrow_string_handle_with_epoch(
            obj.clone(),
            source_handle,
            source_drop_epoch,
        );
    }
    int_arg_to_box(source_handle)
}

#[inline(always)]
pub(crate) fn is_string_handle_source(source_obj: &Arc<dyn NyashBox>) -> bool {
    source_obj.as_any().downcast_ref::<StringBox>().is_some()
        || source_obj
            .as_any()
            .downcast_ref::<crate::exports::string_view::StringViewBox>()
            .is_some()
}

#[inline(always)]
pub(crate) fn store_string_box_from_string_source(
    source_handle: i64,
    source_obj: &Arc<dyn NyashBox>,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    debug_assert!(source_handle > 0);
    debug_assert!(is_string_handle_source(source_obj));
    crate::observe::record_birth_placement_store_from_source();
    maybe_borrow_string_handle_with_epoch(source_obj.clone(), source_handle, source_drop_epoch)
}

#[inline(always)]
pub(crate) fn maybe_store_string_box_from_verified_source(
    source_handle: i64,
    source_obj: Option<&Arc<dyn NyashBox>>,
    source_drop_epoch: u64,
    source_is_string: bool,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    let Some(obj) = source_obj else {
        return int_arg_to_box(source_handle);
    };
    if source_is_string {
        crate::observe::record_birth_placement_store_from_source();
        return maybe_borrow_string_handle_with_epoch(
            obj.clone(),
            source_handle,
            source_drop_epoch,
        );
    }
    int_arg_to_box(source_handle)
}
