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
    fn evaluation(&self) -> &super::source_loop_v2::Evaluation {
        match &self.prefix {
            Prefix::BoolCondition { frame, .. } => frame.evaluation(),
            Prefix::InitializerExpression { frame, .. } => frame.evaluation(),
            Prefix::BindingRebind { frame, .. } => frame.evaluation(),
            _ => unreachable!("source projector frame"),
        }
    }
    pub(crate) fn calls(&self) -> &[crate::mir::resolved_semantics::home_new_prefix::LocalCallObservationV1] {
        self.evaluation().calls()
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
    pub(crate) binding: Option<LoopBindingKeyV1>,
    pub(crate) write_item: Option<LoopItemKeyV1>,
}
#[derive(Debug, Clone)]
pub(crate) struct SourceLoopTextBranchPhiV2 {
    pub(crate) source: Rc<crate::mir::resolved_semantics::home_new_prefix::BranchNormalTextSourceV1>,
    pub(crate) item: LoopItemKeyV1,
    pub(crate) result: LoopValueKeyV1,
    pub(crate) binding: Option<LoopBindingKeyV1>,
    pub(crate) write_item: Option<LoopItemKeyV1>,
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
        joins: Vec<SourceLoopBranchPhiV2>, text_joins: Vec<SourceLoopTextBranchPhiV2> },
    Return { source: Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>,
        operations: Range<usize>, item: LoopItemKeyV1, exit: LoopExitKeyV1 },
    Exit { item: LoopItemKeyV1, site: SourceStmtSiteV1, record: ResolvedExitRecordV1,
        port: usize, exit: LoopExitKeyV1 },
    /// A child source Loop executed through its own verified product. Inputs
    /// read this Loop's bindings at the child entry; each output pairs the
    /// child's After result with the write of the same parent binding.
    ChildLoop { site: SourceStmtSiteV1,
        product: Rc<super::source_loop_v2::VerifiedSourceLoopRecipeProductV2>,
        inputs: Vec<(BindingRefV1, Option<LoopItemKeyV1>, LoopValueKeyV1)>,
        outputs: Vec<(BindingRefV1, LoopItemKeyV1, Option<LoopItemKeyV1>, LoopValueKeyV1)> },
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
    pub(crate) fn branch_joins_v2(&self) -> impl Iterator<Item = &SourceLoopBranchPhiV2> {
        self.0.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::If { joins, .. } => Some(joins.as_slice()), _ => None,
        }).flatten()
    }
    pub(crate) fn text_branch_joins_v2(&self) -> impl Iterator<Item = &SourceLoopTextBranchPhiV2> {
        self.0.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::If { text_joins, .. } => Some(text_joins.as_slice()), _ => None,
        }).flatten()
    }
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
    pub(crate) fn child_loops_v2(&self) -> impl Iterator<Item = &Rc<super::source_loop_v2::VerifiedSourceLoopRecipeProductV2>> {
        self.0.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::ChildLoop { product, .. } => Some(product), _ => None,
        })
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
    pub(super) fn child_loops_v2(&self) -> impl Iterator<Item = &SourceLoopScheduledStepV2> {
        self.blocks.values().flatten().filter(|step| matches!(step, SourceLoopScheduledStepV2::ChildLoop { .. }))
    }
    pub(super) fn joins_v2(&self) -> impl Iterator<Item = &SourceLoopBranchPhiV2> {
        self.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::If { joins, .. } => Some(joins.as_slice()), _ => None,
        }).flatten()
    }
    pub(super) fn text_joins_v2(&self) -> impl Iterator<Item = &SourceLoopTextBranchPhiV2> {
        self.blocks.values().flatten().filter_map(|step| match step {
            SourceLoopScheduledStepV2::If { text_joins, .. } => Some(text_joins.as_slice()), _ => None,
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

}

#[path = "source_loop_schedule_verify.rs"]
mod verify;

#[path = "source_loop_schedule_child.rs"]
mod child;

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
        for (step_index, step) in steps.iter().enumerate() {
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
                SourceLoopScheduledStepV2::ChildLoop { site, product, inputs, outputs } => {
                    self.child_loop_step(block, &mut position, &mut available, site, product, inputs, outputs)?;
                }
                SourceLoopScheduledStepV2::Call { item, site, observation, result } => {
                    let scalar = matches!(observation.result(),
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64
                        | crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool);
                    let result_required = (observation.local_binding().is_some() && scalar)
                        || observation.is_expression_value()
                            && super::source_loop_v2::source_condition_call_v2(self.source, observation);
                    let result_class = match observation.result() {
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::I64 => Some(LoopValueClassV2::I64),
                        crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1::Bool => Some(LoopValueClassV2::Bool),
                        _ => None,
                    };
                    if super::source_loop_v2::source_call_v2(self.source, site) != Some(observation)
                        || block.items.get(position) != Some(item) || !self.items.insert(*item)
                        || self.draft.operations.contains_key(item)
                        || result.is_some() != result_required
                        || result.is_some_and(|result| result_class.is_none_or(|class|
                            self.recipe.values.get(result.raw() as usize)
                                .is_none_or(|value| value.key != result || value.class != class)))
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
                    let mut embedded_calls = BTreeMap::new();
                    let bool_projection = match frame.prefix() {
                        Prefix::BoolCondition { frame: original, .. } => Some(
                            self.source.bool_expressions.as_ref().map_err(|_| reject("schedule-frame-call-source"))?
                                .iter().find(|projection| projection.matches_frame(original))
                                .ok_or_else(|| reject("schedule-frame-call-source"))?),
                        _ => None,
                    };
                    for call in frame.calls() {
                        use crate::mir::resolved_semantics::home_new_prefix::LocalCallResultClassV1 as ResultClass;
                        let result_class = match call.result() {
                            ResultClass::Dynamic => LoopValueClassV2::Dynamic,
                            ResultClass::I64 => LoopValueClassV2::I64,
                            ResultClass::Bool => LoopValueClassV2::Bool,
                            ResultClass::Text => LoopValueClassV2::Text,
                            _ => return Err(reject("schedule-frame-call-class")),
                        };
                        if call.owner() != frame.source().owner()
                            || call.statement() != frame.prefix.statement()
                            || !call.is_expression_value() || call.result_source().is_none() {
                            return Err(reject("schedule-frame-call-source"));
                        }
                        let mut nodes = frame.source().nodes().iter().enumerate().filter(|(_, node)|
                            node.site() == call.site().site()
                                && matches!(node.source(), super::source_loop_v2::Node::MethodCall));
                        let (node, _) = nodes.next().ok_or_else(|| reject("schedule-frame-call-node"))?;
                        if nodes.next().is_some() { return Err(reject("schedule-frame-call-node")); }
                        let rows: Vec<_> = self.values.iter().filter(|row|
                            Rc::ptr_eq(&row.source, frame.source()) && row.node == node).collect();
                        if let Some(projection) = bool_projection {
                            let is_root_result = node == 0 && rows.len() == 1 && rows[0].value == frame.result();
                            if !rows.is_empty() && !is_root_result {
                                return Err(reject("schedule-frame-call-value"));
                            }
                            let eager_root = node == 0
                                && matches!(result_class, LoopValueClassV2::I64 | LoopValueClassV2::Bool);
                            if eager_root {
                                let mut imports = projection.imports().iter().enumerate().filter(|(_, (source_node, class))|
                                    *source_node == node && *class == result_class);
                                let (ordinal, _) = imports.next().ok_or_else(|| reject_at("schedule-frame-call-slot", 1))?;
                                if imports.next().is_some()
                                    || projection.region().nodes.iter().filter(|node| matches!(node,
                                        super::bool_region::LoopBoolNodeV1::Import { ordinal: actual, class }
                                            if *actual as usize == ordinal && *class == result_class)).count() != 1 {
                                    return Err(reject_at("schedule-frame-call-slot", 2));
                                }
                                let mut scheduled = steps[..step_index].iter().filter_map(|step| match step {
                                    SourceLoopScheduledStepV2::Call { site, observation, result: Some(result), .. }
                                        if site == call.statement() && observation == call => Some(*result),
                                    _ => None,
                                });
                                let value = scheduled.next().ok_or_else(|| reject_at("schedule-frame-call-slot", 3))?;
                                if scheduled.next().is_some() || rows.len() != 1
                                    || rows[0].value != frame.result()
                                    || self.recipe.values.get(value.raw() as usize)
                                        .is_none_or(|actual| actual.key != value || actual.class != result_class) {
                                    return Err(reject_at("schedule-frame-call-slot", 4));
                                }
                                let mut expressions = block.items[frame.items.clone()].iter().filter_map(|item|
                                    match &self.recipe.items[item.raw() as usize].item {
                                        LoopRecipeItemV2::Operation { operation: LoopOperationV2::BoolExpression { inputs, region, result } }
                                            if *result == frame.result() && region == projection.region() => Some(inputs),
                                        _ => None,
                                    });
                                let inputs = expressions.next().ok_or_else(|| reject_at("schedule-frame-call-slot", 5))?;
                                if expressions.next().is_some() || inputs.get(ordinal) != Some(&value) {
                                    return Err(reject_at("schedule-frame-call-slot", 6));
                                }
                            } else {
                                let mut effects = projection.calls().iter().enumerate().filter(|(_, (source_node, _))|
                                    *source_node == node);
                                let (ordinal, effect_class) = effects.next()
                                    .map(|(ordinal, (_, class))| (ordinal, *class))
                                    .ok_or_else(|| reject("schedule-frame-call-effect"))?;
                                if effects.next().is_some() || effect_class != result_class
                                    || projection.region().nodes.iter().filter(|node| matches!(node,
                                        super::bool_region::LoopBoolNodeV1::Call { ordinal: actual, class }
                                            if *actual as usize == ordinal && *class == result_class)).count() != 1
                                    || self.draft.calls_v2().any(|step| matches!(step,
                                        SourceLoopScheduledStepV2::Call { observation, .. } if observation == call)) {
                                    return Err(reject("schedule-frame-call-effect"));
                                }
                            }
                        } else {
                            let value = rows.first().ok_or_else(|| reject("schedule-frame-call-value"))?.value;
                            if rows.len() != 1
                                || self.recipe.values.get(value.raw() as usize)
                                    .is_none_or(|actual| actual.key != value || actual.class != result_class)
                                || embedded_calls.insert(value, call).is_some() {
                                return Err(reject("schedule-frame-call-value"));
                            }
                        }
                    }
                    if let Some(projection) = bool_projection {
                        let mut conditions = block.items[frame.items.clone()].iter().filter_map(|item|
                            match &self.recipe.items[item.raw() as usize].item {
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::BoolExpression { inputs, region, result } }
                                    if *result == frame.result() && region == projection.region() => Some(inputs),
                                _ => None,
                            });
                        let inputs = conditions.next().ok_or_else(|| reject("schedule-frame-bool-expression"))?;
                        let eager_calls = projection.imports().iter().filter(|(node, _)| frame.source().nodes().get(*node)
                            .is_some_and(|source| matches!(source.source(), super::source_loop_v2::Node::MethodCall))).count();
                        if conditions.next().is_some() || inputs.len() != projection.imports().len()
                            || projection.calls().len() + eager_calls != frame.calls().len()
                            || projection.imports().iter().any(|(node, _)| *node != 0
                                && frame.source().nodes().get(*node).is_some_and(|source|
                                    matches!(source.source(), super::source_loop_v2::Node::MethodCall)))
                            || projection.calls().iter().any(|(node, _)| frame.source().nodes().get(*node)
                                .is_none_or(|source| !matches!(source.source(), super::source_loop_v2::Node::MethodCall))) {
                            return Err(reject("schedule-frame-bool-expression"));
                        }
                    }
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
                        if let LoopOperationV2::CallSlot { receiver: None, args, result } = operation {
                            if bool_projection.is_some() { return Err(reject("schedule-frame-eager-bool-call")); }
                            if args.is_empty() {
                                let Some(result) = result else {
                                    return Err(reject_at("schedule-frame-call-slot", 7));
                                };
                                if embedded_calls.remove(result).is_none() {
                                    return Err(reject_at("schedule-frame-call-slot", 8));
                                }
                            } else if !self.array_dynamic_core_slot_matches(frame, args, *result) {
                                return Err(reject_at("schedule-frame-call-slot", 9));
                            }
                        }
                        if let LoopOperationV2::BinaryI64 { op, left, right, result } = operation {
                            self.integer_operation(frame, *op, *left, *right, *result)?;
                        }
                        if let LoopOperationV2::TextAdd { left, right, result } = operation {
                            self.text_add_operation(frame, *left, *right, *result)?;
                        }
                        if let LoopOperationV2::ReadBinding { binding, result } = operation {
                            if self.recipe.values.get(result.raw() as usize)
                                .is_some_and(|value| value.class == LoopValueClassV2::Text) {
                                self.text_read(frame, *binding, *result)?;
                            }
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
                            if !embedded_calls.is_empty() { return Err(reject("schedule-frame-call-census")); }
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
                        let expected = super::source_loop_v2::producer_value_v2(original.producer(), self.source, self.recipe,
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
                SourceLoopScheduledStepV2::If { item, site, bundle, condition, then_block, else_block, joins, text_joins } => {
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
                        let then_value = super::source_loop_v2::producer_value_v2(then_source, self.source, self.recipe,
                            self.values, self.bindings, self.draft).ok_or_else(|| reject("schedule-operation-source"))?;
                        let else_value = super::source_loop_v2::producer_value_v2(else_source, self.source, self.recipe,
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
                        match (join.binding, join.write_item) {
                            (Some(binding), Some(write_item)) => {
                                let is_carrier = self.recipe.carriers.iter().any(|row|
                                    row.owner_loop == self.recipe.root_loop && row.binding == binding
                                        && row.class == LoopValueClassV2::I64);
                                if self.bindings.get(&join.source.binding()) != Some(&binding)
                                    || !self.source.headers.contains_key(&join.source.binding())
                                    || !join.source.corroborates_original() || !is_carrier
                                    || block.items.get(position) != Some(&write_item)
                                    || !self.items.insert(write_item)
                                    || self.draft.operations.contains_key(&write_item)
                                    || !matches!(self.recipe.items[write_item.raw() as usize].item,
                                        LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding {
                                            binding: actual_binding, value
                                        } } if actual_binding == binding && value == join.result) {
                                    return Err(reject("schedule-operation-source"));
                                }
                                position += 1;
                            }
                            (None, None) => {}
                            _ => return Err(reject("schedule-operation-source")),
                        }
                    }
                    for join in text_joins {
                        let [(_, _, then_source), (_, _, else_source)] = join.source.incoming() else {
                            return Err(reject("schedule-text-operation-source"));
                        };
                        let then_value = super::source_loop_v2::text_producer_value_v2(then_source,
                            self.source, self.recipe, self.values, self.bindings, self.draft)
                            .ok_or_else(|| reject("schedule-text-operation-source"))?;
                        let else_value = super::source_loop_v2::text_producer_value_v2(else_source,
                            self.source, self.recipe, self.values, self.bindings, self.draft)
                            .ok_or_else(|| reject("schedule-text-operation-source"))?;
                        if !join.source.corroborates_original() || join.source.site() != site
                            || block.items.get(position) != Some(&join.item) || !self.items.insert(join.item)
                            || self.draft.operations.contains_key(&join.item)
                            || yes.as_ref().is_none_or(|values| !values.contains(&then_value))
                            || no.as_ref().is_none_or(|values| !values.contains(&else_value))
                            || available.contains(&join.result)
                            || self.recipe.values.get(join.result.raw() as usize)
                                .is_none_or(|value| value.key != join.result || value.class != LoopValueClassV2::Text)
                            || !matches!(self.recipe.items[join.item.raw() as usize].item,
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::BranchPhi {
                                    source_if, then_value: actual_yes, else_value: actual_no, result
                                } } if source_if == *item && actual_yes == then_value && actual_no == else_value
                                    && result == join.result) {
                            return Err(reject("schedule-text-operation-source"));
                        }
                        position += 1;
                        match (join.binding, join.write_item) {
                            (Some(binding), Some(write_item)) => {
                                let is_carrier = self.recipe.carriers.iter().any(|row|
                                    row.owner_loop == self.recipe.root_loop && row.binding == binding
                                        && row.class == LoopValueClassV2::Text);
                                if self.bindings.get(&join.source.binding()) != Some(&binding)
                                    || !self.source.headers.contains_key(&join.source.binding())
                                    || !is_carrier || block.items.get(position) != Some(&write_item)
                                    || !self.items.insert(write_item)
                                    || self.draft.operations.contains_key(&write_item)
                                    || !matches!(self.recipe.items[write_item.raw() as usize].item,
                                        LoopRecipeItemV2::Operation { operation: LoopOperationV2::WriteBinding {
                                            binding: actual_binding, value
                                        } } if actual_binding == binding && value == join.result) {
                                    return Err(reject("schedule-text-operation-source"));
                                }
                                position += 1;
                            }
                            (None, None) => {}
                            _ => return Err(reject("schedule-text-operation-source")),
                        }
                    }
                    match (yes, no) {
                        (None, None) => alive = false,
                        (Some(values), None) | (None, Some(values)) => available = values,
                        (Some(yes), Some(no)) => available = yes.intersection(&no).copied().collect(),
                    }
                    for join in joins {
                        if !available.insert(join.result) { return Err(reject("schedule-result-unavailable")); }
                    }
                    for join in text_joins {
                        if !available.insert(join.result) { return Err(reject("schedule-text-result-unavailable")); }
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
                    let expected_class = if source.is_null_literal_v1() { LoopValueClassV2::Dynamic } else { LoopValueClassV2::I64 };
                    if returned != 1 || class != Some(expected_class) { return Err(reject("schedule-result-unavailable")); }
                    if let Some(read) = &source.read {
                        let producer = super::source_loop_v2::producer_value_v2(read.producer(), self.source, self.recipe,
                            self.values, self.bindings, self.draft);
                        if operations.is_empty() {
                            if producer != Some(value) { return Err(reject("schedule-operation-source")); }
                        } else {
                            let binding = self.bindings.get(&read.binding()).ok_or_else(|| reject("schedule-operation-source"))?;
                            let role = self.source.source.bindings().bindings().find_map(|(actual, role)|
                                (actual == read.binding()).then_some(role));
                            match (role, read.producer()) {
                                (Some(crate::mir::builder::CallableLoopSourceBindingClassV1::Carrier), super::source_loop_v2::Producer::LoopHeader(basis))
                                    if self.source.headers.get(&read.binding()).is_some_and(|original| Rc::ptr_eq(original, basis))
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
                    } else if source.is_null_literal_v1() {
                        if operations.len() != 1 { return Err(reject("schedule-operation-source")); }
                        let null_item = block.items[position];
                        if !self.items.insert(null_item) || self.draft.operations.get(&null_item)
                            .is_none_or(|plan| !Rc::ptr_eq(plan, &source.source))
                            || !matches!(self.recipe.items[null_item.raw() as usize].item,
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::ConstNull { result } }
                                    if result == value) {
                            return Err(reject("schedule-operation-source"));
                        }
                        available.insert(value);
                    } else {
                        let literal = source.integer_literal_value_v1()
                            .ok_or_else(|| reject("schedule-operation-source"))?;
                        if operations.len() != 1 { return Err(reject("schedule-operation-source")); }
                        let literal_item = block.items[position];
                        if !self.items.insert(literal_item) || self.draft.operations.get(&literal_item)
                            .is_none_or(|plan| !Rc::ptr_eq(plan, &source.source))
                            || !matches!(self.recipe.items[literal_item.raw() as usize].item,
                                LoopRecipeItemV2::Operation { operation: LoopOperationV2::ConstI64 { result, value: actual } }
                                    if result == value && actual == literal) {
                            return Err(reject("schedule-operation-source"));
                        }
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

/// Default-off (builder debug flag): which exit of a schedule check rejected.
#[inline(never)]
fn reject_at(token: &str, exit: u32) -> String {
    if crate::config::env::builder_debug_enabled() {
        eprintln!("[source-loop] schedule reject {token} at exit {exit}");
    }
    reject(token)
}
