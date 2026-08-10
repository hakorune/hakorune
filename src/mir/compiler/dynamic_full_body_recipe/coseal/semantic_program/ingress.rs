//! Exact callable-parameter to root-carrier ingress co-seal.

use crate::mir::loop_recipe_contract::{
    LoopBindingKeyV1, LoopCarrierKeyV1, LoopJoinEdgeRoleV1, LoopJoinPortV1, LoopNodeKeyV1,
    LoopValueClassV2, LoopValueKeyV1,
};
use crate::mir::resolved_semantics::{BindingRefV1, HomeDemandV1};

use super::super::super::super::dynamic_full_body_source::{
    DynamicFullBodyBindingRoleV1, DynamicFullBodySourceRoleV1,
};
use super::super::super::claims::DynamicFullLoopClaimTargetV2;
use super::VerifiedDynamicOperatorCarrierLifecycleProgramV1;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir) enum DynamicCarrierIngressLifecycleProgramRejectV1 {
    ParameterDemand,
    SourceRelation,
    RecipeRelation,
    JoinRelation,
}

/// Whole Dynamic lifecycle program with one exact borrowed ingress relation.
#[derive(Debug)]
pub(in crate::mir) struct VerifiedDynamicCarrierIngressLifecycleProgramV1 {
    program: VerifiedDynamicOperatorCarrierLifecycleProgramV1,
    _ingress: BorrowedIngressNoEndV1,
}

#[derive(Debug, Clone, Copy)]
pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) struct BorrowedIngressNoEndV1
{
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) parameter_ordinal:
        u32,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) parameter_binding:
        BindingRefV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) demand:
        HomeDemandV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) root:
        LoopNodeKeyV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) carrier:
        LoopCarrierKeyV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) recipe_binding:
        LoopBindingKeyV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) source_binding:
        BindingRefV1,
    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) entry:
        LoopValueKeyV1,
}

impl VerifiedDynamicCarrierIngressLifecycleProgramV1 {
    pub(in crate::mir) fn with_semantic_program<R>(
        &self,
        callback: impl for<'program> FnOnce(
            &'program super::VerifiedDynamicFullLoopSemanticProgramV2,
        ) -> R,
    ) -> R {
        self.program.with_semantic_program(callback)
    }

    pub(in crate::mir::compiler::dynamic_full_body_recipe::coseal::semantic_program) fn relation(
        &self,
    ) -> BorrowedIngressNoEndV1 {
        self._ingress
    }

    pub(in crate::mir) fn program(&self) -> &VerifiedDynamicOperatorCarrierLifecycleProgramV1 {
        &self.program
    }
}

pub(in crate::mir) fn issue_dynamic_carrier_ingress_lifecycle_program_v1(
    program: VerifiedDynamicOperatorCarrierLifecycleProgramV1,
    parameter_ordinal: u32,
    parameter_binding: BindingRefV1,
    demand: HomeDemandV1,
) -> Result<
    VerifiedDynamicCarrierIngressLifecycleProgramV1,
    DynamicCarrierIngressLifecycleProgramRejectV1,
> {
    if parameter_ordinal != 1 || demand != HomeDemandV1::Handle {
        return Err(DynamicCarrierIngressLifecycleProgramRejectV1::ParameterDemand);
    }
    let semantic = &program.invocation_program.program;
    let envelope = &semantic.envelope;
    let pos = envelope
        .source
        .bindings
        .iter()
        .find(|row| row.role() == DynamicFullBodyBindingRoleV1::Pos)
        .ok_or(DynamicCarrierIngressLifecycleProgramRejectV1::SourceRelation)?;
    let source_binding = pos.binding();
    if source_binding != parameter_binding {
        return Err(DynamicCarrierIngressLifecycleProgramRejectV1::SourceRelation);
    }
    let induction_binding = match envelope
        .coverage
        .binding_target(DynamicFullBodyBindingRoleV1::Induction)
    {
        Some(DynamicFullLoopClaimTargetV2::Binding(binding)) => binding,
        _ => return Err(DynamicCarrierIngressLifecycleProgramRejectV1::RecipeRelation),
    };
    let entry = match envelope
        .coverage
        .source_target(DynamicFullBodySourceRoleV1::PreludeInitializerPos)
    {
        Some(DynamicFullLoopClaimTargetV2::Value(value)) => value,
        _ => return Err(DynamicCarrierIngressLifecycleProgramRejectV1::RecipeRelation),
    };
    let (root, carrier) = match envelope
        .coverage
        .source_target(DynamicFullBodySourceRoleV1::PreludeLocalI)
    {
        Some(DynamicFullLoopClaimTargetV2::PreludeInduction {
            binding,
            carrier,
            entry: claimed_entry,
        }) if binding == induction_binding && claimed_entry == entry => {
            (envelope.artifact.recipe().root_loop(), carrier)
        }
        _ => return Err(DynamicCarrierIngressLifecycleProgramRejectV1::RecipeRelation),
    };
    let recipe = envelope.artifact.recipe();
    let recipe_wire = recipe.as_recipe();
    let exact_carrier = recipe_wire.carriers.iter().find(|row| row.key == carrier);
    if !recipe_wire.inputs.contains(&entry)
        || !matches!(
            exact_carrier,
            Some(row)
                if row.owner_loop == root
                    && row.binding == induction_binding
                    && row.entry_value == entry
                    && row.class == LoopValueClassV2::Dynamic
        )
    {
        return Err(DynamicCarrierIngressLifecycleProgramRejectV1::RecipeRelation);
    }
    let join = semantic.control.join_sig().as_sig();
    let enter = join
        .loops
        .iter()
        .find(|row| row.key == root)
        .into_iter()
        .flat_map(|row| row.edges.iter())
        .filter(|edge| {
            edge.from == LoopJoinPortV1::Preheader
                && edge.to == LoopJoinPortV1::Header
                && edge.role == LoopJoinEdgeRoleV1::Enter
        })
        .collect::<Vec<_>>();
    if enter.len() != 1
        || enter[0].payload.len() != 1
        || enter[0].payload[0].binding != induction_binding
        || enter[0].payload[0].value != entry
        || enter[0].payload[0].class != LoopValueClassV2::Dynamic
    {
        return Err(DynamicCarrierIngressLifecycleProgramRejectV1::JoinRelation);
    }
    Ok(VerifiedDynamicCarrierIngressLifecycleProgramV1 {
        program,
        _ingress: BorrowedIngressNoEndV1 {
            parameter_ordinal,
            parameter_binding,
            demand,
            root,
            carrier,
            recipe_binding: induction_binding,
            source_binding,
            entry,
        },
    })
}
