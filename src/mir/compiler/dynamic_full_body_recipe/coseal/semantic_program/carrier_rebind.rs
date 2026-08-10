//! Commit-before-end semantic relation for one Dynamic carrier replacement.
//!
//! This module only seals the relation. It does not execute a replacement,
//! create a cleanup token, mutate Binding SSA, or issue a physical end.

use crate::mir::compiler::dynamic_full_body_recipe::claims::DynamicFullLoopClaimTargetV2;
use crate::mir::compiler::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1, DynamicFullBodySourceSiteV1,
};
use crate::mir::dynamic_operator_contract::{
    DynamicOperatorFaultV1, DynamicOperatorNormalResultV1,
};
use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopItemKeyV1, LoopOperationV2, LoopRecipeItemV2, LoopValueClassV2,
    LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, SourceExprSiteV1, SourceStmtSiteV1};

use super::operator_carrier_lifecycle::DynamicOperatorCarrierDestinationRefV1;
use super::{DynamicFullLoopFaultFamilyV2, VerifiedDynamicCarrierIngressLifecycleProgramV1};

#[path = "carrier_flow.rs"]
mod carrier_flow;

pub(in crate::mir) use carrier_flow::{
    issue_dynamic_carrier_flow_program_v1, DynamicCarrierFlowProgramRejectV1,
    VerifiedDynamicCarrierFlowProgramV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierRebindTransactionRejectV1 {
    Ingress,
    ReadBinding,
    OperatorLifecycle,
    FaultRelation,
    CommitRelation,
    BackedgeRelation,
}

/// The only current disposition issued by this first ingress cohort.
///
/// The owned variant is vocabulary for the later carrier-flow row; it is not
/// manufactured by this issuer and carries no physical owner or cleanup.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierCurrentDispositionV1 {
    BorrowedIngressNoEnd,
    OwnedCarrierEndExactlyOnceUnlessForwarded,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicCarrierRebindReadRelationV1 {
    pub(in crate::mir) item: LoopItemKeyV1,
    pub(in crate::mir) binding: LoopBindingKeyV1,
    pub(in crate::mir) result: LoopValueKeyV1,
    pub(in crate::mir) source: SourceExprSiteV1,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir) struct DynamicCarrierRebindCommitRelationV1 {
    pub(in crate::mir) write: LoopItemKeyV1,
    pub(in crate::mir) result: LoopValueKeyV1,
    pub(in crate::mir) binding: LoopBindingKeyV1,
    pub(in crate::mir) source_binding: BindingRefV1,
    pub(in crate::mir) assignment_source: SourceStmtSiteV1,
    pub(in crate::mir) target_source: SourceExprSiteV1,
    pub(in crate::mir) backedge_loop: crate::mir::loop_recipe_contract::LoopNodeKeyV1,
}

/// One non-splittable semantic state-machine contract.
///
/// `ingress` remains the whole verified upstream product. The owned relation
/// rows are a projection of that product, never an independently constructible
/// rebind plan.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicCarrierRebindTransactionProgramV1 {
    ingress: VerifiedDynamicCarrierIngressLifecycleProgramV1,
    current: DynamicCarrierCurrentDispositionV1,
    read: DynamicCarrierRebindReadRelationV1,
    commit: DynamicCarrierRebindCommitRelationV1,
    fault: DynamicOperatorFaultV1,
}

impl VerifiedDynamicCarrierRebindTransactionProgramV1 {
    pub(in crate::mir) fn current(&self) -> DynamicCarrierCurrentDispositionV1 {
        self.current
    }

    #[cfg(test)]
    pub(in crate::mir) fn read(&self) -> &DynamicCarrierRebindReadRelationV1 {
        &self.read
    }

    #[cfg(test)]
    pub(in crate::mir) fn commit(&self) -> &DynamicCarrierRebindCommitRelationV1 {
        &self.commit
    }

    #[cfg(test)]
    pub(in crate::mir) fn fault(&self) -> DynamicOperatorFaultV1 {
        self.fault
    }
}

pub(in crate::mir) fn issue_dynamic_carrier_rebind_transaction_program_v1(
    ingress: VerifiedDynamicCarrierIngressLifecycleProgramV1,
) -> Result<VerifiedDynamicCarrierRebindTransactionProgramV1, DynamicCarrierRebindTransactionRejectV1>
{
    let ingress_relation = ingress.relation();
    let operator = ingress.program();
    let semantic = &operator.invocation_program.program;
    let recipe = semantic.envelope.artifact.recipe().as_recipe();
    let Some(carrier_row) = recipe
        .carriers
        .iter()
        .find(|row| row.key == ingress_relation.carrier)
    else {
        return Err(DynamicCarrierRebindTransactionRejectV1::Ingress);
    };
    if !recipe.inputs.contains(&ingress_relation.entry)
        || carrier_row.owner_loop != ingress_relation.root
        || carrier_row.binding != ingress_relation.recipe_binding
        || carrier_row.entry_value != ingress_relation.entry
        || carrier_row.class != LoopValueClassV2::Dynamic
        || ingress_relation.parameter_binding != ingress_relation.source_binding
    {
        return Err(DynamicCarrierRebindTransactionRejectV1::Ingress);
    }
    let read_item = source_item(semantic, DynamicFullBodySourceRoleV1::StepReadI)?;
    let read_source = source_expr(semantic, DynamicFullBodySourceRoleV1::StepReadI)?;
    let (read_binding, read_result) = read_binding(recipe, read_item)?;
    let induction_binding = loop_binding(semantic, DynamicFullBodyBindingRoleV1::Induction)?;
    if ingress_relation.parameter_ordinal != 1
        || ingress_relation.demand != crate::mir::resolved_semantics::HomeDemandV1::Handle
        || read_binding != induction_binding
        || read_binding != ingress_relation.recipe_binding
        || value_class(recipe, read_result)? != LoopValueClassV2::Dynamic
    {
        return Err(DynamicCarrierRebindTransactionRejectV1::ReadBinding);
    }

    let step_add = source_item(semantic, DynamicFullBodySourceRoleV1::StepAdd)?;
    let (left, right, next_result) = dynamic_add(recipe, step_add)?;
    if left != read_result || value_class(recipe, right)? != LoopValueClassV2::I64 {
        return Err(DynamicCarrierRebindTransactionRejectV1::CommitRelation);
    }
    let write = source_item(semantic, DynamicFullBodySourceRoleV1::StepAssignment)?;
    let assignment_source = source_stmt(semantic, DynamicFullBodySourceRoleV1::StepAssignment)?;
    let target_source = source_expr(semantic, DynamicFullBodySourceRoleV1::StepTargetI)?;
    let (write_binding, write_value) = write_binding(recipe, write)?;
    if write_value != next_result || write_binding != induction_binding {
        return Err(DynamicCarrierRebindTransactionRejectV1::CommitRelation);
    }

    let lifecycle_rows = operator.operator_lifecycle().rows().collect::<Vec<_>>();
    if lifecycle_rows.len() != 2 {
        return Err(DynamicCarrierRebindTransactionRejectV1::OperatorLifecycle);
    }
    let Some(forward) = lifecycle_rows.iter().find(|row| {
        matches!(
            row.destination(),
            DynamicOperatorCarrierDestinationRefV1::ForwardToBindingAtRebindCommit { write: row_write, .. }
                if row_write == write
        )
    }) else {
        return Err(DynamicCarrierRebindTransactionRejectV1::OperatorLifecycle);
    };
    if forward.producer() != step_add
        || forward.result() != next_result
        || forward.contract().normal_result()
            != DynamicOperatorNormalResultV1::SelfContainedNonAliasingDynamicCarrier
        || forward.contract().fault()
            != DynamicOperatorFaultV1::TypeErrorBeforeResultNoOperandMutationNoRebind
    {
        return Err(DynamicCarrierRebindTransactionRejectV1::FaultRelation);
    }
    let fault_rows = semantic.fault_cut_points();
    if fault_rows
        .rows()
        .iter()
        .filter(|row| {
            row.item() == step_add
                && row.normal_result() == next_result
                && row.family() == DynamicFullLoopFaultFamilyV2::DynamicAdd
        })
        .count()
        != 1
    {
        return Err(DynamicCarrierRebindTransactionRejectV1::FaultRelation);
    }
    if operator.after().loop_key() != ingress_relation.root
        || operator.after().binding() != induction_binding
        || operator.after().class() != LoopValueClassV2::Dynamic
    {
        return Err(DynamicCarrierRebindTransactionRejectV1::BackedgeRelation);
    }

    let source_binding = ingress_relation.source_binding;
    let backedge_loop = operator.after().loop_key();
    let fault = forward.contract().fault();
    Ok(VerifiedDynamicCarrierRebindTransactionProgramV1 {
        ingress,
        current: DynamicCarrierCurrentDispositionV1::BorrowedIngressNoEnd,
        read: DynamicCarrierRebindReadRelationV1 {
            item: read_item,
            binding: read_binding,
            result: read_result,
            source: read_source.clone(),
        },
        commit: DynamicCarrierRebindCommitRelationV1 {
            write,
            result: next_result,
            binding: write_binding,
            source_binding,
            assignment_source: assignment_source.clone(),
            target_source: target_source.clone(),
            backedge_loop,
        },
        fault,
    })
}

fn source_item(
    semantic: &super::VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodySourceRoleV1,
) -> Result<LoopItemKeyV1, DynamicCarrierRebindTransactionRejectV1> {
    match semantic.envelope.coverage.source_target(role) {
        Some(DynamicFullLoopClaimTargetV2::Item(item)) => Ok(item),
        _ => Err(DynamicCarrierRebindTransactionRejectV1::ReadBinding),
    }
}

fn loop_binding(
    semantic: &super::VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodyBindingRoleV1,
) -> Result<LoopBindingKeyV1, DynamicCarrierRebindTransactionRejectV1> {
    match semantic.envelope.coverage.binding_target(role) {
        Some(DynamicFullLoopClaimTargetV2::Binding(binding)) => Ok(binding),
        _ => Err(DynamicCarrierRebindTransactionRejectV1::CommitRelation),
    }
}

fn source_expr(
    semantic: &super::VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodySourceRoleV1,
) -> Result<SourceExprSiteV1, DynamicCarrierRebindTransactionRejectV1> {
    semantic
        .envelope
        .source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Expression(site) => Some(site.clone()),
                DynamicFullBodySourceSiteV1::Statement(_) => None,
            })?
        })
        .ok_or(DynamicCarrierRebindTransactionRejectV1::ReadBinding)
}

fn source_stmt(
    semantic: &super::VerifiedDynamicFullLoopSemanticProgramV2,
    role: DynamicFullBodySourceRoleV1,
) -> Result<SourceStmtSiteV1, DynamicCarrierRebindTransactionRejectV1> {
    semantic
        .envelope
        .source
        .rows
        .iter()
        .find_map(|row| {
            (row.role() == role).then(|| match row.site() {
                DynamicFullBodySourceSiteV1::Statement(site) => Some(site.clone()),
                DynamicFullBodySourceSiteV1::Expression(_) => None,
            })?
        })
        .ok_or(DynamicCarrierRebindTransactionRejectV1::CommitRelation)
}

fn read_binding(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
) -> Result<(LoopBindingKeyV1, LoopValueKeyV1), DynamicCarrierRebindTransactionRejectV1> {
    match recipe
        .items
        .iter()
        .find(|row| row.key == item)
        .map(|row| &row.item)
    {
        Some(LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::ReadBinding { binding, result },
        }) => Ok((*binding, *result)),
        _ => Err(DynamicCarrierRebindTransactionRejectV1::ReadBinding),
    }
}

fn dynamic_add(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
) -> Result<(LoopValueKeyV1, LoopValueKeyV1, LoopValueKeyV1), DynamicCarrierRebindTransactionRejectV1>
{
    match recipe
        .items
        .iter()
        .find(|row| row.key == item)
        .map(|row| &row.item)
    {
        Some(LoopRecipeItemV2::Operation {
            operation:
                LoopOperationV2::DynamicAdd {
                    left,
                    right,
                    result,
                },
        }) => Ok((*left, *right, *result)),
        _ => Err(DynamicCarrierRebindTransactionRejectV1::CommitRelation),
    }
}

fn write_binding(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    item: LoopItemKeyV1,
) -> Result<(LoopBindingKeyV1, LoopValueKeyV1), DynamicCarrierRebindTransactionRejectV1> {
    match recipe
        .items
        .iter()
        .find(|row| row.key == item)
        .map(|row| &row.item)
    {
        Some(LoopRecipeItemV2::Operation {
            operation: LoopOperationV2::WriteBinding { binding, value },
        }) => Ok((*binding, *value)),
        _ => Err(DynamicCarrierRebindTransactionRejectV1::CommitRelation),
    }
}

fn value_class(
    recipe: &crate::mir::loop_recipe_contract::LoopRecipeV2,
    value: LoopValueKeyV1,
) -> Result<LoopValueClassV2, DynamicCarrierRebindTransactionRejectV1> {
    recipe
        .values
        .iter()
        .find(|row| row.key == value)
        .map(|row| row.class)
        .ok_or(DynamicCarrierRebindTransactionRejectV1::CommitRelation)
}
