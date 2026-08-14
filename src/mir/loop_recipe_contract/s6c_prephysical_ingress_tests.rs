use super::s6c_scan_with_init_tests::issue_facts;
use super::{
    issue_s6c_prephysical_ingress_v2, issue_s6c_scan_with_init_logical_output_v1,
    produce_s6c_scan_with_init_recipe_v2, S6CPrephysicalOperationRoleV2,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn prephysical_ingress_seals_exact_source_and_transfer_census() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 951))
            .expect("exact S6C Recipe product"),
    )
    .expect("logical output rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("prephysical ingress");

    ingress
        .with_ingress(|view| {
            assert_eq!(view.operation_count(), 13);
            assert_eq!(view.operation_roles().count(), 13);
            assert_eq!(
                view.anchor_count(S6CPrephysicalOperationRoleV2::BodyIndexRead),
                2
            );
            assert_eq!(
                view.anchor_count(S6CPrephysicalOperationRoleV2::StepWrite),
                2
            );
            assert_eq!(view.input_bindings().len(), 3);
            assert!(view
                .operation_execution(S6CPrephysicalOperationRoleV2::LengthCall)
                .is_some());
            assert_eq!(
                view.after(),
                (
                    super::LoopNodeKeyV1::new(0),
                    super::LoopBindingKeyV1::new(0),
                    super::LoopValueClassV2::I64
                )
            );
            assert_eq!(
                view.logical()
                    .logical_transfer()
                    .boundaries()
                    .iter()
                    .filter(|row| { row.role == super::LoopJoinEdgeRoleV1::Backedge })
                    .count(),
                1
            );
            let (loop_return, tail, cleanup_empty) = view.completion();
            assert_ne!(loop_return, tail);
            assert!(cleanup_empty);
            Ok(())
        })
        .expect("ingress façade");
}
