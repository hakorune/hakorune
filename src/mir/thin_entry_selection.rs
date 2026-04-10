/*!
 * Thin-entry selection pilot.
 *
 * This module consumes thin-entry inventory and binds the first manifest-driven
 * public-vs-thin selection decisions without changing canonical MIR or runtime
 * behavior. It is inspection-only metadata for the current phase-163x lane.
 */

use super::thin_entry::{
    ThinEntryCandidate, ThinEntryCurrentCarrier, ThinEntryPreferredEntry, ThinEntrySurface,
    ThinEntryValueClass,
};
use super::{BasicBlockId, MirFunction, MirModule, ValueId};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ThinEntrySelectionState {
    Candidate,
    AlreadySatisfied,
}

impl std::fmt::Display for ThinEntrySelectionState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Candidate => f.write_str("candidate"),
            Self::AlreadySatisfied => f.write_str("already_satisfied"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThinEntrySelection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub value: Option<ValueId>,
    pub surface: ThinEntrySurface,
    pub subject: String,
    pub manifest_row: &'static str,
    pub selected_entry: ThinEntryPreferredEntry,
    pub state: ThinEntrySelectionState,
    pub current_carrier: ThinEntryCurrentCarrier,
    pub value_class: ThinEntryValueClass,
    pub reason: String,
}

impl ThinEntrySelection {
    pub fn summary(&self) -> String {
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        format!(
            "bb{}#{} {} {} row={} selected={} [{}] current={} value_class={}{} reason={}",
            self.block.as_u32(),
            self.instruction_index,
            self.surface,
            self.subject,
            self.manifest_row,
            self.selected_entry,
            self.state,
            self.current_carrier,
            self.value_class,
            value_suffix,
            self.reason
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThinEntryManifestValueClass {
    Any,
    InlineScalar,
}

impl ThinEntryManifestValueClass {
    fn matches(self, value_class: ThinEntryValueClass) -> bool {
        match self {
            Self::Any => true,
            Self::InlineScalar => matches!(
                value_class,
                ThinEntryValueClass::InlineI64
                    | ThinEntryValueClass::InlineBool
                    | ThinEntryValueClass::InlineF64
            ),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct ThinEntryManifestRow {
    row: &'static str,
    surface: ThinEntrySurface,
    value_class: ThinEntryManifestValueClass,
    selected_entry: ThinEntryPreferredEntry,
    reason: &'static str,
}

const MANIFEST_ROWS: &[ThinEntryManifestRow] = &[
    ThinEntryManifestRow {
        row: "user_box_field_get.inline_scalar",
        surface: ThinEntrySurface::UserBoxFieldGet,
        value_class: ThinEntryManifestValueClass::InlineScalar,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "known primitive field reads stay on a thin internal scalar lane below canonical field.get",
    },
    ThinEntryManifestRow {
        row: "user_box_field_get.public_default",
        surface: ThinEntrySurface::UserBoxFieldGet,
        value_class: ThinEntryManifestValueClass::Any,
        selected_entry: ThinEntryPreferredEntry::PublicEntry,
        reason:
            "non-scalar or not-yet-proved field reads keep the stable public entry in this pilot",
    },
    ThinEntryManifestRow {
        row: "user_box_field_set.inline_scalar",
        surface: ThinEntrySurface::UserBoxFieldSet,
        value_class: ThinEntryManifestValueClass::InlineScalar,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "known primitive field writes stay on a thin internal scalar lane below canonical field.set",
    },
    ThinEntryManifestRow {
        row: "user_box_field_set.public_default",
        surface: ThinEntrySurface::UserBoxFieldSet,
        value_class: ThinEntryManifestValueClass::Any,
        selected_entry: ThinEntryPreferredEntry::PublicEntry,
        reason:
            "non-scalar or not-yet-proved field writes keep the stable public entry in this pilot",
    },
    ThinEntryManifestRow {
        row: "user_box_method.known_receiver",
        surface: ThinEntrySurface::UserBoxMethod,
        value_class: ThinEntryManifestValueClass::Any,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "known user-box methods stay eligible for a thin monomorphic internal entry beneath canonical Call",
    },
    ThinEntryManifestRow {
        row: "variant_make.aggregate_local",
        surface: ThinEntrySurface::VariantMake,
        value_class: ThinEntryManifestValueClass::Any,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "variant.make stays aggregate-first in the manifest plan and keeps compat boxing as fallback only",
    },
    ThinEntryManifestRow {
        row: "variant_tag.tag_local",
        surface: ThinEntrySurface::VariantTag,
        value_class: ThinEntryManifestValueClass::InlineScalar,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "variant.tag keeps discriminant reads on the thin local route while compat carriers remain runtime fallback",
    },
    ThinEntryManifestRow {
        row: "variant_project.payload_local",
        surface: ThinEntrySurface::VariantProject,
        value_class: ThinEntryManifestValueClass::Any,
        selected_entry: ThinEntryPreferredEntry::ThinInternalEntry,
        reason:
            "variant.project keeps payload projection on the thin local route while compat carriers remain runtime fallback",
    },
];

pub fn refresh_module_thin_entry_selections(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_thin_entry_selections(function);
    }
}

pub fn refresh_function_thin_entry_selections(function: &mut MirFunction) {
    let candidates = function.metadata.thin_entry_candidates.clone();
    function.metadata.thin_entry_selections.clear();

    for candidate in &candidates {
        let Some(row) = manifest_row(candidate) else {
            continue;
        };
        function
            .metadata
            .thin_entry_selections
            .push(bind_selection(candidate, row));
    }
}

fn manifest_row(candidate: &ThinEntryCandidate) -> Option<&'static ThinEntryManifestRow> {
    MANIFEST_ROWS.iter().find(|row| {
        row.surface == candidate.surface && row.value_class.matches(candidate.value_class)
    })
}

fn bind_selection(
    candidate: &ThinEntryCandidate,
    row: &ThinEntryManifestRow,
) -> ThinEntrySelection {
    ThinEntrySelection {
        block: candidate.block,
        instruction_index: candidate.instruction_index,
        value: candidate.value,
        surface: candidate.surface,
        subject: candidate.subject.clone(),
        manifest_row: row.row,
        selected_entry: row.selected_entry,
        state: selection_state(row.selected_entry, candidate.current_carrier),
        current_carrier: candidate.current_carrier,
        value_class: candidate.value_class,
        reason: row.reason.to_string(),
    }
}

fn selection_state(
    selected_entry: ThinEntryPreferredEntry,
    current_carrier: ThinEntryCurrentCarrier,
) -> ThinEntrySelectionState {
    match (selected_entry, current_carrier) {
        (ThinEntryPreferredEntry::PublicEntry, ThinEntryCurrentCarrier::PublicRuntime) => {
            ThinEntrySelectionState::AlreadySatisfied
        }
        (ThinEntryPreferredEntry::ThinInternalEntry, ThinEntryCurrentCarrier::BackendTyped) => {
            ThinEntrySelectionState::AlreadySatisfied
        }
        _ => ThinEntrySelectionState::Candidate,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::{BasicBlockId, EffectMask, FunctionSignature, MirType, ValueId};

    #[test]
    fn refresh_function_binds_manifest_rows_for_selection_pilot() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function.metadata.thin_entry_candidates = vec![
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                value: Some(ValueId::new(3)),
                surface: ThinEntrySurface::UserBoxFieldGet,
                subject: "Point.x".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::BackendTyped,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "inventory".to_string(),
            },
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                value: None,
                surface: ThinEntrySurface::UserBoxFieldSet,
                subject: "Point.name".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::PublicRuntime,
                value_class: ThinEntryValueClass::BorrowedText,
                reason: "inventory".to_string(),
            },
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 2,
                value: Some(ValueId::new(4)),
                surface: ThinEntrySurface::UserBoxMethod,
                subject: "Point.move".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::PublicRuntime,
                value_class: ThinEntryValueClass::Unknown,
                reason: "inventory".to_string(),
            },
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 3,
                value: Some(ValueId::new(5)),
                surface: ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::AggLocal,
                reason: "inventory".to_string(),
            },
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 4,
                value: Some(ValueId::new(7)),
                surface: ThinEntrySurface::VariantTag,
                subject: "Option".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "inventory".to_string(),
            },
            ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 5,
                value: Some(ValueId::new(6)),
                surface: ThinEntrySurface::VariantProject,
                subject: "Option::Some".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::CompatBox,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "inventory".to_string(),
            },
        ];

        refresh_function_thin_entry_selections(&mut function);

        let selections = &function.metadata.thin_entry_selections;
        assert_eq!(selections.len(), 6);
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "user_box_field_get.inline_scalar"
                && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
                && selection.state == ThinEntrySelectionState::AlreadySatisfied
                && selection.subject == "Point.x"
        }));
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "user_box_field_set.public_default"
                && selection.selected_entry == ThinEntryPreferredEntry::PublicEntry
                && selection.state == ThinEntrySelectionState::AlreadySatisfied
                && selection.subject == "Point.name"
        }));
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "user_box_method.known_receiver"
                && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
                && selection.state == ThinEntrySelectionState::Candidate
                && selection.subject == "Point.move"
        }));
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "variant_make.aggregate_local"
                && selection.state == ThinEntrySelectionState::Candidate
                && selection.subject == "Option::Some"
        }));
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "variant_tag.tag_local"
                && selection.state == ThinEntrySelectionState::Candidate
                && selection.subject == "Option"
        }));
        assert!(selections.iter().any(|selection| {
            selection.manifest_row == "variant_project.payload_local"
                && selection.state == ThinEntrySelectionState::Candidate
                && selection.subject == "Option::Some"
        }));
    }

    #[test]
    fn refresh_function_clears_stale_selection_rows() {
        let signature = FunctionSignature {
            name: "test_func".to_string(),
            params: vec![],
            return_type: MirType::Void,
            effects: EffectMask::PURE,
        };
        let mut function = MirFunction::new(signature, BasicBlockId::new(0));
        function
            .metadata
            .thin_entry_candidates
            .push(ThinEntryCandidate {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                value: Some(ValueId::new(2)),
                surface: ThinEntrySurface::UserBoxFieldGet,
                subject: "Point.x".to_string(),
                preferred_entry: ThinEntryPreferredEntry::ThinInternalEntry,
                current_carrier: ThinEntryCurrentCarrier::BackendTyped,
                value_class: ThinEntryValueClass::InlineI64,
                reason: "inventory".to_string(),
            });

        refresh_function_thin_entry_selections(&mut function);
        assert_eq!(function.metadata.thin_entry_selections.len(), 1);

        function.metadata.thin_entry_candidates.clear();
        refresh_function_thin_entry_selections(&mut function);
        assert!(function.metadata.thin_entry_selections.is_empty());
    }
}
