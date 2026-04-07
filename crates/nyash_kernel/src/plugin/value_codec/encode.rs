use super::borrowed_handle::BorrowedHandleBox;
use crate::observe;
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox},
    runtime::host_handles as handles,
};

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
    if let Some(alias) = value.as_any().downcast_ref::<BorrowedHandleBox>() {
        let plan = plan_borrowed_alias_runtime_i64(alias);
        return execute_borrowed_alias_runtime_i64(alias, plan, caller);
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

enum BorrowedAliasEncodePlan {
    ReuseSourceHandle(i64),
    ReturnScalar(i64),
    EncodeFallback,
}

#[inline(always)]
fn plan_borrowed_alias_runtime_i64(alias: &BorrowedHandleBox) -> BorrowedAliasEncodePlan {
    if alias.source_handle() > 0 {
        let current_epoch = handles::drop_epoch();
        if alias.source_drop_epoch() == current_epoch {
            observe::record_borrowed_alias_encode_epoch_hit();
            return BorrowedAliasEncodePlan::ReuseSourceHandle(alias.source_handle());
        }
    }
    if let Some(iv) = integer_box_to_i64(alias.encode_fallback_box_ref()) {
        return BorrowedAliasEncodePlan::ReturnScalar(iv);
    }
    if let Some(bv) = bool_box_to_i64(alias.encode_fallback_box_ref()) {
        return BorrowedAliasEncodePlan::ReturnScalar(bv);
    }
    if alias.source_handle() > 0 {
        if let Some(source_obj) = handles::get(alias.source_handle() as u64) {
            if alias.ptr_eq_source_object(&source_obj) {
                observe::record_borrowed_alias_encode_ptr_eq_hit();
                return BorrowedAliasEncodePlan::ReuseSourceHandle(alias.source_handle());
            }
        }
    }
    BorrowedAliasEncodePlan::EncodeFallback
}

#[inline(always)]
fn execute_borrowed_alias_runtime_i64(
    alias: &BorrowedHandleBox,
    plan: BorrowedAliasEncodePlan,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    match plan {
        BorrowedAliasEncodePlan::ReuseSourceHandle(handle) => handle,
        BorrowedAliasEncodePlan::ReturnScalar(value) => value,
        BorrowedAliasEncodePlan::EncodeFallback => {
            observe::record_borrowed_alias_encode_to_handle_arc();
            caller.record();
            handles::to_handle_arc(alias.clone_stable_box_for_encode_fallback()) as i64
        }
    }
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
