use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_region_flow::ResolvedElseFallthroughV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1, ResolvedOwnerHeaderFamilyV1,
    ResolvedOwnerHeaderSealErrorV1,
};
use super::{CanonicalLoweringErrorV1, MirCompiler, VerifiedResolvedSourceUnitV1};

fn literal(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn bool_literal(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(value))],
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

fn block_expr(prelude_stmts: Vec<ASTNode>, tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts,
        tail_expr: Box::new(tail),
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
    named_function("capability_fixture", body)
}

fn named_function(name: &str, body: Vec<ASTNode>) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.into(),
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

#[test]
fn direct_accum_preflight_issues_one_whole_function_plan() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
    let plan = super::direct_accum_capability::verify_direct_accum_function_v1(input, loop_stmt)
        .expect("DirectAccum plan");
    let (input, loop_stmt, receipt, _recipe, effect_plan, completion) = plan.into_parts();

    assert_eq!(loop_stmt.owner(), input.owner());
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("loop source");
    assert!(receipt.frame_key().matches(&source.frame_key()));
    assert_eq!(effect_plan.entries().len(), 5);
    assert!(completion.is_implicit_void());
}

#[test]
fn resolved_owner_header_seals_zero_arity_binding_ssa_before_plan_consumption() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", literal(0)),
        if_stmt(
            bool_literal(true),
            vec![assignment("x", 1)],
            Some(vec![assignment("x", 2)]),
        ),
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]))
    .unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();

    let header = plan.seal_resolved_owner_header_v1().unwrap();

    assert_eq!(
        header.family(),
        ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
    );
    assert_eq!(header.arity(), 0);
    assert_eq!(header.symbol().as_mir_name(), "capability_fixture/0");
    header.require_same_plan(&plan).unwrap();

    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) = plan else {
        panic!("fixture must retain the exact Binding-SSA family")
    };
    let (input, ..) = plan.into_parts();
    assert_eq!(header.owner(), input.owner());
}

#[test]
fn resolved_owner_header_seals_a_plus_family_without_exact_i64_profile() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(named_function(
        "a_plus_header",
        vec![
            local("x", literal(0)),
            if_stmt(literal(1), vec![assignment("x", 1)], None),
        ],
    ))
    .unwrap();
    let plan = CanonicalLoweringPreflightV1::verify(&unit).unwrap();

    let header = plan.seal_resolved_owner_header_v1().unwrap();

    assert_eq!(
        header.family(),
        ResolvedOwnerHeaderFamilyV1::CurrentCanonicalAPlus
    );
    assert_eq!(header.symbol().as_mir_name(), "a_plus_header/0");
    assert_eq!(header.arity(), 0);
    header.require_same_plan(&plan).unwrap();
}

#[test]
fn resolved_owner_header_rejects_foreign_plan_pairing() {
    let first = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![literal(1)])).unwrap();
    let foreign = VerifiedResolvedSourceUnitV1::resolve_function(named_function(
        "foreign_a_plus",
        vec![
            local("x", literal(0)),
            if_stmt(literal(1), vec![assignment("x", 1)], None),
        ],
    ))
    .unwrap();
    let first_plan = CanonicalLoweringPreflightV1::verify(&first).unwrap();
    let foreign_plan = CanonicalLoweringPreflightV1::verify(&foreign).unwrap();
    let header = first_plan.seal_resolved_owner_header_v1().unwrap();

    assert!(matches!(
        header.require_same_plan(&foreign_plan),
        Err(ResolvedOwnerHeaderSealErrorV1::ForeignPlan { .. })
    ));
}

fn unsupported_branch(statement: ASTNode) -> VerifiedResolvedSourceUnitV1 {
    VerifiedResolvedSourceUnitV1::resolve_function(function(vec![if_stmt(
        literal(1),
        vec![statement],
        None,
    )]))
    .unwrap()
}

#[test]
fn preflight_owns_nested_if_flow_with_blockexpr_condition_and_optional_else() {
    let inner = if_stmt(literal(1), vec![assignment("x", 2)], None);
    let condition = block_expr(vec![assignment("x", 1)], literal(1));
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", literal(0)),
        if_stmt(condition, vec![inner], Some(Vec::new())),
    ]))
    .unwrap();

    let CanonicalFirstFamilyPlanV1::CurrentCanonicalAPlus(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).unwrap()
    else {
        panic!("non-Bool If condition must select the temporary A+ whole-unit route")
    };
    let (input, flow, completion, block_expr_count) = plan.into_parts();
    let [outer, inner] = flow.if_flows() else {
        panic!("expected exact outer and nested If rows")
    };

    assert_eq!(flow.owner(), input.owner());
    assert!(completion.is_implicit_void());
    assert_eq!(block_expr_count, 1);
    assert_eq!(
        outer.site().node().segments(),
        &[SourcePathSegmentV1::Body(1)]
    );
    assert_eq!(
        inner.site().node().segments(),
        &[SourcePathSegmentV1::Body(1), SourcePathSegmentV1::IfThen(0),]
    );
    assert_eq!(outer.condition_effects().may_rebind_outer().len(), 1);
    assert!(outer.regions().else_pair().is_some());
    assert!(matches!(
        outer.else_port(),
        ResolvedElseFallthroughV1::Explicit(_)
    ));
    assert!(inner.regions().else_pair().is_none());
    assert!(matches!(
        inner.else_port(),
        ResolvedElseFallthroughV1::ImplicitIdentity
    ));
}

#[test]
fn preflight_selects_trivial_binding_ssa_with_carrier_free_if_control() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", literal(0)),
        if_stmt(
            bool_literal(true),
            vec![assignment("x", 1)],
            Some(vec![assignment("x", 2)]),
        ),
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]))
    .unwrap();

    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).unwrap()
    else {
        panic!("exact homogeneous trivial owner must select Binding SSA")
    };
    let (input, if_control, completion, profile, block_expr_count) = plan.into_parts();

    assert_eq!(profile.owner(), input.owner());
    assert_eq!(if_control.owner(), input.owner());
    assert_eq!(if_control.row_count(), 1);
    assert_eq!(if_control.explicit_else_count(), 1);
    assert!(completion.returns_value());
    assert_eq!(block_expr_count, 0);
}

#[test]
fn preflight_rejects_nonfallthrough_branch_routes() {
    let return_unit = unsupported_branch(ASTNode::Return {
        value: Some(Box::new(literal(1))),
        span: Span::unknown(),
    });
    let loop_unit = unsupported_branch(ASTNode::Loop {
        condition: Box::new(literal(1)),
        body: Vec::new(),
        span: Span::unknown(),
    });

    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&return_unit),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "return_not_allowed_here",
            ..
        })
    ));
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&loop_unit),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "statement_not_in_first_family",
            ..
        })
    ));
}

#[test]
fn preflight_preserves_root_return_error_priority_and_rejects_outer_loop() {
    let nonterminal_return = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        ASTNode::Return {
            value: Some(Box::new(literal(1))),
            span: Span::unknown(),
        },
        literal(2),
    ]))
    .unwrap();
    let outer_loop =
        VerifiedResolvedSourceUnitV1::resolve_function(function(vec![ASTNode::Loop {
            condition: Box::new(literal(1)),
            body: Vec::new(),
            span: Span::unknown(),
        }]))
        .unwrap();

    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&nonterminal_return),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "return_not_allowed_here",
            ..
        })
    ));
    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&outer_loop),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape {
            reason: "statement_not_in_first_family",
            ..
        })
    ));
}

#[test]
fn compile_resolved_preflight_error_leaves_builder_unopened() {
    let unit = unsupported_branch(ASTNode::Return {
        value: Some(Box::new(literal(1))),
        span: Span::unknown(),
    });
    let mut compiler = MirCompiler::with_options(false);

    assert!(compiler.builder.current_module.is_none());
    assert!(matches!(
        compiler.compile_resolved(unit.lowering_input(), None),
        Err(CanonicalLoweringErrorV1::UnsupportedFirstFamilyShape { .. })
    ));
    assert!(compiler.builder.current_module.is_none());
}
