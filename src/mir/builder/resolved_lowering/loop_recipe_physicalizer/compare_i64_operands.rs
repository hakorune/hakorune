//! Loop-side co-seal for CompareI64 operands.
//!
//! This module validates the full Published ledger receipt, creates only a
//! neutral canonical-SSA request, and retains the receipt together with the
//! session-issued same-block Integer witness. It never returns a bare
//! `ValueId` and never emits MIR.

use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSameBlockIntegerRejectV1;
use crate::mir::loop_recipe_contract::{LoopValueClassV1, LoopValueKeyV1};

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum CanonicalLoopCompareI64OperandRejectV1 {
    ReceiptUnavailable(LoopValueKeyV1),
    ReceiptOwnerMismatch,
    TargetOwnerMismatch,
    ReceiptClassMismatch,
    ReceiptTargetMismatch,
    Canonical(CanonicalSameBlockIntegerRejectV1),
}

fn validate_published_receipt(
    receipt: &LoopOperationValueReceiptV1,
    key: LoopValueKeyV1,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    target_block: crate::mir::BasicBlockId,
) -> Result<(), CanonicalLoopCompareI64OperandRejectV1> {
    if receipt.key() != key || receipt.owner() != owner {
        return Err(CanonicalLoopCompareI64OperandRejectV1::ReceiptOwnerMismatch);
    }
    if receipt.class() != LoopValueClassV1::I64 {
        return Err(CanonicalLoopCompareI64OperandRejectV1::ReceiptClassMismatch);
    }
    if receipt.physical_block() != target_block {
        return Err(CanonicalLoopCompareI64OperandRejectV1::ReceiptTargetMismatch);
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;

    fn owner() -> crate::mir::resolved_semantics::FunctionOwnerIdV1 {
        FunctionOwnerIssuerV1::new_for_compilation()
            .expect("owner issuer")
            .issue()
            .expect("owner")
    }

    fn receipt(
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        key: LoopValueKeyV1,
        class: LoopValueClassV1,
        block: crate::mir::BasicBlockId,
    ) -> LoopOperationValueReceiptV1 {
        LoopOperationValueReceiptV1::new(
            owner,
            key,
            class,
            crate::mir::loop_recipe_contract::LoopItemKeyV1::new(0),
            block,
            crate::mir::ValueId::new(7),
        )
    }

    #[test]
    fn only_full_published_i64_receipt_for_exact_target_is_admitted() {
        let owner = owner();
        let key = LoopValueKeyV1::new(1);
        let target = crate::mir::BasicBlockId::new(3);
        let expected = receipt(owner, key, LoopValueClassV1::I64, target);
        let mut ledger = LoopOperationValueLedgerV1::default();
        ledger.publish(expected).unwrap();
        let published = ledger.receipt(key).expect("full Published receipt");

        assert_eq!(
            validate_published_receipt(&published, key, owner, target),
            Ok(())
        );
        assert_eq!(ledger.receipt(key), Some(expected));

        assert_eq!(
            validate_published_receipt(
                &receipt(owner, key, LoopValueClassV1::Bool, target),
                key,
                owner,
                target,
            ),
            Err(CanonicalLoopCompareI64OperandRejectV1::ReceiptClassMismatch)
        );
        assert_eq!(
            validate_published_receipt(
                &receipt(
                    owner,
                    key,
                    LoopValueClassV1::I64,
                    crate::mir::BasicBlockId::new(4)
                ),
                key,
                owner,
                target,
            ),
            Err(CanonicalLoopCompareI64OperandRejectV1::ReceiptTargetMismatch)
        );
    }
}
