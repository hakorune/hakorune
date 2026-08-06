use crate::mir::loop_structural_facts::generic_g0::{
    GenericG0ConditionSitesV1, GenericG0TailSitesV1, GenericG0UpdateSitesV1,
};
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1,
};

use super::super::ids::{LoopBindingKeyV1, LoopCarrierKeyV1};
use super::super::schema::LoopValueClassV1;
use super::super::source_bound_core::{
    LoopBindingEffectAnchorV1, LoopBindingEffectRelationV1, LoopBindingEffectRoleV1,
    LoopRecipeBindingRelationV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericG0RelationRejectV1 {
    ParameterCardinality,
    ParameterBindingMismatch,
    ParameterOwnerMismatch,
    ConditionBindingMismatch,
    UpdateBindingMismatch,
    TailBindingMismatch,
}

pub(super) fn build_relations(
    owner: FunctionOwnerIdV1,
    parameter_rows: &[crate::mir::resolved_semantics::generic_g0::GenericG0ParameterTypeRowV1],
    outer_condition: &GenericG0ConditionSitesV1,
    inner_condition: &GenericG0ConditionSitesV1,
    outer_update: &GenericG0UpdateSitesV1,
    inner_update: &GenericG0UpdateSitesV1,
    tail: &GenericG0TailSitesV1,
    root_anchor: &SourceStmtSiteV1,
    child_anchor: &SourceStmtSiteV1,
) -> Result<
    (
        Vec<LoopRecipeBindingRelationV1>,
        Vec<LoopBindingEffectRelationV1>,
    ),
    GenericG0RelationRejectV1,
> {
    if parameter_rows.len() != 2 {
        return Err(GenericG0RelationRejectV1::ParameterCardinality);
    }
    let b0 = LoopBindingKeyV1::new(0);
    let b1 = LoopBindingKeyV1::new(1);
    if parameter_rows[0].binding != outer_condition.binding
        || parameter_rows[1].binding != inner_condition.binding
    {
        return Err(GenericG0RelationRejectV1::ParameterBindingMismatch);
    }
    if parameter_rows
        .iter()
        .any(|row| row.binding.owner() != owner)
    {
        return Err(GenericG0RelationRejectV1::ParameterOwnerMismatch);
    }
    if outer_condition.binding != outer_update.binding {
        return Err(GenericG0RelationRejectV1::ConditionBindingMismatch);
    }
    if inner_condition.binding != inner_update.binding {
        return Err(GenericG0RelationRejectV1::UpdateBindingMismatch);
    }
    if inner_condition.binding != tail.binding {
        return Err(GenericG0RelationRejectV1::TailBindingMismatch);
    }

    let bindings = vec![
        LoopRecipeBindingRelationV1::new(
            b0,
            parameter_rows[0].binding,
            LoopValueClassV1::I64,
            parameter_rows[0].binding_origin.clone(),
        ),
        LoopRecipeBindingRelationV1::new(
            b1,
            parameter_rows[1].binding,
            LoopValueClassV1::I64,
            parameter_rows[1].binding_origin.clone(),
        ),
    ];
    let effects = vec![
        expr_effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            b0,
            outer_condition.binding,
            owner,
            outer_condition.lhs.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            b0,
            outer_update.binding,
            owner,
            outer_update.lhs.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            b0,
            outer_update.binding,
            owner,
            outer_update.target.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 0 },
            b1,
            inner_condition.binding,
            owner,
            inner_condition.lhs.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 1 },
            b1,
            inner_update.binding,
            owner,
            inner_update.lhs.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceRead { ordinal: 2 },
            b1,
            tail.binding,
            owner,
            tail.value.clone(),
        ),
        expr_effect(
            LoopBindingEffectRoleV1::SourceWrite { ordinal: 0 },
            b1,
            inner_update.binding,
            owner,
            inner_update.target.clone(),
        ),
        carrier_effect(
            owner,
            b0,
            parameter_rows[0].binding,
            root_anchor,
            LoopCarrierKeyV1::new(0),
        ),
        carrier_effect(
            owner,
            b1,
            parameter_rows[1].binding,
            root_anchor,
            LoopCarrierKeyV1::new(1),
        ),
        carrier_effect(
            owner,
            b1,
            parameter_rows[1].binding,
            child_anchor,
            LoopCarrierKeyV1::new(2),
        ),
    ];
    Ok((bindings, effects))
}

fn expr_effect(
    role: LoopBindingEffectRoleV1,
    recipe_binding: LoopBindingKeyV1,
    source_binding: BindingRefV1,
    owner: FunctionOwnerIdV1,
    site: crate::mir::resolved_semantics::SourceExprSiteV1,
) -> LoopBindingEffectRelationV1 {
    LoopBindingEffectRelationV1::new(
        role,
        recipe_binding,
        source_binding,
        LoopValueClassV1::I64,
        LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site)),
    )
}

fn carrier_effect(
    owner: FunctionOwnerIdV1,
    recipe_binding: LoopBindingKeyV1,
    source_binding: BindingRefV1,
    source_loop: &SourceStmtSiteV1,
    carrier: LoopCarrierKeyV1,
) -> LoopBindingEffectRelationV1 {
    LoopBindingEffectRelationV1::new(
        LoopBindingEffectRoleV1::DerivedCarrierEntry,
        recipe_binding,
        source_binding,
        LoopValueClassV1::I64,
        LoopBindingEffectAnchorV1::DerivedCarrierEntry {
            owner,
            source_loop: source_loop.clone(),
            carrier,
        },
    )
}
