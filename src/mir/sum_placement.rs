/*!
 * Enum-local placement/objectization facts.
 *
 * This is the enum proving slice for the later generic placement/effect pass.
 * For now it stays inspection-only metadata over canonical MIR so the current
 * phase-163x lane can prove where local enums do or do not need an outer runtime
 * `__NyVariant_*` object.
 */

use std::collections::{BTreeMap, BTreeSet, HashMap};

use super::{
    build_value_def_map, resolve_value_origin,
    thin_entry::{ThinEntryPreferredEntry, ThinEntrySurface, ThinEntryValueClass},
    thin_entry_selection::{ThinEntrySelection, ThinEntrySelectionState},
    BasicBlockId, MirFunction, MirInstruction, MirModule, ValueId,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SumPlacementState {
    LocalAggregateCandidate,
    NeedsObjectization,
}

impl std::fmt::Display for SumPlacementState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::LocalAggregateCandidate => f.write_str("local_agg_candidate"),
            Self::NeedsObjectization => f.write_str("needs_objectization"),
        }
    }
}

/// Objectization blockers for the current enum-local proving slice.
///
/// This is a barrier-cause vocabulary for sum/local-aggregate objectization,
/// not a generic lifecycle/outcome seam.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum SumObjectizationBarrier {
    Return,
    Call,
    StoreLike,
    PhiMerge,
    Capture,
    DebugObserve,
    UnknownUse,
}

impl std::fmt::Display for SumObjectizationBarrier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Return => f.write_str("return"),
            Self::Call => f.write_str("call"),
            Self::StoreLike => f.write_str("store_like"),
            Self::PhiMerge => f.write_str("phi_merge"),
            Self::Capture => f.write_str("capture"),
            Self::DebugObserve => f.write_str("debug_observe"),
            Self::UnknownUse => f.write_str("unknown_use"),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SumPlacementFact {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub value: Option<ValueId>,
    pub surface: ThinEntrySurface,
    pub subject: String,
    pub source_sum: Option<ValueId>,
    pub value_class: ThinEntryValueClass,
    pub state: SumPlacementState,
    pub tag_reads: usize,
    pub project_reads: usize,
    pub barriers: Vec<SumObjectizationBarrier>,
    pub reason: String,
}

impl SumPlacementFact {
    pub fn summary(&self) -> String {
        let value_suffix = self
            .value
            .map(|value| format!(" value=%{}", value.as_u32()))
            .unwrap_or_default();
        let source_suffix = self
            .source_sum
            .map(|value| format!(" source_sum=%{}", value.as_u32()))
            .unwrap_or_default();
        let barrier_text = if self.barriers.is_empty() {
            "-".to_string()
        } else {
            self.barriers
                .iter()
                .map(ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "bb{}#{} {} {} state={} value_class={}{}{} tag_reads={} project_reads={} barriers=[{}] reason={}",
            self.block.as_u32(),
            self.instruction_index,
            self.surface,
            self.subject,
            self.state,
            self.value_class,
            value_suffix,
            source_suffix,
            self.tag_reads,
            self.project_reads,
            barrier_text,
            self.reason
        )
    }
}

#[derive(Debug, Clone)]
struct SumMakeInfo {
    block: BasicBlockId,
    instruction_index: usize,
    dst: ValueId,
    subject: String,
    value_class: ThinEntryValueClass,
}

#[derive(Debug, Default, Clone)]
struct SumRootAnalysis {
    tag_reads: usize,
    project_reads: usize,
    barriers: BTreeSet<SumObjectizationBarrier>,
}

pub fn refresh_module_sum_placement_facts(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_sum_placement_facts(function);
    }
}

pub fn refresh_function_sum_placement_facts(function: &mut MirFunction) {
    let selections = function.metadata.thin_entry_selections.clone();
    let def_map = build_value_def_map(function);
    let variant_make_infos = collect_variant_make_infos(function, &selections);
    let root_analyses = analyze_sum_roots(function, &def_map, &variant_make_infos);
    let mut facts = collect_variant_make_facts(&variant_make_infos, &root_analyses);
    facts.extend(collect_variant_tag_facts(
        function,
        &selections,
        &def_map,
        &root_analyses,
    ));
    facts.extend(collect_variant_project_facts(
        function,
        &selections,
        &def_map,
        &root_analyses,
    ));
    facts.sort_by_key(|fact| (fact.block.as_u32(), fact.instruction_index));
    function.metadata.sum_placement_facts = facts;
}

fn collect_variant_make_infos(
    function: &MirFunction,
    selections: &[ThinEntrySelection],
) -> BTreeMap<ValueId, SumMakeInfo> {
    let selection_sites = build_sum_selection_site_map(selections, ThinEntrySurface::VariantMake);
    let mut infos = BTreeMap::new();

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::VariantMake { dst, .. } = inst else {
                continue;
            };
            let Some(selection) = selection_sites.get(&(block_id.as_u32(), instruction_index))
            else {
                continue;
            };
            infos.insert(
                *dst,
                SumMakeInfo {
                    block: block_id,
                    instruction_index,
                    dst: *dst,
                    subject: selection.subject.clone(),
                    value_class: selection.value_class,
                },
            );
        }
    }

    infos
}

fn analyze_sum_roots(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    variant_make_infos: &BTreeMap<ValueId, SumMakeInfo>,
) -> BTreeMap<ValueId, SumRootAnalysis> {
    let mut analyses: BTreeMap<ValueId, SumRootAnalysis> = variant_make_infos
        .keys()
        .copied()
        .map(|value| (value, SumRootAnalysis::default()))
        .collect();

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for inst in &block.instructions {
            observe_instruction(function, def_map, variant_make_infos, &mut analyses, inst);
        }
        if let Some(term) = &block.terminator {
            observe_instruction(function, def_map, variant_make_infos, &mut analyses, term);
        }
        if let Some(return_env) = &block.return_env {
            for value in return_env {
                let root = resolve_value_origin(function, def_map, *value);
                if let Some(analysis) = analyses.get_mut(&root) {
                    analysis.barriers.insert(SumObjectizationBarrier::Return);
                }
            }
        }
    }

    analyses
}

fn observe_instruction(
    function: &MirFunction,
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    variant_make_infos: &BTreeMap<ValueId, SumMakeInfo>,
    analyses: &mut BTreeMap<ValueId, SumRootAnalysis>,
    inst: &MirInstruction,
) {
    let mut roots = BTreeSet::new();
    for value in inst.used_values() {
        let root = resolve_value_origin(function, def_map, value);
        if variant_make_infos.contains_key(&root) {
            roots.insert(root);
        }
    }
    if roots.is_empty() {
        return;
    }

    match inst {
        MirInstruction::Copy { .. }
        | MirInstruction::KeepAlive { .. }
        | MirInstruction::Jump { .. } => {}
        MirInstruction::Branch { condition, .. } => {
            let condition_root = resolve_value_origin(function, def_map, *condition);
            if let Some(analysis) = analyses.get_mut(&condition_root) {
                analysis
                    .barriers
                    .insert(SumObjectizationBarrier::UnknownUse);
            }
        }
        MirInstruction::VariantTag { .. } => {
            for root in roots {
                if let Some(analysis) = analyses.get_mut(&root) {
                    analysis.tag_reads += 1;
                }
            }
        }
        MirInstruction::VariantProject { .. } => {
            for root in roots {
                if let Some(analysis) = analyses.get_mut(&root) {
                    analysis.project_reads += 1;
                }
            }
        }
        MirInstruction::Return { .. } => {
            add_barrier(analyses, &roots, SumObjectizationBarrier::Return)
        }
        MirInstruction::Call { .. } => add_barrier(analyses, &roots, SumObjectizationBarrier::Call),
        MirInstruction::Store { .. } | MirInstruction::FieldSet { .. } => {
            add_barrier(analyses, &roots, SumObjectizationBarrier::StoreLike)
        }
        MirInstruction::Phi { .. } => {
            add_barrier(analyses, &roots, SumObjectizationBarrier::PhiMerge)
        }
        MirInstruction::NewClosure { .. } => {
            add_barrier(analyses, &roots, SumObjectizationBarrier::Capture)
        }
        MirInstruction::Debug { .. } => {
            add_barrier(analyses, &roots, SumObjectizationBarrier::DebugObserve)
        }
        _ => add_barrier(analyses, &roots, SumObjectizationBarrier::UnknownUse),
    }
}

fn add_barrier(
    analyses: &mut BTreeMap<ValueId, SumRootAnalysis>,
    roots: &BTreeSet<ValueId>,
    barrier: SumObjectizationBarrier,
) {
    for root in roots {
        if let Some(analysis) = analyses.get_mut(root) {
            analysis.barriers.insert(barrier);
        }
    }
}

fn collect_variant_make_facts(
    variant_make_infos: &BTreeMap<ValueId, SumMakeInfo>,
    root_analyses: &BTreeMap<ValueId, SumRootAnalysis>,
) -> Vec<SumPlacementFact> {
    variant_make_infos
        .iter()
        .map(|(value, info)| {
            let analysis = root_analyses.get(value).expect("enum root analysis");
            let barriers = analysis.barriers.iter().copied().collect::<Vec<_>>();
            SumPlacementFact {
                block: info.block,
                instruction_index: info.instruction_index,
                value: Some(info.dst),
                surface: ThinEntrySurface::VariantMake,
                subject: info.subject.clone(),
                source_sum: None,
                value_class: info.value_class,
                state: state_from_analysis(analysis),
                tag_reads: analysis.tag_reads,
                project_reads: analysis.project_reads,
                barriers,
                reason: reason_for_variant_make(analysis),
            }
        })
        .collect()
}

fn collect_variant_project_facts(
    function: &MirFunction,
    selections: &[ThinEntrySelection],
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    root_analyses: &BTreeMap<ValueId, SumRootAnalysis>,
) -> Vec<SumPlacementFact> {
    let selection_sites =
        build_sum_selection_site_map(selections, ThinEntrySurface::VariantProject);
    let mut facts = Vec::new();

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::VariantProject { dst, value, .. } = inst else {
                continue;
            };
            let Some(selection) = selection_sites.get(&(block_id.as_u32(), instruction_index))
            else {
                continue;
            };
            let source_sum = resolve_value_origin(function, def_map, *value);
            let (state, tag_reads, project_reads, barriers, reason) = if let Some(analysis) =
                root_analyses.get(&source_sum)
            {
                (
                    state_from_analysis(analysis),
                    analysis.tag_reads,
                    analysis.project_reads,
                    analysis.barriers.iter().copied().collect::<Vec<_>>(),
                    reason_for_variant_project(analysis),
                )
            } else {
                (
                        SumPlacementState::NeedsObjectization,
                        0,
                        0,
                        vec![SumObjectizationBarrier::UnknownUse],
                        "variant.project source is not a local variant.make root in this function; keep compat/runtime objectization in the current pilot".to_string(),
                    )
            };
            facts.push(SumPlacementFact {
                block: block_id,
                instruction_index,
                value: Some(*dst),
                surface: ThinEntrySurface::VariantProject,
                subject: selection.subject.clone(),
                source_sum: Some(source_sum),
                value_class: selection.value_class,
                state,
                tag_reads,
                project_reads,
                barriers,
                reason,
            });
        }
    }

    facts
}

fn collect_variant_tag_facts(
    function: &MirFunction,
    selections: &[ThinEntrySelection],
    def_map: &HashMap<ValueId, (BasicBlockId, usize)>,
    root_analyses: &BTreeMap<ValueId, SumRootAnalysis>,
) -> Vec<SumPlacementFact> {
    let selection_sites = build_sum_selection_site_map(selections, ThinEntrySurface::VariantTag);
    let mut facts = Vec::new();

    for block_id in function.block_ids() {
        let Some(block) = function.blocks.get(&block_id) else {
            continue;
        };
        for (instruction_index, inst) in block.instructions.iter().enumerate() {
            let MirInstruction::VariantTag { dst, value, .. } = inst else {
                continue;
            };
            let Some(selection) = selection_sites.get(&(block_id.as_u32(), instruction_index))
            else {
                continue;
            };
            let source_sum = resolve_value_origin(function, def_map, *value);
            let (state, tag_reads, project_reads, barriers, reason) = if let Some(analysis) =
                root_analyses.get(&source_sum)
            {
                (
                    state_from_analysis(analysis),
                    analysis.tag_reads,
                    analysis.project_reads,
                    analysis.barriers.iter().copied().collect::<Vec<_>>(),
                    reason_for_variant_tag(analysis),
                )
            } else {
                (
                    SumPlacementState::NeedsObjectization,
                    0,
                    0,
                    vec![SumObjectizationBarrier::UnknownUse],
                    "variant.tag source is not a local variant.make root in this function; keep compat/runtime objectization in the current pilot".to_string(),
                )
            };
            facts.push(SumPlacementFact {
                block: block_id,
                instruction_index,
                value: Some(*dst),
                surface: ThinEntrySurface::VariantTag,
                subject: selection.subject.clone(),
                source_sum: Some(source_sum),
                value_class: selection.value_class,
                state,
                tag_reads,
                project_reads,
                barriers,
                reason,
            });
        }
    }

    facts
}

fn state_from_analysis(analysis: &SumRootAnalysis) -> SumPlacementState {
    if analysis.barriers.is_empty() {
        SumPlacementState::LocalAggregateCandidate
    } else {
        SumPlacementState::NeedsObjectization
    }
}

fn reason_for_variant_make(analysis: &SumRootAnalysis) -> String {
    if analysis.barriers.is_empty() {
        format!(
            "variant value stays on local variant.tag/variant.project routes (tag_reads={}, project_reads={}); this variant-specific proof should fold into a later generic placement/effect pass",
            analysis.tag_reads, analysis.project_reads
        )
    } else {
        format!(
            "variant value still crosses {} so the current pilot keeps outer objectization until an explicit barrier-aware placement/effect pass generalizes this route",
            join_barriers(&analysis.barriers)
        )
    }
}

fn reason_for_variant_project(analysis: &SumRootAnalysis) -> String {
    if analysis.barriers.is_empty() {
        "variant.project reads from a non-escaping local variant candidate and can stay on the future unboxed read path in this proving slice".to_string()
    } else {
        format!(
            "variant.project source still crosses {} so the current pilot keeps compat/runtime objectization",
            join_barriers(&analysis.barriers)
        )
    }
}

fn reason_for_variant_tag(analysis: &SumRootAnalysis) -> String {
    if analysis.barriers.is_empty() {
        "variant.tag reads from a non-escaping local variant candidate and can stay on the future unboxed tag path in this proving slice".to_string()
    } else {
        format!(
            "variant.tag source still crosses {} so the current pilot keeps compat/runtime objectization",
            join_barriers(&analysis.barriers)
        )
    }
}

fn join_barriers(barriers: &BTreeSet<SumObjectizationBarrier>) -> String {
    barriers
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn build_sum_selection_site_map(
    selections: &[ThinEntrySelection],
    surface: ThinEntrySurface,
) -> BTreeMap<(u32, usize), ThinEntrySelection> {
    selections
        .iter()
        .filter(|selection| {
            selection.surface == surface
                && selection.selected_entry == ThinEntryPreferredEntry::ThinInternalEntry
                && matches!(
                    selection.state,
                    ThinEntrySelectionState::Candidate | ThinEntrySelectionState::AlreadySatisfied
                )
        })
        .map(|selection| {
            (
                (selection.block.as_u32(), selection.instruction_index),
                selection.clone(),
            )
        })
        .collect()
}

#[cfg(test)]
mod tests {
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
}
