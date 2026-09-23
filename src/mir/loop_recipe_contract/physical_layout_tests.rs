use super::*;
use crate::mir::compiler::callable_single_loop_operation_effect::callable_operation_demand_parts_for_test;
use crate::mir::loop_recipe_contract::generic_g0::generic_operation_demand_parts_for_test;
use crate::mir::loop_recipe_contract::join_sig::{
    LoopJoinBoundaryTransferRefV1, LoopJoinBranchArmTransferRefV1, LoopJoinBranchExitRefV1,
    LoopJoinBranchTransferRefV1, LoopJoinEdgeRoleV1, LoopJoinLogicalTransferViewV1,
    LoopJoinNextItemV1, LoopJoinPayloadV1, LoopJoinPortV1,
};
use crate::mir::loop_recipe_contract::VerifiedLoopOperationPhysicalDemandV1;

fn transfer(
    loop_key: LoopNodeKeyV1,
    from: LoopJoinPortV1,
    to: LoopJoinPortV1,
    role: LoopJoinEdgeRoleV1,
    condition: Option<(LoopBlockKeyV1, LoopValueKeyV1)>,
) -> LoopJoinBoundaryTransferRefV1<'static> {
    LoopJoinBoundaryTransferRefV1 {
        loop_key,
        from,
        to,
        role,
        condition,
        payload: &[],
    }
}

fn callable_layout() -> PreparedLoopPhysicalLayoutV1 {
    let (effect, context, continuation) = callable_operation_demand_parts_for_test();
    VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
        .expect("callable demand")
        .prepare_all()
        .expect("callable program")
        .prepare_physical_layout()
        .expect("callable layout")
}

#[test]
fn if_continue_layout_splits_the_source_block_and_targets_the_loop_header() {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition_block = LoopBlockKeyV1::new(0);
    let body_block = LoopBlockKeyV1::new(1);
    let then_block = LoopBlockKeyV1::new(2);
    let condition_value = LoopValueKeyV1::new(0);
    let branch_value = LoopValueKeyV1::new(1);
    let branch_item = LoopItemKeyV1::new(1);
    let exit_item = LoopItemKeyV1::new(3);
    let continuation_item = LoopItemKeyV1::new(4);
    let header = LoopPhysicalSegmentKeyV1::new(loop_key, condition_block, 0);
    let continue_segment = LoopPhysicalSegmentKeyV1::new(loop_key, then_block, 0);
    let resume_segment = LoopPhysicalSegmentKeyV1::new(loop_key, body_block, 1);
    let recipe = LoopRecipeV1 {
        root_loop: loop_key,
        loops: vec![super::super::schema::LoopNodeV1 {
            key: loop_key,
            parent: None,
            condition: super::super::schema::LoopConditionV1::Predicate {
                block: condition_block,
                value: condition_value,
            },
            body: body_block,
        }],
        blocks: vec![
            super::super::schema::LoopRecipeBlockV1 {
                key: condition_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(0)],
            },
            super::super::schema::LoopRecipeBlockV1 {
                key: body_block,
                owner_loop: loop_key,
                items: vec![branch_item, continuation_item],
            },
            super::super::schema::LoopRecipeBlockV1 {
                key: then_block,
                owner_loop: loop_key,
                items: vec![LoopItemKeyV1::new(2), exit_item],
            },
        ],
        items: vec![
            super::super::schema::LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(0),
                item: LoopRecipeItemV1::Operation {
                    operation: super::super::schema::LoopOperationV1::ConstI64 {
                        result: condition_value,
                        value: 1,
                    },
                },
            },
            super::super::schema::LoopRecipeItemRowV1 {
                key: branch_item,
                item: LoopRecipeItemV1::If {
                    condition: branch_value,
                    then_block,
                    else_block: None,
                },
            },
            super::super::schema::LoopRecipeItemRowV1 {
                key: LoopItemKeyV1::new(2),
                item: LoopRecipeItemV1::Operation {
                    operation: super::super::schema::LoopOperationV1::ConstI64 {
                        result: branch_value,
                        value: 1,
                    },
                },
            },
            super::super::schema::LoopRecipeItemRowV1 {
                key: exit_item,
                item: LoopRecipeItemV1::Exit {
                    exit: super::super::ids::LoopExitKeyV1::new(0),
                },
            },
            super::super::schema::LoopRecipeItemRowV1 {
                key: continuation_item,
                item: LoopRecipeItemV1::Operation {
                    operation: super::super::schema::LoopOperationV1::ConstI64 {
                        result: LoopValueKeyV1::new(2),
                        value: 1,
                    },
                },
            },
        ],
        bindings: Vec::new(),
        values: Vec::new(),
        inputs: Vec::new(),
        carriers: Vec::new(),
        exits: Vec::new(),
    };
    let no_payload: &'static [LoopJoinPayloadV1] = &[];
    let boundaries = vec![
        transfer(
            loop_key,
            LoopJoinPortV1::Preheader,
            LoopJoinPortV1::Header,
            LoopJoinEdgeRoleV1::Enter,
            None,
        ),
        transfer(
            loop_key,
            LoopJoinPortV1::Header,
            LoopJoinPortV1::Body,
            LoopJoinEdgeRoleV1::PredicateTrue,
            Some((condition_block, condition_value)),
        ),
        transfer(
            loop_key,
            LoopJoinPortV1::Header,
            LoopJoinPortV1::After,
            LoopJoinEdgeRoleV1::PredicateFalse,
            Some((condition_block, condition_value)),
        ),
        transfer(
            loop_key,
            LoopJoinPortV1::Body,
            LoopJoinPortV1::Header,
            LoopJoinEdgeRoleV1::Backedge,
            None,
        ),
    ];
    let branch = LoopJoinBranchTransferRefV1 {
        owner_loop: loop_key,
        if_item: branch_item,
        condition: branch_value,
        then_arm: LoopJoinBranchArmTransferRefV1::Exit(LoopJoinBranchExitRefV1 {
            exit_item,
            role: LoopJoinEdgeRoleV1::Continue,
            target_loop: loop_key,
            payload: no_payload,
        }),
        else_arm: LoopJoinBranchArmTransferRefV1::Fallthrough {
            continuation: LoopJoinNextItemV1 {
                block: body_block,
                item: continuation_item,
            },
            payload: no_payload,
        },
    };
    let transfers = LoopJoinLogicalTransferViewV1::for_test(boundaries.clone(), vec![branch]);
    let mut builder = LayoutBuilder::new(&recipe, &transfers);
    builder
        .build_loop(loop_key, LoopPhysicalTargetV1::OpenRootAfter)
        .expect("If/Continue layout");
    let (segments, visited_items, operations) = builder.finish().expect("exact coverage");
    assert_eq!(visited_items.len(), 5);
    assert_eq!(
        operations,
        [
            LoopItemKeyV1::new(0),
            LoopItemKeyV1::new(2),
            continuation_item
        ]
    );
    assert_eq!(segments.len(), 4);
    assert!(matches!(
        segments[1].transfer(),
        LoopPhysicalTransferV1::Predicate {
            condition,
            on_true,
            on_false: LoopPhysicalTargetV1::Segment(false_target),
        } if condition == branch_value && on_true == continue_segment && false_target == resume_segment
    ));
    assert!(matches!(
        segments[2].transfer(),
        LoopPhysicalTransferV1::Jump {
            target: LoopPhysicalTargetV1::Segment(target)
        } if target == header
    ));
    assert_eq!(segments[3].key(), resume_segment);
    assert!(matches!(
        segments[3].transfer(),
        LoopPhysicalTransferV1::Jump {
            target: LoopPhysicalTargetV1::Segment(target)
        } if target == header
    ));

    let mut unconsumed = branch;
    unconsumed.if_item = LoopItemKeyV1::new(99);
    let transfers = LoopJoinLogicalTransferViewV1::for_test(boundaries, vec![branch, unconsumed]);
    let mut builder = LayoutBuilder::new(&recipe, &transfers);
    builder
        .build_loop(loop_key, LoopPhysicalTargetV1::OpenRootAfter)
        .expect("selected branch remains valid");
    assert!(matches!(
        builder.finish(),
        Err(LoopPhysicalLayoutRejectV1::BranchCoverage { expected, consumed })
            if expected.len() == 2 && consumed.len() == 1
    ));
}

fn generic_layout() -> PreparedLoopPhysicalLayoutV1 {
    let (effect, context, continuation) = generic_operation_demand_parts_for_test();
    VerifiedLoopOperationPhysicalDemandV1::issue(context, effect, continuation)
        .expect("generic demand")
        .prepare_all()
        .expect("generic program")
        .prepare_physical_layout()
        .expect("generic layout")
}

#[test]
fn callable_layout_keeps_one_segment_per_logical_block() {
    let layout = callable_layout();
    assert_eq!(layout.coverage().item_count(), 7);
    assert_eq!(layout.coverage().operation_count(), 7);
    assert_eq!(layout.coverage().segment_count(), 2);
    assert_eq!(layout.entry_segment(), layout.segments()[0].key());
    assert_eq!(
        layout.segments()[0].role(),
        LoopPhysicalSegmentRoleV1::Header
    );
    assert_eq!(layout.segments()[1].role(), LoopPhysicalSegmentRoleV1::Body);
    assert_eq!(
        layout.segments()[0].operations(),
        [
            LoopItemKeyV1::new(0),
            LoopItemKeyV1::new(1),
            LoopItemKeyV1::new(2),
        ]
    );
    assert_eq!(layout.segments()[1].operations().len(), 4);
    assert!(matches!(
        layout.segments()[0].transfer(),
        LoopPhysicalTransferV1::Predicate { .. }
    ));
}

#[test]
fn generic_layout_splits_parent_around_nested_loop_and_resumes() {
    let layout = generic_layout();
    let root = LoopNodeKeyV1::new(0);
    let child = LoopNodeKeyV1::new(1);
    let root_condition = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(0), 0);
    let root_before_child = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(1), 0);
    let child_condition = LoopPhysicalSegmentKeyV1::new(child, LoopBlockKeyV1::new(2), 0);
    let child_body = LoopPhysicalSegmentKeyV1::new(child, LoopBlockKeyV1::new(3), 0);
    let root_resume = LoopPhysicalSegmentKeyV1::new(root, LoopBlockKeyV1::new(1), 1);
    assert_eq!(layout.coverage().item_count(), 16);
    assert_eq!(layout.coverage().operation_count(), 15);
    assert_eq!(layout.coverage().segment_count(), 5);
    assert_eq!(layout.entry_segment(), layout.segments()[0].key());
    assert_eq!(
        layout.segments()[0].role(),
        LoopPhysicalSegmentRoleV1::Header
    );
    assert_eq!(layout.segments()[1].role(), LoopPhysicalSegmentRoleV1::Body);
    assert_eq!(
        layout.segments()[2].role(),
        LoopPhysicalSegmentRoleV1::Header
    );
    assert_eq!(layout.segments()[0].key(), root_condition);
    assert_eq!(layout.segments()[1].key(), root_before_child);
    assert_eq!(layout.segments()[2].key(), child_condition);
    assert_eq!(layout.segments()[3].key(), child_body);
    assert_eq!(layout.segments()[4].key(), root_resume);
    assert_eq!(layout.segments()[1].operations(), [LoopItemKeyV1::new(3)]);
    assert_eq!(
        layout.segments()[4].operations(),
        [
            LoopItemKeyV1::new(12),
            LoopItemKeyV1::new(13),
            LoopItemKeyV1::new(14),
            LoopItemKeyV1::new(15),
        ]
    );
    assert!(matches!(
        layout.segments()[1].transfer(),
        LoopPhysicalTransferV1::OpenNestedLoop {
            loop_key,
            entry
        } if loop_key == child && entry == child_condition
    ));
    assert!(matches!(
        layout.segments()[2].transfer(),
        LoopPhysicalTransferV1::Predicate {
            on_false: LoopPhysicalTargetV1::Segment(target), ..
        } if target == root_resume
    ));
}

#[test]
fn predicate_transfer_binder_rejects_role_port_loop_and_condition_drift() {
    let loop_key = LoopNodeKeyV1::new(0);
    let condition = Some((LoopBlockKeyV1::new(0), LoopValueKeyV1::new(1)));
    let true_target = LoopPhysicalSegmentKeyV1::new(loop_key, LoopBlockKeyV1::new(0), 0);
    let false_target = LoopPhysicalTargetV1::OpenRootAfter;

    assert!(matches!(
        super::super::physical_transfer::bind_predicate(
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::Body,
                LoopJoinEdgeRoleV1::Backedge,
                condition,
            ),
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::After,
                LoopJoinEdgeRoleV1::PredicateFalse,
                condition,
            ),
            true_target,
            false_target,
        ),
        Err(LoopPhysicalTransferBindingRejectV1::RoleMismatch { .. })
    ));
    assert!(matches!(
        super::super::physical_transfer::bind_predicate(
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::Body,
                LoopJoinEdgeRoleV1::PredicateTrue,
                condition,
            ),
            transfer(
                loop_key,
                LoopJoinPortV1::Body,
                LoopJoinPortV1::After,
                LoopJoinEdgeRoleV1::PredicateFalse,
                condition,
            ),
            true_target,
            false_target,
        ),
        Err(LoopPhysicalTransferBindingRejectV1::PortMismatch { .. })
    ));
    assert!(matches!(
        super::super::physical_transfer::bind_predicate(
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::Body,
                LoopJoinEdgeRoleV1::PredicateTrue,
                condition,
            ),
            transfer(
                LoopNodeKeyV1::new(1),
                LoopJoinPortV1::Header,
                LoopJoinPortV1::After,
                LoopJoinEdgeRoleV1::PredicateFalse,
                condition,
            ),
            true_target,
            false_target,
        ),
        Err(LoopPhysicalTransferBindingRejectV1::LoopMismatch { .. })
    ));
    assert!(matches!(
        super::super::physical_transfer::bind_predicate(
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::Body,
                LoopJoinEdgeRoleV1::PredicateTrue,
                condition,
            ),
            transfer(
                loop_key,
                LoopJoinPortV1::Header,
                LoopJoinPortV1::After,
                LoopJoinEdgeRoleV1::PredicateFalse,
                Some((LoopBlockKeyV1::new(0), LoopValueKeyV1::new(2))),
            ),
            true_target,
            false_target,
        ),
        Err(LoopPhysicalTransferBindingRejectV1::ConditionMismatch { .. })
    ));
}

#[test]
fn nested_and_backedge_binders_reject_wrong_loop_or_role() {
    let loop_key = LoopNodeKeyV1::new(0);
    let segment = LoopPhysicalSegmentKeyV1::new(loop_key, LoopBlockKeyV1::new(0), 0);
    assert!(matches!(
        super::super::physical_transfer::bind_backedge(
            transfer(
                loop_key,
                LoopJoinPortV1::Body,
                LoopJoinPortV1::Header,
                LoopJoinEdgeRoleV1::Enter,
                None,
            ),
            LoopPhysicalTargetV1::Segment(segment),
        ),
        Err(LoopPhysicalTransferBindingRejectV1::RoleMismatch { .. })
    ));
    assert!(matches!(
        super::super::physical_transfer::bind_nested_loop(
            transfer(
                loop_key,
                LoopJoinPortV1::Preheader,
                LoopJoinPortV1::Header,
                LoopJoinEdgeRoleV1::Enter,
                None,
            ),
            LoopNodeKeyV1::new(1),
            segment,
        ),
        Err(LoopPhysicalTransferBindingRejectV1::LoopMismatch { .. })
    ));
}
