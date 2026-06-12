use super::*;
use crate::ast::{ASTNode, LiteralValue, Span};
use crate::mir::builder::stmts::variable_stmt::build_local_statement;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use std::collections::BTreeMap;

fn span() -> Span {
    Span::unknown()
}

fn lit_int(value: i64) -> ASTNode {
    ASTNode::Literal {
        value: LiteralValue::Integer(value),
        span: span(),
    }
}

fn var(name: &str) -> ASTNode {
    ASTNode::Variable {
        name: name.to_string(),
        span: span(),
    }
}

#[test]
fn return_prelude_scopebox_keeps_locals_scoped_and_outer_assignments_visible() {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("return_prelude_scopebox_scope".to_string());
    let _scope = LexicalScopeGuard::new(&mut builder);
    build_local_statement(
        &mut builder,
        vec!["outer".to_string()],
        vec![Some(Box::new(lit_int(0)))],
    )
    .expect("declare outer");

    let mut bindings: BTreeMap<String, crate::mir::ValueId> =
        builder.variable_ctx.variable_map.clone();
    let stmt = ASTNode::ScopeBox {
        body: vec![
            ASTNode::Local {
                variables: vec!["tmp".to_string()],
                initial_values: vec![Some(Box::new(lit_int(1)))],
                declared_type_names: Vec::new(),
                span: span(),
            },
            ASTNode::Assignment {
                target: Box::new(var("outer")),
                value: Box::new(lit_int(2)),
                span: span(),
            },
        ],
        span: span(),
    };

    let plans = lower_return_prelude_stmt(
        &mut builder,
        &mut bindings,
        &BTreeMap::new(),
        None,
        &stmt,
        "test_scopebox",
    )
    .expect("lower scopebox");

    assert!(!plans.is_empty());
    assert!(
        !bindings.contains_key("tmp"),
        "ScopeBox local must not leak into branch bindings"
    );
    assert!(
        !builder.variable_ctx.variable_map.contains_key("tmp"),
        "ScopeBox local must not leak into builder variable_map"
    );
    assert!(
        bindings.contains_key("outer"),
        "assignment to preexisting outer binding must remain visible"
    );

    builder.exit_function_for_test();
}
