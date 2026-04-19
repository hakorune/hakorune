use super::string_classify::VerifiedTextSource;
use crate::observe;
use nyash_rust::{
    box_trait::{NyashBox, StringBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

mod backing;
mod box_impl;

pub(crate) use backing::SourceLifetimeKeep;
pub(crate) use box_impl::{
    runtime_i64_from_borrowed_alias, BorrowedAliasEncodeCaller, BorrowedHandleBox,
};

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
) -> Box<dyn NyashBox> {
    maybe_borrow_string_handle_with_epoch(obj, source_handle, handles::drop_epoch())
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle_with_epoch(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            SourceLifetimeKeep::string_box(obj),
            source_handle,
            source_drop_epoch,
        ));
    }
    if obj
        .as_any()
        .downcast_ref::<crate::exports::string_view::StringViewBox>()
        .is_some()
    {
        return promote_string_view_to_owned_box_cold(obj);
    }
    obj.clone_box()
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_keep_with_epoch(
    keep: SourceLifetimeKeep,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if keep.supports_borrowed_alias() {
        return Box::new(BorrowedHandleBox::new(
            keep,
            source_handle,
            source_drop_epoch,
        ));
    }
    promote_source_keep_to_owned_box_cold(keep)
}

#[inline(always)]
pub(crate) fn keep_borrowed_string_slot_source_keep(
    alias: &mut BorrowedHandleBox,
    source_keep: SourceLifetimeKeep,
) {
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if alias.ptr_eq_source_keep(&source_keep) {
        if observe::enabled() {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        }
        return;
    }
    if observe::enabled() {
        observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
    }
    alias.replace_source_keep(source_keep);
    alias.invalidate_cached_runtime_handle();
}

#[inline(always)]
pub(crate) fn update_borrowed_string_slot_alias(
    alias: &mut BorrowedHandleBox,
    source_handle: i64,
    source_drop_epoch: u64,
) {
    observe::record_store_array_str_reason_retarget_alias_update();
    alias.replace_source_alias(source_handle, source_drop_epoch);
    alias.invalidate_cached_runtime_handle();
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_take_keep(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_keep: SourceLifetimeKeep,
    source_drop_epoch: u64,
) -> Result<(), SourceLifetimeKeep> {
    if source_handle <= 0 {
        return Err(source_keep);
    }
    let Some(alias) = slot.as_any_mut().downcast_mut::<BorrowedHandleBox>() else {
        return Err(source_keep);
    };
    keep_borrowed_string_slot_source_keep(alias, source_keep);
    update_borrowed_string_slot_alias(alias, source_handle, source_drop_epoch);
    Ok(())
}

#[inline(always)]
pub(crate) fn try_retarget_borrowed_string_slot_take_verified_text_source(
    slot: &mut Box<dyn NyashBox>,
    source_handle: i64,
    source_text: VerifiedTextSource,
    source_drop_epoch: u64,
) -> Result<(), VerifiedTextSource> {
    let proof = source_text.proof();
    match try_retarget_borrowed_string_slot_take_keep(
        slot,
        source_handle,
        source_text.into_keep(),
        source_drop_epoch,
    ) {
        Ok(()) => Ok(()),
        Err(source_keep) => Err(VerifiedTextSource::new(proof, source_keep)),
    }
}

#[cold]
#[inline(never)]
fn promote_string_view_to_owned_box_cold(obj: Arc<dyn NyashBox>) -> Box<dyn NyashBox> {
    observe::record_birth_backend_publish_reason_need_stable_object();
    observe::record_birth_backend_carrier_kind_stable_box();
    Box::new(StringBox::new(
        SourceLifetimeKeep::string_view(obj).copy_owned_text_cold(),
    ))
}

#[cold]
#[inline(never)]
fn promote_source_keep_to_owned_box_cold(keep: SourceLifetimeKeep) -> Box<dyn NyashBox> {
    observe::record_birth_backend_publish_reason_need_stable_object();
    observe::record_birth_backend_carrier_kind_stable_box();
    Box::new(StringBox::new(keep.copy_owned_text_cold()))
}
