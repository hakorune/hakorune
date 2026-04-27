/*!
 * Sum placement selection pilot.
 *
 * This consumes `sum_placement_facts` and records whether each current sum site
 * is selected for the local aggregate route or stays on compat/runtime fallback.
 * It remains inspection-only metadata and should later fold into a generic
 * placement/effect pass instead of becoming a permanent sum-only subsystem.
 */

use super::{
    sum_placement::{SumPlacementFact, SumPlacementState},
    thin_entry::ThinEntrySurface,
    BasicBlockId, MirFunction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumPlacementPath {
    LocalAggregate,
    CompatRuntimeBox,
}

impl std::fmt::Display for SumPlacementPath {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalAggregate => f.write_str("local_aggregate"),
            Self::CompatRuntimeBox => f.write_str("compat_runtime_box"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumPlacementSelection {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub value: Option<ValueId>,
    pub surface: ThinEntrySurface,
    pub subject: String,
    pub source_sum: Option<ValueId>,
    pub manifest_row: &'static str,
    pub selected_path: SumPlacementPath,
    pub reason: String,
}

impl SumPlacementSelection {
    pub fn summary(&self) -> String {
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        let source_suffix = self
            .source_sum
            .map(|value| format!(" source_sum=%{}", value.as_u32()))
            .unwrap_or_default();
        format!(
            "bb{}#{} {} {} row={} selected={}{}{} reason={}",
            self.block.as_u32(),
            self.instruction_index,
            self.surface,
            self.subject,
            self.manifest_row,
            self.selected_path,
            value_suffix,
            source_suffix,
            self.reason
        )
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct SumPlacementManifestRow {
    row: &'static str,
    surface: ThinEntrySurface,
    selected_path: SumPlacementPath,
    reason: &'static str,
}

const MANIFEST_ROWS: &[SumPlacementManifestRow] = &[
    SumPlacementManifestRow {
        row: "variant_make.local_aggregate",
        surface: ThinEntrySurface::VariantMake,
        selected_path: SumPlacementPath::LocalAggregate,
        reason:
            "variant.make stays on the selected local aggregate route in this proving slice; later passes should generalize this through placement/effect selection",
    },
    SumPlacementManifestRow {
        row: "variant_make.compat_fallback",
        surface: ThinEntrySurface::VariantMake,
        selected_path: SumPlacementPath::CompatRuntimeBox,
        reason:
            "variant.make still needs compat/runtime outer boxing in the current proving slice because the variant crosses an objectization barrier",
    },
    SumPlacementManifestRow {
        row: "variant_tag.local_aggregate",
        surface: ThinEntrySurface::VariantTag,
        selected_path: SumPlacementPath::LocalAggregate,
        reason:
            "variant.tag reads from a selected local aggregate variant route in this proving slice; later passes should fold this into a generic placement/effect selection",
    },
    SumPlacementManifestRow {
        row: "variant_tag.compat_fallback",
        surface: ThinEntrySurface::VariantTag,
        selected_path: SumPlacementPath::CompatRuntimeBox,
        reason:
            "variant.tag still reads through the compat/runtime outer box because the current variant source crosses an objectization barrier",
    },
    SumPlacementManifestRow {
        row: "variant_project.local_aggregate",
        surface: ThinEntrySurface::VariantProject,
        selected_path: SumPlacementPath::LocalAggregate,
        reason:
            "variant.project reads from a selected local aggregate variant route in this proving slice; later passes should fold this into a generic placement/effect selection",
    },
    SumPlacementManifestRow {
        row: "variant_project.compat_fallback",
        surface: ThinEntrySurface::VariantProject,
        selected_path: SumPlacementPath::CompatRuntimeBox,
        reason:
            "variant.project still reads through the compat/runtime outer box because the current variant source crosses an objectization barrier",
    },
];

pub fn refresh_module_sum_placement_selections(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_sum_placement_selections(function);
    }
}

pub fn refresh_function_sum_placement_selections(function: &mut MirFunction) {
    let facts = function.metadata.sum_placement_facts.clone();
    function.metadata.sum_placement_selections.clear();
    for fact in &facts {
        let Some(row) = manifest_row(fact) else {
            continue;
        };
        function
            .metadata
            .sum_placement_selections
            .push(bind_selection(fact, row));
    }
}

fn manifest_row(fact: &SumPlacementFact) -> Option<&'static SumPlacementManifestRow> {
    let selected_path = match fact.state {
        SumPlacementState::LocalAggregateCandidate => SumPlacementPath::LocalAggregate,
        SumPlacementState::NeedsObjectization => SumPlacementPath::CompatRuntimeBox,
    };
    MANIFEST_ROWS
        .iter()
        .find(|row| row.surface == fact.surface && row.selected_path == selected_path)
}

fn bind_selection(fact: &SumPlacementFact, row: &SumPlacementManifestRow) -> SumPlacementSelection {
    SumPlacementSelection {
        block: fact.block,
        instruction_index: fact.instruction_index,
        value: fact.value,
        surface: fact.surface,
        subject: fact.subject.clone(),
        source_sum: fact.source_sum,
        manifest_row: row.row,
        selected_path: row.selected_path,
        reason: row.reason.to_string(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::thin_entry::ThinEntryValueClass;
    use crate::mir::{BasicBlockId, SumObjectizationBarrier, ValueId};

    #[test]
    fn refresh_function_binds_local_aggregate_and_compat_sum_selections() {
        let mut function = MirFunction::new(
            crate::mir::FunctionSignature {
                name: "test_func".to_string(),
                params: vec![],
                return_type: crate::mir::MirType::Void,
                effects: crate::mir::EffectMask::PURE,
            },
            BasicBlockId::new(0),
        );
        function.metadata.sum_placement_facts = vec![
            SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: 0,
                value: Some(ValueId::new(1)),
                surface: ThinEntrySurface::VariantMake,
                subject: "Option::Some".to_string(),
                source_sum: None,
                value_class: ThinEntryValueClass::AggLocal,
                state: SumPlacementState::LocalAggregateCandidate,
                tag_reads: 1,
                project_reads: 1,
                barriers: Vec::new(),
                reason: "local".to_string(),
            },
            SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: 1,
                value: Some(ValueId::new(2)),
                surface: ThinEntrySurface::VariantTag,
                subject: "Option".to_string(),
                source_sum: Some(ValueId::new(1)),
                value_class: ThinEntryValueClass::InlineI64,
                state: SumPlacementState::LocalAggregateCandidate,
                tag_reads: 1,
                project_reads: 1,
                barriers: Vec::new(),
                reason: "local".to_string(),
            },
            SumPlacementFact {
                block: BasicBlockId::new(0),
                instruction_index: 2,
                value: Some(ValueId::new(3)),
                surface: ThinEntrySurface::VariantProject,
                subject: "Option::Some".to_string(),
                source_sum: Some(ValueId::new(1)),
                value_class: ThinEntryValueClass::InlineI64,
                state: SumPlacementState::NeedsObjectization,
                tag_reads: 0,
                project_reads: 1,
                barriers: vec![SumObjectizationBarrier::Return],
                reason: "return".to_string(),
            },
        ];

        refresh_function_sum_placement_selections(&mut function);

        assert_eq!(function.metadata.sum_placement_selections.len(), 3);
        assert!(function
            .metadata
            .sum_placement_selections
            .iter()
            .any(|selection| {
                selection.surface == ThinEntrySurface::VariantMake
                    && selection.manifest_row == "variant_make.local_aggregate"
                    && selection.selected_path == SumPlacementPath::LocalAggregate
            }));
        assert!(function
            .metadata
            .sum_placement_selections
            .iter()
            .any(|selection| {
                selection.surface == ThinEntrySurface::VariantTag
                    && selection.manifest_row == "variant_tag.local_aggregate"
                    && selection.selected_path == SumPlacementPath::LocalAggregate
                    && selection.source_sum == Some(ValueId::new(1))
            }));
        assert!(function
            .metadata
            .sum_placement_selections
            .iter()
            .any(|selection| {
                selection.surface == ThinEntrySurface::VariantProject
                    && selection.manifest_row == "variant_project.compat_fallback"
                    && selection.selected_path == SumPlacementPath::CompatRuntimeBox
                    && selection.source_sum == Some(ValueId::new(1))
            }));
    }
}
