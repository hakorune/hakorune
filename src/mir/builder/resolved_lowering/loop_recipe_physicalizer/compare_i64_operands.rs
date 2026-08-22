//! Loop-side co-seal for CompareI64 operands.
//!
//! This module validates the full Published ledger receipt, creates only a
//! neutral canonical-SSA request, and retains the receipt together with the
//! session-issued same-block Integer witness. It never returns a bare
//! `ValueId` and never emits MIR.

use super::operation_ledger::{LoopOperationValueLedgerV1, LoopOperationValueReceiptV1};
use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedCanonicalOpenInstructionTargetV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalSameBlockIntegerRejectV1, CanonicalSameBlockIntegerRequestV1,
    CanonicalSsaFunctionSessionV2, VerifiedCanonicalSameBlockIntegerOperandV1,
};
use crate::mir::builder::MirBuilder;
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

#[derive(Debug)]
pub(super) struct VerifiedCanonicalLoopCompareI64OperandV1 {
    receipt: LoopOperationValueReceiptV1,
    definition: VerifiedCanonicalSameBlockIntegerOperandV1,
    _seal: CanonicalLoopCompareI64OperandSealV1,
}

#[derive(Debug)]
struct CanonicalLoopCompareI64OperandSealV1;

pub(super) fn issue(
    ledger: &LoopOperationValueLedgerV1,
    key: LoopValueKeyV1,
    target: VerifiedCanonicalOpenInstructionTargetV1,
    session: &CanonicalSsaFunctionSessionV2<'_>,
    builder: &MirBuilder,
) -> Result<VerifiedCanonicalLoopCompareI64OperandV1, CanonicalLoopCompareI64OperandRejectV1> {
    let receipt =
        ledger
            .receipt(key)
            .ok_or(CanonicalLoopCompareI64OperandRejectV1::ReceiptUnavailable(
                key,
            ))?;
    let owner = session.owner();
    if target.owner() != owner {
        return Err(CanonicalLoopCompareI64OperandRejectV1::TargetOwnerMismatch);
    }
    validate_published_receipt(&receipt, key, owner, target.block())?;

    let request = CanonicalSameBlockIntegerRequestV1::from_parts(
        owner,
        target.block(),
        receipt.physical_value(),
    );
    let definition = session
        .prepare_existing_same_block_integer(builder, request)
        .map_err(CanonicalLoopCompareI64OperandRejectV1::Canonical)?;
    Ok(VerifiedCanonicalLoopCompareI64OperandV1 {
        receipt,
        definition,
        _seal: CanonicalLoopCompareI64OperandSealV1,
    })
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
