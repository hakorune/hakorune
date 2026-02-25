use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
use crate::mir::{MirCompiler, MirVerifier};

fn lit_i(i: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(i),
        span: Span::unknown(),
    }
}

fn bin(op: BinaryOperator, l: ASTNode, r: ASTNode) -> ASTNode {
    ASTNode::BinaryOp {
        operator: op,
        left: Box::new(l),
        right: Box::new(r),
        span: Span::unknown(),
    }
}

/// Basic PHI/SSA sanity: simple counted loop must verify without Undefined value.
#[test]
fn mir_phi_basic_counted_loop_verifies() {
    // i = 0;
    // loop (i < 3) {
    //   i = i + 1;
    // }
    // return i;
    let ast = ASTNode::Program {
        statements: vec![
            ASTNode::Assignment {
                target: Box::new(ASTNode::Variable {
                    name: "i".into(),
                    span: Span::unknown(),
                }),
                value: Box::new(lit_i(0)),
                span: Span::unknown(),
            },
            ASTNode::Loop {
                condition: Box::new(bin(
                    BinaryOperator::Less,
                    ASTNode::Variable {
                        name: "i".into(),
                        span: Span::unknown(),
                    },
                    lit_i(3),
                )),
                body: vec![ASTNode::Assignment {
                    target: Box::new(ASTNode::Variable {
                        name: "i".into(),
                        span: Span::unknown(),
                    }),
                    value: Box::new(bin(
                        BinaryOperator::Add,
                        ASTNode::Variable {
                            name: "i".into(),
                            span: Span::unknown(),
                        },
                        lit_i(1),
                    )),
                    span: Span::unknown(),
                }],
                span: Span::unknown(),
            },
            ASTNode::Return {
                value: Some(Box::new(ASTNode::Variable {
                    name: "i".into(),
                    span: Span::unknown(),
                })),
                span: Span::unknown(),
            },
        ],
        span: Span::unknown(),
    };

    let mut mc = MirCompiler::with_options(false);
    let cr = mc.compile(ast).expect("compile");

    let mut verifier = MirVerifier::new();
    if let Err(errors) = verifier.verify_module(&cr.module) {
        for e in &errors {
            eprintln!("[rust-mir-verify] {}", e);
        }
        panic!("MIR verification failed for basic counted loop");
    }
}
