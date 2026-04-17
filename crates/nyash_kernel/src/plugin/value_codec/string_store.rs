use super::borrowed_handle::{
    maybe_borrow_string_handle_with_epoch, maybe_borrow_string_keep_with_epoch, SourceLifetimeKeep,
};
use super::decode::int_arg_to_box;
use super::string_classify::VerifiedTextSource;
use super::string_materialize::KernelTextSlot;
use nyash_rust::box_trait::{NyashBox, StringBox};
use std::sync::Arc;

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

#[cfg(test)]
#[inline(always)]
pub(crate) fn store_string_box_from_source_keep(
    source_handle: i64,
    source_keep: &SourceLifetimeKeep,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    store_string_box_from_source_keep_owned(source_handle, source_keep.clone(), source_drop_epoch)
}

#[inline(always)]
pub(crate) fn store_string_box_from_source_keep_owned(
    source_handle: i64,
    source_keep: SourceLifetimeKeep,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    crate::observe::record_birth_placement_store_from_source();
    crate::observe::record_birth_backend_carrier_kind_source_keep();
    maybe_borrow_string_keep_with_epoch(source_keep, source_handle, source_drop_epoch)
}

#[cfg_attr(feature = "perf-observe", inline(never))]
#[cfg_attr(not(feature = "perf-observe"), inline(always))]
pub(crate) fn store_string_box_from_verified_text_source(
    source_handle: i64,
    source_text: VerifiedTextSource,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    store_string_box_from_source_keep_owned(
        source_handle,
        source_text.into_keep(),
        source_drop_epoch,
    )
}

#[inline(always)]
pub(crate) fn maybe_store_non_string_box_from_verified_source(
    source_handle: i64,
    _source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if source_handle <= 0 {
        return int_arg_to_box(source_handle);
    }
    int_arg_to_box(source_handle)
}

#[inline(always)]
pub(crate) fn store_string_box_from_kernel_text_slot(
    slot: &mut KernelTextSlot,
) -> Option<Box<dyn NyashBox>> {
    let bytes = slot.take_owned_bytes()?;
    Some(Box::new(StringBox::new(bytes.into_string())) as Box<dyn NyashBox>)
}
