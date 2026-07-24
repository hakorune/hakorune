//! BODY0-S0-B disconnected lowerer fixtures.

use super::MirBuilder;
use crate::ast::{BinaryOperator, LiteralValue, Span};
use crate::mir::builder::root_body_completion::RootBodyResultV1;
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawRootBodyEntryV1, RawRootBodyRecipeV1,
    RawRootBodySourceSiteV1,
};

fn site(path: &[usize]) -> RawRootBodySourceSiteV1 {
    RawRootBodySourceSiteV1::new(path, Span::unknown())
}

#[test]
fn linear_recipe_lowers_without_ast_reconstruction() {
    let expr = RawLinearScalarExprV1::Binary {
        operator: BinaryOperator::Add,
        left: Box::new(RawLinearScalarExprV1::Literal {
            value: LiteralValue::Integer(1),
            site: site(&[0, 0]),
        }),
        right: Box::new(RawLinearScalarExprV1::Literal {
            value: LiteralValue::Integer(2),
            site: site(&[0, 1]),
        }),
        site: site(&[0]),
    };
    let recipe = RawRootBodyRecipeV1::from_parts(
        RawRootBodyEntryV1::Script,
        vec![RawLinearScalarStmtV1::Expr {
            expression: expr,
            site: site(&[0]),
        }]
        .into_boxed_slice(),
    )
    .unwrap();

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw/main/0".to_string());
    let _scope = LexicalScopeGuard::new(&mut builder);
    let result = builder.lower_linear_scalar_recipe_v1(&recipe).unwrap();
    assert!(matches!(result, RootBodyResultV1::Value(_)));
    assert!(!builder.current_function_instructions().is_empty());
    drop(_scope);
    builder.exit_function_for_test();
}

#[test]
fn linear_recipe_lowers_local_assignment_and_print() {
    let recipe = RawRootBodyRecipeV1::from_parts(
        RawRootBodyEntryV1::Script,
        vec![
            RawLinearScalarStmtV1::Local {
                variables: vec!["x".into()].into_boxed_slice(),
                initialized: vec![Some(RawLinearScalarExprV1::Literal {
                    value: LiteralValue::Integer(1),
                    site: site(&[0, 0]),
                })]
                .into_boxed_slice(),
                site: site(&[0]),
            },
            RawLinearScalarStmtV1::Assignment {
                target: "x".into(),
                value: RawLinearScalarExprV1::Literal {
                    value: LiteralValue::Integer(2),
                    site: site(&[1, 1]),
                },
                site: site(&[1]),
            },
            RawLinearScalarStmtV1::Print {
                expression: RawLinearScalarExprV1::Variable {
                    name: "x".into(),
                    site: site(&[2, 0]),
                },
                site: site(&[2]),
            },
        ]
        .into_boxed_slice(),
    )
    .unwrap();

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("raw/main/0".to_string());
    let _scope = LexicalScopeGuard::new(&mut builder);
    let result = builder.lower_linear_scalar_recipe_v1(&recipe).unwrap();
    assert!(matches!(result, RootBodyResultV1::Value(_)));
    assert!(builder.variable_bindings().iter().any(|(name, _)| name == "x"));
    drop(_scope);
    builder.exit_function_for_test();
}
