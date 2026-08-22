//! The first post-CallOut physical continuation for the selected Dynamic lane.
//!
//! I8 and I9 remain ordinary Recipe operations.  Their placement is derived
//! from the I7 Normal landing, not from the logical `BodyPrelude` label, which
//! is already terminated by the I6 CheckedCallOut.  This module only projects
//! existing rows and uses the canonical SSA/CFG issuers.

use super::callout_corridor::{require_compare, require_const, DynamicV2CallOutCorridorV1};
use super::i64_const;
use super::operation_cursor::DynamicV2PhysicalOperationCensusV1;
use super::targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};
use super::value_ledger::DynamicV2PhysicalValueLedgerV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::emission::compare_type::PreparedCanonicalCompareBoolTypeV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::{
    CanonicalSameBlockIntegerRequestV1, CanonicalSsaFunctionSessionV2,
};
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::DynamicV2I8EvidenceV1;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::{
    DynamicV2CompareI64CapabilityDemandV1, DynamicV2PhysicalRepresentationV1,
    DynamicV2ProducerFamilyV1,
};
use crate::mir::builder::resolved_lowering::CanonicalLoopCompareI64WriterV1;
use crate::mir::compiler::dynamic_full_body_recipe::PreparedDynamicLoopOperationProgramV2;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopValueKeyV1};
use crate::mir::{BasicBlockId, CompareOp, MirInstruction};

const I8: LoopItemKeyV1 = LoopItemKeyV1::new(8);
const I9: LoopItemKeyV1 = LoopItemKeyV1::new(9);
const V11: LoopValueKeyV1 = LoopValueKeyV1::new(11);
const V12: LoopValueKeyV1 = LoopValueKeyV1::new(12);
const V13: LoopValueKeyV1 = LoopValueKeyV1::new(13);

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::PhysicalCorridor(message.into())
}

fn value_at(
    values: &DynamicV2PhysicalValueLedgerV1,
    producer: LoopItemKeyV1,
    result: LoopValueKeyV1,
    block: BasicBlockId,
) -> Result<crate::mir::ValueId, DynamicV2I8EmitterRejectV1> {
    values
        .with_value(
            result,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
            |view| {
                if view.producer() == producer && view.result() == result && view.block() == block {
                    Ok(view.value())
                } else {
                    Err(reject(format!(
                        "value {result:?} has foreign producer, result, or I7 Normal landing"
                    )))
                }
            },
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?
}

fn validate_i7_normal_predecessor(
    outer: &CanonicalFunctionLoweringSessionV1<'_>,
    corridor: &DynamicV2CallOutCorridorV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    corridor.with_i7_normal(|normal| {
        let function = outer
            .builder_view()
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(|| reject("missing function while validating I7 Normal landing"))?;
        let landing = function
            .get_block(normal.block())
            .ok_or_else(|| reject("I7 Normal landing block is missing"))?;
        if landing.predecessors.len() != 1 {
            return Err(reject("I7 Normal landing must have one predecessor"));
        }
        let source = landing
            .predecessors
            .iter()
            .next()
            .copied()
            .expect("one predecessor was checked");
        match function
            .get_block(source)
            .and_then(|block| block.terminator.as_ref())
        {
            Some(MirInstruction::CheckedCallOut {
                site_id,
                normal_landing,
                ..
            }) if *site_id == corridor.i7_site() && *normal_landing == normal.block() => Ok(()),
            _ => Err(reject(
                "I7 Normal landing predecessor is not the admitted CallOut",
            )),
        }
    })
}

fn emit_branch(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    source: BasicBlockId,
    condition: crate::mir::ValueId,
    targets: &DynamicV2PhysicalTargetSetV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting I9 branch"))?;
    targets.with_role(DynamicV2PhysicalTargetRoleV1::ThenTerminal, |then_target| {
        targets.with_role(
            DynamicV2PhysicalTargetRoleV1::Continuation,
            |continuation_target| {
                canonical
                    .cfg
                    .emit_branch(
                        function,
                        source,
                        condition,
                        then_target.block(),
                        continuation_target.block(),
                    )
                    .map_err(|error| reject(error.to_string()))
            },
        )
    })
}

struct SelectedDynamicI9CompareHandoffIssuerV1;

impl SelectedDynamicI9CompareHandoffIssuerV1 {
    fn issue(
        canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
        outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
        program: &PreparedDynamicLoopOperationProgramV2<'_>,
        corridor: &DynamicV2CallOutCorridorV1,
        targets: &DynamicV2PhysicalTargetSetV1,
        values: &mut DynamicV2PhysicalValueLedgerV1,
        brand: &DynamicV2PhysicalSessionBrandV1,
        evidence: DynamicV2I8EvidenceV1,
        compare_i64: DynamicV2CompareI64CapabilityDemandV1,
        operation_census: &mut DynamicV2PhysicalOperationCensusV1,
    ) -> Result<(), DynamicV2I8EmitterRejectV1> {
        if !corridor.matches(brand) {
            return Err(reject("I7 corridor has a foreign session brand"));
        }
        if evidence.item() != I8
        || evidence.result() != V12
        || evidence.literal() != 0
        || evidence.target()
            != crate::mir::builder::resolved_lowering::selected_dynamic_physical_abi::
                DynamicV2PhysicalBlockTargetV1::BodyPrelude
    {
        return Err(reject("I8 evidence drift"));
    }
        let rows = program.operation_rows();
        if rows.len() != 15 {
            return Err(reject(
                "I8/I9 continuation requires exactly 15 operation rows",
            ));
        }
        if compare_i64.item() != I9
            || compare_i64.left() != V11
            || compare_i64.right() != V12
            || compare_i64.result() != V13
            || compare_i64.v11().producer().raw() != 7
            || compare_i64.v11().result() != V11
            || compare_i64.v11().family() != DynamicV2ProducerFamilyV1::DynamicCallSlot
            || compare_i64.v11().representation() != DynamicV2PhysicalRepresentationV1::ImmediateI64
            || compare_i64.v12().producer().raw() != 8
            || compare_i64.v12().result() != V12
            || compare_i64.v12().family() != DynamicV2ProducerFamilyV1::ConstI64
            || compare_i64.v12().representation() != DynamicV2PhysicalRepresentationV1::ImmediateI64
        {
            return Err(reject("I9 compare demand was not consumed by its emitter"));
        }
        require_const(&rows[8], V12, 0)?;
        require_compare(&rows[9], V11, V12, V13)?;
        validate_i7_normal_predecessor(outer, corridor)?;
        let owner = canonical.owner();
        if brand.owner() != owner || values.owner() != owner || !values.matches_brand(brand) {
            return Err(reject(
                "I9 Dynamic brand and canonical owner are inconsistent",
            ));
        }

        // Consume the existing physical census before the first I8 ValueId or
        // instruction effect.  The outer unpublished session is the only
        // recovery boundary; a failed claim is terminal and never retried.
        operation_census
            .claim_operation(I8)
            .map_err(|error| reject(format!("I8 physical operation claim: {error:?}")))?;
        operation_census
            .claim_operation(I9)
            .map_err(|error| reject(format!("I9 physical operation claim: {error:?}")))?;
        operation_census
            .claim_if()
            .map_err(|error| reject(format!("If physical claim: {error:?}")))?;

        corridor.with_i7_normal(|normal| {
            let block = normal.block();
            let value_v12 = canonical
                .issue_physical_value_id(outer.builder_view_mut_for_lowering())
                .map_err(reject)?;
            let _receipt = i64_const::emit_with_dst(
                outer.builder_view_mut_for_lowering(),
                normal,
                evidence,
                brand,
                values,
                value_v12,
            )?;
            let value_v11 = value_at(values, compare_i64.v11().producer(), V11, block)?;
            let value_v12 = value_at(values, compare_i64.v12().producer(), V12, block)?;
            let lhs = canonical
                .prepare_existing_same_block_integer(
                    outer.builder_view_mut_for_lowering(),
                    CanonicalSameBlockIntegerRequestV1::from_parts(owner, block, value_v11),
                )
                .map_err(|error| reject(format!("I9 lhs canonical witness: {error:?}")))?;
            let rhs = canonical
                .prepare_existing_same_block_integer(
                    outer.builder_view_mut_for_lowering(),
                    CanonicalSameBlockIntegerRequestV1::from_parts(owner, block, value_v12),
                )
                .map_err(|error| reject(format!("I9 rhs canonical witness: {error:?}")))?;
            let destination = canonical
                .reserve_compare_destination(outer.builder_view_mut_for_lowering())
                .map_err(reject)?;
            let bool_plan = PreparedCanonicalCompareBoolTypeV1::prepare(
                outer
                    .builder_view_mut_for_lowering()
                    .function_state
                    .type_ctx
                    .get_type(destination.value()),
            )
            .map_err(|error| reject(format!("I9 Bool plan: {error:?}")))?;
            let pending = values
                .reserve_result(
                    I9,
                    V13,
                    normal,
                    destination.value(),
                    DynamicV2PhysicalRepresentationV1::ImmediateBool,
                )
                .map_err(|error| reject(format!("I9 result reservation: {error:?}")))?;
            let target = lhs.target();
            let definition = CanonicalLoopCompareI64WriterV1::emit(
                outer.builder_view_mut_for_lowering(),
                target,
                lhs,
                rhs,
                destination,
                CompareOp::Lt,
                bool_plan,
            )
            .map_err(|error| reject(format!("I9 strict Compare writer: {error:?}")))?;
            let _published = pending.commit(&definition);
            emit_branch(
                canonical,
                outer,
                block,
                definition.physical_value(),
                targets,
            )
        })
    }
}

pub(super) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
    corridor: &DynamicV2CallOutCorridorV1,
    targets: &DynamicV2PhysicalTargetSetV1,
    values: &mut DynamicV2PhysicalValueLedgerV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
    evidence: DynamicV2I8EvidenceV1,
    compare_i64: DynamicV2CompareI64CapabilityDemandV1,
    operation_census: &mut DynamicV2PhysicalOperationCensusV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    SelectedDynamicI9CompareHandoffIssuerV1::issue(
        canonical,
        outer,
        program,
        corridor,
        targets,
        values,
        brand,
        evidence,
        compare_i64,
        operation_census,
    )
}
