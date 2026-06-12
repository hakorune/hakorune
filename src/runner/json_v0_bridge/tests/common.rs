use crate::mir::sum_placement::SumPlacementState;
use crate::mir::sum_placement_layout::SumLocalAggregateLayout;
use crate::mir::sum_placement_selection::SumPlacementPath;
use crate::mir::thin_entry::{
    ThinEntryCurrentCarrier, ThinEntryPreferredEntry, ThinEntrySurface, ThinEntryValueClass,
};
use crate::mir::thin_entry_selection::ThinEntrySelectionState;
use crate::mir::{MirInstruction, MirModule};
use serde_json::json;
use std::sync::{Mutex, OnceLock};

pub(super) fn env_guard() -> &'static Mutex<()> {
    static GUARD: OnceLock<Mutex<()>> = OnceLock::new();
    GUARD.get_or_init(|| Mutex::new(()))
}

pub(super) fn option_enum_decls() -> serde_json::Value {
    json!([
        {
            "name": "Option",
            "type_parameters": [],
            "variants": [
                { "name": "None", "payload_type": null },
                { "name": "Some", "payload_type": "Integer" }
            ]
        }
    ])
}

pub(super) fn option_some_ctor(value: i64) -> serde_json::Value {
    json!({
        "type": "EnumCtor",
        "enum": "Option",
        "variant": "Some",
        "payload_type": "Integer",
        "args": [{ "type": "Int", "value": value }]
    })
}

pub(super) fn option_some_null_ctor() -> serde_json::Value {
    json!({
        "type": "EnumCtor",
        "enum": "Option",
        "variant": "Some",
        "payload_type": "Integer",
        "args": [{ "type": "Null" }]
    })
}

pub(super) fn main_instructions(module: &MirModule) -> Vec<&MirInstruction> {
    let func = module.get_function("main").expect("main exists");
    let mut block_ids: Vec<_> = func.blocks.keys().copied().collect();
    block_ids.sort();
    let mut out = Vec::new();
    for block_id in block_ids {
        let block = func
            .blocks
            .get(&block_id)
            .unwrap_or_else(|| panic!("block {:?} exists", block_id));
        out.extend(block.instructions.iter());
        if let Some(term) = block.terminator.as_ref() {
            out.push(term);
        }
    }
    out
}

pub(super) fn assert_sum_lane_candidates(module: &MirModule) {
    let func = module.get_function("main").expect("main exists");
    assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
        candidate.surface == ThinEntrySurface::VariantMake
            && candidate.subject == "Option::Some"
            && candidate.preferred_entry == ThinEntryPreferredEntry::ThinInternalEntry
            && candidate.current_carrier == ThinEntryCurrentCarrier::CompatBox
    }));
    assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
        candidate.surface == ThinEntrySurface::VariantTag
            && candidate.subject == "Option"
            && candidate.value_class == ThinEntryValueClass::InlineI64
    }));
    assert!(func.metadata.thin_entry_candidates.iter().any(|candidate| {
        candidate.surface == ThinEntrySurface::VariantProject
            && candidate.subject == "Option::Some"
            && candidate.value_class == ThinEntryValueClass::InlineI64
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantMake
            && selection.subject == "Option::Some"
            && selection.manifest_row == "variant_make.aggregate_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
            && selection.state == ThinEntrySelectionState::Candidate
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantTag
            && selection.subject == "Option"
            && selection.manifest_row == "variant_tag.tag_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    }));
    assert!(func.metadata.thin_entry_selections.iter().any(|selection| {
        selection.surface == ThinEntrySurface::VariantProject
            && selection.subject == "Option::Some"
            && selection.manifest_row == "variant_project.payload_local"
            && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantMake
            && fact.subject == "Option::Some"
            && fact.state == SumPlacementState::LocalAggregateCandidate
            && fact.tag_reads >= 1
            && fact.project_reads >= 1
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantTag
            && fact.subject == "Option"
            && fact.source_sum.is_some()
            && fact.state == SumPlacementState::LocalAggregateCandidate
    }));
    assert!(func.metadata.sum_placement_facts.iter().any(|fact| {
        fact.surface == ThinEntrySurface::VariantProject
            && fact.subject == "Option::Some"
            && fact.source_sum.is_some()
            && fact.state == SumPlacementState::LocalAggregateCandidate
    }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantMake
                && selection.subject == "Option::Some"
                && selection.manifest_row == "variant_make.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
        }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantTag
                && selection.subject == "Option"
                && selection.manifest_row == "variant_tag.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
                && selection.source_sum.is_some()
        }));
    assert!(func
        .metadata
        .sum_placement_selections
        .iter()
        .any(|selection| {
            selection.surface == ThinEntrySurface::VariantProject
                && selection.subject == "Option::Some"
                && selection.manifest_row == "variant_project.local_aggregate"
                && selection.selected_path == SumPlacementPath::LocalAggregate
                && selection.source_sum.is_some()
        }));
    assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
        layout.surface == ThinEntrySurface::VariantMake
            && layout.subject == "Option::Some"
            && layout.layout == SumLocalAggregateLayout::TagI64Payload
    }));
    assert!(func.metadata.sum_placement_layouts.iter().any(|layout| {
        layout.surface == ThinEntrySurface::VariantProject
            && layout.subject == "Option::Some"
            && layout.layout == SumLocalAggregateLayout::TagI64Payload
            && layout.source_sum.is_some()
    }));
}
