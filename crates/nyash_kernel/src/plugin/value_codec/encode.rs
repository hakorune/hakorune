pub(crate) use super::borrowed_handle::BorrowedAliasEncodeCaller;
use super::borrowed_handle::{runtime_i64_from_borrowed_alias, BorrowedHandleBox};
use nyash_rust::{box_trait::NyashBox, runtime::host_handles as handles};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeValueCarrierMode {
    MixedI64OrHandle,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum RuntimeValueCarrierSite {
    ArraySlotLoad,
    MapSlotLoad,
    MapRuntimeDataGetAnyKey,
}

impl RuntimeValueCarrierSite {
    #[inline(always)]
    fn borrowed_alias_caller(self) -> BorrowedAliasEncodeCaller {
        match self {
            Self::ArraySlotLoad => BorrowedAliasEncodeCaller::ArrayGetIndexEncoded,
            Self::MapSlotLoad => BorrowedAliasEncodeCaller::MapSlotLoad,
            Self::MapRuntimeDataGetAnyKey => BorrowedAliasEncodeCaller::MapRuntimeDataGetAnyKey,
        }
    }
}

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
#[cfg(test)]
fn runtime_i64_from_box_ref_caller(value: &dyn NyashBox, caller: BorrowedAliasEncodeCaller) -> i64 {
    runtime_i64_from_box_ref_impl(value, caller, true)
}

/// Encode a runtime collection value into the i64 carrier ABI.
///
/// The carrier preserves bits only: scalar `-1` and a typed-object handle `-1`
/// are distinguishable only by MIR route facts such as `value_demand`,
/// `return_shape`, and result-origin metadata. Runtime code must not infer the
/// value kind from the sign of this carrier.
#[inline(always)]
pub(crate) fn encode_runtime_value_carrier(
    value: &dyn NyashBox,
    mode: RuntimeValueCarrierMode,
    site: RuntimeValueCarrierSite,
) -> i64 {
    match mode {
        RuntimeValueCarrierMode::MixedI64OrHandle => {
            runtime_i64_from_box_ref_impl(value, site.borrowed_alias_caller(), true)
        }
    }
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
