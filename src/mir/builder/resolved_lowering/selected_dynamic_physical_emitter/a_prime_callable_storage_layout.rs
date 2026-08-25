//! Close-time issuer for the selected A-prime callable storage policy.
//!
//! This module only co-seals existing source/Recipe, session, ledger, formal,
//! and physical-receipt facts.  It does not inspect MIR/AST/JSON or infer a
//! layout from a type spelling.

use super::formal_header::DynamicV2OpenedFormalHeaderV1;
use super::value_ledger::DynamicV2PhysicalValueLedgerV1;
use super::DynamicV2PhysicalSessionBrandV1;
use crate::mir::a_prime_i64_physical_receipt::{
    APrimeI64LaneV1, APrimeI64PhysicalReceiptRejectV1, APrimeI64PhysicalReceiptV1,
};
use crate::mir::compiler::a_prime_i64_physical_capability::{
    APrimeI64PhysicalRequirementV1, VerifiedAPrimeI64PhysicalDemandV1,
};
use crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopParameterClassV2;
use crate::mir::policies::a_prime_i64_callable_storage_layout::APrimeI64CallableStorageLayoutV1;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(super) enum APrimeI64CallableStorageLayoutRejectV1 {
    Requirement,
    Owner,
    SessionBrand,
    FormalRelation,
    Receipt(APrimeI64PhysicalReceiptRejectV1),
    ReceiptFormal,
}

/// Issue the sole plain storage-policy row for the exact A-prime cohort.
///
/// The row is intentionally a zero-argument enum: callers cannot construct it
/// from a MIR type, storage class, JSON, or an ABI lane.  The selected close is
/// the only place that can prove the borrowed rows share one session brand.
pub(super) fn issue(
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    formals: &DynamicV2OpenedFormalHeaderV1,
    values: &DynamicV2PhysicalValueLedgerV1,
    receipt: &APrimeI64PhysicalReceiptV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<APrimeI64CallableStorageLayoutV1, APrimeI64CallableStorageLayoutRejectV1> {
    if demand.requirement() != APrimeI64PhysicalRequirementV1::DirectExactI64 {
        return Err(APrimeI64CallableStorageLayoutRejectV1::Requirement);
    }
    let relation = demand.source_relation();
    let owner = demand.identity().owner();
    if owner != relation.owner() || owner != brand.owner() || owner != values.owner() {
        return Err(APrimeI64CallableStorageLayoutRejectV1::Owner);
    }
    if !values.matches_brand(brand) {
        return Err(APrimeI64CallableStorageLayoutRejectV1::SessionBrand);
    }

    let expected_roles = [
        crate::box_callable::provider_admission::DynamicV2AotFormalRoleV1::Src,
        crate::box_callable::provider_admission::DynamicV2AotFormalRoleV1::Pos,
        crate::box_callable::provider_admission::DynamicV2AotFormalRoleV1::End,
        crate::box_callable::provider_admission::DynamicV2AotFormalRoleV1::PredChars,
    ];
    let relation_rows = relation.formal_rows();
    let opened_rows = formals.transport_rows();
    for (index, row) in relation_rows.iter().copied().enumerate() {
        let expected_class = if index == 1 || index == 2 {
            DynamicFullLoopParameterClassV2::I64
        } else {
            DynamicFullLoopParameterClassV2::Dynamic
        };
        let Some(opened_value) = formals.value_for_recipe(row.recipe_value()) else {
            return Err(APrimeI64CallableStorageLayoutRejectV1::FormalRelation);
        };
        if row.ordinal() != index as u32
            || row.binding().owner() != owner
            || row.class() != expected_class
            || opened_rows[index].role() != expected_roles[index]
            || opened_rows[index].value_id() != opened_value
            || (index == 1 || index == 2)
                && opened_rows[index].lane() != APrimeI64LaneV1::ImmediateI64
        {
            return Err(APrimeI64CallableStorageLayoutRejectV1::FormalRelation);
        }
    }

    receipt
        .validate()
        .map_err(APrimeI64CallableStorageLayoutRejectV1::Receipt)?;
    for row in receipt.parameters() {
        let index = row.formal_parameter_index;
        if index > 3
            || row.value_id != opened_rows[index].value_id()
            || row.lane != APrimeI64LaneV1::ImmediateI64
        {
            return Err(APrimeI64CallableStorageLayoutRejectV1::ReceiptFormal);
        }
    }
    if receipt
        .returns()
        .iter()
        .any(|row| row.lane != APrimeI64LaneV1::ImmediateI64)
    {
        return Err(APrimeI64CallableStorageLayoutRejectV1::ReceiptFormal);
    }

    Ok(APrimeI64CallableStorageLayoutV1::NonAddressableSsaI64)
}
