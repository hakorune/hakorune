//! BODY0-S0 recipe boundary fixtures.

use super::raw_root_source_facts::{RawRootSourceFactsV1, RawRootSourceRouteV1};
use crate::mir::raw_root_body_recipe::{
    RawLinearScalarExprV1, RawLinearScalarStmtV1, RawRootBodyEntryV1, RawRootBodyRecipeErrorV1,
    RawRootBodyRecipeV1, RawRootBodySourceSiteV1,
};

#[test]
fn empty_script_post_install_facts_produce_linear_recipe() {
    let facts = RawRootSourceFactsV1::empty_for_test(RawRootSourceRouteV1::Script);
    let (post_install, _) = facts.into_post_install_parts().unwrap();
    let recipe = post_install.into_linear_body_recipe();
    assert!(matches!(recipe.entry(), RawRootBodyEntryV1::Script));
    assert!(recipe.statements().is_empty());
}

#[test]
fn recipe_rejects_duplicate_source_paths() {
    let site = || RawRootBodySourceSiteV1::new(&[0], crate::ast::Span::unknown());
    let statements = vec![
        RawLinearScalarStmtV1::Expr {
            expression: RawLinearScalarExprV1::Literal {
                value: crate::ast::LiteralValue::Integer(1),
                site: site(),
            },
            site: site(),
        },
        RawLinearScalarStmtV1::Expr {
            expression: RawLinearScalarExprV1::Literal {
                value: crate::ast::LiteralValue::Integer(2),
                site: site(),
            },
            site: site(),
        },
    ]
    .into_boxed_slice();
    let error =
        RawRootBodyRecipeV1::from_parts(RawRootBodyEntryV1::Script, statements).unwrap_err();
    assert!(matches!(
        error,
        RawRootBodyRecipeErrorV1::DuplicateSourcePath { .. }
    ));
}
