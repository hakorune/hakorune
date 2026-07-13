use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::resolved_semantics::{SourcePathSegmentV1, SourcePathV1};

use super::analyzer::{analyze_resolved_function_flow_v1, ResolvedRegionFlowErrorV1};
use super::coverage::IfFlowCoverageDraftV1;
use super::if_flow::{
    ResolvedFunctionFlowDraftV1, ResolvedIfFlowDraftV1, VerifiedResolvedFunctionFlowV1,
};
use super::ports::{ResolvedElseFallthroughV1, ResolvedIfPortValueSourceV1};
use super::verifier::{verify_if_flow_draft, ResolvedRegionFlowVerificationErrorV1};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(literal(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(literal(value)),
        span: Span::unknown(),
    }
}

fn if_stmt(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn function(body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: "flow_fixture".into(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body,
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

/// The returned product outlives every borrowed source carrier in this helper.
fn analyze_owned(body: Vec<ASTNode>) -> VerifiedResolvedFunctionFlowV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(body)).unwrap();
    analyze_resolved_function_flow_v1(unit.root_function_input().unwrap()).unwrap()
}

fn branch(rebind: bool, value: i64) -> Vec<ASTNode> {
    rebind.then(|| assignment("x", value)).into_iter().collect()
}

#[test]
fn no_if_function_publishes_empty_lifetime_free_flow() {
    let flow = analyze_owned(vec![local("x", 0), assignment("x", 1)]);

    assert!(flow.if_flows().is_empty());
    assert_eq!(flow.coverage().function_direct().len(), 1);
}

#[test]
fn one_and_two_sided_rebinds_cover_the_join_source_matrix() {
    let then_only = analyze_owned(vec![
        local("x", 0),
        if_stmt(literal(1), branch(true, 1), None),
    ]);
    let then_row = &then_only.if_flows()[0].join().rows()[0];
    assert_eq!(
        then_row.then_source(),
        ResolvedIfPortValueSourceV1::BranchExit
    );
    assert_eq!(
        then_row.else_source(),
        ResolvedIfPortValueSourceV1::PostConditionEntry
    );

    let else_only = analyze_owned(vec![
        local("x", 0),
        if_stmt(literal(1), branch(false, 1), Some(branch(true, 2))),
    ]);
    let else_row = &else_only.if_flows()[0].join().rows()[0];
    assert_eq!(
        else_row.then_source(),
        ResolvedIfPortValueSourceV1::PostConditionEntry
    );
    assert_eq!(
        else_row.else_source(),
        ResolvedIfPortValueSourceV1::BranchExit
    );

    let both = analyze_owned(vec![
        local("x", 0),
        if_stmt(literal(1), branch(true, 1), Some(branch(true, 2))),
    ]);
    let both_row = &both.if_flows()[0].join().rows()[0];
    assert_eq!(
        both_row.then_source(),
        ResolvedIfPortValueSourceV1::BranchExit
    );
    assert_eq!(
        both_row.else_source(),
        ResolvedIfPortValueSourceV1::BranchExit
    );
}

#[test]
fn condition_blockexpr_effects_remain_separate_from_branch_join_rows() {
    let condition = ASTNode::BlockExpr {
        prelude_stmts: vec![assignment("x", 1)],
        tail_expr: Box::new(literal(1)),
        span: Span::unknown(),
    };
    let flow = analyze_owned(vec![local("x", 0), if_stmt(condition, Vec::new(), None)]);
    let row = &flow.if_flows()[0];

    assert_eq!(row.condition_effects().may_rebind_outer().len(), 1);
    assert!(row.then_port().may_rebind_outer().is_empty());
    assert!(row.join().rows().is_empty());
    assert_eq!(row.whole_effects().may_rebind_outer().len(), 1);
    assert_eq!(row.coverage().condition_direct().len(), 1);
    assert!(row.coverage().then_direct().is_empty());
}

#[test]
fn implicit_identity_and_explicit_empty_else_remain_distinct() {
    let implicit = analyze_owned(vec![if_stmt(literal(1), Vec::new(), None)]);
    let explicit = analyze_owned(vec![if_stmt(literal(1), Vec::new(), Some(Vec::new()))]);
    let implicit_row = &implicit.if_flows()[0];
    let explicit_row = &explicit.if_flows()[0];

    assert!(matches!(
        implicit_row.else_port(),
        ResolvedElseFallthroughV1::ImplicitIdentity
    ));
    assert!(implicit_row.regions().else_pair().is_none());
    let explicit_port = explicit_row
        .else_port()
        .explicit_port()
        .expect("explicit empty else port");
    assert!(explicit_port.may_rebind_outer().is_empty());
    assert!(explicit_row.regions().else_pair().is_some());
}

#[test]
fn explicit_else_source_rejects_a_false_syntax_has_else_draft() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![if_stmt(
        literal(1),
        Vec::new(),
        Some(Vec::new()),
    )]))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let site = SourcePathV1::root_body(0).stmt();
    let draft = ResolvedIfFlowDraftV1::new(
        site.clone(),
        false,
        Vec::new(),
        Vec::new(),
        None,
        IfFlowCoverageDraftV1::default(),
    );

    assert!(matches!(
        verify_if_flow_draft(input.function(), draft),
        Err(ResolvedRegionFlowVerificationErrorV1::OptionalElseMismatch {
            site: actual,
            syntax_has_else: false,
            bundle_has_else: true,
        }) if actual == site
    ));
}

#[test]
fn branch_local_same_name_shadow_is_excluded_from_ports_and_join() {
    let flow = analyze_owned(vec![
        local("x", 0),
        if_stmt(literal(1), vec![local("x", 1), assignment("x", 2)], None),
    ]);
    let row = &flow.if_flows()[0];

    assert!(row.then_port().may_rebind_outer().is_empty());
    assert!(row.join().rows().is_empty());
    assert!(row.whole_effects().may_rebind_outer().is_empty());
    assert_eq!(row.coverage().then_direct().len(), 1);
}

#[test]
fn nested_if_publishes_preorder_and_parent_consumes_child_summary() {
    let nested = if_stmt(literal(1), vec![assignment("x", 2)], None);
    let flow = analyze_owned(vec![local("x", 0), if_stmt(literal(1), vec![nested], None)]);
    let [outer, child] = flow.if_flows() else {
        panic!("expected outer and child flow rows")
    };

    assert_eq!(
        outer.site().node().segments(),
        &[SourcePathSegmentV1::Body(1)]
    );
    assert_eq!(
        child.site().node().segments(),
        &[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::IfThen(0),]
    );
    assert!(outer.coverage().then_direct().is_empty());
    assert_eq!(child.coverage().then_direct().len(), 1);
    assert_eq!(outer.then_port().may_rebind_outer().len(), 1);
    assert_eq!(child.whole_effects().may_rebind_outer().len(), 1);
    assert_eq!(
        outer.then_port().may_rebind_outer(),
        child.whole_effects().may_rebind_outer()
    );
}

#[test]
fn same_span_if_sites_remain_distinct_in_source_preorder() {
    let flow = analyze_owned(vec![
        if_stmt(literal(1), Vec::new(), None),
        if_stmt(literal(1), Vec::new(), Some(Vec::new())),
    ]);
    let [first, second] = flow.if_flows() else {
        panic!("expected two exact If rows")
    };

    assert_ne!(first.site(), second.site());
    assert_eq!(
        first.site().node().segments(),
        &[SourcePathSegmentV1::Body(0)]
    );
    assert_eq!(
        second.site().node().segments(),
        &[SourcePathSegmentV1::Body(1)]
    );
}

#[test]
fn missing_and_duplicate_assignment_coverage_publish_no_product() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", 0),
        assignment("x", 1),
    ]))
    .unwrap();
    let input = unit.root_function_input().unwrap();
    let site = input
        .function()
        .assignment_targets()
        .next()
        .unwrap()
        .0
        .clone();

    let missing = ResolvedFunctionFlowDraftV1::new(input.owner()).seal(input.function());
    assert!(matches!(
        missing,
        Err(ResolvedRegionFlowVerificationErrorV1::MissingAssignmentCoverage(actual))
            if actual == site
    ));

    let mut duplicate = ResolvedFunctionFlowDraftV1::new(input.owner());
    duplicate.coverage_mut().record_direct(site.clone());
    duplicate.coverage_mut().record_direct(site.clone());
    assert!(matches!(
        duplicate.seal(input.function()),
        Err(ResolvedRegionFlowVerificationErrorV1::DuplicateAssignmentCoverage(actual))
            if actual == site
    ));
}

#[test]
fn unsupported_branch_exit_returns_error_without_partial_product() {
    let ast = function(vec![if_stmt(
        literal(1),
        vec![ASTNode::Return {
            value: Some(Box::new(literal(1))),
            span: Span::unknown(),
        }],
        None,
    )]);
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(ast).unwrap();
    let result = analyze_resolved_function_flow_v1(unit.root_function_input().unwrap());

    assert!(matches!(
        result,
        Err(ResolvedRegionFlowErrorV1::UnsupportedStatement { .. })
    ));
}
