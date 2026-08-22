use super::super::ids::{LoopBlockKeyV1, LoopItemKeyV1, LoopValueKeyV1};
use super::super::produce_s6c_scan_with_init_recipe_v2;
use super::super::s6c_prephysical_ingress::issue_s6c_prephysical_ingress_v2;
use super::super::s6c_scan_with_init_joinir_output::issue_s6c_scan_with_init_logical_output_v1;
use super::super::s6c_scan_with_init_tests::issue_facts;
use super::super::{issue_s6c_v2_substring_call_target_plan_v1, SubstringCallTargetPlanRejectV1};
use super::{
    issue_s6c_common_v2_pre_session_v1, CommonV2IssuerRejectV1, PreparedLoopControlPlacementV2,
};

const FIXTURE: &str = include_str!("../../../apps/tests/scan_with_init_typed_ok_min.hako");

#[test]
fn common_v2_issues_generic_operation_control_and_passive_coverage() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1401)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    ingress
        .with_ingress(|view| {
            let envelope =
                issue_s6c_common_v2_pre_session_v1(view, owner).expect("common V2 envelope");
            assert_eq!(envelope.operations().operation_count(), 13);
            assert_eq!(envelope.control().control_count(), 2);
            assert_eq!(envelope.coverage().placement_count(), 15);
            assert_eq!(envelope.coverage().operation_count(), 13);
            assert_eq!(envelope.return_source_binding().owner(), owner);
            assert_eq!(envelope.return_source_binding().join_exit_item().raw(), 10);
            let return_read = envelope.return_read_co_seal();
            assert_eq!(return_read.owner(), owner);
            assert_eq!(return_read.return_item().raw(), 9);
            assert_eq!(return_read.return_block().raw(), 2);
            assert_eq!(return_read.return_value().raw(), 11);
            assert_eq!(return_read.return_split_ordinal(), 2);
            assert_eq!(return_read.if_item().raw(), 8);
            assert_eq!(return_read.if_block().raw(), 1);
            assert_eq!(return_read.if_condition().raw(), 10);
            assert_eq!(return_read.if_split_ordinal(), 1);
            assert_eq!(return_read.continuation().block.raw(), 1);
            assert_eq!(return_read.continuation().item.raw(), 11);
            assert_eq!(return_read.join_exit_item().raw(), 10);
            assert_eq!(
                return_read.join_target(),
                super::super::join_sig::LoopJoinBranchExitTargetV2::FunctionExit
            );
            assert_eq!(envelope.layout().loop_count(), 1);
            assert_eq!(envelope.layout().segment_count(), 3);
            assert_eq!(envelope.layout().segments()[0].split_ordinal(), 0);
            assert_eq!(
                envelope.layout().segments()[0].loop_key(),
                envelope.layout().after().0
            );
            let branch = envelope.predicate_branch();
            assert_eq!(branch.loop_key(), envelope.layout().after().0);
            assert_eq!(
                branch.condition().class(),
                super::super::schema_v2::LoopValueClassV2::Bool
            );
            assert_eq!(branch.true_target(), envelope.layout().loops()[0].body());
            assert_eq!(
                branch.false_target(),
                super::super::common_v2_predicate_branch_plan::
                    PreparedLoopV2PredicateFalseTargetV1::RootAfter
            );
            let producer = envelope.condition_producer();
            assert_eq!(producer.owner(), owner);
            assert_eq!(producer.loop_key(), branch.loop_key());
            assert_eq!(producer.condition_block(), branch.condition().block());
            assert_eq!(producer.result(), branch.condition().value());
            assert_eq!(
                producer.class(),
                super::super::schema_v2::LoopValueClassV2::Bool
            );
            assert_eq!(
                producer.op(),
                super::super::schema_v2::LoopCompareI64OpV2::Less
            );
            assert!(envelope
                .operations()
                .rows()
                .iter()
                .any(|row| row.item() == producer.producer_item()));
            let target_plan = issue_s6c_v2_substring_call_target_plan_v1(&envelope, owner)
                .expect("source-backed StringSubstring target plan");
            assert_eq!(target_plan.owner(), owner);
            assert_eq!(target_plan.item().raw(), 6);
            assert_eq!(target_plan.block().raw(), 1);
            assert_eq!(target_plan.result().raw(), 9);
            assert_eq!(target_plan.method_name(), "substring");
            assert_eq!(
                target_plan.provider().entry,
                crate::abi::text_scan_aot_export_facts::TextScanAotEntryIdV1::Substring
            );
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn substring_target_plan_rejects_foreign_owner_before_effect() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1404)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    let mut issuer = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("foreign compilation brand");
    let foreign_owner = issuer.issue().expect("foreign owner");
    ingress
        .with_ingress(|view| {
            let envelope =
                issue_s6c_common_v2_pre_session_v1(view, owner).expect("common V2 envelope");
            assert!(matches!(
                issue_s6c_v2_substring_call_target_plan_v1(&envelope, foreign_owner),
                Err(SubstringCallTargetPlanRejectV1::ForeignOwner)
            ));
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn common_v2_rejects_foreign_owner_before_product() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1402)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let mut issuer = crate::mir::resolved_semantics::FunctionOwnerIssuerV1::new_for_compilation()
        .expect("foreign compilation brand");
    let foreign_owner = issuer.issue().expect("foreign owner");
    ingress
        .with_ingress(|view| {
            let result = issue_s6c_common_v2_pre_session_v1(view, foreign_owner);
            assert!(matches!(result, Err(CommonV2IssuerRejectV1::ForeignOwner)));
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn common_v2_rejects_item_block_drift_for_operation_if_and_exit() {
    let output = issue_s6c_scan_with_init_logical_output_v1(
        produce_s6c_scan_with_init_recipe_v2(issue_facts(FIXTURE, 1403)).expect("S6C recipe"),
    )
    .expect("logical rows");
    let ingress = issue_s6c_prephysical_ingress_v2(output).expect("ingress");
    let owner = ingress
        .with_ingress(|view| Ok(view.source_owner()))
        .expect("owner view");
    ingress
        .with_ingress(|view| {
            let layout =
                super::super::common_v2_layout_input::issue_s6c_v2_layout_input(view, owner)
                    .expect("layout");
            let drift_block = |original: LoopBlockKeyV1, item: LoopItemKeyV1| {
                layout
                    .segments()
                    .iter()
                    .find(|segment| segment.block() != original && !segment.items().contains(&item))
                    .map(|segment| segment.block())
                    .expect("drift block")
            };

            let mut operations = super::issue_operation_source(view).expect("operations");
            let operation_item = operations.rows()[0].item();
            operations.rows[0].block = drift_block(operations.rows[0].block, operation_item);
            let control = super::issue_control_source(view).expect("control");
            assert!(matches!(
                super::validate_layout_relation(&layout, &operations, &control),
                Err(CommonV2IssuerRejectV1::LayoutRelation)
            ));

            let operations = super::issue_operation_source(view).expect("operations");
            let mut control = super::issue_control_source(view).expect("control");
            let if_index = control
                .rows
                .iter()
                .position(|row| matches!(row, PreparedLoopControlPlacementV2::If { .. }))
                .expect("If row");
            let PreparedLoopControlPlacementV2::If {
                item,
                block,
                condition,
                then_block,
                else_block,
            } = control.rows[if_index]
            else {
                unreachable!()
            };
            control.rows[if_index] = PreparedLoopControlPlacementV2::If {
                item,
                block: drift_block(block, item),
                condition,
                then_block,
                else_block,
            };
            assert!(matches!(
                super::validate_layout_relation(&layout, &operations, &control),
                Err(CommonV2IssuerRejectV1::LayoutRelation)
            ));

            let operations = super::issue_operation_source(view).expect("operations");
            let mut control = super::issue_control_source(view).expect("control");
            let exit_index = control
                .rows
                .iter()
                .position(|row| matches!(row, PreparedLoopControlPlacementV2::Exit { .. }))
                .expect("Exit row");
            let PreparedLoopControlPlacementV2::Exit {
                item,
                block,
                exit,
                value,
            } = control.rows[exit_index]
            else {
                unreachable!()
            };
            control.rows[exit_index] = PreparedLoopControlPlacementV2::Exit {
                item,
                block: drift_block(block, item),
                exit,
                value,
            };
            assert!(matches!(
                super::validate_layout_relation(&layout, &operations, &control),
                Err(CommonV2IssuerRejectV1::LayoutRelation)
            ));
            Ok(())
        })
        .expect("ingress view");
}

#[test]
fn return_read_co_seal_rejects_operation_and_exit_drift() {
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
            let layout =
                super::super::common_v2_layout_input::issue_s6c_v2_layout_input(view, owner)
                    .expect("layout");
            let control = super::issue_control_source(view).expect("control");
            let mut operations = super::issue_operation_source(view).expect("operations");
            let return_index = operations
                .rows
                .iter()
                .position(|row| row.item().raw() == 9)
                .expect("Return-read row");
            operations.rows[return_index].block = LoopBlockKeyV1::new(0);
            assert!(matches!(
                super::super::issue_s6c_v2_return_read_co_seal_v1(
                    view,
                    &operations,
                    &control,
                    &layout,
                ),
                Err(super::super::ReturnReadCoSealRejectV1::ReturnOperationMismatch)
            ));

            let operations = super::issue_operation_source(view).expect("operations");
            let mut control = super::issue_control_source(view).expect("control");
            let exit_index = control
                .rows
                .iter()
                .position(|row| matches!(row, PreparedLoopControlPlacementV2::Exit { .. }))
                .expect("Exit row");
            let PreparedLoopControlPlacementV2::Exit {
                item, block, exit, ..
            } = control.rows[exit_index]
            else {
                unreachable!()
            };
            control.rows[exit_index] = PreparedLoopControlPlacementV2::Exit {
                item,
                block,
                exit,
                value: LoopValueKeyV1::new(12),
            };
            assert!(matches!(
                super::super::issue_s6c_v2_return_read_co_seal_v1(
                    view,
                    &operations,
                    &control,
                    &layout,
                ),
                Err(super::super::ReturnReadCoSealRejectV1::ExitPlacementMismatch)
            ));
            Ok(())
        })
        .expect("ingress view");
}
