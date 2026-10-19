//! Same-projector source/Recipe execution view. No AST traversal, physical IDs,
//! expression evaluator or new execution permission lives in this owner.
use super::{ids::*, schema_v2::*, source_loop_v2::{reject, Summary, Source, SourceLoopItemV2, SourceLoopValueV2},
    source_loop_execution::SourceLoopEvaluationLifecycleV2, typed_schema_v2::VerifiedLoopRecipeV2};
use crate::mir::compiler::function_input::ResolvedFunctionLoweringInputV1;
use crate::mir::resolved_semantics::{BindingRefV1, OwnedExprSiteV1, ResolvedIfRegionBundleV1,
    ResolvedExitRecordV1, ResolvedExitSiteV1, ResolvedExitOriginV1,
    ResolvedControlTransferV1, SourceStmtSiteV1};
use crate::mir::resolved_semantics::home_new_prefix::{LifecyclePrefixSourceV1 as Prefix, SourceLoopPortKindV1};
use std::{collections::{BTreeMap, BTreeSet}, ops::Range, rc::Rc};

#[derive(Debug, Clone)]
pub(crate) struct SourceLoopScheduledFrameV2 {
    prefix: Prefix,
    block: LoopBlockKeyV1,
    items: Range<usize>,
    result: LoopValueKeyV1,
}
impl SourceLoopScheduledFrameV2 {
    pub(crate) fn prefix(&self) -> &Prefix { &self.prefix }
    pub(crate) fn block(&self) -> LoopBlockKeyV1 { self.block }
    pub(crate) fn items(&self) -> Range<usize> { self.items.clone() }
    pub(crate) fn result(&self) -> LoopValueKeyV1 { self.result }
    fn references(&self) -> &BTreeMap<usize, super::source_loop_v2::Reference> {
        match &self.prefix {
            Prefix::BoolCondition { frame, .. } => frame.references(),
            Prefix::InitializerExpression { frame, .. } => frame.evaluation().references(),
            Prefix::BindingRebind { frame, .. } => frame.evaluation().references(),
            _ => unreachable!("source projector frame"),
        }
    }
    pub(crate) fn source(&self) -> &Rc<Source> {
        match &self.prefix {
            Prefix::BoolCondition { frame, .. } => frame.source(),
            Prefix::InitializerExpression { frame, .. } => frame.source(),
            Prefix::BindingRebind { frame, .. } => frame.source(),
            _ => unreachable!("only the source projector constructs scheduled frames"),
        }
    }
}

/// Coordinate projection retained by the original If, never a new source proof.
#[derive(Debug, Clone)]
pub(crate) struct SourceLoopBranchPhiV2 {
    pub(crate) source: Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalScalarSourceV1>,
    pub(crate) item: LoopItemKeyV1,
    pub(crate) result: LoopValueKeyV1,
}
#[derive(Debug, Clone)]
pub(crate) enum SourceLoopScheduledStepV2 {
    Frame(usize),
    Call { item: LoopItemKeyV1, site: SourceStmtSiteV1,
        observation: crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1,
        result: Option<LoopValueKeyV1> },
    OwnedArrayAppend { item: LoopItemKeyV1, site: SourceStmtSiteV1,
        observation: crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1 },
    If { item: LoopItemKeyV1, site: SourceStmtSiteV1, bundle: ResolvedIfRegionBundleV1,
        condition: usize, then_block: LoopBlockKeyV1, else_block: Option<LoopBlockKeyV1>,
        joins: Vec<SourceLoopBranchPhiV2> },
    Return { source: Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>,
        operations: Range<usize>, item: LoopItemKeyV1, exit: LoopExitKeyV1 },
    Exit { item: LoopItemKeyV1, site: SourceStmtSiteV1, record: ResolvedExitRecordV1,
        port: usize, exit: LoopExitKeyV1 },
}

#[derive(Default, Clone)]
pub(super) struct SourceLoopScheduleDraftV2 {
    frames: Vec<SourceLoopScheduledFrameV2>,
    operations: BTreeMap<LoopItemKeyV1, Rc<Source>>,
    blocks: BTreeMap<LoopBlockKeyV1, Vec<SourceLoopScheduledStepV2>>,
}
#[derive(Debug)]
pub(crate) struct VerifiedSourceLoopScheduleV2(SourceLoopScheduleDraftV2);
impl std::fmt::Debug for SourceLoopScheduleDraftV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SourceLoopScheduleDraftV2").field("frames", &self.frames).field("blocks", &self.blocks).finish()
    }
}
impl VerifiedSourceLoopScheduleV2 {
    pub(crate) fn blocks_array_appends_v2(&self)
        -> impl Iterator<Item = &crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1> {
        self.0.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::OwnedArrayAppend { observation, .. } => Some(observation), _ => None,
        })
    }
    pub(crate) fn returns_v2(&self) -> impl Iterator<Item = &Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>> {
        self.0.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::Return { source, .. } => Some(source), _ => None,
        })
    }
    pub(crate) fn calls_v2(&self) -> impl Iterator<Item = &SourceLoopScheduledStepV2> {
        self.0.blocks.values().flatten().filter(|step| matches!(step, SourceLoopScheduledStepV2::Call { .. }))
    }
    pub(crate) fn frames(&self) -> &[SourceLoopScheduledFrameV2] { &self.0.frames }
    pub(crate) fn block(&self, key: LoopBlockKeyV1) -> Option<&[SourceLoopScheduledStepV2]> {
        self.0.blocks.get(&key).map(Vec::as_slice)
    }
}
impl SourceLoopScheduleDraftV2 {
    pub(super) fn calls_v2(&self) -> impl Iterator<Item = &SourceLoopScheduledStepV2> {
        self.blocks.values().flatten().filter(|step| matches!(step, SourceLoopScheduledStepV2::Call { .. }))
    }
    pub(super) fn joins_v2(&self) -> impl Iterator<Item = &SourceLoopBranchPhiV2> {
        self.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::If { joins, .. } => Some(joins.as_slice()), _ => None,
        }).flatten()
    }
    pub(super) fn block(&mut self, key: LoopBlockKeyV1) { self.blocks.insert(key, Vec::new()); }
    pub(super) fn operation(&mut self, item: LoopItemKeyV1, source: &Rc<Source>) {
        assert!(self.operations.insert(item, Rc::clone(source)).is_none(), "projector operation issued once");
    }
    pub(super) fn frame_count(&self) -> usize { self.frames.len() }
    pub(super) fn frame(&mut self, prefix: Prefix, block: LoopBlockKeyV1,
        items: Range<usize>, result: LoopValueKeyV1) {
        let index = self.frames.len();
        self.frames.push(SourceLoopScheduledFrameV2 { prefix, block, items, result });
        self.step(block, SourceLoopScheduledStepV2::Frame(index));
    }
    pub(super) fn step(&mut self, block: LoopBlockKeyV1, step: SourceLoopScheduledStepV2) {
        self.blocks.get_mut(&block).expect("same projector allocated this Recipe block").push(step);
    }
    pub(super) fn finish(self, input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
        recipe: &VerifiedLoopRecipeV2, execution: &[SourceLoopEvaluationLifecycleV2],
        values: &[SourceLoopValueV2], control: &[SourceLoopItemV2], bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    ) -> Result<VerifiedSourceLoopScheduleV2, String> {
        self.verify(input, source, recipe, execution, values, control, bindings)?;
        Ok(VerifiedSourceLoopScheduleV2(self))
    }
    fn verify(&self, input: ResolvedFunctionLoweringInputV1<'_>, source: &Summary,
        verified: &VerifiedLoopRecipeV2, execution: &[SourceLoopEvaluationLifecycleV2],
        values: &[SourceLoopValueV2], control: &[SourceLoopItemV2], bindings: &BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    ) -> Result<(), String> {
        let recipe = verified.as_recipe();
        let demanded = super::source_loop_v2::demanded_branch_joins_v2(source)?;
        if demanded.len() != self.joins_v2().count() || demanded.iter().any(|source|
            self.joins_v2().filter(|join| &join.source == source).count() != 1) {
            return Err(reject("schedule-operation-source"));
        }
        if source.source.owner() != input.owner() || self.frames.len() != execution.len()
            || self.blocks.len() != recipe.blocks.len() || recipe.loops.len() != 1
            || self.operations.len() + self.blocks.values().flatten()
                .filter(|step| matches!(step, SourceLoopScheduledStepV2::OwnedArrayAppend { .. } | SourceLoopScheduledStepV2::Call { .. })).count()
                + self.joins_v2().count()
                != recipe.items.iter().filter(|row| matches!(row.item, LoopRecipeItemV2::Operation { .. })).count() {
            return Err(reject("schedule-census"));
        }
        for (frame, execution) in self.frames.iter().zip(execution) {
            let original = match frame.prefix() {
                Prefix::BoolCondition { frame, .. } => Rc::ptr_eq(frame, &source.condition)
                    || source.body_expressions.iter().any(|plan| plan.condition_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                Prefix::InitializerExpression { frame, .. } => source.body_expressions.iter()
                    .any(|plan| plan.initializer_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                Prefix::BindingRebind { frame, .. } => source.body_expressions.iter()
                    .any(|plan| plan.rebind_frame().is_some_and(|old| Rc::ptr_eq(old, frame))),
                _ => false,
            };
            if !original || !Rc::ptr_eq(frame.source(), &execution.source)
                || frame.prefix.site() != &OwnedExprSiteV1::new(input.owner(), frame.source().nodes()[0].site().clone())
                || values.iter().filter(|row| row.node == 0 && Rc::ptr_eq(&row.source, frame.source())
                    && row.value == frame.result).count() != 1 {
                return Err(reject("schedule-original-frame"));
            }
        }
        let root = &recipe.loops[0];
        let LoopConditionV2::Predicate { block, value } = root.condition else { return Err(reject("schedule-root-condition")); };
        if !matches!(self.blocks.get(&block).map(Vec::as_slice), Some([SourceLoopScheduledStepV2::Frame(0)]))
            || self.frames.first().is_none_or(|frame| !Rc::ptr_eq(frame.source(), source.condition.source()) || frame.result != value) {
            return Err(reject("schedule-root-condition"));
        }
        let mut check = ScheduleCheck { draft: self, recipe, input, source, bindings, values,
            frames: Vec::new(), items: BTreeSet::new(), blocks: BTreeSet::new(), control: Vec::new() };
        let available = recipe.inputs.iter().copied().collect();
        let available = check.block(block, available)?.ok_or_else(|| reject("schedule-condition-exit"))?;
        check.block(root.body, available)?;
        if check.blocks.len() != recipe.blocks.len() || check.items.len() != recipe.items.len()
            || check.frames != (0..execution.len()).collect::<Vec<_>>()
            || check.control.len() != control.len()
            || check.control.iter().zip(control).any(|((site, item), old)| site != &old.site || item != &old.item) {
            return Err(reject("schedule-ordered-coverage"));
        }
        Ok(())
    }
}

struct ScheduleCheck<'a, 'source> {
    draft: &'a SourceLoopScheduleDraftV2,
    recipe: &'a LoopRecipeV2,
    input: ResolvedFunctionLoweringInputV1<'source>,
    source: &'a Summary,
    bindings: &'a BTreeMap<BindingRefV1, LoopBindingKeyV1>,
    values: &'a [SourceLoopValueV2],
    frames: Vec<usize>,
    items: BTreeSet<LoopItemKeyV1>,
    blocks: BTreeSet<LoopBlockKeyV1>,
    control: Vec<(SourceStmtSiteV1, LoopItemKeyV1)>,
}
impl ScheduleCheck<'_, '_> {
    fn block(&mut self, key: LoopBlockKeyV1, mut available: BTreeSet<LoopValueKeyV1>)
        -> Result<Option<BTreeSet<LoopValueKeyV1>>, String> {
        if !self.blocks.insert(key) { return Err(reject("schedule-block-repeat")); }
        let block = self.recipe.blocks.get(key.raw() as usize).filter(|row| row.key == key)
            .ok_or_else(|| reject("schedule-block-missing"))?;
        let steps = self.draft.blocks.get(&key).ok_or_else(|| reject("schedule-block-missing"))?;
        let mut position = 0;
        let mut alive = true;
        for step in steps {
            if !alive { return Err(reject("schedule-after-exit")); }
            match step {
                SourceLoopScheduledStepV2::OwnedArrayAppend { item, site, observation } => {
                    if super::source_loop_v2::source_array_append_v2(self.source, site) != Some(observation)
                        || block.items.get(position) != Some(item) || !self.items.insert(*item)
                        || self.draft.operations.contains_key(item)
                        || !matches!(&self.recipe.items[item.raw() as usize].item,
                            LoopRecipeItemV2::Operation { operation: LoopOperationV2::CallSlot { receiver: None, args, result: None } }
                                if args.is_empty()) {
                        return Err(reject("schedule-array-source"));
                    }
                    self.control.push((site.clone(), *item)); position += 1;
                }
                SourceLoopScheduledStepV2::Call { item, site, observation, result } => {
                    if super::source_loop_v2::source_call_v2(self.source, site) != Some(observation)
                        || block.items.get(position) != Some(item) || !self.items.insert(*item)
                        || self.draft.operations.contains_key(item)
                        || result.is_some() != observation.local_binding().is_some()
                        || !matches!(&self.recipe.items[item.raw() as usize].item,
                            LoopRecipeItemV2::Operation { operation: LoopOperationV2::CallSlot { receiver: None, args, result: actual } }
                                if args.is_empty() && actual == result) {
                        return Err(reject("schedule-operation-source"));
                    }
                    if let Some(result) = result { available.insert(*result); }
                    self.control.push((site.clone(), *item)); position += 1;
                }
                SourceLoopScheduledStepV2::Frame(index) => {
                    let frame = self.draft.frames.get(*index).ok_or_else(|| reject("schedule-frame-index"))?;
                    if frame.block != key || frame.items.start != position || frame.items.end < position
                        || frame.items.end > block.items.len() { return Err(reject("schedule-frame-range")); }
                    for (offset, item) in block.items[frame.items.clone()].iter().enumerate() {
                        if !self.items.insert(*item) { return Err(reject("schedule-item-repeat")); }
                        let LoopRecipeItemV2::Operation { operation } = &self.recipe.items[item.raw() as usize].item
                            else { return Err(reject("schedule-frame-control-overlap")); };
                        if self.draft.operations.get(item).is_none_or(|source| !Rc::ptr_eq(source, frame.source())) {
                            return Err(reject("schedule-operation-source"));
                        }
                        if matches!(operation, LoopOperationV2::BranchPhi { .. }) {
                            return Err(reject("schedule-operation-source"));
                        }
                        if let LoopOperationV2::WriteBinding { binding, value } = operation {
                            if !matches!(frame.prefix(), Prefix::BindingRebind { frame: rebind, .. }
                                if self.bindings.get(&rebind.target()) == Some(binding) && *value == frame.result
                                    && position + offset + 1 == frame.items.end) {
                                return Err(reject("schedule-write-purpose"));
                            }
                            self.control.push((frame.prefix.statement().clone(), *item));
                        }
                        if let Some(result) = super::join_sig::operation_result_v2(operation) { available.insert(result); }
                    }
                    if matches!(frame.prefix(), Prefix::BindingRebind { frame: rebind, .. }
                        if self.bindings.get(&rebind.target()).is_some_and(|binding|
                            self.recipe.carriers.iter().any(|row| row.binding == *binding))) && (frame.items.is_empty() || !matches!(block.items.get(frame.items.end.wrapping_sub(1))
                        .and_then(|item| self.recipe.items.get(item.raw() as usize)).map(|row| &row.item),
                        Some(LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding { .. } }))) {
                        return Err(reject("schedule-rebind-write-missing"));
                    }
                    for (&node, reference) in frame.references() {
                        let super::source_loop_v2::Reference::Scalar(read) = reference else { continue; };
                        let Some(original) = read.original_read() else { continue; };
                        let super::source_loop_v2::Producer::BranchNormal(join) = original.producer() else { continue; };
                        let expected = super::source_loop_v2::producer_value_v2(original.producer(), self.recipe,
                            self.values, self.bindings, self.draft).ok_or_else(|| reject("schedule-operation-source"))?;
                        if join.binding() != reference.binding() || !available.contains(&expected)
                            || self.values.iter().filter(|row| Rc::ptr_eq(&row.source, frame.source())
                                && row.node == node && row.value == expected).count() != 1 {
                            return Err(reject("schedule-operation-source"));
                        }
                    }
                    if !available.contains(&frame.result) { return Err(reject("schedule-result-unavailable")); }
                    self.frames.push(*index); position = frame.items.end;
                }
                SourceLoopScheduledStepV2::If { item, site, bundle, condition, then_block, else_block, joins } => {
                    if block.items.get(position) != Some(item) || !self.items.insert(*item)
                        || self.frames.last() != Some(condition) { return Err(reject("schedule-if-order")); }
                    let frame = &self.draft.frames[*condition];
                    if frame.items.end != position || self.input.function().if_region_bundle(site).ok() != Some(bundle)
                        || !matches!(frame.prefix(), Prefix::BoolCondition { .. }) || frame.prefix.statement() != site
                        || !matches!(self.recipe.items[item.raw() as usize].item,
                            LoopRecipeItemV2::If { condition: value, then_block: yes, else_block: no }
                                if value == frame.result && yes == *then_block && no == *else_block) {
                        return Err(reject("schedule-if-source"));
                    }
                    self.control.push((site.clone(), *item));
                    let yes = self.block(*then_block, available.clone())?;
                    let no = match else_block { Some(key) => self.block(*key, available.clone())?, None => Some(available.clone()) };
                    position += 1;
                    for join in joins {
                        let [(_, _, then_source), (_, _, else_source)] = join.source.incoming() else {
                            return Err(reject("schedule-operation-source"));
                        };
                        let then_value = super::source_loop_v2::producer_value_v2(then_source, self.recipe,
                            self.values, self.bindings, self.draft).ok_or_else(|| reject("schedule-operation-source"))?;
                        let else_value = super::source_loop_v2::producer_value_v2(else_source, self.recipe,
                            self.values, self.bindings, self.draft).ok_or_else(|| reject("schedule-operation-source"))?;
                        if !join.source.corroborates_original() || join.source.site() != site
                            || block.items.get(position) != Some(&join.item) || !self.items.insert(join.item)
                            || self.draft.operations.contains_key(&join.item)
                            || yes.as_ref().is_none_or(|values| !values.contains(&then_value))
                            || no.as_ref().is_none_or(|values| !values.contains(&else_value))
                            || available.contains(&join.result)
                            || !matches!(self.recipe.items[join.item.raw() as usize].item,
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::BranchPhi {
                                    source_if, then_value: actual_yes, else_value: actual_no, result
                                } } if source_if == *item && actual_yes == then_value && actual_no == else_value
                                    && result == join.result) {
                            return Err(reject("schedule-operation-source"));
                        }
                        position += 1;
                    }
                    match (yes, no) {
                        (None, None) => alive = false,
                        (Some(values), None) | (None, Some(values)) => available = values,
                        (Some(yes), Some(no)) => available = yes.intersection(&no).copied().collect(),
                    }
                    for join in joins {
                        if !available.insert(join.result) { return Err(reject("schedule-result-unavailable")); }
                    }
                }
                SourceLoopScheduledStepV2::Return { source, operations, item, exit } => {
                    if !source.corroborates(self.input) || !super::source_loop_v2::source_returns_v2(self.source)
                        .iter().any(|original| Rc::ptr_eq(original, source))
                        || operations.start != position || operations.end > block.items.len() || operations.end < position {
                        return Err(reject("schedule-exit-source"));
                    }
                    let LoopExitKindV2::Return { value: Some(value) } = self.recipe.exits.get(exit.raw() as usize)
                        .filter(|row| row.key == *exit).ok_or_else(|| reject("schedule-exit-source"))?.kind
                        else { return Err(reject("schedule-exit-source")); };
                    let class = self.recipe.values.get(value.raw() as usize).filter(|row| row.key == value).map(|row| row.class);
                    let returned = self.values.iter().filter(|row| Rc::ptr_eq(&row.source, &source.source) && row.node == 0 && row.value == value).count();
                    if returned != 1 || class != Some(LoopValueClassV2::I64) { return Err(reject("schedule-result-unavailable")); }
                    let producer = super::source_loop_v2::producer_value_v2(source.read.producer(), self.recipe,
                        self.values, self.bindings, self.draft);
                    if operations.is_empty() {
                        if producer != Some(value) { return Err(reject("schedule-operation-source")); }
                    } else {
                        let binding = self.bindings.get(&source.read.binding()).ok_or_else(|| reject("schedule-operation-source"))?;
                        let role = self.source.source.bindings().bindings().find_map(|(actual, role)|
                            (actual == source.read.binding()).then_some(role));
                        match (role, source.read.producer()) {
                            (Some(crate::mir::builder::CallableLoopSourceBindingClassV1::Carrier), super::source_loop_v2::Producer::LoopHeader(basis))
                                if self.source.headers.get(&source.read.binding()).is_some_and(|original| Rc::ptr_eq(original, basis))
                                    && Rc::ptr_eq(basis.source(), &self.source.source) => {},
                            (Some(crate::mir::builder::CallableLoopSourceBindingClassV1::ReadOnlyOperand), producer)
                                if !matches!(producer, super::source_loop_v2::Producer::LoopHeader(_))
                                    && self.recipe.read_only_inputs.iter().any(|row| row.binding == *binding && row.class == LoopValueClassV2::I64) => {},
                            _ => return Err(reject("schedule-operation-source")),
                        }
                        if operations.len() != 1 { return Err(reject("schedule-operation-source")); }
                        let read_item = block.items[position];
                        if !self.items.insert(read_item) || self.draft.operations.get(&read_item)
                            .is_none_or(|plan| !Rc::ptr_eq(plan, &source.source))
                            || !matches!(self.recipe.items[read_item.raw() as usize].item,
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::ReadBinding { binding: actual, result } }
                                    if actual == *binding && result == value) { return Err(reject("schedule-operation-source")); }
                        available.insert(value);
                    }
                    if !available.contains(&value) || block.items.get(operations.end) != Some(item) || !self.items.insert(*item)
                        || !matches!(self.recipe.items[item.raw() as usize].item, LoopRecipeItemV2::Exit { exit: actual } if actual == *exit) {
                        return Err(reject("schedule-exit-source"));
                    }
                    self.control.push((source.site.clone(), *item)); position = operations.end + 1; alive = false;
                }
                SourceLoopScheduledStepV2::Exit { item, site, record, port, exit } => {
                    let original = self.input.function().resolved_exit(&ResolvedExitSiteV1::Statement(site.clone()));
                    let target = self.input.function().loop_region_bundle(self.source.source.site())
                        .map_err(|_| reject("schedule-exit-target"))?.loop_pair().region();
                    let (kind, expected) = match (record.origin(), record.transfer()) {
                        (ResolvedExitOriginV1::ExplicitBreak, ResolvedControlTransferV1::Break { target_loop }) if target_loop == target =>
                            (LoopExitKindV2::Break { target_loop: self.recipe.root_loop }, SourceLoopPortKindV1::Break(site.clone())),
                        (ResolvedExitOriginV1::ExplicitContinue, ResolvedControlTransferV1::Continue { target_loop }) if target_loop == target =>
                            (LoopExitKindV2::Continue { target_loop: self.recipe.root_loop }, SourceLoopPortKindV1::Continue(site.clone())),
                        _ => return Err(reject("schedule-exit-source")),
                    };
                    if original != Some(record) || self.source.ports.get(*port).is_none_or(|row| row.kind != expected)
                        || block.items.get(position) != Some(item) || !self.items.insert(*item)
                        || !matches!(self.recipe.items[item.raw() as usize].item, LoopRecipeItemV2::Exit { exit: actual } if actual == *exit)
                        || self.recipe.exits.get(exit.raw() as usize).is_none_or(|row| row.key != *exit || row.kind != kind) {
                        return Err(reject("schedule-exit-source"));
                    }
                    self.control.push((site.clone(), *item)); position += 1; alive = false;
                }
            }
        }
        if position != block.items.len() { return Err(reject("schedule-block-coverage")); }
        Ok(alive.then_some(available))
    }
}

#[cfg(test)]
pub(super) fn assert_original_schedule_for_test(input: ResolvedFunctionLoweringInputV1<'_>,
    product: &super::source_loop_v2::VerifiedSourceLoopRecipeProductV2,
) {
    let original = &product.schedule().0;
    let verify = |draft: &SourceLoopScheduleDraftV2| draft.verify(input, &product.source, &product.recipe,
        &product.execution, &product.values, &product.control, &product.bindings);
    verify(original).unwrap();
    let root = &product.recipe.as_recipe().loops[0];
    let body = root.body;
    let first_body = original.blocks[&body].iter().find_map(|step| match step {
        SourceLoopScheduledStepV2::Frame(index) => Some(*index), _ => None,
    }).unwrap();
    for mutation in 0..13 {
        let mut draft = original.clone();
        match mutation {
            0 => { draft.frames.pop(); }
            1 => draft.frames.swap(0, first_body),
            2 => { let step = draft.blocks[&body][0].clone(); draft.blocks.get_mut(&body).unwrap().insert(1, step); }
            3 => draft.frames[first_body].result = draft.frames[0].result,
            4 => draft.frames[first_body].items.end = draft.frames[first_body].items.start,
            5 => {
                let condition = draft.blocks[&body].iter().find_map(|step| match step {
                    SourceLoopScheduledStepV2::If { condition, .. } => Some(*condition), _ => None,
                }).unwrap();
                draft.frames[condition].items.end += 1;
            }
            6 => {
                let steps = draft.blocks.values_mut().find(|steps| steps.iter()
                    .any(|step| matches!(step, SourceLoopScheduledStepV2::Exit { .. }))).unwrap();
                steps.pop();
            }
            7 => { draft.blocks.insert(LoopBlockKeyV1::new(999), Vec::new()); }
            8 => {
                let step = draft.blocks.values_mut().flatten()
                    .find(|step| matches!(step, SourceLoopScheduledStepV2::If { .. })).unwrap();
                let SourceLoopScheduledStepV2::If { then_block, .. } = step else { unreachable!() };
                *then_block = body;
            }
            9 => {
                let step = draft.blocks.values_mut().flatten()
                    .find(|step| matches!(step, SourceLoopScheduledStepV2::Exit { .. })).unwrap();
                let SourceLoopScheduledStepV2::Exit { port, .. } = step else { unreachable!() };
                *port += product.source.ports.len();
            }
            10 => {
                let frame = draft.frames.iter_mut().find(|frame| matches!(frame.prefix(), Prefix::BindingRebind { .. })).unwrap();
                frame.items.end -= 1;
            }
            11 => {
                let condition = draft.blocks[&body].iter().find_map(|step| match step {
                    SourceLoopScheduledStepV2::If { condition, .. } => Some(*condition), _ => None,
                }).unwrap();
                assert_eq!(draft.frames[first_body].items.end, draft.frames[condition].items.start);
                let end = draft.frames[condition].items.end;
                draft.frames[first_body].items.end = end;
                draft.frames[condition].items.start = end;
            }
            12 => { draft.operations.pop_first(); }
            _ => unreachable!(),
        }
        assert!(verify(&draft).is_err(), "source/Recipe schedule mutation {mutation} must refuse before effects");
    }
}
