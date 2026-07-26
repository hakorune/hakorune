use super::{LoweredScriptTerminalV1, LoweredScriptUnitPayloadV1, ScriptRecipeLoweringOperationV1};
use crate::ast::{BinaryOperator, LiteralValue, Span};
use crate::mir::builder::vars::lexical_scope::LexicalScopeGuard;
use crate::mir::builder::MirBuilder;
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawRootBodySourceSiteV1, RawScriptBodyRecipeV1,
    RawScriptTerminalRecipeV1, RawScriptUnitOriginV1,
};

fn site(path: &[usize]) -> RawRootBodySourceSiteV1 {
    RawRootBodySourceSiteV1::new(path, Span::unknown())
}

fn integer(value: i64, path: &[usize]) -> RawLinearScalarExprV1 {
    RawLinearScalarExprV1::Literal {
        value: LiteralValue::Integer(value),
        site: site(path),
    }
}

fn lower(recipe: &RawScriptBodyRecipeV1) -> Result<LoweredScriptTerminalV1, String> {
    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("script_terminal_kernel/0".to_string());
    let result = {
        let scope = LexicalScopeGuard::new(&mut builder);
        let result = builder
            .lower_script_body_recipe_v1(recipe)
            .map_err(|error| error.to_string());
        drop(scope);
        result
    };
    builder.exit_function_for_test();
    result
}

#[test]
fn script_terminal_kernel_classifies_empty_value_and_explicit_void() {
    let empty =
        RawScriptBodyRecipeV1::from_parts(Box::new([]), RawScriptTerminalRecipeV1::EmptyUnit)
            .expect("empty Script recipe");
    assert!(matches!(
        lower(&empty).expect("empty lowering"),
        LoweredScriptTerminalV1::Unit {
            origin: RawScriptUnitOriginV1::EmptyBody,
            payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
        }
    ));

    let value = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(integer(42, &[0])),
    )
    .expect("value Script recipe");
    assert!(matches!(
        lower(&value).expect("value lowering"),
        LoweredScriptTerminalV1::Value { .. }
    ));

    let explicit_void = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::UnitExpression {
            expression: RawLinearScalarExprV1::Literal {
                value: LiteralValue::Null,
                site: site(&[2]),
            },
            origin: RawScriptUnitOriginV1::VoidExpression,
        },
    )
    .expect("explicit Unit Script recipe");
    assert!(matches!(
        lower(&explicit_void).expect("explicit Unit lowering"),
        LoweredScriptTerminalV1::Unit {
            origin: RawScriptUnitOriginV1::VoidExpression,
            payload: LoweredScriptUnitPayloadV1::ExistingVoid { .. },
        }
    ));
}

#[test]
fn script_terminal_kernel_preserves_unit_statement_origins() {
    let cases = [
        (
            RawLinearScalarStmtV1::Print {
                expression: integer(1, &[0, 0]),
                site: site(&[0]),
            },
            RawScriptUnitOriginV1::PrintStatement,
        ),
        (
            RawLinearScalarStmtV1::Local {
                variables: vec!["x".into()].into_boxed_slice(),
                initialized: vec![Some(integer(1, &[1, 0]))].into_boxed_slice(),
                site: site(&[1]),
            },
            RawScriptUnitOriginV1::LocalStatement,
        ),
    ];

    for (statement, origin) in cases {
        let recipe = RawScriptBodyRecipeV1::from_parts(
            Box::new([]),
            RawScriptTerminalRecipeV1::UnitStatement { statement, origin },
        )
        .expect("Unit statement Script recipe");
        assert!(matches!(
            lower(&recipe).expect("Unit statement lowering"),
            LoweredScriptTerminalV1::Unit {
                origin: actual_origin,
                payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
            } if actual_origin == origin
        ));
    }

    for (statement, origin) in [
        (
            RawLinearScalarStmtV1::Assignment {
                target: "x".into(),
                value: integer(2, &[3, 0]),
                site: site(&[3]),
            },
            RawScriptUnitOriginV1::AssignmentStatement,
        ),
        (
            RawLinearScalarStmtV1::CompoundAssignment {
                target: "x".into(),
                operator: BinaryOperator::Add,
                value: integer(2, &[4, 0]),
                site: site(&[4]),
            },
            RawScriptUnitOriginV1::CompoundAssignmentStatement,
        ),
    ] {
        let recipe = RawScriptBodyRecipeV1::from_parts(
            vec![RawLinearScalarStmtV1::Local {
                variables: vec!["x".into()].into_boxed_slice(),
                initialized: vec![Some(integer(1, &[2, 0]))].into_boxed_slice(),
                site: site(&[2]),
            }]
            .into_boxed_slice(),
            RawScriptTerminalRecipeV1::UnitStatement { statement, origin },
        )
        .expect("bound Unit statement Script recipe");
        assert!(matches!(
            lower(&recipe).expect("bound Unit statement lowering"),
            LoweredScriptTerminalV1::Unit {
                origin: actual_origin,
                payload: LoweredScriptUnitPayloadV1::SyntheticVoid,
            } if actual_origin == origin
        ));
    }
}

#[test]
fn script_terminal_kernel_reports_exact_terminal_failure() {
    let recipe = RawScriptBodyRecipeV1::from_parts(
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(RawLinearScalarExprV1::Variable {
            name: "missing".into(),
            site: site(&[7]),
        }),
    )
    .expect("failing Script recipe");

    let mut builder = MirBuilder::new();
    builder.enter_function_for_test("script_terminal_failure/0".to_string());
    let scope = LexicalScopeGuard::new(&mut builder);
    let error = builder
        .lower_script_body_recipe_v1(&recipe)
        .expect_err("undefined terminal variable must fail");
    drop(scope);
    builder.exit_function_for_test();

    assert_eq!(
        error.operation(),
        ScriptRecipeLoweringOperationV1::TerminalValueExpression
    );
    assert_eq!(error.site().path(), &[7]);
    assert!(!error.detail().is_empty());
}
