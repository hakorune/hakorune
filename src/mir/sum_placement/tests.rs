use super::*;
use crate::mir::thin_entry::ThinEntryDemand;
use crate::mir::{EffectMask, FunctionSignature, MirType};

#[test]
fn refresh_function_marks_local_sum_routes_as_local_candidates() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![],
        return_type: MirType::Integer,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let sum_value = ValueId::new(1);
    let alias_value = ValueId::new(2);
    let tag_value = ValueId::new(3);
    let project_value = ValueId::new(4);

    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::VariantMake {
            dst: sum_value,
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(10)),
            payload_type: Some(MirType::Integer),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::Copy {
            dst: alias_value,
            src: sum_value,
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::VariantTag {
            dst: tag_value,
            value: alias_value,
            enum_name: "Option".to_string(),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::VariantProject {
            dst: project_value,
            value: alias_value,
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload_type: Some(MirType::Integer),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::Return {
            value: Some(project_value),
        });

    function.metadata.thin_entry_selections = vec![
        ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 0,
            value: Some(sum_value),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            manifest_row: "variant_make.aggregate_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: super::super::thin_entry::ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::AggLocal,
            demand: ThinEntryDemand::LocalAggregate,
            reason: "inventory".to_string(),
        },
        ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            value: Some(tag_value),
            surface: ThinEntrySurface::VariantTag,
            subject: "Option".to_string(),
            manifest_row: "variant_tag.tag_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: super::super::thin_entry::ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::InlineI64,
            demand: ThinEntryDemand::InlineScalar,
            reason: "inventory".to_string(),
        },
        ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 3,
            value: Some(project_value),
            surface: ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            manifest_row: "variant_project.payload_local",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::Candidate,
            current_carrier: super::super::thin_entry::ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::InlineI64,
            demand: ThinEntryDemand::InlineScalar,
            reason: "inventory".to_string(),
        },
    ];

    refresh_function_sum_placement_facts(&mut function);

    assert!(function.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantMake
            && fact.subject == "Option::Some"
            && fact.state == SumPlacementState::LocalAggregateCandidate
            && fact.tag_reads == 1
            && fact.project_reads == 1
            && fact.barriers.is_empty()
    }));
    assert!(function.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantTag
            && fact.subject == "Option"
            && fact.source_sum == Some(sum_value)
            && fact.state == SumPlacementState::LocalAggregateCandidate
            && fact.value_class == ThinEntryValueClass::InlineI64
    }));
    assert!(function.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantProject
            && fact.subject == "Option::Some"
            && fact.source_sum == Some(sum_value)
            && fact.state == SumPlacementState::LocalAggregateCandidate
    }));
}

#[test]
fn refresh_function_marks_returned_sum_as_needing_objectization() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![],
        return_type: MirType::Box("Option".to_string()),
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    let sum_value = ValueId::new(1);

    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::VariantMake {
            dst: sum_value,
            enum_name: "Option".to_string(),
            variant: "Some".to_string(),
            tag: 1,
            payload: Some(ValueId::new(10)),
            payload_type: Some(MirType::Integer),
        });
    function
        .get_block_mut(BasicBlockId::new(0))
        .expect("entry block")
        .add_instruction(MirInstruction::Return {
            value: Some(sum_value),
        });

    function.metadata.thin_entry_selections = vec![ThinEntrySelection {
        block: BasicBlockId::new(0),
        instruction_index: 0,
        value: Some(sum_value),
        surface: ThinEntrySurface::VariantMake,
        subject: "Option::Some".to_string(),
        manifest_row: "variant_make.aggregate_local",
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        state: ThinEntrySelectionState::Candidate,
        current_carrier: super::super::thin_entry::ThinEntryCurrentCarrier::CompatBox,
        value_class: ThinEntryValueClass::AggLocal,
        demand: ThinEntryDemand::LocalAggregate,
        reason: "inventory".to_string(),
    }];

    refresh_function_sum_placement_facts(&mut function);

    assert!(function.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantMake
            && fact.subject == "Option::Some"
            && fact.state == SumPlacementState::NeedsObjectization
            && fact.barriers == vec![SumObjectizationBarrier::Return]
    }));
}
