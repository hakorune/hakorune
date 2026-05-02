use super::*;
use crate::mir::{
    sum_placement::{SumObjectizationBarrier, SumPlacementFact},
    thin_entry::{ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryValueClass},
    thin_entry_selection::ThinEntrySelectionState,
    EffectMask, FunctionSignature,
};

fn make_function() -> MirFunction {
    MirFunction::new(
        FunctionSignature {
            name: "main".to_string(),
            params: vec![],
            return_type: MirType::Integer,
            effects: EffectMask::PURE,
        },
        BasicBlockId::new(0),
    )
}

fn push_metadata(
    function: &mut MirFunction,
    make_idx: usize,
    tag_idx: usize,
    enum_name: &str,
    variant: &str,
    sum_value: ValueId,
    tag_value: ValueId,
    layout: SumLocalAggregateLayout,
) {
    let subject = format!("{enum_name}::{variant}");
    function.metadata.thin_entry_selections = vec![
        ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: make_idx,
            value: Some(sum_value),
            surface: ThinEntrySurface::VariantMake,
            subject: subject.clone(),
            manifest_row: "variant_make.aggregate_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::AggLocal,
            demand: ThinEntryDemand::LocalAggregate,
            reason: "test make".to_string(),
        },
        ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: tag_idx,
            value: Some(tag_value),
            surface: ThinEntrySurface::VariantTag,
            subject: enum_name.to_string(),
            manifest_row: "variant_tag.tag_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::InlineI64,
            demand: ThinEntryDemand::InlineScalar,
            reason: "test tag".to_string(),
        },
    ];
    function.metadata.sum_placement_facts = vec![
        SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: make_idx,
            value: Some(sum_value),
            surface: ThinEntrySurface::VariantMake,
            subject: subject.clone(),
            source_sum: None,
            value_class: ThinEntryValueClass::AggLocal,
            state: SumPlacementState::LocalAggregateCandidate,
            tag_reads: 1,
            project_reads: 0,
            barriers: Vec::<SumObjectizationBarrier>::new(),
            reason: "test make fact".to_string(),
        },
        SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: tag_idx,
            value: Some(tag_value),
            surface: ThinEntrySurface::VariantTag,
            subject: enum_name.to_string(),
            source_sum: Some(sum_value),
            value_class: ThinEntryValueClass::InlineI64,
            state: SumPlacementState::LocalAggregateCandidate,
            tag_reads: 1,
            project_reads: 0,
            barriers: Vec::<SumObjectizationBarrier>::new(),
            reason: "test tag fact".to_string(),
        },
    ];
    function.metadata.sum_placement_selections = vec![
        SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: make_idx,
            value: Some(sum_value),
            surface: ThinEntrySurface::VariantMake,
            subject: subject.clone(),
            source_sum: None,
            manifest_row: "variant_make.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "test make selection".to_string(),
        },
        SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: tag_idx,
            value: Some(tag_value),
            surface: ThinEntrySurface::VariantTag,
            subject: enum_name.to_string(),
            source_sum: Some(sum_value),
            manifest_row: "variant_tag.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "test tag selection".to_string(),
        },
    ];
    function.metadata.sum_placement_layouts = vec![SumPlacementLayout {
        block: BasicBlockId::new(0),
        instruction_index: make_idx,
        value: Some(sum_value),
        surface: ThinEntrySurface::VariantMake,
        subject,
        source_sum: None,
        layout,
        reason: "test layout".to_string(),
    }];
}

#[test]
fn sum_variant_tag_seed_detects_i64_route_from_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(41),
    });
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "Result".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: Some(ValueId::new(1)),
        payload_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::VariantTag {
        dst: ValueId::new(3),
        value: ValueId::new(2),
        enum_name: "Result".to_string(),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    push_metadata(
        &mut function,
        1,
        2,
        "Result",
        "Ok",
        ValueId::new(2),
        ValueId::new(3),
        SumLocalAggregateLayout::TagI64Payload,
    );

    refresh_function_sum_variant_tag_seed_route(&mut function);

    let route = function
        .metadata
        .sum_variant_tag_seed_route
        .expect("sum variant tag route");
    assert_eq!(route.kind, SumVariantTagSeedKind::LocalI64);
    assert_eq!(route.subject, "Result::Ok");
    assert_eq!(route.layout, SumLocalAggregateLayout::TagI64Payload);
    assert_eq!(route.variant_tag, 0);
    assert_eq!(route.payload_value, Some(ValueId::new(1)));
    assert_eq!(route.copy_value, None);
}

#[test]
fn sum_variant_tag_seed_detects_copy_i64_route_from_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(41),
    });
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "Result".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: Some(ValueId::new(1)),
        payload_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(4),
        src: ValueId::new(2),
    });
    block.add_instruction(MirInstruction::VariantTag {
        dst: ValueId::new(3),
        value: ValueId::new(4),
        enum_name: "Result".to_string(),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    push_metadata(
        &mut function,
        1,
        3,
        "Result",
        "Ok",
        ValueId::new(2),
        ValueId::new(3),
        SumLocalAggregateLayout::TagI64Payload,
    );

    refresh_function_sum_variant_tag_seed_route(&mut function);

    let route = function
        .metadata
        .sum_variant_tag_seed_route
        .expect("sum variant tag route");
    assert_eq!(route.kind, SumVariantTagSeedKind::CopyLocalI64);
    assert_eq!(route.copy_value, Some(ValueId::new(4)));
    assert_eq!(route.tag_source_value, ValueId::new(4));
}

#[test]
fn sum_variant_tag_seed_rejects_missing_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(1),
        enum_name: "ResultUnit".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: None,
        payload_type: None,
    });
    block.add_instruction(MirInstruction::VariantTag {
        dst: ValueId::new(2),
        value: ValueId::new(1),
        enum_name: "ResultUnit".to_string(),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(2)),
    });

    refresh_function_sum_variant_tag_seed_route(&mut function);

    assert!(function.metadata.sum_variant_tag_seed_route.is_none());
}
