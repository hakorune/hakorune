use super::generic_g0_source_parent::{
    with_generic_g0_source_parent_v1, GenericG0SourceParentRejectV1,
};
use crate::mir::compiler::VerifiedResolvedSourceUnitV1;
use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;

#[test]
fn source_parent_lends_one_cohort_with_exact_entry_rows() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("root input");
    let owner = input.owner();
    let result = with_generic_g0_source_parent_v1(input, selection, |cohort| {
        assert_eq!(cohort.owner(), owner);
        assert_eq!(cohort.entries().len(), 2);
        assert_eq!(cohort.entries()[0].parameter_index(), 0);
        assert_eq!(cohort.entries()[1].parameter_index(), 1);
        cohort.product().core().owner()
    })
    .expect("source cohort");
    assert_eq!(result, owner);
}

#[test]
fn source_parent_rejects_foreign_resolver_input_before_product() {
    let (unit_a, selection) = generic_source_unit_and_selection_for_test();
    let (unit_b, _) = generic_source_unit_and_selection_for_test();
    let _input_a = unit_a.root_function_input().expect("root A");
    let input_b = unit_b.root_function_input().expect("root B");
    let result = with_generic_g0_source_parent_v1(input_b, selection, |_| ());
    assert!(matches!(
        result,
        Err(GenericG0SourceParentRejectV1::SelectionOwnerMismatch)
            | Err(GenericG0SourceParentRejectV1::SelectionOriginMismatch)
            | Err(GenericG0SourceParentRejectV1::SelectionSiteMismatch)
    ));
}
