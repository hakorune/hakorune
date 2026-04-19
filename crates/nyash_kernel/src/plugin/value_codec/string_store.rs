use super::borrowed_handle::{
    maybe_borrow_string_handle_with_epoch, maybe_borrow_string_keep_with_epoch, SourceLifetimeKeep,
};
use super::decode::int_arg_to_box;
use super::string_classify::VerifiedTextSource;
use super::string_materialize::{
    objectize_kernel_text_slot_stable_box, with_const_suffix_ptr_text, KernelTextSlot,
};
use crate::exports::string::to_owned_string_handle_arg;
use nyash_rust::box_trait::{NyashBox, StringBox};
use nyash_rust::runtime::host_handles as handles;
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
    let bytes = slot.take_materialized_owned_bytes()?;
    Some(Box::new(StringBox::new(bytes.into_string())) as Box<dyn NyashBox>)
}

#[inline(always)]
fn overwrite_string_box_from_const_suffix(
    value: &mut StringBox,
    source: &str,
    suffix_ptr: *const i8,
) -> bool {
    with_const_suffix_ptr_text(suffix_ptr, |suffix| {
        if value.value.as_str() == source {
            value.value.reserve(suffix.len());
            value.value.push_str(suffix.as_str());
            return;
        }
        let total = source.len().saturating_add(suffix.len());
        value.value.clear();
        value.value.reserve(total);
        value.value.push_str(source);
        value.value.push_str(suffix.as_str());
    })
    .is_some()
}

#[inline(always)]
pub(crate) fn store_string_into_existing_string_box_from_kernel_text_slot(
    slot: &mut KernelTextSlot,
    value: &mut StringBox,
) -> bool {
    if let Some((source_h, suffix_ptr)) = slot.take_deferred_const_suffix() {
        if let Some(hit) = handles::with_text_read_session_ready(|session| {
            session.str_handle(source_h as u64, |source| {
                overwrite_string_box_from_const_suffix(value, source, suffix_ptr)
            })
        })
        .flatten()
        {
            return hit;
        }
        let source = to_owned_string_handle_arg(source_h);
        return overwrite_string_box_from_const_suffix(value, source.as_str(), suffix_ptr);
    }
    let Some(bytes) = slot.take_materialized_owned_bytes() else {
        return false;
    };
    value.value = bytes.into_string();
    true
}

#[inline(always)]
pub(crate) fn store_string_keep_from_kernel_text_slot(
    slot: &mut KernelTextSlot,
) -> Option<SourceLifetimeKeep> {
    let stable_box = objectize_kernel_text_slot_stable_box(slot)?;
    Some(SourceLifetimeKeep::string_box(stable_box))
}
