use super::*;
use crate::ast::{LiteralValue, Span};
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawRootBodyEntryContractV1, RawRootBodySourceSiteV1,
    RawScriptTerminalRecipeV1, RawScriptUnitOriginV1,
};

fn site(index: usize) -> RawRootBodySourceSiteV1 {
    RawRootBodySourceSiteV1::new(&[index], Span::unknown())
}

#[test]
fn script_terminal_recipe_keeps_value_expression_source_owned() {
    let expression = RawLinearScalarExprV1::Literal {
        value: LiteralValue::Integer(7),
        site: site(0),
    };
    let recipe = crate::mir::raw_root_body_recipe::RawRootBodyRecipeV1::from_script_parts(
        RawRootBodyEntryContractV1::script(),
        Box::new([]),
        RawScriptTerminalRecipeV1::ValueExpression(expression),
    )
    .expect("script recipe");
    let script = recipe.script().expect("script payload");
    assert!(script.prelude().is_empty());
    assert!(matches!(
        script.terminal(),
        RawScriptTerminalRecipeV1::ValueExpression(RawLinearScalarExprV1::Literal {
            value: LiteralValue::Integer(7),
            ..
        })
    ));
}

#[test]
fn script_unit_expression_retains_unit_origin() {
    let recipe = crate::mir::raw_root_body_recipe::RawRootBodyRecipeV1::from_script_parts(
        RawRootBodyEntryContractV1::script(),
        Box::new([]),
        RawScriptTerminalRecipeV1::UnitExpression {
            expression: RawLinearScalarExprV1::Literal {
                value: LiteralValue::Void,
                site: site(0),
            },
            origin: RawScriptUnitOriginV1::VoidExpression,
        },
    )
    .expect("script recipe");
    assert!(matches!(
        recipe.script().expect("script payload").terminal(),
        RawScriptTerminalRecipeV1::UnitExpression {
            origin: RawScriptUnitOriginV1::VoidExpression,
            ..
        }
    ));
}
