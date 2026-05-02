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
    project_idx: usize,
    enum_name: &str,
    variant: &str,
    sum_value: ValueId,
    project_value: ValueId,
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
            instruction_index: project_idx,
            value: Some(project_value),
            surface: ThinEntrySurface::VariantProject,
            subject: subject.clone(),
            manifest_row: "variant_project.payload_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::AggLocal,
            demand: ThinEntryDemand::LocalAggregate,
            reason: "test project".to_string(),
        },
    ];
    function.metadata.sum_placement_facts = vec![SumPlacementFact {
        block: BasicBlockId::new(0),
        instruction_index: make_idx,
        value: Some(sum_value),
        surface: ThinEntrySurface::VariantMake,
        subject: subject.clone(),
        source_sum: None,
        value_class: ThinEntryValueClass::AggLocal,
        state: SumPlacementState::LocalAggregateCandidate,
        tag_reads: 0,
        project_reads: 1,
        barriers: Vec::<SumObjectizationBarrier>::new(),
        reason: "test make fact".to_string(),
    }];
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
            instruction_index: project_idx,
            value: Some(project_value),
            surface: ThinEntrySurface::VariantProject,
            subject: subject.clone(),
            source_sum: Some(sum_value),
            manifest_row: "variant_project.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "test project selection".to_string(),
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
fn sum_variant_project_seed_detects_i64_route_from_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(73),
    });
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "ResultInt".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: Some(ValueId::new(1)),
        payload_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::VariantProject {
        dst: ValueId::new(3),
        value: ValueId::new(2),
        enum_name: "ResultInt".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload_type: Some(MirType::Integer),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });
    push_metadata(
        &mut function,
        1,
        2,
        "ResultInt",
        "Ok",
        ValueId::new(2),
        ValueId::new(3),
        SumLocalAggregateLayout::TagI64Payload,
    );

    refresh_function_sum_variant_project_seed_route(&mut function);

    let route = function
        .metadata
        .sum_variant_project_seed_route
        .expect("sum variant project route");
    assert_eq!(route.kind, SumVariantProjectSeedKind::LocalI64);
    assert_eq!(route.subject, "ResultInt::Ok");
    assert_eq!(route.payload, SumVariantProjectSeedPayload::I64(73));
    assert_eq!(route.copy_value, None);
}

#[test]
fn sum_variant_project_seed_detects_copy_handle_route_from_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::String("hako".to_string()),
    });
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "ResultHandle".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: Some(ValueId::new(1)),
        payload_type: Some(MirType::String),
    });
    block.add_instruction(MirInstruction::Copy {
        dst: ValueId::new(3),
        src: ValueId::new(2),
    });
    block.add_instruction(MirInstruction::VariantProject {
        dst: ValueId::new(4),
        value: ValueId::new(3),
        enum_name: "ResultHandle".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload_type: Some(MirType::String),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(4)),
    });
    push_metadata(
        &mut function,
        1,
        3,
        "ResultHandle",
        "Ok",
        ValueId::new(2),
        ValueId::new(4),
        SumLocalAggregateLayout::TagHandlePayload,
    );

    refresh_function_sum_variant_project_seed_route(&mut function);

    let route = function
        .metadata
        .sum_variant_project_seed_route
        .expect("sum variant project route");
    assert_eq!(route.kind, SumVariantProjectSeedKind::CopyLocalHandle);
    assert_eq!(route.copy_value, Some(ValueId::new(3)));
    assert_eq!(
        route.payload,
        SumVariantProjectSeedPayload::String("hako".to_string())
    );
}

#[test]
fn sum_variant_project_seed_rejects_missing_metadata() {
    let mut function = make_function();
    let block = function.get_block_mut(BasicBlockId::new(0)).unwrap();
    block.add_instruction(MirInstruction::Const {
        dst: ValueId::new(1),
        value: ConstValue::Integer(73),
    });
    block.add_instruction(MirInstruction::VariantMake {
        dst: ValueId::new(2),
        enum_name: "ResultInt".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload: Some(ValueId::new(1)),
        payload_type: Some(MirType::Integer),
    });
    block.add_instruction(MirInstruction::VariantProject {
        dst: ValueId::new(3),
        value: ValueId::new(2),
        enum_name: "ResultInt".to_string(),
        variant: "Ok".to_string(),
        tag: 0,
        payload_type: Some(MirType::Integer),
    });
    block.set_terminator(MirInstruction::Return {
        value: Some(ValueId::new(3)),
    });

    refresh_function_sum_variant_project_seed_route(&mut function);

    assert!(function.metadata.sum_variant_project_seed_route.is_none());
}
