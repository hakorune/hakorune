//! Physical continuation and loop backedge for the selected Dynamic cohort.
//!
//! This leaf consumes the already verified I13-I16 rows and the retained
//! induction relation.  Canonical SSA/CFG/PhiTxn remain the only issuers of
//! values, assignment definitions, edges, and Header PHI inputs.

use super::super::selected_dynamic_physical_capability::DynamicV2PhysicalRepresentationV1;
use super::callout_corridor::{
    require_add, require_const, require_read, DynamicV2CallOutCorridorV1,
};
use super::formal_header::DynamicV2OpenedFormalHeaderV1;
use super::lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1;
use super::targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};
use super::value_ledger::DynamicV2PhysicalValueLedgerV1;
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::emission::{constant, loop_operation};
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::PreparedDynamicLoopOperationProgramV2;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopOperationV2, LoopValueKeyV1};
use crate::mir::MirInstruction;

const I13: LoopItemKeyV1 = LoopItemKeyV1::new(13);
const I14: LoopItemKeyV1 = LoopItemKeyV1::new(14);
const I15: LoopItemKeyV1 = LoopItemKeyV1::new(15);
const I16: LoopItemKeyV1 = LoopItemKeyV1::new(16);
const V15: LoopValueKeyV1 = LoopValueKeyV1::new(15);
const V16: LoopValueKeyV1 = LoopValueKeyV1::new(16);
const V17: LoopValueKeyV1 = LoopValueKeyV1::new(17);

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::PhysicalCorridor(message.into())
}

fn row<'a>(
    program: &'a PreparedDynamicLoopOperationProgramV2<'a>,
    item: LoopItemKeyV1,
) -> Result<
    &'a crate::mir::compiler::dynamic_full_body_recipe::DynamicFullLoopOperationPhysicalRefV2<'a>,
    DynamicV2I8EmitterRejectV1,
> {
    program
        .operation_rows()
        .iter()
        .find(|candidate| candidate.item() == item)
        .ok_or_else(|| reject(format!("missing continuation operation {item:?}")))
}

fn validate_continuation_predecessor(
    outer: &CanonicalFunctionLoweringSessionV1<'_>,
    corridor: &DynamicV2CallOutCorridorV1,
    continuation: crate::mir::BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view()
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| reject("missing function while validating Continuation"))?;
    let block = function
        .get_block(continuation)
        .ok_or_else(|| reject("Continuation block is missing"))?;
    if block.predecessors.len() != 1 {
        return Err(reject("Continuation must have one predecessor"));
    }
    let source = *block
        .predecessors
        .iter()
        .next()
        .expect("one Continuation predecessor was checked");
    corridor.with_i7_normal(|normal| {
        match function
            .get_block(source)
            .and_then(|source| source.terminator.as_ref())
        {
            Some(MirInstruction::Branch { else_bb, .. })
                if source == normal.block() && *else_bb == continuation =>
            {
                Ok(())
            }
            _ => Err(reject("Continuation predecessor is not the I9 false arm")),
        }
    })
}

fn emit_program(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: &PreparedDynamicLoopOperationProgramV2<'_>,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    formals: &DynamicV2OpenedFormalHeaderV1,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    values: &mut DynamicV2PhysicalValueLedgerV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    if program.operation_rows().len() != 15 {
        return Err(reject("continuation requires exactly 15 operation rows"));
    }
    if !lifecycle
        .end_cutpoints()
        .contains(&super::lifecycle_terminal::DynamicV2PhysicalEndCutPointV1::Backedge)
    {
        return Err(reject("Backedge End cutpoint is missing"));
    }
    let relation = demand.source_relation();
    let i13 = row(program, I13)?;
    let i14 = row(program, I14)?;
    let i15 = row(program, I15)?;
    let i16 = row(program, I16)?;
    require_read(i13, V15, relation.induction_key())?;
    require_const(i14, V16, 1)?;
    require_add(i15, V15, V16, V17)?;
    match i16.operation() {
        LoopOperationV2::WriteBinding { binding, value }
            if *binding == relation.induction_key() && *value == V17 => {}
        _ => return Err(reject("I16 WriteBinding relation drift")),
    }

    let continuation =
        targets.with_role(DynamicV2PhysicalTargetRoleV1::Continuation, |target| target);
    if !continuation.matches(brand) {
        return Err(reject("Continuation has a foreign session brand"));
    }
    validate_continuation_predecessor(outer, corridor, continuation.block())?;
    canonical
        .cfg
        .select_block(outer.builder_view_mut_for_lowering(), continuation.block())
        .map_err(|error| reject(error.to_string()))?;

    canonical
        .identity
        .claim_variable_use_binding(relation.step_read_i(), relation.induction_binding())
        .map_err(reject)?;
    let current = formals.header_current_value();
    values
        .publish(
            i13.item(),
            V15,
            &continuation,
            current,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;

    let one = canonical
        .issue_physical_value_id(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    constant::emit_integer_at_with_dst(
        outer.builder_view_mut_for_lowering(),
        continuation.block(),
        one,
        1,
    )
    .map_err(reject)?;
    canonical
        .publish_physical_value_type(
            outer.builder_view_mut_for_lowering(),
            one,
            crate::mir::MirType::Integer,
        )
        .map_err(reject)?;
    values
        .publish(
            i14.item(),
            V16,
            &continuation,
            one,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;

    let next = canonical
        .issue_physical_value_id(outer.builder_view_mut_for_lowering())
        .map_err(reject)?;
    loop_operation::emit_add_i64_at_with_dst(
        outer.builder_view_mut_for_lowering(),
        continuation.block(),
        next,
        current,
        one,
    )
    .map_err(reject)?;
    canonical
        .publish_physical_value_type(
            outer.builder_view_mut_for_lowering(),
            next,
            crate::mir::MirType::Integer,
        )
        .map_err(reject)?;
    values
        .publish(
            i15.item(),
            V17,
            &continuation,
            next,
            DynamicV2PhysicalRepresentationV1::ImmediateI64,
        )
        .map_err(|error| reject(format!("physical value ledger: {error:?}")))?;

    canonical
        .identity
        .define_assignment_exact(
            relation.step_target_i(),
            relation.induction_binding(),
            continuation.block(),
            next,
        )
        .map_err(reject)?;
    canonical
        .emit_checked_callout_end(
            outer.builder_view_mut_for_lowering(),
            continuation.block(),
            lifecycle.i6_site(),
            lifecycle.lease_slot(),
        )
        .map_err(reject)?;

    let (enter, header) =
        targets.with_enter_header(|enter, header| (enter.block(), header.block()));
    if !targets.with_role(DynamicV2PhysicalTargetRoleV1::Header, |target| {
        target.matches(brand)
    }) {
        return Err(reject("Header has a foreign session brand"));
    }
    let function = outer
        .builder_view_mut_for_lowering()
        .function_state
        .current_function
        .as_mut()
        .ok_or_else(|| reject("missing function while emitting Backedge"))?;
    canonical
        .cfg
        .emit_jump(function, continuation.block(), header)
        .map_err(|error| reject(error.to_string()))?;

    let (continuation_witness, enter_witness, header_witness) = {
        let function = outer
            .builder_view_mut_for_lowering()
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| reject("missing function while sealing Backedge"))?;
        let continuation_witness = canonical
            .cfg
            .seal_block(function, continuation.block())
            .map_err(|error| reject(error.to_string()))?;
        let enter_witness = canonical
            .cfg
            .seal_block(function, enter)
            .map_err(|error| reject(error.to_string()))?;
        let header_witness = canonical
            .cfg
            .seal_block(function, header)
            .map_err(|error| reject(error.to_string()))?;
        (continuation_witness, enter_witness, header_witness)
    };
    if continuation_witness.predecessors().len() != 1
        || !continuation_witness
            .predecessors()
            .contains(&corridor.with_i7_normal(|target| target.block()))
        || enter_witness.predecessors().len() != 0
        || header_witness.predecessors().len() != 2
        || !header_witness.predecessors().contains(&enter)
        || !header_witness
            .predecessors()
            .contains(&continuation.block())
    {
        return Err(reject("Continuation/Header predecessor census drift"));
    }
    canonical
        .identity
        .seal_block(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            continuation.block(),
            &continuation_witness,
        )
        .map_err(reject)?;
    canonical
        .identity
        .seal_block(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            enter,
            &enter_witness,
        )
        .map_err(reject)?;
    canonical
        .identity
        .seal_block(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            header,
            &header_witness,
        )
        .map_err(reject)
}

pub(super) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    formals: &DynamicV2OpenedFormalHeaderV1,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    values: &mut DynamicV2PhysicalValueLedgerV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    demand.with_operation_program(|program| {
        emit_program(
            canonical, outer, program, demand, formals, targets, corridor, lifecycle, values, brand,
        )
    })
}
