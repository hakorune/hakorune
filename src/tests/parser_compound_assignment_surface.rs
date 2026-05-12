use crate::ast::{ASTNode, BinaryOperator};
use crate::parser::NyashParser;

fn c199_source() -> &'static str {
    r#"
box Counter {
    value: i64 = 1
}

static box Main {
    main(args) {
        local x = 1
        x += 2

        local counter = new Counter()
        counter.value *= x

        local array = new ArrayBox()
        array.push(10)
        array[0] += counter.value

        return array[0]
    }
}
"#
}

fn collect_assignments<'a>(node: &'a ASTNode, out: &mut Vec<&'a ASTNode>) {
    if matches!(node, ASTNode::Assignment { .. }) {
        out.push(node);
    }
    node.for_each_child(&mut |child| collect_assignments(child, out));
}

#[test]
fn c199_compound_assignment_parses_default_surface() {
    crate::tests::helpers::env::with_env_var("NYASH_SYNTAX_SUGAR_LEVEL", "basic", || {
        let ast = NyashParser::parse_from_string(c199_source()).expect("C199 parse");
        let mut assignments = Vec::new();
        collect_assignments(&ast, &mut assignments);

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::Assignment {
                    target,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == "x")
                    && matches!(
                        value.as_ref(),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            ..
                        }
                    )
            )),
            "local += should lower to Assignment(BinaryOp::Add)"
        );

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::Assignment {
                    target,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::FieldAccess { field, .. } if field == "value")
                    && matches!(
                        value.as_ref(),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Multiply,
                            ..
                        }
                    )
            )),
            "field *= should lower to Assignment(BinaryOp::Multiply)"
        );

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::Assignment {
                    target,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::Index { .. })
                    && matches!(
                        value.as_ref(),
                        ASTNode::BinaryOp {
                            operator: BinaryOperator::Add,
                            ..
                        }
                    )
            )),
            "index += should lower to Assignment(BinaryOp::Add)"
        );
    });
}

#[test]
fn c199_compound_assignment_respects_sugar_off() {
    crate::tests::helpers::env::with_env_var("NYASH_SYNTAX_SUGAR_LEVEL", "none", || {
        NyashParser::parse_from_string(
            r#"
static box Main {
    main(args) {
        local x = 1
        x += 1
        return x
    }
}
"#,
        )
        .expect_err("compound assignment should reject when syntax sugar is disabled");
    });
}
