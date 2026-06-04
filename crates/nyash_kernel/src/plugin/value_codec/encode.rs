pub(crate) use super::borrowed_handle::BorrowedAliasEncodeCaller;
use super::borrowed_handle::{runtime_i64_from_borrowed_alias, BorrowedHandleBox};
use nyash_rust::{box_trait::NyashBox, runtime::host_handles as handles};

pub(crate) fn box_to_handle(value: Box<dyn NyashBox>) -> i64 {
    let arc: std::sync::Arc<dyn NyashBox> = std::sync::Arc::from(value);
    handles::to_handle_arc(arc) as i64
}

#[cfg(test)]
#[inline(always)]
pub(crate) fn runtime_i64_from_box_ref(value: &dyn NyashBox) -> i64 {
    runtime_i64_from_box_ref_caller(value, BorrowedAliasEncodeCaller::Generic)
}

#[inline(always)]
pub(crate) fn runtime_i64_from_box_ref_caller(
    value: &dyn NyashBox,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    runtime_i64_from_box_ref_impl(value, caller, true)
}

#[inline(always)]
pub(crate) fn runtime_i64_from_scalar_checked_box_ref_caller(
    value: &dyn NyashBox,
    caller: BorrowedAliasEncodeCaller,
) -> i64 {
    runtime_i64_from_box_ref_impl(value, caller, false)
}

#[inline(always)]
fn runtime_i64_from_box_ref_impl(
    value: &dyn NyashBox,
    caller: BorrowedAliasEncodeCaller,
    probe_scalars: bool,
) -> i64 {
    if let Some(alias) = value.as_any().downcast_ref::<BorrowedHandleBox>() {
        return runtime_i64_from_borrowed_alias(alias, caller);
    }
    if probe_scalars {
        if let Some(iv) = scalar_i64_from_box_ref(value) {
            return iv;
        }
    }
    let cloned = if value.is_identity() {
        value.share_box()
    } else {
        value.clone_box()
    };
    box_to_handle(cloned)
}

#[inline(always)]
fn scalar_i64_from_box_ref(value: &dyn NyashBox) -> Option<i64> {
    if let Some(iv) = value
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::IntegerBox>()
        .map(|ib| ib.value)
    {
        return Some(iv);
    }
    value
        .as_any()
        .downcast_ref::<nyash_rust::box_trait::BoolBox>()
        .map(|bb| if bb.value { 1 } else { 0 })
}
