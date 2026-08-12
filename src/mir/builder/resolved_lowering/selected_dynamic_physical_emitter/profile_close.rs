//! Final unpublished Dynamic profile close before the common DraftSeal.
//!
//! This leaf consumes the retained source/Completion relation and the blocks
//! already issued by the physical corridor. It does not create a Return, a
//! second CFG, or a new Completion authority; DraftSeal remains the sole
//! Return writer.

use super::callout_corridor::DynamicV2CallOutCorridorV1;
use super::lifecycle_terminal::DynamicV2PhysicalLifecycleTerminalPlanV1;
use super::targets::{DynamicV2PhysicalTargetRoleV1, DynamicV2PhysicalTargetSetV1};
use super::{DynamicV2I8EmitterRejectV1, DynamicV2PhysicalSessionBrandV1};
use crate::mir::builder::calls::CanonicalFunctionLoweringSessionV1;
use crate::mir::builder::resolved_lowering::canonical_ssa::CanonicalSsaFunctionSessionV2;
use crate::mir::compiler::a_prime_i64_physical_capability::VerifiedAPrimeI64PhysicalDemandV1;
use crate::mir::resolved_semantics::ResolvedExitSiteV1;
use crate::mir::{BasicBlockId, MirInstruction, MirType};

#[derive(Debug, Clone, Copy)]
pub(super) struct DynamicV2PhysicalProfileCloseV1 {
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    terminal: BasicBlockId,
}

impl DynamicV2PhysicalProfileCloseV1 {
    pub(super) fn finish(
        self,
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        terminal: BasicBlockId,
    ) -> Result<(), String> {
        if self.owner != owner || self.terminal != terminal {
            return Err("Dynamic profile close owner/terminal mismatch".to_owned());
        }
        Ok(())
    }
}

fn reject(message: impl Into<String>) -> DynamicV2I8EmitterRejectV1 {
    DynamicV2I8EmitterRejectV1::ProfileClose(message.into())
}

fn role_block(
    targets: &DynamicV2PhysicalTargetSetV1,
    role: DynamicV2PhysicalTargetRoleV1,
) -> BasicBlockId {
    targets.with_role(role, |target| target.block())
}

fn exact_predecessor(
    function: &crate::mir::MirFunction,
    block: BasicBlockId,
    expected: BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let target = function
        .get_block(block)
        .ok_or_else(|| reject(format!("profile block missing: {block:?}")))?;
    if target.predecessors.len() != 1 || !target.predecessors.contains(&expected) {
        return Err(reject(format!(
            "profile predecessor drift block={block:?} expected={expected:?}"
        )));
    }
    Ok(())
}

fn seal_block(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    block: BasicBlockId,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let witness = {
        let function = outer
            .builder_view_mut_for_lowering()
            .function_state
            .current_function
            .as_mut()
            .ok_or_else(|| reject("profile close function is missing"))?;
        canonical
            .cfg
            .seal_block(function, block)
            .map_err(|error| reject(error.to_string()))?
    };
    canonical
        .identity
        .seal_block(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            block,
            &witness,
        )
        .map_err(reject)
}

fn validate_corridor(
    outer: &CanonicalFunctionLoweringSessionV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<(), DynamicV2I8EmitterRejectV1> {
    let header = role_block(targets, DynamicV2PhysicalTargetRoleV1::Header);
    let body = role_block(targets, DynamicV2PhysicalTargetRoleV1::BodyPrelude);
    let then_block = role_block(targets, DynamicV2PhysicalTargetRoleV1::ThenTerminal);
    let continuation = role_block(targets, DynamicV2PhysicalTargetRoleV1::Continuation);
    let after = role_block(targets, DynamicV2PhysicalTargetRoleV1::After);
    let (i6_normal, i6_fault, i7_normal, i7_fault) = corridor.with_i6_normal(|i6_normal| {
        corridor.with_i6_fault(|i6_fault| {
            corridor.with_i7_normal(|i7_normal| {
                corridor.with_i7_fault(|i7_fault| {
                    (
                        i6_normal.block(),
                        i6_fault.block(),
                        i7_normal.block(),
                        i7_fault.block(),
                    )
                })
            })
        })
    });
    if ![i6_normal, i6_fault, i7_normal, i7_fault].iter().all(|_| {
        corridor.with_i6_normal(|target| target.matches(brand))
            && corridor.with_i6_fault(|target| target.matches(brand))
            && corridor.with_i7_normal(|target| target.matches(brand))
            && corridor.with_i7_fault(|target| target.matches(brand))
    }) {
        return Err(reject("profile close has a foreign CallOut landing"));
    }
    let function = outer
        .builder_view()
        .function_state
        .current_function
        .as_ref()
        .ok_or_else(|| reject("profile close function is missing"))?;
    exact_predecessor(function, body, header)?;
    exact_predecessor(function, i6_normal, body)?;
    exact_predecessor(function, i6_fault, body)?;
    exact_predecessor(function, i7_normal, i6_normal)?;
    exact_predecessor(function, i7_fault, i6_normal)?;
    exact_predecessor(function, after, header)?;

    let body_term = function
        .get_block(body)
        .and_then(|block| block.terminator.as_ref());
    if !matches!(
        body_term,
        Some(MirInstruction::CheckedCallOut {
            site_id,
            normal_landing,
            fault_landing,
            ..
        }) if site_id.0 == 0 && *normal_landing == i6_normal && *fault_landing == i6_fault
    ) {
        return Err(reject("I6 CheckedCallOut topology drift"));
    }
    let i6_term = function
        .get_block(i6_normal)
        .and_then(|block| block.terminator.as_ref());
    if !matches!(
        i6_term,
        Some(MirInstruction::CheckedCallOut {
            site_id,
            normal_landing,
            fault_landing,
            ..
        }) if site_id.0 == 1 && *normal_landing == i7_normal && *fault_landing == i7_fault
    ) {
        return Err(reject("I7 CheckedCallOut topology drift"));
    }
    if !matches!(
        function.get_block(i6_fault).and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::CheckedCallOutFault { site_id }) if site_id.0 == 0
    ) || !matches!(
        function.get_block(i7_fault).and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::CheckedCallOutFault { site_id }) if site_id.0 == 1
    ) {
        return Err(reject("CallOut Fault topology drift"));
    }
    if !matches!(
        function.get_block(i7_normal).and_then(|block| block.terminator.as_ref()),
        Some(MirInstruction::Branch { then_bb, else_bb, .. })
            if *then_bb == then_block && *else_bb == continuation
    ) || function
        .get_block(after)
        .and_then(|block| block.terminator.as_ref())
        .is_some()
    {
        return Err(reject("profile normal corridor topology drift"));
    }

    let mut ends = 0usize;
    let mut projections = [0usize; 2];
    for block in function.blocks.values() {
        for instruction in &block.instructions {
            match instruction {
                MirInstruction::CheckedCallOutEnd {
                    site_id,
                    lease_slot,
                } if site_id.0 == 0 && lease_slot.0 == 0 => ends += 1,
                MirInstruction::CheckedCallOutEnd { .. } => {
                    return Err(reject("foreign CheckedCallOut End in profile"))
                }
                MirInstruction::CheckedCallOutNormalResult { site_id, .. } if site_id.0 < 2 => {
                    projections[site_id.0 as usize] += 1
                }
                _ => {}
            }
        }
    }
    if ends != 3 || projections != [1, 1] {
        return Err(reject(format!(
            "profile lifecycle census drift ends={ends} projections={projections:?}"
        )));
    }
    Ok(())
}

pub(super) fn emit(
    canonical: &mut CanonicalSsaFunctionSessionV2<'_>,
    outer: &mut CanonicalFunctionLoweringSessionV1<'_>,
    demand: &VerifiedAPrimeI64PhysicalDemandV1<'_>,
    targets: &DynamicV2PhysicalTargetSetV1,
    corridor: &DynamicV2CallOutCorridorV1,
    lifecycle: &DynamicV2PhysicalLifecycleTerminalPlanV1,
    brand: &DynamicV2PhysicalSessionBrandV1,
) -> Result<DynamicV2PhysicalProfileCloseV1, DynamicV2I8EmitterRejectV1> {
    if lifecycle.end_cutpoints().len() != 3 || !corridor.matches(brand) {
        return Err(reject("profile lifecycle/corridor brand mismatch"));
    }
    validate_corridor(outer, targets, corridor, brand)?;
    let after = role_block(targets, DynamicV2PhysicalTargetRoleV1::After);
    canonical
        .cfg
        .select_block(outer.builder_view_mut_for_lowering(), after)
        .map_err(|error| reject(error.to_string()))?;
    let relation = demand.source_relation();
    let outer_site = relation
        .completion_sites()
        .get(1)
        .copied()
        .ok_or_else(|| reject("outer Completion site is missing"))?;
    canonical
        .identity
        .claim_variable_use_binding(relation.outer_return_i(), relation.induction_binding())
        .map_err(reject)?;
    let receipt = canonical
        .identity
        .read_entry_receipt(
            outer.builder_view_mut_for_lowering(),
            &mut canonical.phis,
            after,
            relation.induction_binding(),
        )
        .map_err(reject)?;
    if receipt.owner() != relation.owner()
        || receipt.binding() != relation.induction_binding()
        || receipt.physical_block() != after
    {
        return Err(reject("outer Completion binding receipt drift"));
    }
    canonical
        .publish_physical_value_type(
            outer.builder_view_mut_for_lowering(),
            receipt.physical_value(),
            MirType::Integer,
        )
        .map_err(reject)?;
    canonical
        .completion
        .claim_explicit_return(
            outer_site,
            demand.input().function().function_region(),
            after,
            receipt.physical_value(),
        )
        .map_err(reject)?;
    canonical
        .identity
        .mark_return(ResolvedExitSiteV1::Statement(outer_site.clone()))
        .map_err(reject)?;

    // DraftSeal needs an open, site-keyed exit block. It writes the two Return
    // instructions only on its detached projection, never in this live CFG.
    seal_block(canonical, outer, after)?;
    let body = role_block(targets, DynamicV2PhysicalTargetRoleV1::BodyPrelude);
    let (i6_normal, i6_fault, i7_normal, i7_fault) = corridor.with_i6_normal(|i6_normal| {
        corridor.with_i6_fault(|i6_fault| {
            corridor.with_i7_normal(|i7_normal| {
                corridor.with_i7_fault(|i7_fault| {
                    (
                        i6_normal.block(),
                        i6_fault.block(),
                        i7_normal.block(),
                        i7_fault.block(),
                    )
                })
            })
        })
    });
    for block in [body, i6_normal, i6_fault, i7_normal, i7_fault] {
        seal_block(canonical, outer, block)?;
    }
    Ok(DynamicV2PhysicalProfileCloseV1 {
        owner: demand.identity().owner(),
        terminal: after,
    })
}
