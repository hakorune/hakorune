pub(crate) use super::borrowed_handle::BorrowedAliasEncodeCaller;
use super::borrowed_handle::{runtime_i64_from_borrowed_alias, BorrowedHandleBox};
use nyash_rust::{
    box_trait::{BoolBox, IntegerBox, NyashBox},
    runtime::host_handles as handles,
};

pub(crate) fn box_to_handle(value: Box<dyn NyashBox>) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::from(value);
    handles::to_handle_arc(arc) as i64
}

pub(crate) fn box_to_runtime_i64(value: Box<dyn NyashBox>) -> i64 {
    runtime_i64_from_box_ref(value.as_ref())
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
        return runtime_i64_from_borrowed_alias(alias, caller);
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
