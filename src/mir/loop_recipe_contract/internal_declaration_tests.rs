use super::internal_declaration::{
    LoopInternalDeclarationModeV1, VerifiedLoopInternalDeclarationSetV1,
};
use super::{direct_accum_product_for_test, LoopOperationV1, LoopRecipeItemV1};
use crate::mir::compiler::nested_predicate_producer::{
    produce_nested_predicate_recipe_v1, VerifiedNestedPredicateRecipeProductV1,
};
use crate::mir::compiler::{
    nested_function_for_p3_test as nested_function, nested_projection_for_test as projection_for,
    VerifiedResolvedSourceUnitV1,
};
use crate::mir::loop_recipe_contract::loop_cond_break_continue_producer_tests as cond_tests;
use crate::mir::loop_recipe_contract::loop_true_break_continue_producer_tests as true_tests;
use crate::mir::resolved_semantics::{SourceBindingSiteV1, SourceStmtSiteV1};

fn nested_product() -> VerifiedNestedPredicateRecipeProductV1 {
    let unit = VerifiedResolvedSourceUnitV1::resolve_function(nested_function()).unwrap();
    let input = unit.root_function_input().unwrap();
    produce_nested_predicate_recipe_v1(projection_for(input), input.function())
        .expect("nested product")
}

fn root_loop_site(product: &VerifiedNestedPredicateRecipeProductV1) -> SourceStmtSiteV1 {
    let root = product.operations().core().recipe().as_recipe().root_loop;
    product
        .operations()
        .evidence()
        .iter()
        .find(|evidence| evidence.owner_loop() == root)
        .expect("root-loop evidence")
        .source_loop()
        .clone()
}

#[test]
fn nested_product_derives_one_internal_declaration_for_child_local() {
    let product = nested_product();
    let set =
        VerifiedLoopInternalDeclarationSetV1::issue(product.operations(), &root_loop_site(&product))
            .expect("internal declaration set");
    assert_eq!(set.rows().len(), 1);
    let row = &set.rows()[0];
    let SourceBindingSiteV1::Local { statement, .. } = row.site() else {
        panic!("internal declaration must keep its Local source site");
    };
    // `j` is declared inside the outer loop body: its statement path must
    // strictly extend the outer loop's source path.
    let loop_segments = root_loop_site(&product).node().segments().to_vec();
    let decl_segments = statement.node().segments();
    assert!(decl_segments.len() > loop_segments.len());
    assert!(decl_segments.starts_with(&loop_segments));
}

#[test]
fn nested_declaration_publishes_with_the_carrier_entry_value() {
    let product = nested_product();
    let set =
        VerifiedLoopInternalDeclarationSetV1::issue(product.operations(), &root_loop_site(&product))
            .expect("internal declaration set");
    let row = &set.rows()[0];
    let LoopInternalDeclarationModeV1::PublishWithEntry { value } = row.mode() else {
        panic!("child local must publish with its carrier entry value");
    };
    let recipe = product.operations().core().recipe().as_recipe();
    // The entry value is the inner carrier's declared entry value.
    assert_eq!(recipe.carriers[2].entry_value, value);
    // The producing item is the operation whose result is the entry value.
    let producing = recipe
        .items
        .iter()
        .find(|candidate| candidate.key == row.producing_item())
        .expect("producing item exists in recipe");
    let LoopRecipeItemV1::Operation { operation } = &producing.item else {
        panic!("producing item must be an operation");
    };
    let LoopOperationV1::ConstI64 { result, .. } = operation else {
        panic!("child entry producer must be the initializer const");
    };
    assert_eq!(*result, value);
}

#[test]
fn declaration_rows_are_scoped_to_their_producing_item() {
    let product = nested_product();
    let set =
        VerifiedLoopInternalDeclarationSetV1::issue(product.operations(), &root_loop_site(&product))
            .expect("internal declaration set");
    let row = &set.rows()[0];
    assert_eq!(set.for_item(row.producing_item()).count(), 1);
    let other = product
        .operations()
        .evidence()
        .iter()
        .map(|evidence| evidence.item())
        .find(|item| *item != row.producing_item())
        .expect("another operation item");
    assert_eq!(set.for_item(other).count(), 0);
}

#[test]
fn direct_accum_loop_true_and_loop_cond_derive_no_internal_declarations() {
    let direct = direct_accum_product_for_test();
    let site = direct
        .operations()
        .evidence()
        .first()
        .expect("direct evidence")
        .source_loop()
        .clone();
    assert!(
        VerifiedLoopInternalDeclarationSetV1::issue(direct.operations(), &site)
            .expect("direct accum set")
            .is_empty()
    );
    let loop_true = true_tests::product();
    let site = loop_true
        .operations()
        .evidence()
        .first()
        .expect("loop true evidence")
        .source_loop()
        .clone();
    assert!(
        VerifiedLoopInternalDeclarationSetV1::issue(loop_true.operations(), &site)
            .expect("loop true set")
            .is_empty()
    );
    let loop_cond = cond_tests::product();
    let site = loop_cond
        .operations()
        .evidence()
        .first()
        .expect("loop cond evidence")
        .source_loop()
        .clone();
    assert!(
        VerifiedLoopInternalDeclarationSetV1::issue(loop_cond.operations(), &site)
            .expect("loop cond set")
            .is_empty()
    );
}
