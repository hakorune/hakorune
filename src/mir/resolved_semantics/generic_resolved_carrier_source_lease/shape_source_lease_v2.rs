//! Test-only V2 role catalog over the sealed resolver source inventory.
//!
//! This is the first versioned extension after the immutable V1 carrier proof.
//! It derives only exact inner-loop Condition+Step paths.  It does not inspect
//! AST nodes or interpret operators/literals; later shape proofs own those
//! semantics.

use std::collections::BTreeSet;

use super::carrier_proof_witness::VerifiedGenericCarrierProofHandoffV1;
use super::GenericSourceAncestryV1;
use crate::mir::resolved_semantics::{
    BindingOriginV1, BindingRefV1, FunctionOwnerIdV1, ResolvedLexicalRefV1, ScopeId,
    SourceExprSiteV1, SourceNodeSiteV1, SourcePathSegmentV1, SourceStmtSiteV1,
    VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericShapeSourceLeaseRejectV2 {
    ForeignOwner,
    ForeignOrigin,
    SourceKindMismatch,
    InventoryBrandMismatch,
    MissingStatementSite,
    MissingExpressionSite,
    StepTargetPlacementMismatch,
    MissingVariableUse,
    UpvarOrCapture,
    BindingMismatch,
    ScopeMissing,
    AncestryMismatch,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericBindingUseRoleClaimV2 {
    site: SourceExprSiteV1,
    binding: BindingRefV1,
    binding_scope: ScopeId,
    site_scope: ScopeId,
    ancestry_chain: Box<[ScopeId]>,
    ancestry: GenericSourceAncestryV1,
}

impl GenericBindingUseRoleClaimV2 {
    pub(crate) fn site(&self) -> &SourceExprSiteV1 {
        &self.site
    }

    pub(crate) const fn binding(&self) -> BindingRefV1 {
        self.binding
    }

    pub(crate) fn ancestry_chain(&self) -> &[ScopeId] {
        &self.ancestry_chain
    }

    pub(crate) const fn ancestry(&self) -> GenericSourceAncestryV1 {
        self.ancestry
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericConditionRoleClaimV2 {
    condition_site: SourceExprSiteV1,
    induction: GenericBindingUseRoleClaimV2,
    bound_site: SourceExprSiteV1,
}

impl GenericConditionRoleClaimV2 {
    pub(crate) fn condition_site(&self) -> &SourceExprSiteV1 {
        &self.condition_site
    }

    pub(crate) fn induction(&self) -> &GenericBindingUseRoleClaimV2 {
        &self.induction
    }

    pub(crate) fn bound_site(&self) -> &SourceExprSiteV1 {
        &self.bound_site
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericStepRoleClaimV2 {
    statement_site: SourceStmtSiteV1,
    target_site: SourceExprSiteV1,
    value_site: SourceExprSiteV1,
    operand_read: GenericBindingUseRoleClaimV2,
    delta_site: SourceExprSiteV1,
}

impl GenericStepRoleClaimV2 {
    pub(crate) fn statement_site(&self) -> &SourceStmtSiteV1 {
        &self.statement_site
    }

    pub(crate) fn target_site(&self) -> &SourceExprSiteV1 {
        &self.target_site
    }

    pub(crate) fn value_site(&self) -> &SourceExprSiteV1 {
        &self.value_site
    }

    pub(crate) fn operand_read(&self) -> &GenericBindingUseRoleClaimV2 {
        &self.operand_read
    }

    pub(crate) fn delta_site(&self) -> &SourceExprSiteV1 {
        &self.delta_site
    }
}

/// Versioned, move-only extension of the immutable V1 carrier handoff.
#[derive(Debug, PartialEq, Eq)]
pub(crate) struct GenericShapeSourceLeaseV2 {
    carrier: VerifiedGenericCarrierProofHandoffV1,
    condition: GenericConditionRoleClaimV2,
    step: GenericStepRoleClaimV2,
    _seal: GenericShapeSourceLeaseSealV2,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericShapeSourceLeaseSealV2;

impl GenericShapeSourceLeaseV2 {
    pub(crate) fn carrier(&self) -> &VerifiedGenericCarrierProofHandoffV1 {
        &self.carrier
    }

    pub(crate) fn condition(&self) -> &GenericConditionRoleClaimV2 {
        &self.condition
    }

    pub(crate) fn step(&self) -> &GenericStepRoleClaimV2 {
        &self.step
    }
}

pub(crate) fn issue_generic_shape_source_lease_v2(
    function: &VerifiedResolvedFunctionV1,
    carrier: VerifiedGenericCarrierProofHandoffV1,
) -> Result<GenericShapeSourceLeaseV2, GenericShapeSourceLeaseRejectV2> {
    let (
        proof_owner,
        proof_origin,
        proof_source_kind,
        root_site,
        loop_site,
        nested_write_site,
        post_loop_read_site,
        proof_binding,
    ) = {
        let proof = carrier.proof();
        (
            proof.owner(),
            proof.function_origin(),
            proof.source_kind(),
            proof.root_site().clone(),
            proof.loop_site().clone(),
            proof.nested_write_site().clone(),
            proof.post_loop_read_site().clone(),
            proof.binding(),
        )
    };
    if proof_owner != function.owner() || carrier.lease().owner() != function.owner() {
        return Err(GenericShapeSourceLeaseRejectV2::ForeignOwner);
    }
    if proof_origin != function.function_origin()
        || carrier.lease().function_origin() != function.function_origin()
    {
        return Err(GenericShapeSourceLeaseRejectV2::ForeignOrigin);
    }
    if proof_source_kind != function.source_kind()
        || carrier.lease().source_kind() != function.source_kind()
    {
        return Err(GenericShapeSourceLeaseRejectV2::SourceKindMismatch);
    }
    let inventory = function.source_site_inventory();
    if inventory.owner() != function.owner()
        || inventory.function_origin() != function.function_origin()
        || inventory.source_kind() != function.source_kind()
    {
        return Err(GenericShapeSourceLeaseRejectV2::InventoryBrandMismatch);
    }

    require_statement(inventory, &root_site)?;
    require_statement(inventory, &loop_site)?;
    require_expression(inventory, &nested_write_site)?;
    require_expression(inventory, &post_loop_read_site)?;

    let condition_site = child_expr(&loop_site, SourcePathSegmentV1::LoopCondition);
    let induction_site = child_expr_from_expr(&condition_site, SourcePathSegmentV1::Lhs);
    let bound_site = child_expr_from_expr(&condition_site, SourcePathSegmentV1::Rhs);
    require_expression(inventory, &condition_site)?;
    require_expression(inventory, &induction_site)?;
    require_expression(inventory, &bound_site)?;
    let induction = issue_binding_use_claim(function, induction_site, proof_binding)?;

    let target_segments = nested_write_site.node().segments();
    if target_segments.last() != Some(&SourcePathSegmentV1::Target) {
        return Err(GenericShapeSourceLeaseRejectV2::StepTargetPlacementMismatch);
    }
    let parent_segments = &target_segments[..target_segments.len() - 1];
    let loop_segments = loop_site.node().segments();
    if parent_segments.len() != loop_segments.len() + 1
        || !parent_segments.starts_with(loop_segments)
        || !matches!(
            parent_segments.last(),
            Some(SourcePathSegmentV1::LoopBody(_))
        )
    {
        return Err(GenericShapeSourceLeaseRejectV2::StepTargetPlacementMismatch);
    }
    let statement_site =
        SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(parent_segments.to_vec()));
    require_statement(inventory, &statement_site)?;
    let value_site = child_expr(&statement_site, SourcePathSegmentV1::Value);
    let operand_site = child_expr_from_expr(&value_site, SourcePathSegmentV1::Lhs);
    let delta_site = child_expr_from_expr(&value_site, SourcePathSegmentV1::Rhs);
    require_expression(inventory, &value_site)?;
    require_expression(inventory, &operand_site)?;
    require_expression(inventory, &delta_site)?;
    let operand_read = issue_binding_use_claim(function, operand_site, proof_binding)?;

    Ok(GenericShapeSourceLeaseV2 {
        carrier,
        condition: GenericConditionRoleClaimV2 {
            condition_site,
            induction,
            bound_site,
        },
        step: GenericStepRoleClaimV2 {
            statement_site,
            target_site: nested_write_site,
            value_site,
            operand_read,
            delta_site,
        },
        _seal: GenericShapeSourceLeaseSealV2,
    })
}

fn child_expr(parent: &SourceStmtSiteV1, segment: SourcePathSegmentV1) -> SourceExprSiteV1 {
    let mut segments = parent.node().segments().to_vec();
    segments.push(segment);
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn child_expr_from_expr(
    parent: &SourceExprSiteV1,
    segment: SourcePathSegmentV1,
) -> SourceExprSiteV1 {
    let mut segments = parent.node().segments().to_vec();
    segments.push(segment);
    SourceExprSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
}

fn require_statement(
    inventory: &crate::mir::resolved_semantics::VerifiedResolvedSourceSiteInventoryV1,
    site: &SourceStmtSiteV1,
) -> Result<(), GenericShapeSourceLeaseRejectV2> {
    inventory
        .contains_statement(site)
        .then_some(())
        .ok_or(GenericShapeSourceLeaseRejectV2::MissingStatementSite)
}

fn require_expression(
    inventory: &crate::mir::resolved_semantics::VerifiedResolvedSourceSiteInventoryV1,
    site: &SourceExprSiteV1,
) -> Result<(), GenericShapeSourceLeaseRejectV2> {
    inventory
        .contains_expression(site)
        .then_some(())
        .ok_or(GenericShapeSourceLeaseRejectV2::MissingExpressionSite)
}

fn issue_binding_use_claim(
    function: &VerifiedResolvedFunctionV1,
    site: SourceExprSiteV1,
    expected: BindingRefV1,
) -> Result<GenericBindingUseRoleClaimV2, GenericShapeSourceLeaseRejectV2> {
    let binding = match function.variable_ref(&site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => binding,
        Some(ResolvedLexicalRefV1::Upvar(_)) => {
            return Err(GenericShapeSourceLeaseRejectV2::UpvarOrCapture)
        }
        None => return Err(GenericShapeSourceLeaseRejectV2::MissingVariableUse),
    };
    if binding != expected || binding.owner() != function.owner() {
        return Err(GenericShapeSourceLeaseRejectV2::BindingMismatch);
    }
    let record = function
        .binding(binding)
        .ok_or(GenericShapeSourceLeaseRejectV2::MissingVariableUse)?;
    if !matches!(record.origin(), BindingOriginV1::Source(_)) {
        return Err(GenericShapeSourceLeaseRejectV2::BindingMismatch);
    }
    let binding_scope = record.owner_scope();
    let site_scope = function
        .exact_scope_containing(site.node())
        .ok_or(GenericShapeSourceLeaseRejectV2::ScopeMissing)?;
    let (ancestry, ancestry_chain) = scope_ancestry(function, binding_scope, site_scope)?;
    Ok(GenericBindingUseRoleClaimV2 {
        site,
        binding,
        binding_scope,
        site_scope,
        ancestry_chain,
        ancestry,
    })
}

fn scope_ancestry(
    function: &VerifiedResolvedFunctionV1,
    ancestor: ScopeId,
    descendant: ScopeId,
) -> Result<(GenericSourceAncestryV1, Box<[ScopeId]>), GenericShapeSourceLeaseRejectV2> {
    if ancestor.owner() != function.owner() || descendant.owner() != function.owner() {
        return Err(GenericShapeSourceLeaseRejectV2::ScopeMissing);
    }
    if ancestor == descendant {
        return Ok((
            GenericSourceAncestryV1::SameScope,
            vec![descendant].into_boxed_slice(),
        ));
    }
    let mut chain = vec![descendant];
    let mut seen = BTreeSet::from([descendant]);
    let mut current = descendant;
    while let Some(parent) = function.scope(current).and_then(|scope| scope.parent()) {
        if parent.owner() != function.owner() || !seen.insert(parent) {
            return Err(GenericShapeSourceLeaseRejectV2::ScopeMissing);
        }
        chain.push(parent);
        if parent == ancestor {
            return Ok((
                GenericSourceAncestryV1::StrictAncestor,
                chain.into_boxed_slice(),
            ));
        }
        current = parent;
    }
    Err(GenericShapeSourceLeaseRejectV2::AncestryMismatch)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::{
        carrier_proof_witness::issue_carrier_proof_v1, tests as lease_tests,
    };

    const CONDITION_MISMATCH_SOURCE: &str = r#"
function generic_condition_mismatch(i, j, k) {
    loop(i < 3) {
        loop(k < 3) {
            j = j + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const STEP_MISMATCH_SOURCE: &str = r#"
function generic_step_mismatch(i, j, k) {
    loop(i < 3) {
        loop(j < 3) {
            j = k + 1
        }
        i = i + 1
    }
    return j
}
"#;

    const NON_CANONICAL_SOURCE: &str = r#"
function generic_noncanonical(i, j) {
    loop(i <= 7) {
        loop(j <= 7) {
            j = j * 2
        }
        i = i * 2
    }
    return j
}
"#;

    fn positive_v2(source: &str) -> (FunctionOwnerIdV1, GenericShapeSourceLeaseV2) {
        let unit = lease_tests::unit(source);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        let v2 = issue_generic_shape_source_lease_v2(function, handoff)
            .expect("condition/step role catalog");
        (function.owner(), v2)
    }

    fn source_lifetime_free(source: &str) -> GenericShapeSourceLeaseV2 {
        let unit = lease_tests::unit(source);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        issue_generic_shape_source_lease_v2(function, handoff).expect("condition/step role catalog")
    }

    #[test]
    fn issues_exact_condition_and_step_roles_without_shape_interpretation() {
        let (owner, v2) = positive_v2(NON_CANONICAL_SOURCE);
        assert_eq!(
            v2.carrier().proof().binding(),
            v2.step().operand_read().binding()
        );
        assert_eq!(
            v2.carrier().proof().binding(),
            v2.condition().induction().binding()
        );
        assert_eq!(
            v2.condition().condition_site().node().segments().last(),
            Some(&SourcePathSegmentV1::LoopCondition)
        );
        assert_eq!(
            v2.step().target_site(),
            v2.carrier().proof().nested_write_site()
        );
        assert_eq!(v2.carrier().proof().owner(), owner);
    }

    #[test]
    fn output_has_no_source_unit_lifetime() {
        let v2 = source_lifetime_free(lease_tests::SOURCE);
        assert_eq!(
            v2.carrier().proof().transfer(),
            super::super::carrier_proof_witness::CarrierTransferV1::NestedWriteToPostLoopRead
        );
    }

    #[test]
    fn rejects_foreign_function_before_inventory_lookup() {
        let first = lease_tests::unit(lease_tests::SOURCE);
        let second = lease_tests::unit(lease_tests::SOURCE);
        let (first_input, first_root) = lease_tests::input_and_root(&first);
        let (second_input, _) = lease_tests::input_and_root(&second);
        let lease = lease_tests::positive_lease(first_input, &first_root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        assert_eq!(
            issue_generic_shape_source_lease_v2(second_input.function(), handoff),
            Err(GenericShapeSourceLeaseRejectV2::ForeignOwner)
        );
    }

    #[test]
    fn rejects_condition_binding_mismatch() {
        let unit = lease_tests::unit(CONDITION_MISMATCH_SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        assert_eq!(
            issue_generic_shape_source_lease_v2(function, handoff),
            Err(GenericShapeSourceLeaseRejectV2::BindingMismatch)
        );
    }

    #[test]
    fn rejects_step_operand_binding_mismatch() {
        let unit = lease_tests::unit(STEP_MISMATCH_SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let function = input.function();
        let lease = lease_tests::positive_lease(input, &root);
        let handoff = issue_carrier_proof_v1(lease).expect("carrier proof");
        assert_eq!(
            issue_generic_shape_source_lease_v2(function, handoff),
            Err(GenericShapeSourceLeaseRejectV2::BindingMismatch)
        );
    }
}
