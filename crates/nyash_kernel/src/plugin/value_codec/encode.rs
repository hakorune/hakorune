use super::borrowed_handle::BorrowedHandleBox;
use crate::observe;
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox},
    runtime::host_handles as handles,
};
use std::sync::Arc;

#[derive(Clone, Copy)]
pub(crate) enum BorrowedAliasEncodeCaller {
    Generic,
    ArrayGetIndexEncoded,
    MapRuntimeDataGetAnyKey,
}

pub(crate) fn box_to_handle(value: Box<dyn NyashBox>) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::from(value);
    handles::to_handle_arc(arc) as i64
}

pub(crate) fn box_to_runtime_i64(value: Box<dyn NyashBox>) -> i64 {
    runtime_i64_from_box_ref_caller(value.as_ref(), BorrowedAliasEncodeCaller::Generic)
}

#[inline(always)]
pub(crate) fn runtime_i64_from_box_ref(value: &dyn NyashBox) -> i64 {
    runtime_i64_from_box_ref_caller(value, BorrowedAliasEncodeCaller::Generic)
}

#[inline(always)]
pub(crate) fn runtime_i64_from_box_ref_caller(
    value: &dyn NyashBox,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    // Borrowed alias aware runtime encoder:
    // reuse source handle only while the alias epoch is still live,
    // otherwise fall back to conservative re-materialization.
    if let Some(alias) = value.as_any().downcast_ref::<BorrowedHandleBox>() {
        if alias.source_handle > 0 {
            // Fast path: if no handle drop happened since alias creation,
            // source handle still points to the same object.
            let current_epoch = handles::drop_epoch();
            if alias.source_drop_epoch == current_epoch {
                observe::record_borrowed_alias_encode_epoch_hit();
                return alias.source_handle;
            }
        }
        if let Some(iv) = integer_box_to_i64(alias.stable_box_ref().as_ref()) {
            return iv;
        }
        if let Some(bv) = bool_box_to_i64(alias.stable_box_ref().as_ref()) {
            return bv;
        }
        if alias.source_handle > 0 {
            if let Some(source_obj) = handles::get(alias.source_handle as u64) {
                if Arc::ptr_eq(&source_obj, alias.stable_box_ref()) {
                    observe::record_borrowed_alias_encode_ptr_eq_hit();
                    return alias.source_handle;
                }
            }
        }
        observe::record_borrowed_alias_encode_to_handle_arc();
        caller.record();
        return handles::to_handle_arc(alias.stable_box_ref().clone()) as i64;
    }
    if let Some(iv) = integer_box_to_i64(value) {
        return iv;
    }
    if let Some(bv) = bool_box_to_i64(value) {
        return bv;
    }
    let cloned = if value.is_identity() {
        value.share_box()
    } else {
        value.clone_box()
    };
    box_to_handle(cloned)
}

pub(crate) fn integer_box_to_i64(value: &dyn NyashBox) -> Option<i64> {
    value
        .as_any()
        .downcast_ref::<IntegerBox>()
        .map(|ib| ib.value)
}

pub(crate) fn bool_box_to_i64(value: &dyn NyashBox) -> Option<i64> {
    value
        .as_any()
        .downcast_ref::<BoolBox>()
        .map(|bb| if bb.value { 1 } else { 0 })
}

impl BorrowedAliasEncodeCaller {
    #[inline(always)]
    fn record(self) {
        match self {
            Self::Generic => {}
            Self::ArrayGetIndexEncoded => {
                observe::record_borrowed_alias_encode_to_handle_arc_array_get_index();
            }
            Self::MapRuntimeDataGetAnyKey => {
                observe::record_borrowed_alias_encode_to_handle_arc_map_runtime_data_get_any();
            }
        }
    }
}
