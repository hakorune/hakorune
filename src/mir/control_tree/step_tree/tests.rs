use super::*;
use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};

fn str_lit(s: &str) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::String(s.to_string()),
        span: Span::unknown(),
    }
}

fn eq(a: ASTNode, b: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: BinaryOperator::Equal,
        left: Box::new(a),
        right: Box::new(b),
        span: Span::unknown(),
    }
}

fn assign_x(num: i64) -> ASTNode {
    ASTNode::Assignment {
        target: Box::new(ASTNode::Variable {
            name: "x".to_string(),
            span: Span::unknown(),
        }),
        value: Box::new(ASTNode::Literal {
            value: LiteralValue::Integer(num),
            span: Span::unknown(),
        }),
        span: Span::unknown(),
    }
}

#[test]
fn build_step_tree_if_only_nested_if_is_structural() {
    // Equivalent shape to Phase103 "if-only merge" fixture:
    //
    // local x = 0
    // if "x" == "x" { if "y" == "z" { x=1 } else { x=2 } } else { x=3 }
    // print(x)
    let ast = vec![
        ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(0),
                span: Span::unknown(),
            }))],
            span: Span::unknown(),
        },
        ASTNode::If {
            condition: Box::new(eq(str_lit("x"), str_lit("x"))),
            then_body: vec![ASTNode::If {
                condition: Box::new(eq(str_lit("y"), str_lit("z"))),
                then_body: vec![assign_x(1)],
                else_body: Some(vec![assign_x(2)]),
                span: Span::unknown(),
            }],
            else_body: Some(vec![assign_x(3)]),
            span: Span::unknown(),
        },
        ASTNode::Print {
            expression: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];

    let tree = StepTreeBuilderBox::build_from_block(&ast);
    assert!(tree.features.has_if);
    assert!(!tree.features.has_loop);
    assert_eq!(tree.features.max_if_depth, 2);
    assert_eq!(tree.contract.exits.len(), 0);
    assert!(tree.contract.writes.contains("x"));
    assert!(tree.contract.required_caps.contains(&StepCapability::If));
    assert!(tree
        .contract
        .required_caps
        .contains(&StepCapability::NestedIf));

    let basis = tree.signature_basis_string();
    assert_eq!(
        basis,
        "kinds=Block,Stmt(local(x)),If,Block,If,Block,Stmt(assign(x)),Block,Stmt(assign(x)),Block,Stmt(assign(x)),Stmt(print);exits=;writes=x;reads=;caps=If,NestedIf;conds=(lit:str:x == lit:str:x)|(lit:str:y == lit:str:z)"
    );

    let tree2 = StepTreeBuilderBox::build_from_block(&ast);
    assert_eq!(tree.signature, tree2.signature);

    match tree.root {
        StepNode::Block(nodes) => {
            assert_eq!(nodes.len(), 3);
            match &nodes[1] {
                StepNode::If {
                    then_branch,
                    cond_ast,
                    ..
                } => {
                    // cond_ast should be populated
                    assert!(matches!(&cond_ast.0.as_ref(), ASTNode::BinaryOp { .. }));

                    match &**then_branch {
                        StepNode::Block(inner_nodes) => match &inner_nodes[0] {
                            StepNode::If {
                                cond_ast: inner_cond_ast,
                                ..
                            } => {
                                // inner cond_ast should also be populated
                                assert!(matches!(
                                    &inner_cond_ast.0.as_ref(),
                                    ASTNode::BinaryOp { .. }
                                ));
                            }
                            other => panic!("expected nested If, got {other:?}"),
                        },
                        other => panic!("expected Block in then_branch, got {other:?}"),
                    }
                }
                other => panic!("expected If at index 1, got {other:?}"),
            }
        }
        other => panic!("expected root Block, got {other:?}"),
    }
}

#[test]
fn step_tree_cond_ast_is_populated() {
    // Phase 119: cond_ast should hold AST reference.
    let ast = vec![ASTNode::If {
        condition: Box::new(eq(str_lit("a"), str_lit("b"))),
        then_body: vec![assign_x(1)],
        else_body: None,
        span: Span::unknown(),
    }];

    let tree = StepTreeBuilderBox::build_from_block(&ast);

    match &tree.root {
        StepNode::Block(nodes) => match &nodes[0] {
            StepNode::If { cond_ast, .. } => {
                // cond_ast should be populated with BinaryOp
                assert!(matches!(&cond_ast.0.as_ref(), ASTNode::BinaryOp { .. }));
            }
            other => panic!("expected If, got {other:?}"),
        },
        other => panic!("expected root Block, got {other:?}"),
    }
}

#[test]
fn step_tree_program_node_is_structural_block() {
    // ASTNode::Program can appear nested (e.g., statement-level desugars).
    // StepTree should treat it as a structural Block, not as Other("Program").
    let ast = vec![
        ASTNode::Program {
            statements: vec![assign_x(1)],
            span: Span::unknown(),
        },
        ASTNode::Print {
            expression: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        },
    ];

    let tree = StepTreeBuilderBox::build_from_block(&ast);
    match tree.root {
        StepNode::Block(nodes) => {
            assert_eq!(nodes.len(), 2);
            match &nodes[0] {
                StepNode::Block(inner) => match &inner[0] {
                    StepNode::Stmt {
                        kind: StepStmtKind::Assign { target, .. },
                        ..
                    } => assert_eq!(target.as_deref(), Some("x")),
                    other => {
                        panic!("expected assign stmt inside nested Program block, got {other:?}")
                    }
                },
                StepNode::Stmt {
                    kind: StepStmtKind::Other("Program"),
                    ..
                } => panic!("nested Program must be structural Block, not Other(\"Program\")"),
                other => panic!("expected nested Block for Program, got {other:?}"),
            }
        }
        other => panic!("expected root Block, got {other:?}"),
    }
}

#[test]
fn step_tree_signature_is_stable_with_cond_ast() {
    // Phase 119: cond_ast should NOT affect signature stability.
    // Signature is based on cond_sig (AstSummary), not cond_ast.
    let ast = vec![ASTNode::Loop {
        condition: Box::new(ASTNode::Literal {
            value: LiteralValue::Bool(true),
            span: Span::unknown(),
        }),
        body: vec![assign_x(1)],
        span: Span::unknown(),
    }];

    let tree1 = StepTreeBuilderBox::build_from_block(&ast);
    let tree2 = StepTreeBuilderBox::build_from_block(&ast);

    // Signature should be identical (deterministic)
    assert_eq!(tree1.signature, tree2.signature);

    // cond_ast should be populated
    match &tree1.root {
        StepNode::Block(nodes) => match &nodes[0] {
            StepNode::Loop { cond_ast, .. } => {
                assert!(matches!(&cond_ast.0.as_ref(), ASTNode::Literal { .. }));
            }
            other => panic!("expected Loop, got {other:?}"),
        },
        other => panic!("expected root Block, got {other:?}"),
    }
}

#[test]
fn contract_extracts_loop_exits_and_writes_minimal() {
    fn var(name: &str) -> ASTNode {
        ASTNode::Variable {
            name: name.to_string(),
            span: Span::unknown(),
        }
    }
    fn int_lit(v: i64) -> ASTNode {
        ASTNode::Literal {
            value: LiteralValue::Integer(v),
            span: Span::unknown(),
        }
    }
    fn bin(op: BinaryOperator, lhs: ASTNode, rhs: ASTNode) -> ASTNode {
        ASTNode::BinaryOp {
            operator: op,
            left: Box::new(lhs),
            right: Box::new(rhs),
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

    // local i=0; local x=0;
    // loop(i < 3) { x = x + 1; if x == 2 { break } i = i + 1 }
    let ast = vec![
        ASTNode::Local {
            variables: vec!["i".to_string()],
            initial_values: vec![Some(Box::new(int_lit(0)))],
            span: Span::unknown(),
        },
        ASTNode::Local {
            variables: vec!["x".to_string()],
            initial_values: vec![Some(Box::new(int_lit(0)))],
            span: Span::unknown(),
        },
        ASTNode::Loop {
            condition: Box::new(bin(BinaryOperator::Less, var("i"), int_lit(3))),
            body: vec![
                assign("x", bin(BinaryOperator::Add, var("x"), int_lit(1))),
                ASTNode::If {
                    condition: Box::new(bin(BinaryOperator::Equal, var("x"), int_lit(2))),
                    then_body: vec![ASTNode::Break {
                        span: Span::unknown(),
                    }],
                    else_body: None,
                    span: Span::unknown(),
                },
                assign("i", bin(BinaryOperator::Add, var("i"), int_lit(1))),
            ],
            span: Span::unknown(),
        },
    ];

    let tree = StepTreeBuilderBox::build_from_block(&ast);
    assert!(tree.features.has_loop);
    assert!(tree.contract.exits.contains(&ExitKind::Break));
    assert!(tree.contract.writes.contains("i"));
    assert!(tree.contract.writes.contains("x"));
    assert!(tree.contract.required_caps.contains(&StepCapability::Loop));
    assert!(tree.contract.required_caps.contains(&StepCapability::If));
}
