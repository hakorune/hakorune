use crate::ast::{ASTNode, UnaryOperator};
use crate::parser::NyashParser;

fn c200_source() -> &'static str {
    r#"
static box Main {
    main(args) {
        local ready = 1
        guard ready == 1 else {
            return 9
        }
        return 0
    }
}
"#
}

fn collect_ifs<'a>(node: &'a ASTNode, out: &mut Vec<&'a ASTNode>) {
    if matches!(node, ASTNode::If { .. }) {
        out.push(node);
    }
    node.for_each_child(&mut |child| collect_ifs(child, out));
}

#[test]
fn c200_guard_else_lowers_to_negative_if() {
    let ast = NyashParser::parse_from_string(c200_source()).expect("C200 parse");
    let mut ifs = Vec::new();
    collect_ifs(&ast, &mut ifs);

    assert!(
        ifs.iter().any(|node| matches!(
            node,
            ASTNode::If {
                condition,
                then_body,
                else_body,
                ..
            } if matches!(
                condition.as_ref(),
                ASTNode::UnaryOp {
                    operator: UnaryOperator::Not,
                    ..
                }
            ) && then_body
                .iter()
                .any(|stmt| matches!(stmt, ASTNode::Return { .. }))
                && else_body.is_none()
        )),
        "guard else should lower to If(UnaryOp::Not(condition), body, None)"
    );
}

#[test]
fn c200_guard_requires_else_block() {
    NyashParser::parse_from_string(
        r#"
static box Main {
    main(args) {
        local ready = 1
        guard ready == 1 {
            return 0
        }
        return 1
    }
}
"#,
    )
    .expect_err("guard without else must be rejected");
}
