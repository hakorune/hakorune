use crate::ast::ASTNode;
use crate::parser::NyashParser;

fn c198_source() -> &'static str {
    r#"
static box Main {
    main(args) {
        local observed = 2
        local ok = check "c198 eager proof" {
            "first": observed == 2
            "forced failure": observed == 99
            "still listed": observed == 2
        }
        return ok
    }
}
"#
}

fn find_check_expr(node: &ASTNode) -> Option<&ASTNode> {
    match node {
        ASTNode::CheckExpr { .. } => Some(node),
        ASTNode::Program { statements, .. }
        | ASTNode::ScopeBox {
            body: statements, ..
        } => statements.iter().find_map(find_check_expr),
        ASTNode::BoxDeclaration { methods, .. } => methods.values().find_map(find_check_expr),
        ASTNode::FunctionDeclaration { body, .. } => body.iter().find_map(find_check_expr),
        ASTNode::Local { initial_values, .. } => initial_values
            .iter()
            .filter_map(|value| value.as_deref())
            .find_map(find_check_expr),
        ASTNode::Assignment { target, value, .. } => {
            find_check_expr(target).or_else(|| find_check_expr(value))
        }
        ASTNode::Return { value, .. } => value.as_deref().and_then(find_check_expr),
        ASTNode::BinaryOp { left, right, .. } => {
            find_check_expr(left).or_else(|| find_check_expr(right))
        }
        ASTNode::GroupedAssignmentExpr { rhs, .. } => find_check_expr(rhs),
        _ => None,
    }
}

fn assert_c198_shape(ast: ASTNode) {
    let check = find_check_expr(&ast).expect("check expression should be present");
    let ASTNode::CheckExpr { name, items, .. } = check else {
        unreachable!("find_check_expr only returns CheckExpr");
    };

    assert_eq!(name.as_deref(), Some("c198 eager proof"));
    assert_eq!(items.len(), 3);
    assert_eq!(items[0].label.as_deref(), Some("first"));
    assert_eq!(items[1].label.as_deref(), Some("forced failure"));
    assert_eq!(items[2].label.as_deref(), Some("still listed"));
}

#[test]
fn c198_check_block_parses_default_route() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "0", || {
        let ast = NyashParser::parse_from_string(c198_source()).expect("C198 check block parse");
        assert_c198_shape(ast);
    });
}

#[test]
fn c198_check_block_parses_token_cursor_route() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "1", || {
        let ast =
            NyashParser::parse_from_string(c198_source()).expect("C198 cursor check block parse");
        assert_c198_shape(ast);
    });
}

#[test]
fn c198_check_identifier_still_parses_as_ordinary_name() {
    crate::tests::helpers::env::with_env_var("NYASH_PARSER_TOKEN_CURSOR", "0", || {
        NyashParser::parse_from_string(
            r#"
static box Main {
    main(args) {
        local check = 1
        return check + 1
    }
}
"#,
        )
        .expect("ordinary identifier named check should still parse");
    });
}
