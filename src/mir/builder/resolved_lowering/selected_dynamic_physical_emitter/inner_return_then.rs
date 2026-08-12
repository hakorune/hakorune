//! Physical I11/inner-return preparation for the selected Dynamic session.
//!
//! This closes the ThenTerminal without emitting a MIR Return. DraftSeal is
//! the sole Return writer; this module only consumes the retained I11 source
//! row, canonical binding read, V10 End cutpoint, and exact Completion claim.

use super::callout_corridor::{require_read, DynamicV2CallOutCorridorV1};
use super::lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1;
use super::targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::builder::resolved_lowering::selected_dynamic_physical_capability::DynamicV2TemporaryDischargeRowV1;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::compiler::dynamic_full_body_recipe::DynamicInvocationCleanupRowKindV1;
use crate::mir::loop_recipe_contract::{LoopItemKeyV1, LoopOperationV2, LoopValueKeyV1};
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::{BasicBlockId, MirInstruction};

const I11: LoopItemKeyV1 = LoopItemKeyV1::new(11);
const V14: LoopValueKeyV1 = LoopValueKeyV1::new(14);

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::InnerReturn(message.into())
}

fn validate_then_predecessor(
    outer: &CanonicalFunctionLoweringSessionV1<'_>,
    corridor: &DynamicV2CallOutCorridorV1,
    then_block: BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let function = outer
        .builder_view()
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| reject("missing function while validating ThenTerminal"))?;
    let block = function
        .get_block(then_block)
        .ok_or_else(|| reject("ThenTerminal block is missing"))?;
    if block.predecessors.len() != 1 {
        return Err(reject("ThenTerminal must have one predecessor"));
    }
    let source = *block
        .predecessors
        .iter()
        .next()
        .expect("one ThenTerminal predecessor was checked");
    corridor.with_i7_normal(|normal| {
        match function
            .get_block(source)
            .and_then(|source| source.terminator.as_ref())
        {
            Some(MirInstruction::Branch { then_bb, .. })
                if source == normal.block() && *then_bb == then_block =>
            {
                Ok(())
            }
            _ => Err(reject("ThenTerminal predecessor is not the I9 branch")),
        }
    })
}

fn inner_site(
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    cleanup: &[DynamicV2TemporaryDischargeRowV1; 4],
) -> Result<crate::mir::resolved_semantics::SourceStmtSiteV1, DynamicV2I8EmitterRejectV1> {
    let site = cleanup
        .iter()
        .find(|row| row.kind() == DynamicInvocationCleanupRowKindV1::InnerReturn)
        .and_then(|row| row.inner_return_site())
        .cloned()
        .ok_or_else(|| reject("InnerReturn cleanup site is missing"))?;
    let expected = demand
        .source_relation()
        .completion_sites()
        .first()
        .copied()
        .ok_or_else(|| reject("inner Completion site is missing"))?;
    if &site != expected {
        return Err(reject("InnerReturn cleanup/site relation drift"));
    }
    Ok(site)
}

fn emit_program(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    program: &crate::mir::compiler::dynamic_full_body_recipe::PreparedDynamicLoopOperationProgramV2<
        '_,
    >,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    cleanup: &[DynamicV2TemporaryDischargeRowV1; 4],
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let rows = program.operation_rows();
    if rows.len() != 15 {
        return Err(reject("I11 requires exactly 15 operation rows"));
    }
    let relation = demand.source_relation();
    let i11 = rows
        .iter()
        .find(|row| row.item() == I11)
        .ok_or_else(|| reject("I11 operation row is missing"))?;
    require_read(i11, V14, relation.induction_key())?;
    let site = inner_site(demand, cleanup)?;
    let then_target =
        targets.with_role(DynamicV2PhysicalTargetRoleV1::ThenTerminal, |target| target);
    if !then_target.matches(brand) {
        return Err(reject("ThenTerminal has a foreign session brand"));
    }
    let then_block = then_target.block();
    validate_then_predecessor(outer, corridor, then_block)?;
    canonical
        .cfg
        .select_block(outer.builder_view_mut_for_lowering(), then_block)
        .map_err(|error| reject(error.to_string()))?;

    canonical
        .identity
        .claim_variable_use_binding(relation.inner_return_i(), relation.induction_binding())
        .map_err(reject)?;
    let receipt = canonical
        .identity
        .read_entry_receipt(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            then_block,
            relation.induction_binding(),
        )
        .map_err(reject)?;
    if receipt.owner() != relation.owner()
        || receipt.binding() != relation.induction_binding()
        || receipt.physical_block() != then_block
    {
        return Err(reject("I11 canonical binding receipt drift"));
    }

    canonical
        .emit_checked_callout_end(
            outer.builder_view_mut_for_lowering(),
            then_block,
            lifecycle.i6_site(),
            lifecycle.lease_slot(),
        )
        .map_err(reject)?;
    canonical
        .completion
        .claim_explicit_return(
            &site,
            demand.input().function().function_region(),
            then_block,
            receipt.physical_value(),
        )
        .map_err(reject)?;
    canonical
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(site))
        .map_err(reject)?;

    let witness = {
        let function = outer
            .builder_view_mut_for_lowering()
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| reject("missing function while sealing ThenTerminal"))?;
        canonical
            .cfg
            .seal_block(function, then_block)
            .map_err(|error| reject(error.to_string()))?
    };
    canonical
        .identity
        .seal_block(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            then_block,
            &witness,
        )
        .map_err(reject)
}

pub(super) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    cleanup: &[DynamicV2TemporaryDischargeRowV1; 4],
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    demand.with_operation_program(|program| {
        emit_program(
            canonical, outer, program, demand, targets, corridor, lifecycle, cleanup, brand,
        )
    })
}
