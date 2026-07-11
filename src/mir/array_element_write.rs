//! Canonical Array mutation vocabulary owner.
//!
//! This module classifies and validates write shape. It does not activate
//! Typed `Array<T>`, infer identity from representation facts, or implement
//! Array storage semantics.

use crate::mir::{ArrayElementWriteKind, ArrayWriteSiteId, MirInstruction, ValueId};

pub(crate) const UNCLASSIFIED_SURFACE_TAG: &str = "[mir/array_write/unclassified_surface]";
pub(crate) const INVALID_SHAPE_TAG: &str = "[mir/array_write/invalid_shape]";
pub(crate) const RESIDUAL_CALL_TAG: &str = "[mir/array_write/residual_call]";
pub(crate) const IDENTITY_MISSING_TAG: &str = "[mir/array_write/identity_missing]";
pub(crate) const IDENTITY_DRIFT_TAG: &str = "[mir/array_write/identity_drift]";
pub(crate) const REPRESENTATION_AS_IDENTITY_TAG: &str =
    "[mir/array_write/representation_as_identity]";
pub(crate) const PLANNER_BYPASS_TAG: &str = "[mir/array_write/planner_bypass]";
pub(crate) const COVERED_SITE_DRIFT_TAG: &str = "[mir/array_write/covered_site_drift]";
pub(crate) const OVERLAPPING_ROUTES_TAG: &str = "[mir/array_write/overlapping_selected_routes]";
pub(crate) const PROJECTION_DRIFT_TAG: &str = "[mir/array_write/projection_drift]";
pub(crate) const BACKEND_UNSUPPORTED_TAG: &str = "[mir/array_write/backend_unsupported]";
pub(crate) const RAW_RUNTIME_BYPASS_TAG: &str = "[mir/array_write/raw_runtime_bypass]";

pub(crate) fn instruction(
    site_id: ArrayWriteSiteId,
    dst: Option<ValueId>,
    kind: ArrayElementWriteKind,
    receiver: ValueId,
    index: Option<ValueId>,
    value: ValueId,
) -> Result<MirInstruction, String> {
    validate_shape(kind, index)?;
    Ok(MirInstruction::ArrayElementWrite {
        site_id,
        dst,
        kind,
        receiver,
        index,
        value,
    })
}

pub(crate) fn validate_shape(
    kind: ArrayElementWriteKind,
    index: Option<ValueId>,
) -> Result<(), String> {
    let valid = match kind {
        ArrayElementWriteKind::LiteralAppend | ArrayElementWriteKind::Push => index.is_none(),
        ArrayElementWriteKind::Set | ArrayElementWriteKind::Insert => index.is_some(),
    };
    if valid {
        Ok(())
    } else {
        Err(format!("{} kind={}", INVALID_SHAPE_TAG, kind.as_str()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn owner_enforces_kind_index_shape() {
        assert!(instruction(
            ArrayWriteSiteId::new(1),
            None,
            ArrayElementWriteKind::Push,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .is_ok());
        let error = instruction(
            ArrayWriteSiteId::new(2),
            None,
            ArrayElementWriteKind::Set,
            ValueId::new(0),
            None,
            ValueId::new(1),
        )
        .unwrap_err();
        assert!(error.contains(INVALID_SHAPE_TAG));
    }
}
