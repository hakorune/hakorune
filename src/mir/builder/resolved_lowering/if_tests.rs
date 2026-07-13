#![cfg(feature = "vm-reference")]

use std::collections::BTreeSet;

use crate::ast::{ASTNode, BinaryOperator, DeclarationAttrs, LiteralValue, Span};
use crate::backend::{MirInterpreter, VMValue};
use crate::mir::compiler::{MirCompileResult, VerifiedResolvedSourceUnitV1};
use crate::mir::verification::utils::compute_predecessors;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};

fn int(value: i64) -> ASTNode {
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

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.into(),
        span: Span::unknown(),
    }
}

fn add(left: ASTNode, right: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Add,
        left: Box::new(left),
        right: Box::new(right),
        span: Span::unknown(),
    }
}

fn local(name: &str, initial: ASTNode) -> ASTNode {
    ASTNode::Local {
        variables: vec![name.into()],
        initial_values: vec![Some(Box::new(initial))],
        declared_type_names: vec![None],
        span: Span::unknown(),
    }
}

fn assign(name: &str, value: ASTNode) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(var(name)),
        value: Box::new(value),
        span: Span::unknown(),
    }
}

fn block(prelude: Vec<ASTNode>, tail: ASTNode) -> ASTNode {
    ASTNode::BlockExpr {
        prelude_stmts: prelude,
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

fn returning(name: &str, mut body: Vec<ASTNode>, value: ASTNode) -> ASTNode {
    body.push(ASTNode::Return {
        value: Some(Box::new(value)),
        span: Span::unknown(),
    });
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

fn compile(root: ASTNode) -> MirCompileResult {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(root).unwrap();
    let mut compiler = crate::mir::MirCompiler::with_options(false);
    let result = compiler
        .compile_resolved(unit.lowering_input(), Some("canonical_if_i1b.hako"))
        .unwrap();
    assert!(
        result.verification_result.is_ok(),
        "{:?}",
        result.verification_result
    );
    result
}

fn execute_integer(root: ASTNode, function_name: &str) -> (MirCompileResult, i64) {
    let result = compile(root);
    let value = MirInterpreter::new()
        .execute_function_with_args(&result.module, function_name, &[])
        .unwrap();
    let VMValue::Integer(value) = value else {
        panic!("expected Integer result, got {value:?}")
    };
    (result, value)
}

fn function<'a>(result: &'a MirCompileResult, name: &str) -> &'a MirFunction {
    &result.module.functions[name]
}

fn branch_targets(function: &MirFunction) -> (BasicBlockId, BasicBlockId) {
    function
        .blocks
        .values()
        .find_map(|block| match block.terminator.as_ref() {
            Some(MirInstruction::Branch {
                then_bb, else_bb, ..
            }) => Some((*then_bb, *else_bb)),
            _ => None,
        })
        .expect("fixture must contain one conditional branch")
}

#[test]
fn condition_blockexpr_true_and_false_share_the_post_condition_baseline() {
    for (condition, expected) in [(true, 11), (false, 12)] {
        let name = format!("if_condition_baseline_{condition}");
        let root = returning(
            &name,
            vec![
                local("x", int(0)),
                if_stmt(
                    block(vec![assign("x", int(10))], boolean(condition)),
                    vec![assign("x", add(var("x"), int(1)))],
                    Some(vec![assign("x", add(var("x"), int(2)))]),
                ),
            ],
            var("x"),
        );
        let (_, actual) = execute_integer(root, &format!("{name}/0"));
        assert_eq!(actual, expected, "condition={condition}");
    }
}

#[test]
fn join_source_matrix_executes_then_else_and_two_sided_rebinds() {
    struct Case {
        name: &'static str,
        condition: bool,
        then_value: Option<i64>,
        else_value: Option<i64>,
        explicit_else: bool,
        expected: i64,
    }

    let cases = [
        Case {
            name: "then_only_true",
            condition: true,
            then_value: Some(1),
            else_value: None,
            explicit_else: false,
            expected: 1,
        },
        Case {
            name: "then_only_false",
            condition: false,
            then_value: Some(1),
            else_value: None,
            explicit_else: false,
            expected: 0,
        },
        Case {
            name: "else_only_true",
            condition: true,
            then_value: None,
            else_value: Some(2),
            explicit_else: true,
            expected: 0,
        },
        Case {
            name: "else_only_false",
            condition: false,
            then_value: None,
            else_value: Some(2),
            explicit_else: true,
            expected: 2,
        },
        Case {
            name: "both_true",
            condition: true,
            then_value: Some(1),
            else_value: Some(2),
            explicit_else: true,
            expected: 1,
        },
        Case {
            name: "both_false",
            condition: false,
            then_value: Some(1),
            else_value: Some(2),
            explicit_else: true,
            expected: 2,
        },
    ];

    for case in cases {
        let then_body = case
            .then_value
            .map(|value| vec![assign("x", int(value))])
            .unwrap_or_default();
        let else_body = case.explicit_else.then(|| {
            case.else_value
                .map(|value| vec![assign("x", int(value))])
                .unwrap_or_default()
        });
        let root = returning(
            case.name,
            vec![
                local("x", int(0)),
                if_stmt(boolean(case.condition), then_body, else_body),
            ],
            var("x"),
        );
        let (_, actual) = execute_integer(root, &format!("{}/0", case.name));
        assert_eq!(actual, case.expected, "case={}", case.name);
    }
}

#[test]
fn branch_local_same_name_shadow_is_retired_without_touching_outer_binding() {
    let root = returning(
        "if_branch_shadow",
        vec![
            local("x", int(10)),
            if_stmt(
                boolean(true),
                vec![
                    local("x", int(1)),
                    assign("x", add(var("x"), int(1))),
                    local("observed", var("x")),
                ],
                None,
            ),
        ],
        var("x"),
    );
    let (_, actual) = execute_integer(root, "if_branch_shadow/0");
    assert_eq!(actual, 10);
}

#[test]
fn implicit_and_explicit_empty_else_have_distinct_cfg_topology() {
    let implicit = compile(returning(
        "if_implicit_empty",
        vec![
            local("x", int(4)),
            if_stmt(boolean(false), Vec::new(), None),
        ],
        var("x"),
    ));
    let explicit = compile(returning(
        "if_explicit_empty",
        vec![
            local("x", int(4)),
            if_stmt(boolean(false), Vec::new(), Some(Vec::new())),
        ],
        var("x"),
    ));
    let implicit_fn = function(&implicit, "if_implicit_empty/0");
    let explicit_fn = function(&explicit, "if_explicit_empty/0");
    let (_, implicit_false) = branch_targets(implicit_fn);
    let (_, explicit_false) = branch_targets(explicit_fn);
    let implicit_preds = compute_predecessors(implicit_fn);
    let explicit_preds = compute_predecessors(explicit_fn);

    assert_eq!(implicit_preds[&implicit_false].len(), 2);
    assert_eq!(explicit_preds[&explicit_false].len(), 1);
    let Some(MirInstruction::Jump {
        target: explicit_merge,
        ..
    }) = explicit_fn.blocks[&explicit_false].terminator.as_ref()
    else {
        panic!("explicit empty else must own a block which jumps to merge")
    };
    assert_eq!(explicit_preds[explicit_merge].len(), 2);
    assert_eq!(explicit_fn.blocks.len(), implicit_fn.blocks.len() + 1);
}

#[test]
fn nested_if_consumes_exact_flow_once_and_phi_inputs_use_actual_predecessors() {
    let root = returning(
        "if_nested_flow",
        vec![
            local("x", int(0)),
            if_stmt(
                boolean(true),
                vec![if_stmt(
                    boolean(true),
                    vec![assign("x", int(1))],
                    Some(vec![assign("x", int(2))]),
                )],
                Some(vec![assign("x", int(3))]),
            ),
        ],
        var("x"),
    );
    let (result, actual) = execute_integer(root, "if_nested_flow/0");
    assert_eq!(actual, 1);

    let function = function(&result, "if_nested_flow/0");
    let predecessors = compute_predecessors(function);
    let mut saw_nested_exit_as_predecessor = false;
    let phi_blocks = function
        .blocks
        .iter()
        .filter(|(_, block)| block.phi_instructions().next().is_some())
        .map(|(block, _)| *block)
        .collect::<BTreeSet<_>>();
    assert!(phi_blocks.len() >= 2);
    for (block_id, block) in &function.blocks {
        for phi in block.phi_instructions() {
            let MirInstruction::Phi { inputs, .. } = phi else {
                unreachable!()
            };
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
            saw_nested_exit_as_predecessor |= input_predecessors
                .iter()
                .any(|predecessor| phi_blocks.contains(predecessor));
        }
    }
    assert!(saw_nested_exit_as_predecessor);
}

#[test]
fn same_input_assignment_still_materializes_a_fresh_final_phi() {
    let root = returning(
        "if_same_input_phi",
        vec![
            local("x", int(7)),
            if_stmt(boolean(true), vec![assign("x", var("x"))], None),
        ],
        var("x"),
    );
    let (result, actual) = execute_integer(root, "if_same_input_phi/0");
    assert_eq!(actual, 7);

    let function = function(&result, "if_same_input_phi/0");
    let phis = function
        .blocks
        .values()
        .flat_map(|block| block.phi_instructions())
        .collect::<Vec<_>>();
    assert_eq!(phis.len(), 1);
    let MirInstruction::Phi { dst, inputs, .. } = phis[0] else {
        unreachable!()
    };
    assert_eq!(inputs.len(), 2);
    assert!(inputs.iter().all(|(_, input)| input != dst));
}
