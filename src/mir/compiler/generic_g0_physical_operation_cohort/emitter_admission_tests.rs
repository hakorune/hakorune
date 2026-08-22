use super::issue_generic_g0_physical_emitter_admission_v1;
use crate::mir::loop_route_policy::generic_source_unit_and_selection_for_test;
use crate::mir::EffectMask;

#[test]
fn owns_one_complete_prephysical_cohort_and_lends_mapping_scoped() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("root input");
    let owner = input.owner();
    let target = crate::mir::numeric_substrate::NumericTarget::host();
    let admission = issue_generic_g0_physical_emitter_admission_v1(input, selection)
        .expect("combined Generic emitter admission");

    admission
        .with_mapping(|view, mapping| {
            assert_eq!(view.owner(), owner);
            assert_eq!(view.target(), target);
            assert_eq!(view.program_revision(), 1);
            assert_eq!(view.layout_revision(), 1);
            assert_eq!(view.layout().coverage().item_count(), 16);
            assert_eq!(view.layout().coverage().operation_count(), 15);
            assert_eq!(view.layout().coverage().segment_count(), 5);
            assert_eq!(view.entries().len(), 2);
            assert_eq!(
                view.entries()
                    .iter()
                    .map(|row| row.parameter_index())
                    .collect::<Vec<_>>(),
                vec![0, 1]
            );
            assert!(view
                .entries()
                .iter()
                .all(|row| row.binding().owner() == owner));
            assert_eq!(mapping.owner(), owner);
            assert_eq!(mapping.operation_count(), 15);

            let shell = view.shell_plan();
            assert_eq!(shell.symbol().as_mir_name(), "generic_g0/2");
            assert_eq!(shell.descriptors().len(), 3);
            assert_eq!(shell.result_abi().source_type_name(), "i64");
            assert_eq!(shell.effects().effect_mask(), EffectMask::PURE);
            assert_eq!(shell.effects().operation_count(), 15);

            assert_eq!(view.control().expectation().owner(), owner);
            assert_eq!(view.control().outer_if().owner(), owner);
            assert_eq!(view.control().outer_if().row_count(), 0);
            assert_eq!(view.completion().owner(), owner);
            assert!(view.completion().returns_value());
        })
        .expect("scoped Generic mapping");
}

#[test]
fn rejects_foreign_selection_before_admission_publication() {
    let (_, foreign_selection) = generic_source_unit_and_selection_for_test();
    let (unit, _) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("foreign root input");

    let error = match issue_generic_g0_physical_emitter_admission_v1(input, foreign_selection) {
        Ok(_) => panic!("foreign cohort must reject"),
        Err(error) => error,
    };
    assert!(matches!(
        error,
        super::GenericG0PhysicalEmitterAdmissionRejectV1::SourceParent(
            crate::mir::compiler::generic_g0_source_parent::GenericG0SourceParentRejectV1::SelectionOwnerMismatch
                | crate::mir::compiler::generic_g0_source_parent::GenericG0SourceParentRejectV1::SelectionOriginMismatch
                | crate::mir::compiler::generic_g0_source_parent::GenericG0SourceParentRejectV1::SelectionSiteMismatch
        )
    ));
}

#[test]
fn consume_lends_only_one_callback_scoped_view() {
    let (unit, selection) = generic_source_unit_and_selection_for_test();
    let input = unit.root_function_input().expect("root input");
    let owner = input.owner();
    let admission = issue_generic_g0_physical_emitter_admission_v1(input, selection)
        .expect("combined Generic emitter admission");

    let observed = admission.consume(|view| {
        (
            view.owner(),
            view.layout().coverage().item_count(),
            view.layout().coverage().segment_count(),
        )
    });
    assert_eq!(observed.0, owner);
    assert_eq!(observed.1, 16);
    assert_eq!(observed.2, 5);
}

#[test]
fn admission_source_has_no_physical_state_or_legacy_adapter_surface() {
    let source = include_str!("emitter_admission.rs");
    for forbidden in [
        "MirFunction",
        "MirBuilder",
        "ValueId",
        "BasicBlockId",
        "CanonicalSsaFunctionSessionV2",
        "ReadyLoopEntryV1",
        "LoopPhysicalBlockReceiptV1",
        "PreparedGenericG0PhysicalFunctionSkeletonV1",
        "GenericG0PhysicalEntryAdmissionV1",
        "into_parts",
    ] {
        assert!(
            !source.contains(forbidden),
            "forbidden admission dependency: {forbidden}"
        );
    }
    assert_eq!(
        source
            .matches("pub(crate) fn issue_generic_g0_physical_emitter_admission_v1")
            .count(),
        1
    );
}
