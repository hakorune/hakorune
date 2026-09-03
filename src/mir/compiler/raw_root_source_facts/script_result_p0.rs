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
fn shared_script_projection_has_no_raw_root_or_publication_input() {
    let source = ASTNode::Program {
        statements: vec![ASTNode::Literal {
            value: LiteralValue::Integer(7),
            span: Span::unknown(),
        }],
        span: Span::unknown(),
    };

    let recipe = project_raw_script_body_recipe_v1(&source).expect("shared Script recipe");

    assert!(matches!(
        recipe.terminal(),
        RawScriptTerminalRecipeV1::ValueExpression(RawLinearScalarExprV1::Literal {
            value: LiteralValue::Integer(7),
            ..
        })
    ));
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

#[test]
fn empty_script_post_install_facts_produce_linear_recipe() {
    let facts = RawRootSourceFactsV1::empty_for_test(RawRootSourceRouteV1::Script);
    let (post_install, _) = facts.into_post_install_parts().unwrap();
    let recipe = post_install.into_linear_body_recipe();
    assert_eq!(recipe.entry(), &RawRootBodyEntryContractV1::script());
    assert!(recipe.statements().is_empty());
}
