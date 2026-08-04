use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::mir::resolved_region_flow::ResolvedElseFallthroughV1;
use crate::mir::resolved_semantics::SourcePathSegmentV1;

use super::capability::{
    CanonicalFirstFamilyPlanV1, CanonicalLoopFamilyPlanV1, CanonicalLoweringPreflightV1,
    ResolvedOwnerHeaderFamilyV1, ResolvedOwnerHeaderSealErrorV1,
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

fn bool_assignment(name: &str, value: bool) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(bool_literal(value)),
        span: Span::unknown(),
    }
}

fn binary(operator: BinaryOperator, left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator,
        left: Box::new(left),
        right: Box::new(right),
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
    let (input, loop_stmt, receipt, prefix, _recipe, effect_plan, completion) = plan.into_parts();

    assert_eq!(loop_stmt.owner(), input.owner());
    assert_eq!(prefix.owner(), input.owner());
    assert_eq!(prefix.locals().len(), 2);
    for (ordinal, local) in prefix.locals().iter().enumerate() {
        assert_eq!(local.binding().owner(), input.owner());
        assert_eq!(local.initial(), 0);
        assert_eq!(
            local.kind(),
            crate::mir::resolved_semantics::BindingKindV1::Local {
                ordinal: ordinal as u32,
            }
        );
    }
    let source = input
        .function()
        .resolved_loop_source(loop_stmt.site())
        .expect("loop source");
    assert!(receipt.frame_key().matches(&source.frame_key()));
    assert_eq!(effect_plan.entries().len(), 5);
    assert!(completion.is_implicit_void());
}

#[test]
fn direct_accum_first_family_plan_reuses_binding_ssa_header_identity() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
    let plan = super::direct_accum_capability::verify_direct_accum_first_family_function_v1(
        input, loop_stmt,
    )
    .expect("DirectAccum first-family plan");

    let CanonicalFirstFamilyPlanV1::Loop(CanonicalLoopFamilyPlanV1::DirectAccum(plan)) = plan
    else {
        panic!("DirectAccum issuer must mint the central family-plan variant")
    };
    let header = plan
        .seal_resolved_owner_header_v1()
        .expect("DirectAccum header");

    assert_eq!(
        header.family(),
        ResolvedOwnerHeaderFamilyV1::TrivialBindingSsa
    );
    assert_eq!(header.symbol().as_mir_name(), "accum/0");
    assert_eq!(header.arity(), 0);
    assert_eq!(header.owner(), plan.input().owner());
}

#[test]
fn canonical_preflight_admits_direct_accum_as_the_single_loop_family() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");

    assert!(matches!(
        CanonicalLoweringPreflightV1::verify(&unit),
        Ok(CanonicalFirstFamilyPlanV1::Loop(
            CanonicalLoopFamilyPlanV1::DirectAccum(_),
        ))
    ));
}

#[test]
fn compile_resolved_direct_accum_uses_source_bound_candidate_commit() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let mut compiler = MirCompiler::with_options(false);

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("direct_accum.hako"))
        .expect("DirectAccum source-bound compilation");

    assert!(result.verification_result.is_ok());
    assert_eq!(result.module.functions.len(), 1);
}

#[test]
fn compile_resolved_direct_accum_reuses_one_compiler_after_commit() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let mut compiler = MirCompiler::with_options(false);

    for source_file in ["direct_accum-first.hako", "direct_accum-second.hako"] {
        let result = compiler
            .compile_resolved(unit.lowering_input(), Some(source_file))
            .expect("fresh source-bound DirectAccum compilation");
        assert!(result.verification_result.is_ok());
        assert_eq!(result.module.functions.len(), 1);
    }
}

#[test]
fn direct_accum_candidate_lowerer_consumes_one_canonical_session() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
    let plan = super::direct_accum_capability::verify_direct_accum_function_v1(input, loop_stmt)
        .expect("DirectAccum plan");
    let current = crate::mir::builder::MirBuilder::new();
    let mut candidate = super::module_session::CanonicalModuleLoweringSessionV1::open(&current);
    let draft = candidate
        .builder_mut()
        .lower_resolved_direct_accum_function_draft(plan)
        .expect("candidate DirectAccum draft");
    assert_eq!(draft.signature.name, "accum/0");
    assert_eq!(draft.blocks.len(), 5);
}

#[test]
fn direct_accum_candidate_discards_after_late_draft_seal_failure() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(
        super::direct_accum_projection::direct_accum_function_for_test(),
    )
    .expect("DirectAccum fixture resolves");
    let input = unit.root_function_input().expect("function input");
    let body = input.source().root_body().expect("root body");
    let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
    let plan = super::direct_accum_capability::verify_direct_accum_function_v1(input, loop_stmt)
        .expect("DirectAccum plan");
    let current = crate::mir::builder::MirBuilder::new();
    let mut candidate = super::module_session::CanonicalModuleLoweringSessionV1::open(&current);
    let error = candidate
        .builder_mut()
        .lower_resolved_direct_accum_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late draft-seal failure must reject");
    assert!(matches!(
        error,
        super::CanonicalResolvedBuildErrorV1::BuilderContract(_)
    ));
    assert!(current.current_function_name().is_none());
    assert!(current.current_function_entry_block().is_none());
}

#[test]
fn direct_accum_candidate_can_be_reopened_after_a_discard() {
    let current = crate::mir::builder::MirBuilder::new();
    for _ in 0..2 {
        let unit = VerifiedResolvedSourceUnitV1::resolve_function(
            super::direct_accum_projection::direct_accum_function_for_test(),
        )
        .expect("DirectAccum fixture resolves");
        let input = unit.root_function_input().expect("function input");
        let body = input.source().root_body().expect("root body");
        let loop_stmt = input.source().body_stmt(&body, 1).expect("loop statement");
        let plan =
            super::direct_accum_capability::verify_direct_accum_function_v1(input, loop_stmt)
                .expect("DirectAccum plan");
        let mut candidate = super::module_session::CanonicalModuleLoweringSessionV1::open(&current);
        let draft = candidate
            .builder_mut()
            .lower_resolved_direct_accum_function_draft(plan)
            .expect("fresh candidate DirectAccum draft");
        assert_eq!(draft.signature.name, "accum/0");
        assert_eq!(draft.blocks.len(), 5);
    }
    assert!(current.current_function_name().is_none());
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
fn if_recipe_candidate_discards_after_late_draft_seal_failure_and_reuses_compiler() {
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
    .expect("If fixture resolves");
    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();

    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("If plan")
    else {
        panic!("explicit-else If fixture must select trivial Binding SSA")
    };
    let mut candidate =
        super::module_session::CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    let error = candidate
        .builder_mut()
        .lower_resolved_trivial_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late draft-seal failure must reject after If PHI work");
    assert!(matches!(
        error,
        super::CanonicalResolvedBuildErrorV1::BuilderContract(detail)
            if detail.contains("DraftSeal") || detail.contains("draft_seal")
    ));
    drop(candidate);

    assert_eq!(compiler.builder.loop_candidate_test_fingerprint(), before);
    assert!(compiler.builder.current_module.is_none());
    assert!(compiler.builder.current_function_name().is_none());
    assert!(compiler.builder.current_function_entry_block().is_none());

    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("reused-if.hako"))
        .expect("same compiler must accept a fresh If request");
    assert!(result.verification_result.is_ok());
    assert_eq!(result.module.functions.len(), 2);
    assert!(result.module.functions.contains_key("capability_fixture/0"));
}

#[test]
fn if_recipe_selected_shape_preserves_branch_merge_and_phi_receipt() {
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
    .expect("If fixture resolves");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("if-receipt.hako"))
        .expect("selected If physicalization");
    let function = result
        .module
        .functions
        .get("capability_fixture/0")
        .expect("selected If function");
    let branch_count = function
        .blocks
        .values()
        .filter(|block| {
            matches!(
                block.terminator,
                Some(crate::mir::MirInstruction::Branch { .. })
            )
        })
        .count();
    let phi_inputs = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Phi { inputs, .. } => Some(inputs.len()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(branch_count, 1);
    assert_eq!(phi_inputs, vec![2]);
}

#[test]
fn if_recipe_selected_bool_merge_preserves_bool_phi_receipt() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("flag", bool_literal(true)),
        if_stmt(
            bool_literal(true),
            vec![bool_assignment("flag", false)],
            Some(vec![bool_assignment("flag", true)]),
        ),
        binary(BinaryOperator::Equal, variable("flag"), bool_literal(true)),
    ]))
    .expect("Bool If fixture resolves");
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("Bool If plan")
    else {
        panic!("Bool explicit-else If must select trivial Binding SSA")
    };

    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("if-bool-receipt.hako"))
        .expect("selected Bool If physicalization");
    let function = result
        .module
        .functions
        .get("capability_fixture/0")
        .expect("selected Bool If function");
    let bool_values = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Const {
                dst,
                value: crate::mir::ConstValue::Bool(_),
            } => Some(*dst),
            _ => None,
        })
        .collect::<std::collections::HashSet<_>>();
    let phis = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Phi {
                inputs, type_hint, ..
            } => Some((inputs.clone(), type_hint.clone())),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(phis.len(), 1);
    assert_eq!(phis[0].0.len(), 2);
    assert!(phis[0].1.is_none() || phis[0].1 == Some(crate::mir::MirType::Bool));
    assert!(phis[0]
        .0
        .iter()
        .all(|(_, value)| bool_values.contains(value)));
}

#[test]
fn if_recipe_selected_implicit_fallthrough_uses_header_baseline_phi_input() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(function(vec![
        local("x", literal(0)),
        if_stmt(bool_literal(true), vec![assignment("x", 1)], None),
        ASTNode::Return {
            value: Some(Box::new(variable("x"))),
            span: Span::unknown(),
        },
    ]))
    .expect("implicit If fixture resolves");
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(_) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("implicit If plan")
    else {
        panic!("implicit fallthrough If must select trivial Binding SSA")
    };

    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("if-implicit-receipt.hako"))
        .expect("selected implicit If physicalization");
    let function = result
        .module
        .functions
        .get("capability_fixture/0")
        .expect("selected implicit If function");
    let phi_inputs = function
        .blocks
        .values()
        .flat_map(|block| block.instructions.iter())
        .filter_map(|instruction| match instruction {
            crate::mir::MirInstruction::Phi { inputs, .. } => Some(inputs.clone()),
            _ => None,
        })
        .collect::<Vec<_>>();
    assert_eq!(phi_inputs.len(), 1);
    assert_eq!(phi_inputs[0].len(), 2);
    let merge = function
        .blocks
        .values()
        .find(|block| {
            block
                .instructions
                .iter()
                .any(|instruction| matches!(instruction, crate::mir::MirInstruction::Phi { .. }))
        })
        .expect("implicit merge block");
    let input_blocks = phi_inputs[0]
        .iter()
        .map(|(block, _)| *block)
        .collect::<std::collections::BTreeSet<_>>();
    assert_eq!(input_blocks, merge.predecessors);
    assert_eq!(input_blocks.len(), 2);
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
