use super::producer::produce_generic_g0_recipe_v1;
use crate::mir::exact_trivial_return_abi::ExactTrivialReturnAbiV1;
use crate::mir::loop_recipe_contract::generic_g0_demand::issue_generic_g0_recipe_demand_v1;
use crate::mir::loop_recipe_contract::{LoopBindingEffectAnchorV1, LoopItemKeyV1};
use crate::mir::loop_route_policy::generic_selection_for_test;

#[test]
fn generic_g0_recipe_producer_seals_one_complete_product() {
    let selection = generic_selection_for_test();
    let expected_frame = selection.lease().frame();
    let demand = issue_generic_g0_recipe_demand_v1(selection).expect("natural Generic G0 demand");
    let product = produce_generic_g0_recipe_v1(demand).expect("Generic G0 Recipe product");
    let recipe = product.core().recipe().as_recipe();

    assert_eq!(recipe.loops.len(), 2);
    assert_eq!(recipe.carriers.len(), 3);
    assert_eq!(recipe.values.len(), 15);
    assert_eq!(product.core().binding_relations().len(), 2);
    assert_eq!(product.core().effect_relations().len(), 10);
    assert_eq!(product.operation_effect().evidence().len(), 15);
    let items = product
        .operation_effect()
        .evidence()
        .iter()
        .map(|row| row.item().raw())
        .collect::<Vec<_>>();
    assert_eq!(
        items,
        vec![0, 1, 2, 3, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]
    );
    let child_entry = product
        .operation_effect()
        .evidence()
        .iter()
        .find(|row| row.item() == LoopItemKeyV1::new(3))
        .expect("G0 child-entry operation");
    assert!(matches!(
        child_entry.anchor(),
        LoopBindingEffectAnchorV1::DerivedCarrierEntry { carrier, .. }
            if carrier.raw() == 2
    ));
    assert_eq!(product.after().after_binding().loop_key().raw(), 0);
    assert_eq!(product.after().after_binding().binding().raw(), 1);
    assert_eq!(product.after().return_abi(), ExactTrivialReturnAbiV1::I64);
    assert_eq!(
        product.after().post_loop_read().binding().binding().raw(),
        product.core().binding_relations()[1]
            .source_binding()
            .binding()
            .raw()
    );
    assert_eq!(product.after().owner(), product.core().owner());
    assert_eq!(product.after().frame(), &expected_frame);
    assert_eq!(
        product.target(),
        crate::mir::numeric_substrate::NumericTarget::host()
    );
}

#[test]
fn generic_g0_recipe_mapping_is_deterministic() {
    let first = produce_generic_g0_recipe_v1(
        issue_generic_g0_recipe_demand_v1(generic_selection_for_test()).unwrap(),
    )
    .unwrap();
    let second = produce_generic_g0_recipe_v1(
        issue_generic_g0_recipe_demand_v1(generic_selection_for_test()).unwrap(),
    )
    .unwrap();
    assert_eq!(
        first.core().recipe().as_recipe(),
        second.core().recipe().as_recipe()
    );
    assert_eq!(first.core().effect_relations().len(), 10);
    assert_eq!(second.core().effect_relations().len(), 10);
}
