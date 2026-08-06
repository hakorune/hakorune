//! Test-only resolver-to-candidate envelope for the shallow Generic gate.
//!
//! The issuer consumes one source lease and one borrowed syntax view.  It
//! publishes typed, AST-free body-effect and coverage/exit proofs alongside
//! the existing Carrier/Condition/Step chain.  No policy row or selector is
//! issued here.

use super::carrier_proof_witness::{issue_carrier_proof_v1, CarrierProofRejectV1};
use super::shape_source_lease_v2::{
    issue_generic_shape_source_lease_v2, GenericShapeSourceLeaseRejectV2, GenericShapeSourceLeaseV2,
};
use super::shape_syntax_facts_v3::{
    issue_condition_step_syntax_facts_v3, GenericConditionStepSyntaxFactsV3,
    GenericSyntaxFactRejectV3,
};
use super::GenericSourceLeaseV1;
use crate::ast::ASTNode;
use crate::mir::resolved_semantics::{
    BindingRefV1, FunctionSyntaxViewV1, ProjectedSourceNodeV1, ResolvedAssignmentTargetV1,
    ResolvedExitSiteV1, ResolvedLexicalRefV1, SourceExprSiteV1, SourceNodeSiteV1,
    SourcePathSegmentV1, SourceStmtSiteV1, VerifiedResolvedFunctionV1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericBodyEffectRejectV1 {
    RootNotLoop,
    ChildNotLoop,
    OuterBodyShape,
    InnerBodyShape,
    InnerStepNotBinding,
    OuterStepNotBinding,
    MissingInventory,
    MissingBinding,
    BindingMismatch,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericBodyEffectKindV1 {
    NestedLoop,
    InnerBindingRebind,
    OuterBindingRebind,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericBodyEffectProofV1 {
    root_site: SourceStmtSiteV1,
    child_site: SourceStmtSiteV1,
    inner_step_site: SourceStmtSiteV1,
    outer_step_site: SourceStmtSiteV1,
    outer_binding: BindingRefV1,
    inner_binding: BindingRefV1,
    effects: Box<[GenericBodyEffectKindV1]>,
    _seal: GenericBodyEffectProofSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericBodyEffectProofSealV1;

impl VerifiedGenericBodyEffectProofV1 {
    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.root_site
    }

    pub(crate) fn child_site(&self) -> &SourceStmtSiteV1 {
        &self.child_site
    }

    pub(crate) fn inner_step_site(&self) -> &SourceStmtSiteV1 {
        &self.inner_step_site
    }

    pub(crate) fn outer_step_site(&self) -> &SourceStmtSiteV1 {
        &self.outer_step_site
    }

    pub(crate) const fn outer_binding(&self) -> BindingRefV1 {
        self.outer_binding
    }

    pub(crate) const fn inner_binding(&self) -> BindingRefV1 {
        self.inner_binding
    }

    pub(crate) fn effects(&self) -> &[GenericBodyEffectKindV1] {
        &self.effects
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericCoverageExitRejectV1 {
    RootSiteMismatch,
    IncompleteRootBody,
    MissingReturn,
    ReturnStillInsideLoop,
    MissingInventory,
    MissingResolvedExit,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum GenericCoverageExitKindV1 {
    CompleteNestedWindow,
    ReturnAfterRootLoop,
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) struct VerifiedGenericCoverageExitProofV1 {
    root_site: SourceStmtSiteV1,
    child_site: SourceStmtSiteV1,
    after_return_site: SourceStmtSiteV1,
    kinds: Box<[GenericCoverageExitKindV1]>,
    _seal: GenericCoverageExitProofSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericCoverageExitProofSealV1;

impl VerifiedGenericCoverageExitProofV1 {
    pub(crate) fn root_site(&self) -> &SourceStmtSiteV1 {
        &self.root_site
    }

    pub(crate) fn child_site(&self) -> &SourceStmtSiteV1 {
        &self.child_site
    }

    pub(crate) fn after_return_site(&self) -> &SourceStmtSiteV1 {
        &self.after_return_site
    }

    pub(crate) fn kinds(&self) -> &[GenericCoverageExitKindV1] {
        &self.kinds
    }
}

#[derive(Debug, PartialEq, Eq)]
pub(crate) enum GenericCandidateEnvelopeRejectV1 {
    Carrier(CarrierProofRejectV1),
    Shape(GenericShapeSourceLeaseRejectV2),
    Syntax(GenericSyntaxFactRejectV3),
    BodyEffect(GenericBodyEffectRejectV1),
    CoverageExit(GenericCoverageExitRejectV1),
}

#[derive(Debug, PartialEq)]
pub(crate) struct VerifiedGenericCandidateEnvelopeV1 {
    syntax: GenericConditionStepSyntaxFactsV3,
    body_effect: VerifiedGenericBodyEffectProofV1,
    coverage_exit: VerifiedGenericCoverageExitProofV1,
    _seal: GenericCandidateEnvelopeSealV1,
}

#[derive(Debug, PartialEq, Eq)]
struct GenericCandidateEnvelopeSealV1;

impl VerifiedGenericCandidateEnvelopeV1 {
    pub(crate) fn syntax(&self) -> &GenericConditionStepSyntaxFactsV3 {
        &self.syntax
    }

    pub(crate) fn body_effect(&self) -> &VerifiedGenericBodyEffectProofV1 {
        &self.body_effect
    }

    pub(crate) fn coverage_exit(&self) -> &VerifiedGenericCoverageExitProofV1 {
        &self.coverage_exit
    }
}

pub(crate) fn issue_generic_candidate_envelope_v1<'source>(
    function: &VerifiedResolvedFunctionV1,
    source: FunctionSyntaxViewV1<'source>,
    lease: GenericSourceLeaseV1,
) -> Result<VerifiedGenericCandidateEnvelopeV1, GenericCandidateEnvelopeRejectV1> {
    let handoff =
        issue_carrier_proof_v1(lease).map_err(GenericCandidateEnvelopeRejectV1::Carrier)?;
    let shape = issue_generic_shape_source_lease_v2(function, handoff)
        .map_err(GenericCandidateEnvelopeRejectV1::Shape)?;
    let syntax = issue_condition_step_syntax_facts_v3(function, source, shape)
        .map_err(GenericCandidateEnvelopeRejectV1::Syntax)?;
    let body_effect = issue_body_effect(function, source, syntax.carrier())
        .map_err(GenericCandidateEnvelopeRejectV1::BodyEffect)?;
    let coverage_exit = issue_coverage_exit(function, source, body_effect.root_site())
        .map_err(GenericCandidateEnvelopeRejectV1::CoverageExit)?;
    Ok(VerifiedGenericCandidateEnvelopeV1 {
        syntax,
        body_effect,
        coverage_exit,
        _seal: GenericCandidateEnvelopeSealV1,
    })
}

fn issue_body_effect<'source>(
    function: &VerifiedResolvedFunctionV1,
    source: FunctionSyntaxViewV1<'source>,
    shape: &GenericShapeSourceLeaseV2,
) -> Result<VerifiedGenericBodyEffectProofV1, GenericBodyEffectRejectV1> {
    let root_site = shape.carrier().proof().root_site().clone();
    let child_site = shape.carrier().proof().loop_site().clone();
    let outer = project_stmt(source, &root_site, GenericBodyEffectRejectV1::RootNotLoop)?;
    let child = project_stmt(source, &child_site, GenericBodyEffectRejectV1::ChildNotLoop)?;
    let ASTNode::Loop {
        body: outer_body, ..
    } = outer
    else {
        return Err(GenericBodyEffectRejectV1::RootNotLoop);
    };
    let ASTNode::Loop {
        body: inner_body, ..
    } = child
    else {
        return Err(GenericBodyEffectRejectV1::ChildNotLoop);
    };
    if outer_body.len() != 2 {
        return Err(GenericBodyEffectRejectV1::OuterBodyShape);
    }
    if inner_body.len() != 1 {
        return Err(GenericBodyEffectRejectV1::InnerBodyShape);
    }
    if !matches!(outer_body.first(), Some(ASTNode::Loop { .. })) {
        return Err(GenericBodyEffectRejectV1::OuterBodyShape);
    }
    let inner_step_site = child_stmt(&child_site, SourcePathSegmentV1::LoopBody(0));
    let outer_step_site = child_stmt(&root_site, SourcePathSegmentV1::LoopBody(1));
    require_statement(function, &inner_step_site)?;
    require_statement(function, &outer_step_site)?;
    let inner_binding = binding_target(function, &inner_step_site)
        .ok_or(GenericBodyEffectRejectV1::InnerStepNotBinding)?;
    let outer_binding = binding_target(function, &outer_step_site)
        .ok_or(GenericBodyEffectRejectV1::OuterStepNotBinding)?;
    let outer_condition = child_expr(&root_site, SourcePathSegmentV1::LoopCondition);
    let outer_lhs = child_expr_from_expr(&outer_condition, SourcePathSegmentV1::Lhs);
    let outer_condition_binding =
        local_binding(function, &outer_lhs).ok_or(GenericBodyEffectRejectV1::MissingBinding)?;
    if outer_binding != outer_condition_binding
        || inner_binding != shape.step().operand_read().binding()
    {
        return Err(GenericBodyEffectRejectV1::BindingMismatch);
    }
    Ok(VerifiedGenericBodyEffectProofV1 {
        root_site,
        child_site,
        inner_step_site,
        outer_step_site,
        outer_binding,
        inner_binding,
        effects: Box::new([
            GenericBodyEffectKindV1::NestedLoop,
            GenericBodyEffectKindV1::InnerBindingRebind,
            GenericBodyEffectKindV1::OuterBindingRebind,
        ]),
        _seal: GenericBodyEffectProofSealV1,
    })
}

fn issue_coverage_exit<'source>(
    function: &VerifiedResolvedFunctionV1,
    source: FunctionSyntaxViewV1<'source>,
    root_site: &SourceStmtSiteV1,
) -> Result<VerifiedGenericCoverageExitProofV1, GenericCoverageExitRejectV1> {
    let Some(SourcePathSegmentV1::Body(root_index)) = root_site.node().segments().first() else {
        return Err(GenericCoverageExitRejectV1::RootSiteMismatch);
    };
    if *root_index != 0 || source.body().len() != 2 {
        return Err(GenericCoverageExitRejectV1::IncompleteRootBody);
    }
    let child_site = child_stmt(root_site, SourcePathSegmentV1::LoopBody(0));
    let after_return_site = SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(vec![
        SourcePathSegmentV1::Body(1),
    ]));
    require_statement(function, &child_site)
        .map_err(|_| GenericCoverageExitRejectV1::MissingInventory)?;
    require_statement(function, &after_return_site)
        .map_err(|_| GenericCoverageExitRejectV1::MissingInventory)?;
    let Some(ProjectedSourceNodeV1::Node(ASTNode::Return { value: Some(_), .. })) =
        crate::mir::resolved_semantics::project_source_body_node_v1(
            source.body(),
            after_return_site.node(),
        )
    else {
        return Err(GenericCoverageExitRejectV1::MissingReturn);
    };
    if after_return_site
        .node()
        .segments()
        .starts_with(root_site.node().segments())
    {
        return Err(GenericCoverageExitRejectV1::ReturnStillInsideLoop);
    }
    if function
        .resolved_exit(&ResolvedExitSiteV1::Statement(after_return_site.clone()))
        .is_none()
    {
        return Err(GenericCoverageExitRejectV1::MissingResolvedExit);
    }
    Ok(VerifiedGenericCoverageExitProofV1 {
        root_site: root_site.clone(),
        child_site,
        after_return_site,
        kinds: Box::new([
            GenericCoverageExitKindV1::CompleteNestedWindow,
            GenericCoverageExitKindV1::ReturnAfterRootLoop,
        ]),
        _seal: GenericCoverageExitProofSealV1,
    })
}

fn project_stmt<'source>(
    source: FunctionSyntaxViewV1<'source>,
    site: &SourceStmtSiteV1,
    missing: GenericBodyEffectRejectV1,
) -> Result<&'source ASTNode, GenericBodyEffectRejectV1> {
    match crate::mir::resolved_semantics::project_source_body_node_v1(source.body(), site.node()) {
        Some(ProjectedSourceNodeV1::Node(node)) => Ok(node),
        _ => Err(missing),
    }
}

fn child_stmt(parent: &SourceStmtSiteV1, segment: SourcePathSegmentV1) -> SourceStmtSiteV1 {
    let mut segments = parent.node().segments().to_vec();
    segments.push(segment);
    SourceStmtSiteV1::from_node(SourceNodeSiteV1::from_segments(segments))
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
    function: &VerifiedResolvedFunctionV1,
    site: &SourceStmtSiteV1,
) -> Result<(), GenericBodyEffectRejectV1> {
    function
        .source_site_inventory()
        .contains_statement(site)
        .then_some(())
        .ok_or(GenericBodyEffectRejectV1::MissingInventory)
}

fn binding_target(
    function: &VerifiedResolvedFunctionV1,
    statement: &SourceStmtSiteV1,
) -> Option<BindingRefV1> {
    let target = child_expr(statement, SourcePathSegmentV1::Target);
    match function.assignment_target(&target) {
        Some(ResolvedAssignmentTargetV1::BindingRebind(binding)) => Some(*binding),
        _ => None,
    }
}

fn local_binding(
    function: &VerifiedResolvedFunctionV1,
    site: &SourceExprSiteV1,
) -> Option<BindingRefV1> {
    match function.variable_ref(site) {
        Some(ResolvedLexicalRefV1::Local(binding)) => Some(binding),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::ASTNode;
    use crate::mir::resolved_semantics::generic_resolved_carrier_source_lease::tests as lease_tests;

    fn positive_envelope() -> VerifiedGenericCandidateEnvelopeV1 {
        let unit = lease_tests::unit(lease_tests::SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let syntax_ast = lease_tests::parse_function(lease_tests::SOURCE);
        let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
        issue_generic_candidate_envelope_v1(
            input.function(),
            syntax,
            lease_tests::positive_lease(input, &root),
        )
        .expect("candidate envelope")
    }

    #[test]
    fn natural_fixture_publishes_all_typed_envelope_families() {
        let envelope = positive_envelope();
        assert_eq!(envelope.body_effect().effects().len(), 3);
        assert_eq!(envelope.coverage_exit().kinds().len(), 2);
        assert_eq!(
            envelope.body_effect().inner_binding(),
            envelope.syntax().step().lhs()
        );
        assert_eq!(
            envelope.coverage_exit().root_site(),
            envelope.body_effect().root_site()
        );
    }

    #[test]
    fn body_effect_rejects_extra_statement_before_publication() {
        let unit = lease_tests::unit(lease_tests::SOURCE);
        let (input, root) = lease_tests::input_and_root(&unit);
        let mut syntax_ast = lease_tests::parse_function(lease_tests::SOURCE);
        let ASTNode::FunctionDeclaration { body, .. } = &mut syntax_ast else {
            panic!("function fixture")
        };
        let duplicate = body[1].clone();
        body.insert(1, duplicate);
        let syntax = FunctionSyntaxViewV1::from_ast(&syntax_ast).expect("function view");
        assert_eq!(
            issue_generic_candidate_envelope_v1(
                input.function(),
                syntax,
                lease_tests::positive_lease(input, &root),
            ),
            Err(GenericCandidateEnvelopeRejectV1::CoverageExit(
                GenericCoverageExitRejectV1::IncompleteRootBody,
            ))
        );
    }

    #[test]
    fn envelope_can_outlive_borrowed_source_view() {
        let envelope = positive_envelope();
        assert_eq!(
            envelope.syntax().carrier().carrier().proof().binding(),
            envelope.body_effect().inner_binding()
        );
    }
}
