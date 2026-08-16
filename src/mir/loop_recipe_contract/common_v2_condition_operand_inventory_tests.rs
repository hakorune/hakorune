use super::super::common_v2_after_boundary::issue_s6c_v2_after_boundary_source_relation_v1;
use super::super::common_v2_condition_operand_inventory::{
    issue_s6c_v2_condition_operand_inventory_v1, ConditionOperandInventoryRejectV1,
    PreparedLoopV2ConditionOperandKindV1,
};
use super::super::common_v2_condition_producer::issue_s6c_v2_condition_producer_relation_v1;
use super::super::common_v2_layout_input::issue_s6c_v2_layout_input;
use super::super::common_v2_predicate_branch_plan::issue_s6c_v2_predicate_branch_plan_v1;
use super::super::produce_s6c_scan_with_init_recipe_v2;
use super::super::s6c_prephysical_ingress::issue_s6c_prephysical_ingress_v2;
use super::super::s6c_scan_with_init_joinir_output::issue_s6c_scan_with_init_logical_output_v1;
use super::super::s6c_scan_with_init_tests::issue_facts;
use super::super::schema_v2::LoopOperationV2;
use super::{issue_control_source, issue_operation_source};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

fn issue_producer<'source, 'facts>(
    view: super::super::s6c_prephysical_ingress::S6CPrephysicalIngressRefV2<'_, 'source, 'facts>,
    owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
) -> (
    super::PreparedLoopOperationProgramV2<'source>,
    super::super::common_v2_condition_producer::PreparedLoopV2ConditionProducerRelationV1,
) {
    let operations = issue_operation_source(view).expect("operations");
    let layout = issue_s6c_v2_layout_input(view, owner).expect("layout");
    let control = issue_control_source(view).expect("control");
    let after =
        issue_s6c_v2_after_boundary_source_relation_v1(view, &layout, owner).expect("after");
    let branch =
        issue_s6c_v2_predicate_branch_plan_v1(view, &layout, control.transfer(), &after, owner)
            .expect("branch");
    let producer = issue_s6c_v2_condition_producer_relation_v1(view, &operations, &branch, owner)
        .expect("producer");
    (operations, producer)
}

#[test]
fn condition_operand_inventory_keeps_two_source_rows() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1411)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    ingress
        .with_ingress(|view| {
            let (operations, producer) = issue_producer(view, owner);
            let inventory =
                issue_s6c_v2_condition_operand_inventory_v1(view, &operations, &producer, owner)
                    .expect("operand inventory");
            assert_eq!(inventory.rows().len(), 2);
            assert!(matches!(
                inventory.rows()[0].kind(),
                PreparedLoopV2ConditionOperandKindV1::ReadBinding { .. }
            ));
            assert!(matches!(
                inventory.rows()[1].kind(),
                PreparedLoopV2ConditionOperandKindV1::LengthCall { .. }
            ));
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn condition_operand_inventory_rejects_length_operation_drift() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1412)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    ingress
        .with_ingress(|view| {
            let (mut operations, producer) = issue_producer(view, owner);
            let row = operations
                .rows
                .iter_mut()
                .find(|row| {
                    matches!(
                        row.operation,
                        LoopOperationV2::CallSlot {
                            result: Some(result),
                            ..
                        } if result == producer.right()
                    )
                })
                .expect("length operation");
            row.operation = LoopOperationV2::CallSlot {
                receiver: None,
                args: Vec::new(),
                result: Some(producer.right()),
            };
            let result =
                issue_s6c_v2_condition_operand_inventory_v1(view, &operations, &producer, owner);
            assert!(matches!(
                result,
                Err(ConditionOperandInventoryRejectV1::RightOperationMismatch)
            ));
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn condition_operand_inventory_rejects_foreign_owner() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1413)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    let mut issuer = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("foreign compilation brand");
    let foreign = issuer.issue().expect("foreign owner");
    ingress
        .with_ingress(|view| {
            let (operations, producer) = issue_producer(view, owner);
            let result =
                issue_s6c_v2_condition_operand_inventory_v1(view, &operations, &producer, foreign);
            assert!(matches!(
                result,
                Err(ConditionOperandInventoryRejectV1::ForeignOwner)
            ));
            Ok(())
        })
        .expect("ingress view");
}
