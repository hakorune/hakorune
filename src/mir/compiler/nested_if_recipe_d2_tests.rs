#![cfg(feature = "vm-reference")]

use std::collections::BTreeSet;

use crate::ast::{ASTNode, DeclarationAttrs, LiteralValue, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::verification::utils::compute_predecessors;
use crate::mir::{MirInstruction, MirType};

use super::capability::{CanonicalFirstFamilyPlanV1, CanonicalLoweringPreflightV1};
use super::{MirCompiler, VerifiedResolvedSourceUnitV1};

fn integer(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: Span::unknown(),
    }
}

fn boolean(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_owned(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_owned()],
        initial_values: vec![Some(Box::new(value))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(integer(value)),
        span: Span::unknown(),
    }
}

fn if_statement(
    condition: bool,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(boolean(condition)),
        then_body,
        else_body,
        span: Span::unknown(),
    }
}

fn nested_function(name: &str, outer_condition: bool, inner_condition: bool) -> ASTNode {
    ASTNode::FunctionDeclaration {
        name: name.to_owned(),
        params: Vec::new(),
        param_decls: Vec::new(),
        return_type_name: None,
        body: vec![
            local("x", integer(0)),
            if_statement(
                outer_condition,
                vec![if_statement(
                    inner_condition,
                    vec![assignment("x", 1)],
                    Some(vec![assignment("x", 2)]),
                )],
                Some(vec![assignment("x", 3)]),
            ),
            ASTNode::Return {
                value: Some(Box::new(variable("x"))),
                span: Span::unknown(),
            },
        ],
        uses: Vec::new(),
        contracts: Vec::new(),
        is_static: true,
        is_override: false,
        attrs: DeclarationAttrs::default(),
        span: Span::unknown(),
    }
}

fn compile(root: ASTNode, source_file: &str) -> super::MirCompileResult {
    let unit =
        VerifiedResolvedSourceUnitV1::resolve_function(root).expect("nested fixture resolves");
    let mut compiler = MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some(source_file))
        .expect("nested fixture lowers");
    assert!(
        result.verification_result.is_ok(),
        "{:?}",
        result.verification_result
    );
    result
}

fn assert_two_phi_receipt(function: &crate::mir::MirFunction) {
    let predecessors = compute_predecessors(function);
    let phi_blocks = function
        .blocks
        .iter()
        .filter_map(|(block_id, block)| block.phi_instructions().next().map(|_| *block_id))
        .collect::<BTreeSet<_>>();
    assert_eq!(
        phi_blocks.len(),
        2,
        "nested recipe must publish two merge PHIs"
    );

    let mut nested_merge_links = 0;
    let mut phi_count = 0;
    for (block_id, block) in &function.blocks {
        for instruction in block.phi_instructions() {
            let MirInstruction::Phi {
                inputs, type_hint, ..
            } = instruction
            else {
                unreachable!()
            };
            assert!(type_hint.is_none() || *type_hint == Some(MirType::Integer));
            let input_predecessors = inputs
                .iter()
                .map(|(predecessor, _)| *predecessor)
                .collect::<BTreeSet<_>>();
            let actual_predecessors = predecessors
                .get(block_id)
                .into_iter()
                .flatten()
                .copied()
                .collect::<BTreeSet<_>>();
            assert_eq!(input_predecessors, actual_predecessors);
            nested_merge_links += input_predecessors
                .iter()
                .filter(|predecessor| phi_blocks.contains(predecessor))
                .count();
            phi_count += 1;
        }
    }
    assert_eq!(phi_count, 2);
    assert_eq!(
        nested_merge_links, 1,
        "inner merge must feed the outer then edge"
    );
}

#[test]
fn nested_recipe_produces_all_three_constant_outcomes_and_two_phis() {
    for (name, outer, inner, expected) in [
        ("nested_recipe_d2_then_inner", true, true, 1),
        ("nested_recipe_d2_inner_else", true, false, 2),
        ("nested_recipe_d2_outer_else", false, true, 3),
    ] {
        let result = compile(nested_function(name, outer, inner), "nested-recipe-d2.hako");
        let function = &result.module.functions[&format!("{name}/0")];
        assert_two_phi_receipt(function);
        let actual = MirInterpreter::new()
            .execute_function_with_args(&result.module, &format!("{name}/0"), &[])
            .expect("nested recipe executes");
        assert_eq!(actual, VMValue::Integer(expected), "fixture={name}");
    }
}

#[test]
fn nested_recipe_candidate_abort_preserves_live_state_and_reuses_compiler() {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function(
        "nested_recipe_d2_abort",
        true,
        true,
    ))
    .expect("nested abort fixture resolves");
    let CanonicalFirstFamilyPlanV1::TrivialBindingSsa(plan) =
        CanonicalLoweringPreflightV1::verify(&unit).expect("nested recipe plan")
    else {
        panic!("nested fixture must select the canonical trivial owner")
    };

    let mut compiler = MirCompiler::with_options(false);
    compiler.builder.set_source_file_hint("before.hako");
    compiler.builder.next_value_id();
    compiler.builder.next_block_id();
    let before = compiler.builder.loop_candidate_test_fingerprint();
    let mut candidate =
        super::module_session::CanonicalModuleLoweringSessionV1::open(&compiler.builder);
    let error = candidate
        .builder_mut()
        .lower_resolved_trivial_function_draft_with_seal_failure_for_test(plan)
        .expect_err("late nested draft-seal failure must reject after both PHIs");
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
        .compile_resolved(unit.lowering_input(), Some("nested-recipe-d2-reuse.hako"))
        .expect("same compiler must accept fresh nested request");
    assert!(result.verification_result.is_ok());
    assert!(result
        .module
        .functions
        .contains_key("nested_recipe_d2_abort/0"));
}
