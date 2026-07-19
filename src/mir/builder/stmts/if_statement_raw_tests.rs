use std::collections::BTreeSet;

use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::function::{
    FastMemBranchConditionProofKind, FastMemRegionMetadata, FastMemRegionOrigin,
};
use crate::mir::instruction::FastMemRegionId;
use crate::mir::{BasicBlockId, ConstValue, MirBuilder, MirInstruction, ValueId};

fn bool_lit(value: bool) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Bool(value),
        span: Span::unknown(),
    }
}

fn int_lit(value: i64) -> ASTNode {
    int_lit_at(value, Span::unknown())
}

fn int_lit_at(value: i64, span: Span) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span,
    }
}

fn variable(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: Span::unknown(),
    }
}

fn local(name: &str, value: i64) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.to_string()],
        initial_values: vec![Some(Box::new(int_lit(value)))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assignment(name: &str, value: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(variable(name)),
        value: Box::new(int_lit(value)),
        span: Span::unknown(),
    }
}

fn return_int(value: i64) -> ASTNode {
    ASTNode::Return {
        value: Some(Box::new(int_lit(value))),
        span: Span::unknown(),
    }
}

fn statement_if(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
) -> ASTNode {
    statement_if_at(condition, then_body, else_body, Span::unknown())
}

fn statement_if_at(
    condition: ASTNode,
    then_body: Vec<ASTNode>,
    else_body: Option<Vec<ASTNode>>,
    span: Span,
) -> ASTNode {
    ASTNode::If {
        condition: Box::new(condition),
        then_body,
        else_body,
        span,
    }
}

fn owner_eq_condition() -> ASTNode {
    ASTNode::FunctionCall {
        name: "mem.ownerEq".to_string(),
        arguments: vec![int_lit(1), int_lit(1)],
        span: Span::unknown(),
    }
}

fn builder(name: &str) -> MirBuilder {
    crate::runtime::ring0::ensure_global_ring0_initialized();
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test(name.to_string());
    builder
}

fn void_values(builder: &MirBuilder) -> Vec<(BasicBlockId, ValueId)> {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .iter()
        .flat_map(|(block, body)| {
            body.instructions
                .iter()
                .filter_map(move |instruction| match instruction {
                    MirInstruction::Const {
                        dst,
                        value: ConstValue::Void,
                    } => Some((*block, *dst)),
                    _ => None,
                })
        })
        .collect()
}

fn branch_count(builder: &MirBuilder) -> usize {
    builder
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .blocks
        .values()
        .filter(|block| matches!(block.terminator, Some(MirInstruction::Branch { .. })))
        .count()
}

fn assert_simple_statement_if_shape(
    builder: &MirBuilder,
    then_returns: bool,
    else_returns: bool,
) -> (BasicBlockId, BasicBlockId, BasicBlockId) {
    let function = builder.function_state.current_function.as_ref().unwrap();
    let (then_bb, else_bb) = function
        .blocks
        .values()
        .find_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Branch {
                then_bb, else_bb, ..
            }) => Some((*then_bb, *else_bb)),
            _ => None,
        })
        .expect("production statement If header");
    let merge_bb = builder
        .function_state
        .current_block
        .expect("production statement If merge");

    for (label, block_id, returns) in [
        ("then", then_bb, then_returns),
        ("else", else_bb, else_returns),
    ] {
        let block = &function.blocks[&block_id];
        if returns {
            assert!(
                matches!(block.terminator, Some(MirInstruction::Return { .. })),
                "{label}: {:?}",
                block.terminator
            );
        } else {
            assert!(
                matches!(block.terminator, Some(MirInstruction::Jump { target, .. }) if target == merge_bb),
                "{label}: {:?}",
                block.terminator
            );
        }
    }

    let expected_predecessors = if !then_returns && !else_returns {
        BTreeSet::from([then_bb, else_bb])
    } else {
        BTreeSet::new()
    };
    assert_eq!(
        function.blocks[&merge_bb].predecessors,
        expected_predecessors
    );
    (then_bb, else_bb, merge_bb)
}

#[test]
fn production_statement_if_explicit_else_publishes_merge_phis_then_facade_void() {
    let mut builder = builder("production_statement_if_explicit_else/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    super::block_stmt::build_statement(&mut builder, local("x", 0)).unwrap();

    let output = super::block_stmt::build_statement(
        &mut builder,
        statement_if(
            bool_lit(true),
            vec![assignment("x", 1)],
            Some(vec![assignment("x", 2)]),
        ),
    )
    .unwrap();

    let (then_bb, else_bb, merge_bb) = assert_simple_statement_if_shape(&builder, false, false);
    let function = builder.function_state.current_function.as_ref().unwrap();
    let merge = &function.blocks[&merge_bb];
    assert!(matches!(
        merge.instructions.last(),
        Some(MirInstruction::Const {
            dst,
            value: ConstValue::Void,
        }) if *dst == output
    ));
    let x_value = builder.function_state.variable_ctx.variable_map["x"];
    let x_inputs = merge
        .instructions
        .iter()
        .find_map(|instruction| match instruction {
            MirInstruction::Phi { dst, inputs, .. } if *dst == x_value => Some(inputs),
            _ => None,
        });
    let x_inputs = x_inputs.expect("merge-block variable x Phi");
    assert_eq!(
        x_inputs
            .iter()
            .map(|(predecessor, _)| *predecessor)
            .collect::<BTreeSet<_>>(),
        BTreeSet::from([then_bb, else_bb])
    );
    assert!(merge.instructions.iter().any(|instruction| {
        matches!(instruction, MirInstruction::Phi { dst, inputs, .. }
            if *dst != x_value && inputs.len() == 2)
    }));
}

#[test]
fn production_statement_if_implicit_else_keeps_internal_and_facade_void_distinct() {
    let mut builder = builder("production_statement_if_implicit_else/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let output = super::block_stmt::build_statement(
        &mut builder,
        statement_if(bool_lit(true), vec![int_lit(1)], None),
    )
    .unwrap();

    let (then_bb, else_bb, merge_bb) = assert_simple_statement_if_shape(&builder, false, false);
    let voids = void_values(&builder);
    let internal = voids
        .iter()
        .find(|(block, _)| *block == else_bb)
        .expect("implicit false internal Void");
    let facade = voids
        .iter()
        .find(|(block, value)| *block == merge_bb && *value == output)
        .expect("statement facade Void");
    assert_ne!(internal.1, facade.1);
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert!(function.blocks[&then_bb].successors.contains(&merge_bb));
    assert!(function.blocks[&else_bb].successors.contains(&merge_bb));
}

#[test]
fn production_statement_if_preserves_branch_termination_matrix() {
    for (label, then_body, else_body, then_returns, else_returns) in [
        (
            "both_fallthrough",
            vec![int_lit(1)],
            vec![int_lit(2)],
            false,
            false,
        ),
        (
            "then_return",
            vec![return_int(1)],
            vec![int_lit(2)],
            true,
            false,
        ),
        (
            "else_return",
            vec![int_lit(1)],
            vec![return_int(2)],
            false,
            true,
        ),
        (
            "both_return",
            vec![return_int(1)],
            vec![return_int(2)],
            true,
            true,
        ),
    ] {
        let mut builder = builder(&format!("production_statement_if_termination_{label}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);

        let output = super::block_stmt::build_statement(
            &mut builder,
            statement_if(bool_lit(true), then_body, Some(else_body)),
        )
        .unwrap();

        let (_, _, merge_bb) =
            assert_simple_statement_if_shape(&builder, then_returns, else_returns);
        assert!(void_values(&builder)
            .iter()
            .any(|(block, value)| *block == merge_bb && *value == output));
        let returns = builder
            .function_state
            .current_function
            .as_ref()
            .unwrap()
            .blocks
            .values()
            .filter(|block| matches!(block.terminator, Some(MirInstruction::Return { .. })))
            .count();
        assert_eq!(
            returns,
            usize::from(then_returns) + usize::from(else_returns)
        );
    }
}

#[test]
fn production_statement_if_failures_emit_no_facade_void_and_do_not_retry() {
    for (label, condition, then_body, else_body) in [
        (
            "condition",
            variable("missing_condition"),
            vec![int_lit(1)],
            Some(vec![int_lit(2)]),
        ),
        (
            "then",
            bool_lit(true),
            vec![variable("missing_then")],
            Some(vec![int_lit(2)]),
        ),
        (
            "else",
            bool_lit(true),
            vec![int_lit(1)],
            Some(vec![variable("missing_else")]),
        ),
    ] {
        let mut builder = builder(&format!("production_statement_if_failure_{label}/0"));
        let _scope = LexicalScopeGuard::new(&mut builder);
        let before_voids = void_values(&builder).len();

        let error = super::block_stmt::build_statement(
            &mut builder,
            statement_if(condition, then_body, else_body),
        )
        .unwrap_err();

        assert!(error.contains("Undefined variable"), "{label}: {error}");
        assert_eq!(void_values(&builder).len(), before_voids, "{label}");
        assert_eq!(branch_count(&builder), 0, "{label}");
        assert_eq!(builder.recursion_depth, 0, "{label}");
    }

    let mut builder = builder("production_statement_if_reuse/0");
    let _scope = LexicalScopeGuard::new(&mut builder);
    super::block_stmt::build_statement(
        &mut builder,
        statement_if(
            variable("missing_condition"),
            vec![int_lit(1)],
            Some(vec![int_lit(2)]),
        ),
    )
    .unwrap_err();
    let output = super::block_stmt::build_statement(
        &mut builder,
        statement_if(bool_lit(true), vec![int_lit(1)], Some(vec![int_lit(2)])),
    )
    .unwrap();
    assert_eq!(branch_count(&builder), 1);
    assert!(void_values(&builder)
        .iter()
        .any(|(_, value)| *value == output));
}

#[test]
fn production_statement_if_preserves_program_shell_recursion_boundary() {
    let source = || statement_if(bool_lit(true), vec![int_lit(1)], Some(vec![int_lit(2)]));

    let mut accepted = builder("production_statement_if_recursion_accepted/0");
    accepted.recursion_depth = 198;
    super::block_stmt::build_statement(&mut accepted, source()).unwrap();
    assert_eq!(accepted.recursion_depth, 198);

    let mut branch_child_rejected = builder("production_statement_if_recursion_branch/0");
    branch_child_rejected.recursion_depth = 199;
    let branch_error =
        super::block_stmt::build_statement(&mut branch_child_rejected, source()).unwrap_err();
    assert!(branch_error.contains("Recursion depth exceeded"));
    assert_eq!(branch_child_rejected.recursion_depth, 199);

    let mut condition_rejected = builder("production_statement_if_recursion_condition/0");
    condition_rejected.recursion_depth = 200;
    let condition_error =
        super::block_stmt::build_statement(&mut condition_rejected, source()).unwrap_err();
    assert!(condition_error.contains("Recursion depth exceeded"));
    assert_eq!(condition_rejected.recursion_depth, 200);
}

#[test]
fn production_statement_if_preserves_branch_program_span_shell() {
    let outer_span = Span::new(1, 1, 1, 20);
    let condition_span = Span::new(1, 4, 1, 8);
    let then_span = Span::new(2, 3, 2, 4);
    let else_span = Span::new(4, 3, 4, 4);

    let mut non_empty = builder("production_statement_if_span_non_empty/0");
    super::block_stmt::build_statement(
        &mut non_empty,
        statement_if_at(
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: condition_span,
            },
            vec![int_lit_at(1, then_span)],
            Some(vec![int_lit_at(2, else_span)]),
            outer_span,
        ),
    )
    .unwrap();
    assert_eq!(non_empty.metadata_ctx.current_span(), else_span);

    let mut empty_else = builder("production_statement_if_span_empty_else/0");
    super::block_stmt::build_statement(
        &mut empty_else,
        statement_if_at(
            ASTNode::Literal {
                value: LiteralValue::Bool(true),
                span: condition_span,
            },
            vec![int_lit_at(1, then_span)],
            Some(Vec::new()),
            outer_span,
        ),
    )
    .unwrap();
    assert_eq!(empty_else.metadata_ctx.current_span(), Span::unknown());
}

#[test]
fn production_statement_if_fastmem_preserves_positive_and_negative_admission() {
    fn fastmem_builder(name: &str) -> (MirBuilder, FastMemRegionId) {
        let mut builder = builder(name);
        let region = FastMemRegionId(0);
        builder
            .function_state
            .current_function
            .as_mut()
            .unwrap()
            .metadata
            .fastmem_regions
            .push(FastMemRegionMetadata {
                id: region,
                contract: "ProductionStatementIfV1".to_string(),
                source_span: Span::unknown(),
                origin: FastMemRegionOrigin::SourceFastMemBlock,
                body_statement_count: 1,
                emitted_memop_count: 0,
            });
        builder.push_fastmem_region(region);
        (builder, region)
    }

    let (mut negative, _) = fastmem_builder("production_statement_if_fastmem_negative/0");
    let negative_error = super::block_stmt::build_statement(
        &mut negative,
        statement_if(bool_lit(true), vec![int_lit(1)], Some(vec![int_lit(2)])),
    )
    .unwrap_err();
    assert!(negative_error.contains("fastmem/branch_cfg_requires_owner_eq_condition"));
    assert_eq!(branch_count(&negative), 0);
    assert!(negative
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_branch_condition_facts
        .is_empty());

    let (mut positive, region) = fastmem_builder("production_statement_if_fastmem_positive/0");
    super::block_stmt::build_statement(
        &mut positive,
        statement_if(
            owner_eq_condition(),
            vec![int_lit(1)],
            Some(vec![int_lit(2)]),
        ),
    )
    .unwrap();
    assert_eq!(branch_count(&positive), 1);
    let facts = &positive
        .function_state
        .current_function
        .as_ref()
        .unwrap()
        .metadata
        .fastmem_branch_condition_facts;
    assert_eq!(facts.len(), 1);
    assert_eq!(facts[0].region, region);
    assert_eq!(
        facts[0].proof_kind,
        FastMemBranchConditionProofKind::SourceAssumeOwnerEq
    );
    assert!(facts[0].owner_eq_required);
}

#[test]
fn expression_if_remains_cf_if_value_route_without_statement_void() {
    let mut builder = builder("expression_if_value_route/0");
    let _scope = LexicalScopeGuard::new(&mut builder);

    let output = builder
        .build_expression(statement_if(
            bool_lit(true),
            vec![int_lit(1)],
            Some(vec![int_lit(2)]),
        ))
        .unwrap();

    let merge_bb = builder.function_state.current_block.unwrap();
    let function = builder.function_state.current_function.as_ref().unwrap();
    assert!(function.blocks[&merge_bb].instructions.iter().any(
        |instruction| matches!(instruction, MirInstruction::Phi { dst, .. } if *dst == output)
    ));
    assert!(void_values(&builder).is_empty());
}
