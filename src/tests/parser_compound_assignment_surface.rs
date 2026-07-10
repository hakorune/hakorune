use crate::ast::{ASTNode, BinaryOperator, LiteralValue, Span};
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

fn collect_compound_assignments<'a>(node: &'a ASTNode, out: &mut Vec<&'a ASTNode>) {
    if matches!(node, ASTNode::CompoundAssignment { .. }) {
        out.push(node);
    }
    node.for_each_child(&mut |child| collect_compound_assignments(child, out));
}

#[test]
fn c199_compound_assignment_parses_default_surface() {
    crate::tests::helpers::env::with_env_var("NYASH_SYNTAX_SUGAR_LEVEL", "basic", || {
        let ast = NyashParser::parse_from_string(c199_source()).expect("C199 parse");
        let mut assignments = Vec::new();
        collect_compound_assignments(&ast, &mut assignments);

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::CompoundAssignment {
                    target,
                    operator: BinaryOperator::Add,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::Variable { name, .. } if name == "x")
                    && matches!(value.as_ref(), ASTNode::Literal { .. })
            )),
            "local += should preserve a CompoundAssignment"
        );

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::CompoundAssignment {
                    target,
                    operator: BinaryOperator::Multiply,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::FieldAccess { field, .. } if field == "value")
                    && matches!(value.as_ref(), ASTNode::Variable { name, .. } if name == "x")
            )),
            "field *= should preserve a CompoundAssignment"
        );

        assert!(
            assignments.iter().any(|assignment| matches!(
                assignment,
                ASTNode::CompoundAssignment {
                    target,
                    operator: BinaryOperator::Add,
                    value,
                    ..
                } if matches!(target.as_ref(), ASTNode::Index { .. })
                    && matches!(value.as_ref(), ASTNode::FieldAccess { field, .. } if field == "value")
            )),
            "index += should preserve a CompoundAssignment"
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

#[test]
fn compound_assignment_keeps_side_effecting_place_once() {
    crate::tests::helpers::env::with_env_var("NYASH_SYNTAX_SUGAR_LEVEL", "basic", || {
        let ast = NyashParser::parse_from_string(
            r#"
static box Main {
    main(args) {
        array()[next_index()] += make_value()
    }
}
"#,
        )
        .expect("parse side-effecting compound assignment");
        let mut assignments = Vec::new();
        collect_compound_assignments(&ast, &mut assignments);
        assert_eq!(assignments.len(), 1);

        let ASTNode::CompoundAssignment { target, value, .. } = assignments[0] else {
            unreachable!("collector only returns compound assignments");
        };
        let ASTNode::Index { target, index, .. } = target.as_ref() else {
            panic!("expected index Place target");
        };
        assert!(matches!(target.as_ref(), ASTNode::FunctionCall { name, .. } if name == "array"));
        assert!(
            matches!(index.as_ref(), ASTNode::FunctionCall { name, .. } if name == "next_index")
        );
        assert!(
            matches!(value.as_ref(), ASTNode::FunctionCall { name, .. } if name == "make_value")
        );

        let simple = ASTNode::CompoundAssignment {
            target: Box::new(ASTNode::Variable {
                name: "x".to_string(),
                span: Span::unknown(),
            }),
            operator: BinaryOperator::Add,
            value: Box::new(ASTNode::Literal {
                value: LiteralValue::Integer(1),
                span: Span::unknown(),
            }),
            span: Span::unknown(),
        };
        let json = crate::r#macro::ast_json::ast_to_json_roundtrip(&simple);
        let restored =
            crate::r#macro::ast_json::json_to_ast(&json).expect("roundtrip compound assignment");
        assert_eq!(restored, simple);
    });
}
