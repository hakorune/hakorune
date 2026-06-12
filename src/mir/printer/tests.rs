use super::*;
use crate::mir::storage_class::StorageClass;
use crate::mir::string_corridor::{StringCorridorCarrier, StringCorridorFact};
use crate::mir::string_corridor_placement::{
    StringCorridorCandidate, StringCorridorCandidateKind, StringCorridorCandidateState,
};
use crate::mir::string_corridor_relation::{
    StringCorridorRelation, StringCorridorRelationKind, StringCorridorWindowContract,
};
use crate::mir::sum_placement::{SumObjectizationBarrier, SumPlacementFact, SumPlacementState};
use crate::mir::sum_placement_layout::{SumLocalAggregateLayout, SumPlacementLayout};
use crate::mir::sum_placement_selection::{SumPlacementPath, SumPlacementSelection};
use crate::mir::thin_entry::{
    ThinEntryCandidate, ThinEntryCurrentCarrier, ThinEntryDemand, ThinEntryPreferredEntry,
    ThinEntrySurface, ThinEntryValueClass,
};
use crate::mir::thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState};
use crate::mir::{
    BasicBlockId, EffectMask, FunctionSignature, MirFunction, MirModule, MirType, ValueId,
};

#[test]
fn test_empty_module_printing() {
    let module = MirModule::new("test".to_string());
    let printer = MirPrinter::new();

    let output = printer.print_module(&module);

    assert!(output.contains("MIR Module: test"));
    assert!(!output.is_empty());
}

#[test]
fn test_function_printing() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };

    let function = MirFunction::new(signature, BasicBlockId::new(0));
    let printer = MirPrinter::new();

    let output = printer.print_function(&function);

    assert!(output.contains("define void @test_func(i64 %0)"));
    assert!(output.contains("bb0:"));
}

#[test]
fn test_verbose_printing() {
    let module = MirModule::new("test".to_string());
    let printer = MirPrinter::verbose();

    let output = printer.print_module(&module);

    assert!(output.contains("Module Statistics"));
}

#[test]
fn test_verbose_printing_shows_string_corridor_facts() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function.metadata.string_corridor_facts.insert(
        ValueId::new(1),
        StringCorridorFact::str_len(StringCorridorCarrier::CanonicalIntrinsic),
    );
    function.metadata.string_corridor_candidates.insert(
        ValueId::new(1),
        vec![StringCorridorCandidate {
            kind: StringCorridorCandidateKind::DirectKernelEntry,
            state: StringCorridorCandidateState::Candidate,
            reason: "scalar string consumer can bypass ABI facade on the AOT-internal path",
            plan: None,
            publication_boundary: None,
        }],
    );
    function.metadata.string_corridor_relations.insert(
        ValueId::new(2),
        vec![StringCorridorRelation {
            kind: StringCorridorRelationKind::PhiCarryBase,
            base_value: ValueId::new(1),
            window_contract: StringCorridorWindowContract::StopAtMerge,
            witness_value: None,
            reason: "merged phi continuity keeps the current string corridor lane but stops the proof-bearing plan window at the merge",
        }],
    );
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("String Corridor Facts"));
    assert!(output.contains("String Corridor Relations"));
    assert!(output.contains("String Corridor Candidates"));
    assert!(output.contains("%1: str.len"));
    assert!(output.contains("%2: phi_carry_base"));
    assert!(output.contains("window=stop_at_merge"));
    assert!(output.contains("direct_kernel_entry"));
}

#[test]
fn test_verbose_printing_shows_storage_classes() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .value_storage_classes
        .insert(ValueId::new(1), StorageClass::InlineI64);
    function
        .metadata
        .value_storage_classes
        .insert(ValueId::new(2), StorageClass::BorrowedText);
    function
        .metadata
        .value_storage_classes
        .insert(ValueId::new(3), StorageClass::InlineF64);
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Storage Classes"));
    assert!(output.contains("%1: inline_i64"));
    assert!(output.contains("%2: borrowed_text"));
    assert!(output.contains("%3: inline_f64"));
}

#[test]
fn test_verbose_printing_shows_thin_entry_candidates() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .thin_entry_candidates
        .push(ThinEntryCandidate {
            block: BasicBlockId::new(0),
            instruction_index: 1,
            value: Some(ValueId::new(3)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            current_carrier: ThinEntryCurrentCarrier::CompatBox,
            value_class: ThinEntryValueClass::AggLocal,
            demand: ThinEntryDemand::LocalAggregate,
            reason: "variant.make stays aggregate-first".to_string(),
        });
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Thin Entry Candidates"));
    assert!(output.contains("variant_make Option::Some"));
    assert!(output.contains("thin_internal_entry"));
}

#[test]
fn test_verbose_printing_shows_thin_entry_selections() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .thin_entry_selections
        .push(ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            value: Some(ValueId::new(3)),
            surface: ThinEntrySurface::UserBoxFieldGet,
            subject: "Point.x".to_string(),
            manifest_row: "user_box_field_get.inline_scalar",
            selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
            state: ThinEntrySelectionState::AlreadySatisfied,
            current_carrier: ThinEntryCurrentCarrier::BackendTyped,
            value_class: ThinEntryValueClass::InlineI64,
            demand: ThinEntryDemand::InlineScalar,
            reason: "typed field reads stay on thin internal scalar lane".to_string(),
        });
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Thin Entry Selections"));
    assert!(output.contains("user_box_field_get.inline_scalar"));
    assert!(output.contains("[already_satisfied]"));
}

#[test]
fn test_verbose_printing_shows_sum_placement_facts() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .sum_placement_facts
        .push(SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            value: Some(ValueId::new(5)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            value_class: ThinEntryValueClass::AggLocal,
            state: SumPlacementState::LocalAggregateCandidate,
            tag_reads: 1,
            project_reads: 1,
            barriers: Vec::new(),
            reason: "variant value stays local to variant.tag/variant.project".to_string(),
        });
    function
        .metadata
        .sum_placement_facts
        .push(SumPlacementFact {
            block: BasicBlockId::new(0),
            instruction_index: 3,
            value: Some(ValueId::new(6)),
            surface: ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            source_sum: Some(ValueId::new(5)),
            value_class: ThinEntryValueClass::InlineI64,
            state: SumPlacementState::NeedsObjectization,
            tag_reads: 0,
            project_reads: 1,
            barriers: vec![SumObjectizationBarrier::Return],
            reason: "variant.project source still crosses return".to_string(),
        });
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Sum Placement Facts"));
    assert!(output.contains("local_agg_candidate"));
    assert!(output.contains("source_sum=%5"));
    assert!(output.contains("barriers=[return]"));
}

#[test]
fn test_verbose_printing_shows_sum_placement_selections() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .sum_placement_selections
        .push(SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: 4,
            value: Some(ValueId::new(7)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            manifest_row: "variant_make.local_aggregate",
            selected_path: SumPlacementPath::LocalAggregate,
            reason: "selected local aggregate sum route".to_string(),
        });
    function
        .metadata
        .sum_placement_selections
        .push(SumPlacementSelection {
            block: BasicBlockId::new(0),
            instruction_index: 5,
            value: Some(ValueId::new(8)),
            surface: ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            source_sum: Some(ValueId::new(7)),
            manifest_row: "variant_project.compat_fallback",
            selected_path: SumPlacementPath::CompatRuntimeBox,
            reason: "compat/runtime fallback remains".to_string(),
        });
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Sum Placement Selections"));
    assert!(output.contains("variant_make.local_aggregate"));
    assert!(output.contains("compat_runtime_box"));
    assert!(output.contains("source_sum=%7"));
}

#[test]
fn test_verbose_printing_shows_sum_placement_layouts() {
    let signature = FunctionSignature {
        name: "test_func".to_string(),
        params: vec![MirType::Integer],
        return_type: MirType::Void,
        effects: EffectMask::PURE,
    };
    let mut function = MirFunction::new(signature, BasicBlockId::new(0));
    function
        .metadata
        .sum_placement_layouts
        .push(SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: 6,
            value: Some(ValueId::new(9)),
            surface: ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            source_sum: None,
            layout: SumLocalAggregateLayout::TagI64Payload,
            reason: "selected local aggregate uses tag+i64 payload lane".to_string(),
        });
    function
        .metadata
        .sum_placement_layouts
        .push(SumPlacementLayout {
            block: BasicBlockId::new(0),
            instruction_index: 7,
            value: Some(ValueId::new(10)),
            surface: ThinEntrySurface::VariantProject,
            subject: "Option::Some".to_string(),
            source_sum: Some(ValueId::new(9)),
            layout: SumLocalAggregateLayout::TagHandlePayload,
            reason: "selected local aggregate uses handle payload lane".to_string(),
        });
    let printer = MirPrinter::verbose();

    let output = printer.print_function(&function);

    assert!(output.contains("Sum Placement Layouts"));
    assert!(output.contains("tag_i64_payload"));
    assert!(output.contains("tag_handle_payload"));
    assert!(output.contains("source_sum=%9"));
}
