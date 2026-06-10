use self::backing::TextKeepClass;
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
pub(crate) fn maybe_borrow_string_keep_with_epoch(
    keep: SourceLifetimeKeep,
    source_handle: i64,
    source_drop_epoch: u64,
) -> Box<dyn NyashBox> {
    if keep.class == TextKeepClass::StringBox {
        return Box::new(BorrowedHandleBox::new(
            keep,
            source_handle,
            source_drop_epoch,
        ));
    }
    observe::record_birth_backend_publish_reason_need_stable_object();
    observe::record_birth_backend_carrier_kind_stable_box();
    Box::new(StringBox::new(
        keep.backing.stable_box.as_ref().to_string_box().value,
    ))
}

#[cfg(test)]
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
    observe::record_store_array_str_reason_retarget_keep_source_arc();
    if Arc::ptr_eq(
        &alias.text_keep.source_lifetime.backing.stable_box,
        &source_keep.backing.stable_box,
    ) {
        if observe::enabled() {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_hit();
        }
    } else {
        if observe::enabled() {
            observe::record_store_array_str_reason_retarget_keep_source_arc_ptr_eq_miss();
        }
        alias.text_keep.source_lifetime = source_keep;
        alias.invalidate_cached_runtime_handle();
    }
    observe::record_store_array_str_reason_retarget_alias_update();
    alias.source_meta.source_handle = source_handle;
    alias.source_meta.source_drop_epoch = source_drop_epoch;
    alias.invalidate_cached_runtime_handle();
    Ok(())
}

#[inline(always)]
pub(crate) fn maybe_borrow_string_handle(
    obj: Arc<dyn NyashBox>,
    source_handle: i64,
) -> Box<dyn NyashBox> {
    if obj.as_any().downcast_ref::<StringBox>().is_some() {
        return Box::new(BorrowedHandleBox::new(
            SourceLifetimeKeep::string_box(obj),
            source_handle,
            handles::drop_epoch(),
        ));
    }
    if obj
        .as_any()
        .downcast_ref::<crate::exports::string_view::StringViewBox>()
        .is_some()
    {
        observe::record_birth_backend_publish_reason_need_stable_object();
        observe::record_birth_backend_carrier_kind_stable_box();
        return Box::new(StringBox::new(
            SourceLifetimeKeep::string_view(obj)
                .backing
                .stable_box
                .as_ref()
                .to_string_box()
                .value,
        ));
    }
    obj.clone_box()
}
