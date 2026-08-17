//! Source-to-Recipe/Join binding for the fixed S6C Return arm.
//!
//! The resolver source co-seal owns source sites, regions, and the lexical
//! index binding.  The S6C Recipe producer is the only owner allowed to issue
//! Recipe keys, so this module is the one handoff that binds those existing
//! facts to the exact logical Exit and JoinSig FunctionExit arm.  It has no
//! physical ID, session, CFG, or fallback meaning.

use super::ids::{LoopBlockKeyV1, LoopExitKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::join_sig::{
    LoopJoinBranchArmTransferRefV2, LoopJoinBranchExitTargetV2, LoopJoinBranchTransferRefV2,
    LoopJoinEdgeRoleV1, LoopJoinLogicalTransferViewV2, LoopJoinPortV1,
};
use super::s6c_scan_with_init::S6CScanWithInitRecipeRolesRefV2;
use super::schema_v2::{LoopOperationV2, LoopRecipeItemV2, LoopValueClassV2};
use super::typed_schema_v2::VerifiedLoopRecipeV2;
use crate::mir::loop_structural_facts::VerifiedS6CScanWithInitFactsV1;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionOwnerIdV1, RegionId, SourceExprSiteV1, SourceStmtSiteV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum S6CReturnSourceBindingRejectV1 {
    ForeignOwner,
    SourceRegionMismatch,
    SourceSiteCoverage,
    MissingRecipeRow(&'static str),
    RecipeRowMismatch(&'static str),
    BranchCardinality,
    BranchMismatch(&'static str),
    SummaryMismatch,
}

/// One move-only source-to-Recipe/Join relation for the exact S6C Return arm.
///
/// The source sites/regions are copied from the resolver-owned co-seal only to
/// brand this relation.  The logical keys are issued by the S6C Recipe
/// producer in the same call that issues its Recipe and Join closure.
#[derive(Debug)]
pub(crate) struct VerifiedS6CReturnSourceRecipeBindingV1 {
    owner: FunctionOwnerIdV1,
    return_site: SourceStmtSiteV1,
    return_value: SourceExprSiteV1,
    if_site: SourceStmtSiteV1,
    return_region: RegionId,
    if_then_region: RegionId,
    source_binding: BindingRefV1,
    recipe_if_item: LoopItemKeyV1,
    recipe_if_block: LoopBlockKeyV1,
    recipe_then_block: LoopBlockKeyV1,
    recipe_return_item: LoopItemKeyV1,
    recipe_return_block: LoopBlockKeyV1,
    recipe_exit: LoopExitKeyV1,
    recipe_return_value: LoopValueKeyV1,
    join_exit_item: LoopItemKeyV1,
    join_role: LoopJoinEdgeRoleV1,
    join_target: LoopJoinBranchExitTargetV2,
}

impl VerifiedS6CReturnSourceRecipeBindingV1 {
    pub(crate) const fn owner(&self) -> FunctionOwnerIdV1 {
        self.owner
    }

    pub(crate) fn return_site(&self) -> &SourceStmtSiteV1 {
        &self.return_site
    }

    pub(crate) fn return_value(&self) -> &SourceExprSiteV1 {
        &self.return_value
    }

    pub(crate) fn if_site(&self) -> &SourceStmtSiteV1 {
        &self.if_site
    }

    pub(crate) const fn return_region(&self) -> RegionId {
        self.return_region
    }

    pub(crate) const fn if_then_region(&self) -> RegionId {
        self.if_then_region
    }

    pub(crate) const fn source_binding(&self) -> BindingRefV1 {
        self.source_binding
    }

    pub(crate) const fn recipe_if_item(&self) -> LoopItemKeyV1 {
        self.recipe_if_item
    }

    pub(crate) const fn recipe_if_block(&self) -> LoopBlockKeyV1 {
        self.recipe_if_block
    }

    pub(crate) const fn recipe_then_block(&self) -> LoopBlockKeyV1 {
        self.recipe_then_block
    }

    pub(crate) const fn recipe_return_item(&self) -> LoopItemKeyV1 {
        self.recipe_return_item
    }

    pub(crate) const fn recipe_return_block(&self) -> LoopBlockKeyV1 {
        self.recipe_return_block
    }

    pub(crate) const fn recipe_exit(&self) -> LoopExitKeyV1 {
        self.recipe_exit
    }

    pub(crate) const fn recipe_return_value(&self) -> LoopValueKeyV1 {
        self.recipe_return_value
    }

    pub(crate) const fn join_exit_item(&self) -> LoopItemKeyV1 {
        self.join_exit_item
    }

    pub(crate) const fn join_role(&self) -> LoopJoinEdgeRoleV1 {
        self.join_role
    }

    pub(crate) const fn join_target(&self) -> LoopJoinBranchExitTargetV2 {
        self.join_target
    }
}

pub(crate) fn issue_s6c_return_source_recipe_binding_v1(
    facts: &VerifiedS6CScanWithInitFactsV1,
    recipe: &VerifiedLoopRecipeV2,
    roles: S6CScanWithInitRecipeRolesRefV2<'_>,
    transfer: &LoopJoinLogicalTransferViewV2<'_>,
) -> Result<VerifiedS6CReturnSourceRecipeBindingV1, S6CReturnSourceBindingRejectV1> {
    facts.with_facts(|facts| {
        let source = facts.source();
        let owner = source.calls().length().owner();
        if source.loop_return_binding().owner() != owner
            || source.if_then_region().owner() != owner
            || source.loop_return_region().owner() != owner
            || source.completion().target_function().owner() != owner
        {
            return Err(S6CReturnSourceBindingRejectV1::ForeignOwner);
        }
        if source.loop_return_region() != source.if_then_region() {
            return Err(S6CReturnSourceBindingRejectV1::SourceRegionMismatch);
        }
        if !source
            .completion()
            .explicit_sites()
            .contains(source.loop_return_site())
        {
            return Err(S6CReturnSourceBindingRejectV1::SourceSiteCoverage);
        }

        let raw = recipe.as_recipe();
        let binding = raw
            .bindings
            .iter()
            .find(|row| row.key == roles.index_binding())
            .ok_or(S6CReturnSourceBindingRejectV1::MissingRecipeRow("binding"))?;
        if binding.class != LoopValueClassV2::I64 {
            return Err(S6CReturnSourceBindingRejectV1::RecipeRowMismatch("binding"));
        }

        let return_item = roles.return_index_read().item();
        let return_value = roles.return_index_read().result();
        let return_row = raw.items.iter().find(|row| row.key == return_item).ok_or(
            S6CReturnSourceBindingRejectV1::MissingRecipeRow("return read"),
        )?;
        if !matches!(
            &return_row.item,
            LoopRecipeItemV2::Operation {
                operation: LoopOperationV2::ReadBinding { binding: row_binding, result }
            } if *row_binding == roles.index_binding() && *result == return_value
        ) {
            return Err(S6CReturnSourceBindingRejectV1::RecipeRowMismatch(
                "return read",
            ));
        }

        let exit_item = roles.loop_return().item();
        let exit_key = roles.loop_return().exit();
        let exit_row = raw
            .items
            .iter()
            .find(|row| row.key == exit_item)
            .ok_or(S6CReturnSourceBindingRejectV1::MissingRecipeRow("exit"))?;
        if !matches!(&exit_row.item, LoopRecipeItemV2::Exit { exit } if *exit == exit_key) {
            return Err(S6CReturnSourceBindingRejectV1::RecipeRowMismatch("exit"));
        }

        let if_item = roles.text_equal_if();
        let if_row = raw
            .items
            .iter()
            .find(|row| row.key == if_item)
            .ok_or(S6CReturnSourceBindingRejectV1::MissingRecipeRow("If"))?;
        if !matches!(
            &if_row.item,
            LoopRecipeItemV2::If { condition, then_block, else_block }
                if *condition == roles.text_equal().result()
                    && *then_block == roles.text_eq_then_block()
                    && else_block.is_none()
        ) {
            return Err(S6CReturnSourceBindingRejectV1::RecipeRowMismatch("If"));
        }

        let then_block = raw
            .blocks
            .iter()
            .find(|row| row.key == roles.text_eq_then_block())
            .ok_or(S6CReturnSourceBindingRejectV1::MissingRecipeRow(
                "then block",
            ))?;
        if then_block.items != vec![return_item, exit_item] {
            return Err(S6CReturnSourceBindingRejectV1::RecipeRowMismatch(
                "then block items",
            ));
        }

        let branch = unique_branch(transfer, roles)?;
        let exit = match branch.then_arm {
            LoopJoinBranchArmTransferRefV2::Exit(exit) => exit,
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. } => {
                return Err(S6CReturnSourceBindingRejectV1::BranchMismatch(
                    "then Return arm",
                ))
            }
        };
        if exit.exit_item != exit_item
            || exit.role != LoopJoinEdgeRoleV1::Return
            || exit.target != LoopJoinBranchExitTargetV2::FunctionExit
        {
            return Err(S6CReturnSourceBindingRejectV1::BranchMismatch(
                "Return/FunctionExit arm",
            ));
        }
        if !matches!(
            branch.else_arm,
            LoopJoinBranchArmTransferRefV2::Fallthrough { .. }
        ) {
            return Err(S6CReturnSourceBindingRejectV1::BranchMismatch(
                "else Fallthrough arm",
            ));
        }
        let [summary] = transfer.summary_transfers() else {
            return Err(S6CReturnSourceBindingRejectV1::SummaryMismatch);
        };
        if summary.from != LoopJoinPortV1::Body
            || summary.to != LoopJoinPortV1::FunctionExit
            || summary.role != LoopJoinEdgeRoleV1::Return
            || summary.payload != exit.payload
        {
            return Err(S6CReturnSourceBindingRejectV1::SummaryMismatch);
        }

        Ok(VerifiedS6CReturnSourceRecipeBindingV1 {
            owner,
            return_site: source.loop_return_site().clone(),
            return_value: source.loop_return_value().clone(),
            if_site: source.if_site().clone(),
            return_region: source.loop_return_region(),
            if_then_region: source.if_then_region(),
            source_binding: source.loop_return_binding(),
            recipe_if_item: if_item,
            recipe_if_block: roles.body_block(),
            recipe_then_block: roles.text_eq_then_block(),
            recipe_return_item: return_item,
            recipe_return_block: roles.text_eq_then_block(),
            recipe_exit: exit_key,
            recipe_return_value: return_value,
            join_exit_item: exit.exit_item,
            join_role: exit.role,
            join_target: exit.target,
        })
    })
}

fn unique_branch<'a>(
    transfer: &'a LoopJoinLogicalTransferViewV2<'a>,
    roles: S6CScanWithInitRecipeRolesRefV2<'_>,
) -> Result<&'a LoopJoinBranchTransferRefV2<'a>, S6CReturnSourceBindingRejectV1> {
    let matches = transfer
        .branches()
        .iter()
        .filter(|branch| {
            branch.owner_loop == roles.root_loop()
                && branch.if_item == roles.text_equal_if()
                && branch.condition == roles.text_equal().result()
        })
        .collect::<Vec<_>>();
    if matches.len() != 1 {
        return Err(S6CReturnSourceBindingRejectV1::BranchCardinality);
    }
    Ok(matches[0])
}
