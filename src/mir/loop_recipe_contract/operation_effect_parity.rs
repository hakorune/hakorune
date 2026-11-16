//! Caller-zero parity receipt for profile operation/effect products.
//!
//! This module compares only the neutral contract. It does not compare
//! profile item counts or source order, and it creates no semantic or
//! physical owner. The common product has already sorted item evidence and
//! owns duplicate/missing/foreign/placement rejection.

#![cfg(test)]

use super::ids::LoopItemKeyV1;
use super::operation_effect::VerifiedLoopOperationEffectProductV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopOperationEffectParitySideV1 {
    Callable,
    GenericG0,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum LoopOperationEffectParityRejectV1 {
    EmptyProfile {
        side: LoopOperationEffectParitySideV1,
    },
    DuplicateEvidence {
        side: LoopOperationEffectParitySideV1,
        item: LoopItemKeyV1,
    },
    OwnerMismatch {
        side: LoopOperationEffectParitySideV1,
        item: LoopItemKeyV1,
    },
}

/// Diagnostic-only proof that two profile adapters use one neutral contract.
/// The item counts are observations, not selection keys or Recipe truth.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct LoopOperationEffectParityReceiptV1 {
    callable_items: usize,
    generic_items: usize,
}

impl LoopOperationEffectParityReceiptV1 {
    pub(crate) const fn callable_items(self) -> usize {
        self.callable_items
    }

    pub(crate) const fn generic_items(self) -> usize {
        self.generic_items
    }
}

pub(crate) fn issue_operation_effect_parity_receipt_v1(
    callable: &VerifiedLoopOperationEffectProductV1,
    generic: &VerifiedLoopOperationEffectProductV1,
) -> Result<LoopOperationEffectParityReceiptV1, LoopOperationEffectParityRejectV1> {
    validate_profile(callable, LoopOperationEffectParitySideV1::Callable)?;
    validate_profile(generic, LoopOperationEffectParitySideV1::GenericG0)?;
    Ok(LoopOperationEffectParityReceiptV1 {
        callable_items: callable.evidence().len(),
        generic_items: generic.evidence().len(),
    })
}

fn validate_profile(
    product: &VerifiedLoopOperationEffectProductV1,
    side: LoopOperationEffectParitySideV1,
) -> Result<(), LoopOperationEffectParityRejectV1> {
    if product.evidence().is_empty() {
        return Err(LoopOperationEffectParityRejectV1::EmptyProfile { side });
    }
    let owner = product.core().owner();
    for row in product.evidence() {
        if row.anchor().owner() != owner
            || row
                .source_binding()
                .is_some_and(|binding| binding.owner() != owner)
        {
            return Err(LoopOperationEffectParityRejectV1::OwnerMismatch {
                side,
                item: row.item(),
            });
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_effect_for_test;
    use crate::mir::loop_recipe_contract::generic_g0::generic_operation_effect_for_test;
    use crate::mir::loop_recipe_contract::{
        LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1,
        VerifiedLoopOperationEffectProductV1,
    };

    #[test]
    fn callable_and_generic_g0_share_neutral_operation_effect_contract() {
        let callable = callable_operation_effect_for_test();
        let generic = generic_operation_effect_for_test();
        let receipt =
            issue_operation_effect_parity_receipt_v1(&callable, &generic).expect("profile parity");
        assert_eq!(receipt.callable_items(), 7);
        assert_eq!(receipt.generic_items(), 15);
    }

    #[test]
    fn profile_item_relabel_is_rejected_before_parity() {
        let product = generic_operation_effect_for_test();
        let (core, rows) = product.into_parts();
        let mut raw = rows
            .into_vec()
            .into_iter()
            .map(|row| row.into_unverified_for_test())
            .collect::<Vec<_>>();
        let index = raw
            .iter()
            .position(|row| row.item() == LoopItemKeyV1::new(3))
            .expect("child entry row");
        let row = raw.remove(index);
        raw.push(LoopOperationSourceEvidenceV1::new(
            LoopItemKeyV1::new(4),
            row.anchor().clone(),
            row.source_loop().clone(),
            row.owner_loop(),
            row.block(),
            row.source_binding(),
        ));
        assert!(matches!(
            VerifiedLoopOperationEffectProductV1::issue(core, raw),
            Err(LoopOperationEffectRejectV1::NonOperationItem { item })
                if item == LoopItemKeyV1::new(4)
        ));
    }
}
