//! Generic G0 item-to-source-anchor ledger.
//!
//! The G0 producer issues this ledger while its resolver-backed source facts
//! are still owned by the producer.  The neutral operation/effect product is
//! the only later owner; item keys and Core relations remain the authority.

use crate::mir::loop_recipe_contract::{
    LoopBindingEffectAnchorV1, LoopBlockKeyV1, LoopItemKeyV1, LoopNodeKeyV1,
    LoopOperationEffectRejectV1, LoopOperationSourceEvidenceV1, VerifiedLoopCoreProductV1,
    VerifiedLoopOperationEffectProductV1,
};
use crate::mir::loop_structural_facts::generic_g0::{
    GenericG0ConditionSitesV1, GenericG0TailSitesV1, GenericG0UpdateSitesV1,
};
use crate::mir::resolved_semantics::{FunctionOwnerIdV1, OwnedExprSiteV1, SourceStmtSiteV1};

/// The exact operation-item coverage for the nested Generic G0 profile.
///
/// Item 4 is the nested `Loop` row.  C0/C1 carrier rows and the post-loop tail
/// read are owned by their existing contracts and are intentionally absent.
const G0_OPERATION_ITEMS: &[u32] = &[0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15];

pub(super) fn issue_generic_g0_operation_effect_v1(
    core: VerifiedLoopCoreProductV1,
    root_loop: &SourceStmtSiteV1,
    child_loop: &SourceStmtSiteV1,
    child_entry: &SourceStmtSiteV1,
    outer_condition: &GenericG0ConditionSitesV1,
    inner_condition: &GenericG0ConditionSitesV1,
    outer_update: &GenericG0UpdateSitesV1,
    inner_update: &GenericG0UpdateSitesV1,
    _tail: &GenericG0TailSitesV1,
) -> Result<VerifiedLoopOperationEffectProductV1, LoopOperationEffectRejectV1> {
    let owner = core.owner();
    let mut evidence = Vec::with_capacity(G0_OPERATION_ITEMS.len());
    for raw in G0_OPERATION_ITEMS {
        let item = LoopItemKeyV1::new(*raw);
        let (block, owner_loop) = placement(&core, item)?;
        let (anchor, source_loop, source_binding) = match raw {
            0 => (
                expr_anchor(owner, &outer_condition.lhs),
                root_loop,
                Some(outer_condition.binding),
            ),
            1 => (expr_anchor(owner, &outer_condition.rhs), root_loop, None),
            2 => (
                expr_anchor(owner, &outer_condition.condition),
                root_loop,
                None,
            ),
            3 => (
                LoopBindingEffectAnchorV1::DerivedCarrierEntry {
                    owner,
                    source_loop: child_entry.clone(),
                    carrier: crate::mir::loop_recipe_contract::LoopCarrierKeyV1::new(2),
                },
                child_entry,
                Some(inner_condition.binding),
            ),
            5 => (
                expr_anchor(owner, &inner_condition.lhs),
                child_loop,
                Some(inner_condition.binding),
            ),
            6 => (expr_anchor(owner, &inner_condition.rhs), child_loop, None),
            7 => (
                expr_anchor(owner, &inner_condition.condition),
                child_loop,
                None,
            ),
            8 => (
                expr_anchor(owner, &inner_update.lhs),
                child_loop,
                Some(inner_update.binding),
            ),
            9 => (expr_anchor(owner, &inner_update.rhs), child_loop, None),
            10 => (expr_anchor(owner, &inner_update.value), child_loop, None),
            11 => (
                expr_anchor(owner, &inner_update.target),
                child_loop,
                Some(inner_update.binding),
            ),
            12 => (
                expr_anchor(owner, &outer_update.lhs),
                root_loop,
                Some(outer_update.binding),
            ),
            13 => (expr_anchor(owner, &outer_update.rhs), root_loop, None),
            14 => (expr_anchor(owner, &outer_update.value), root_loop, None),
            15 => (
                expr_anchor(owner, &outer_update.target),
                root_loop,
                Some(outer_update.binding),
            ),
            _ => return Err(LoopOperationEffectRejectV1::ForeignItem { item }),
        };
        evidence.push(LoopOperationSourceEvidenceV1::new(
            item,
            anchor,
            source_loop.clone(),
            owner_loop,
            block,
            source_binding,
        ));
    }
    VerifiedLoopOperationEffectProductV1::issue(core, evidence)
}

fn placement(
    core: &VerifiedLoopCoreProductV1,
    item: LoopItemKeyV1,
) -> Result<(LoopBlockKeyV1, LoopNodeKeyV1), LoopOperationEffectRejectV1> {
    let mut found = None;
    for block in &core.recipe().as_recipe().blocks {
        if block.items.contains(&item) {
            if found.is_some() {
                return Err(LoopOperationEffectRejectV1::PlacementMismatch { item });
            }
            found = Some((block.key, block.owner_loop));
        }
    }
    found.ok_or(LoopOperationEffectRejectV1::MissingEvidence { item })
}

fn expr_anchor(
    owner: FunctionOwnerIdV1,
    site: &crate::mir::resolved_semantics::SourceExprSiteV1,
) -> LoopBindingEffectAnchorV1 {
    LoopBindingEffectAnchorV1::Expr(OwnedExprSiteV1::new(owner, site.clone()))
}
