use super::super::common_v2_after_boundary::issue_s6c_v2_after_boundary_source_relation_v1;
use super::super::common_v2_condition_producer::{
    issue_s6c_v2_condition_producer_relation_v1, ConditionProducerRelationRejectV1,
};
use super::super::common_v2_layout_input::issue_s6c_v2_layout_input;
use super::super::common_v2_predicate_branch_plan::issue_s6c_v2_predicate_branch_plan_v1;
use super::super::produce_s6c_scan_with_init_recipe_v2;
use super::super::s6c_prephysical_ingress::issue_s6c_prephysical_ingress_v2;
use super::super::s6c_scan_with_init_joinir_output::issue_s6c_scan_with_init_logical_output_v1;
use super::super::s6c_scan_with_init_tests::issue_facts;
use super::super::schema_v2::{LoopBinaryI64OpV2, LoopOperationV2};
use super::{issue_control_source, issue_operation_source};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn condition_producer_rejects_non_compare_operation_projection() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1404)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    ingress
        .with_ingress(|view| {
            let mut operations = issue_operation_source(view).expect("operations");
            let compare = operations
                .rows
                .iter_mut()
                .find(|row| matches!(&row.operation, LoopOperationV2::CompareI64 { .. }))
                .expect("condition operation");
            let LoopOperationV2::CompareI64 {
                left,
                right,
                result,
                ..
            } = compare.operation.clone()
            else {
                unreachable!()
            };
            compare.operation = LoopOperationV2::BinaryI64 {
                op: LoopBinaryI64OpV2::Add,
                left,
                right,
                result,
            };
            let layout = issue_s6c_v2_layout_input(view, owner).expect("layout");
            let control = issue_control_source(view).expect("control");
            let after = issue_s6c_v2_after_boundary_source_relation_v1(view, &layout, owner)
                .expect("after");
            let branch = issue_s6c_v2_predicate_branch_plan_v1(
                view,
                &layout,
                control.transfer(),
                &after,
                owner,
            )
            .expect("branch");
            let result =
                issue_s6c_v2_condition_producer_relation_v1(view, &operations, &branch, owner);
            assert!(matches!(
                result,
                Err(ConditionProducerRelationRejectV1::OperationRowMismatch)
            ));
            Ok(())
        })
        .expect("ingress view");
}
