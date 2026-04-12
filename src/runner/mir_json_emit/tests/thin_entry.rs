use super::super::build_mir_json_root;
use super::make_function;
use crate::mir::{BasicBlockId, MirModule};

#[test]
fn build_mir_json_root_emits_thin_entry_candidates() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .thin_entry_candidates
        .push(crate::mir::ThinEntryCandidate {
            block: BasicBlockId::new(0),
            instruction_index: 2,
            value: Some(crate::mir::ValueId::new(7)),
            surface: crate::mir::ThinEntrySurface::VariantMake,
            subject: "Option::Some".to_string(),
            preferred_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
            current_carrier: crate::mir::ThinEntryCurrentCarrier::CompatBox,
            value_class: crate::mir::ThinEntryValueClass::AggLocal,
            reason: "variant.make stays aggregate-first".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let candidates = root["functions"][0]["metadata"]["thin_entry_candidates"]
        .as_array()
        .expect("thin_entry_candidates array");

    assert_eq!(candidates.len(), 1);
    assert_eq!(candidates[0]["surface"], "variant_make");
    assert_eq!(candidates[0]["subject"], "Option::Some");
    assert_eq!(candidates[0]["preferred_entry"], "thin_internal_entry");
    assert_eq!(candidates[0]["current_carrier"], "compat_box");
    assert_eq!(candidates[0]["value_class"], "agg_local");
    assert_eq!(candidates[0]["value"], 7);
}

#[test]
fn build_mir_json_root_emits_thin_entry_selections() {
    let mut module = MirModule::new("test".to_string());
    let mut function = make_function("main", true);
    function
        .metadata
        .thin_entry_selections
        .push(crate::mir::ThinEntrySelection {
            block: BasicBlockId::new(0),
            instruction_index: 3,
            value: Some(crate::mir::ValueId::new(8)),
            surface: crate::mir::ThinEntrySurface::UserBoxFieldGet,
            subject: "Point.x".to_string(),
            manifest_row: "user_box_field_get.inline_scalar",
            selected_entry: crate::mir::ThinEntryPreferredEntry::ThinInternalEntry,
            state: crate::mir::ThinEntrySelectionState::AlreadySatisfied,
            current_carrier: crate::mir::ThinEntryCurrentCarrier::BackendTyped,
            value_class: crate::mir::ThinEntryValueClass::InlineI64,
            reason: "typed field read stays on thin internal scalar lane".to_string(),
        });
    module.functions.insert("main".to_string(), function);

    let root = build_mir_json_root(&module).expect("mir json root");
    let selections = root["functions"][0]["metadata"]["thin_entry_selections"]
        .as_array()
        .expect("thin_entry_selections array");

    assert_eq!(selections.len(), 1);
    assert_eq!(
        selections[0]["manifest_row"],
        "user_box_field_get.inline_scalar"
    );
    assert_eq!(selections[0]["selected_entry"], "thin_internal_entry");
    assert_eq!(selections[0]["state"], "already_satisfied");
    assert_eq!(selections[0]["value"], 8);
}
