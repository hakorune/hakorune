//! Execute the original verified schedule with the invocation's existing writers.
//! This physical graph is not completion: source closure and both final consumers
//! must accept it before the production fences can be removed.
use std::{cell::RefCell, collections::{BTreeMap, BTreeSet}, rc::Rc};
use crate::mir::builder::{MirBuilder, BindingValueObservationV2, LocalSsaPhysicalRelationV1, ConditionMaterializationV1};
use crate::mir::builder::normal_callable_loop_source_facts::SourceLoopCondPhysicalInputV1;
use crate::mir::builder::normal_callable_semantic_lowering_state::{CallableSemanticLoweringState, MaterializedValuesV1};
use crate::mir::builder::recursive_child_lowering::RawInvocationChildPortV1;
use crate::mir::builder::ordinary_new_admission::selected::source_loop_cleanup::{self, EmittedSourceLoopCleanupV2};
use crate::mir::builder::vars::lexical_scope::{try_with_lexical_scope_with_receipt_v2, LocalBindingStateSnapshot, LexicalScopeCloseReceiptV2};
use crate::mir::loop_recipe_contract::{VerifiedSourceLoopRecipeProductV2 as Product,
    SourceLoopScheduledStepV2 as Step, SourceLoopBranchPhiV2, LoopBlockKeyV1, LoopBindingKeyV1, LoopValueKeyV1};
use crate::mir::resolved_semantics::{BindingRefV1, home_new_prefix::{LifecyclePrefixSourceV1 as Prefix,
    SourceLoopPortKindV1 as PortKind}};
use crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1 as Edge;
use crate::mir::{BasicBlockId, ValueId, MirInstruction, MirFunction};

pub(in crate::mir) struct EmittedLoopIncomingV2 {
    pub(in crate::mir) binding: BindingRefV1,
    pub(in crate::mir) logical: LoopValueKeyV1,
    pub(in crate::mir) observation: BindingValueObservationV2,
    pub(in crate::mir) dependencies: Vec<(BasicBlockId, MirInstruction)>,
    pub(in crate::mir) localized: Rc<LocalSsaPhysicalRelationV1>,
    pub(in crate::mir) canonical: Option<Rc<LocalSsaPhysicalRelationV1>>,
}
pub(in crate::mir) struct EmittedLoopEdgeV2 {
    pub(in crate::mir) source: Edge,
    pub(in crate::mir) predecessor: BasicBlockId,
    pub(in crate::mir) target: BasicBlockId,
    pub(in crate::mir) cleanup: Option<EmittedSourceLoopCleanupV2>,
    pub(in crate::mir) values: Vec<EmittedLoopIncomingV2>,
}
pub(in crate::mir) struct EmittedLoopFrameV2 {
    pub(in crate::mir) index: usize,
    pub(in crate::mir) entry: BasicBlockId,
    pub(in crate::mir) end: BasicBlockId,
    pub(in crate::mir) value: ValueId,
    pub(in crate::mir) canonical: Rc<LocalSsaPhysicalRelationV1>,
    pub(in crate::mir) condition: Option<Rc<ConditionMaterializationV1>>,
    pub(in crate::mir) targets: Option<(BasicBlockId, BasicBlockId)>,
}
/// SAME schedule fork and actual arm ends, before any normal join is emitted.
/// These records corroborate source control; they do not classify a new If.
pub(in crate::mir) struct EmittedLoopForkV2 {
    pub(in crate::mir) condition: usize,
    pub(in crate::mir) yes: LoopBlockKeyV1,
    pub(in crate::mir) no: Option<LoopBlockKeyV1>,
    pub(in crate::mir) starts: (BasicBlockId, BasicBlockId),
    pub(in crate::mir) ends: (BasicBlockId, BasicBlockId),
    pub(in crate::mir) normal: (bool, bool),
    pub(in crate::mir) join: Option<BasicBlockId>,
    pub(in crate::mir) condition_materialization: Rc<crate::mir::builder::IfConditionMaterializationV1>,
    pub(in crate::mir) joins: Vec<Rc<crate::mir::normal_callable_semantic_package::BranchScalarPhiObservationV1>>,
}
pub(in crate::mir) struct EmittedLoopBlockV2 {
    pub(in crate::mir) key: LoopBlockKeyV1,
    pub(in crate::mir) entry: BasicBlockId,
    pub(in crate::mir) end: BasicBlockId,
    pub(in crate::mir) normal: bool,
    pub(in crate::mir) scope_close: LexicalScopeCloseReceiptV2,
}
pub(in crate::mir) struct EmittedLoopEffectV2 {
    pub(in crate::mir) site: crate::mir::resolved_semantics::SourceStmtSiteV1,
    pub(in crate::mir) entry: BasicBlockId,
    pub(in crate::mir) end: BasicBlockId,
    pub(in crate::mir) packet: crate::mir::normal_callable_semantic_package::RootLifecyclePrefixBindingGroupV1,
    pub(in crate::mir) result: Option<(LoopValueKeyV1, Rc<LocalSsaPhysicalRelationV1>)>,
}
pub(in crate::mir) struct EmittedLoopReturnV2 {
    pub(in crate::mir) source: Rc<crate::mir::resolved_semantics::home_new_prefix::SourceLoopReturnV1>,
    pub(in crate::mir) entry: BasicBlockId,
    pub(in crate::mir) end: BasicBlockId,
    pub(in crate::mir) value: ValueId,
    pub(in crate::mir) terminal: (BasicBlockId, MirInstruction),
    pub(in crate::mir) read: crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1,
    pub(in crate::mir) bindings: Vec<(BasicBlockId, MirInstruction)>,
}
pub(in crate::mir) struct EmittedSourceLoopGraphV2 {
    pub(in crate::mir) product: Rc<Product>,
    pub(in crate::mir) header: BasicBlockId,
    pub(in crate::mir) after: BasicBlockId,
    pub(in crate::mir) returns: Vec<EmittedLoopReturnV2>,
    pub(in crate::mir) edges: Vec<EmittedLoopEdgeV2>,
    pub(in crate::mir) frames: Vec<EmittedLoopFrameV2>,
    pub(in crate::mir) effects: Vec<EmittedLoopEffectV2>,
    pub(in crate::mir) forks: Vec<EmittedLoopForkV2>,
    pub(in crate::mir) blocks: Vec<EmittedLoopBlockV2>,
    pub(in crate::mir) result: crate::mir::builder::CompletedConstV1,
    before: BTreeMap<BasicBlockId, (Vec<MirInstruction>, Option<MirInstruction>)>,
    before_frame: Option<(BasicBlockId, MirInstruction)>,
    after_frame: Option<(BasicBlockId, MirInstruction)>,
}
pub(super) struct SourceLoopEntryStateV2 {
    pub(super) values: MaterializedValuesV1,
    pub(super) cache: LocalBindingStateSnapshot,
}
impl std::fmt::Debug for EmittedSourceLoopGraphV2 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EmittedSourceLoopGraphV2").field("header", &self.header).field("after", &self.after).finish_non_exhaustive()
    }
}
impl EmittedSourceLoopGraphV2 {
    pub(in crate::mir) fn prepare_phi_completion_v2<'session>(&self, builder: &mut MirBuilder,
        source: &'session SourceLoopCondPhysicalInputV1<'_, '_>, state: &CallableSemanticLoweringState,
    ) -> Result<crate::mir::builder::normal_callable_loop_source_facts::PreparedSourceLoopPhiCompletionV2<'session>, String> {
        let result = (|| {
            if source.checked_source_recipe_v2().is_none_or(|product| !Rc::ptr_eq(product, &self.product)) {
                return Err(reject("finish-source-product"));
            }
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
            let claims = state.ordinary_new_claim_ledger().ok_or_else(|| reject("claims-missing"))?;
            self.verify_control_v2(function)?;
            for effect in &self.effects { effect.packet.check_raw_effect_v2(
                crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims), function)?; }
            let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims);
            for edge in &self.edges {
                if let Some(cleanup) = &edge.cleanup { cleanup.plan.validate_original_emitted_v2(cleanup, context, function)?; }
            }
            let inputs = self.verify_phi_inputs_v2(builder, state)?;
            source.phi_session_v1().prepare_checked_completion_v2(builder, &inputs)
        })();
        result.map_err(|error| source.phi_session_v1().abort_error(builder, error))
    }

    pub(in crate::mir) fn is_prior_binding_v2(&self, row: &(BasicBlockId, MirInstruction)) -> bool {
        self.before.get(&row.0).is_some_and(|(instructions, terminal)| instructions.contains(&row.1) || terminal.as_ref() == Some(&row.1))
    }

    /// Observe appended writes only; source meaning is corroborated separately
    /// by frames/edges and the existing child packets, never by MIR discovery.
    pub(in crate::mir) fn completed_bindings(&self, function: &MirFunction)
        -> Result<Vec<(BasicBlockId, MirInstruction)>, String> {
        let after_frame = self.after_frame.clone();
        let frames = function.blocks.values().flat_map(|block| block.all_instructions())
            .filter(|row| matches!(row, MirInstruction::FaultFrameEnter { .. })).count();
        if frames != usize::from(after_frame.is_some()) || after_frame.as_ref().is_some_and(|(id, expected)|
            function.blocks.get(id).is_none_or(|block| !block.instructions.contains(expected))) {
            return Err(reject("observed-frame-mutated"));
        }
        let new_frame = match (&self.before_frame, after_frame) {
            (None, after) => after,
            (Some(before), Some(after)) if before == &after => None,
            _ => return Err(reject("original-frame-mutated")),
        };
        let mut output = Vec::new();
        for (id, block) in &function.blocks {
            let instructions: Vec<_> = block.instructions.iter().filter(|row|
                new_frame.as_ref().is_none_or(|(frame_block, frame)| frame_block != id || *row != frame)).collect();
            let start = if let Some((before, terminator)) = self.before.get(id) {
                if instructions.len() < before.len() || instructions[..before.len()].iter().copied().ne(before.iter())
                    || terminator.as_ref().is_some_and(|old| block.terminator.as_ref() != Some(old)) {
                    return Err(reject("original-prefix-mutated"));
                }
                before.len()
            } else { 0 };
            if let Some(frame) = new_frame.as_ref().filter(|(block, _)| block == id) { output.push(frame.clone()); }
            output.extend(instructions[start..].iter().map(|row| (*id, (**row).clone())));
            if self.before.get(id).is_none_or(|row| row.1.is_none()) {
                output.extend(block.terminator.iter().cloned().map(|row| (*id, row)));
            }
        }
        if self.before.keys().any(|id| !function.blocks.contains_key(id)) { return Err(reject("original-block-missing")); }
        Ok(output)
    }
}

pub(in crate::mir) fn emit(builder: &mut MirBuilder, port: &mut RawInvocationChildPortV1<'_, '_>,
    source: &SourceLoopCondPhysicalInputV1<'_, '_>,
) -> Result<(EmittedSourceLoopGraphV2, SourceLoopEntryStateV2), String> {
    source.validate_for_source_port()?;
    let product = Rc::clone(source.checked_source_recipe_v2().ok_or_else(|| reject("source-product-missing"))?);
    super::source_loop_frames::input(port, &product)?;
    let ledger = port.callable_ledger.clone().ok_or_else(|| reject("state-missing"))?;
    let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
    let before_frame = ledger.borrow().observed_fault_frame_binding_v2(function)?;
    let before = function.blocks.iter().map(|(id, block)| (*id, (block.instructions.clone(), block.terminator.clone()))).collect();
    let entry_values = ledger.borrow().source_values_snapshot();
    let entry_cache = builder.snapshot_local_binding_state();
    let header = builder.next_block_id();
    let body = builder.next_block_id();
    let false_exit = builder.next_block_id();
    let after = builder.next_block_id();
    source.prepare_checked_phis_v2(builder, header, after)?;
    let mut logical = BTreeMap::new();
    let mut values = BTreeMap::new();
    let recipe = product.recipe.as_recipe();
    for carrier in &recipe.carriers {
        let (&binding, _) = product.bindings.iter().find(|(_, key)| **key == carrier.binding)
            .ok_or_else(|| reject("carrier-source-binding"))?;
        logical.insert(carrier.binding, carrier.entry_value);
        values.insert(carrier.entry_value, source.checked_header_point_v2(binding)?.relation().values().0);
    }
    for input in &recipe.read_only_inputs {
        let (&binding, _) = product.bindings.iter().find(|(_, key)| **key == input.binding)
            .ok_or_else(|| reject("readonly-source-binding"))?;
        logical.insert(input.binding, input.entry_value);
        values.insert(input.entry_value, ledger.borrow().value_for_exact_binding(source.owner(), binding)
            .map_err(|error| error.to_string())?);
    }
    let body_key = recipe.loops.first().ok_or_else(|| reject("root-loop-missing"))?.body;
    let mut driver = Driver { port, source, product: Rc::clone(&product), ledger, header, after,
        logical, values, returns: Vec::new(), edges: Vec::new(), frames: Vec::new(), effects: Vec::new(), forks: Vec::new(), blocks: Vec::new() };
    driver.edge(builder, Edge::LoopEntry, None)?;
    for &binding in product.source.headers.keys() {
        let name = driver.ledger.borrow().source_loop_binding_name(binding,
            &builder.function_state.variable_ctx.variable_map)?.to_owned();
        driver.ledger.borrow_mut().publish_source_loop_point_v2(builder, &source.checked_header_point_v2(binding)?)?;
        let value = driver.ledger.borrow().value_for_exact_binding(source.owner(), binding).map_err(|error| error.to_string())?;
        builder.function_state.variable_ctx.variable_map.insert(name, value);
    }
    builder.start_new_block(header)?;
    driver.condition(builder, 0, (body, false_exit))?;
    let header_values = driver.ledger.borrow().source_values_snapshot();
    let header_cache = builder.snapshot_local_binding_state();
    builder.start_new_block(false_exit)?;
    driver.edge(builder, Edge::ConditionFalse, None)?;
    driver.ledger.borrow_mut().restore_source_values(header_values);
    builder.restore_local_binding_state(header_cache);
    builder.start_new_block(body)?;
    let normal = driver.scoped_body(builder, body_key)?;
    let mut backedges = product.source.ports.iter().enumerate().filter(|(_, port)| matches!(port.kind, PortKind::Backedge));
    match (normal, backedges.next(), backedges.next()) {
        (true, Some((index, _)), None) => driver.edge(builder, Edge::Backedge, Some(index))?,
        (false, None, None) => {},
        _ => return Err(reject("backedge-coverage")),
    }
    if driver.frames.len() != product.schedule().frames().len()
        || driver.frames.iter().enumerate().any(|(index, row)| row.index != index)
        || driver.edges.iter().filter(|edge| edge.cleanup.is_some()).count() != product.source.ports.len() {
        return Err(reject("schedule-or-port-coverage"));
    }
    builder.start_new_block(after)?;
    let result = crate::mir::builder::emission::constant::emit_void_recorded(builder)?;
    let after_frame = driver.ledger.borrow().observed_fault_frame_binding_v2(
        builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?)?;
    Ok((EmittedSourceLoopGraphV2 { product, header, after, returns: driver.returns, edges: driver.edges, frames: driver.frames, effects: driver.effects,
        forks: driver.forks, blocks: driver.blocks, result, before, before_frame, after_frame },
        SourceLoopEntryStateV2 { values: entry_values, cache: entry_cache }))
}

struct Driver<'a, 'port, 'collector, 'input, 'ledger> {
    port: &'a mut RawInvocationChildPortV1<'port, 'collector>,
    source: &'a SourceLoopCondPhysicalInputV1<'input, 'ledger>,
    product: Rc<Product>,
    ledger: Rc<RefCell<CallableSemanticLoweringState>>,
    header: BasicBlockId,
    after: BasicBlockId,
    logical: BTreeMap<LoopBindingKeyV1, LoopValueKeyV1>,
    values: BTreeMap<LoopValueKeyV1, ValueId>,
    returns: Vec<EmittedLoopReturnV2>,
    edges: Vec<EmittedLoopEdgeV2>,
    frames: Vec<EmittedLoopFrameV2>,
    effects: Vec<EmittedLoopEffectV2>,
    forks: Vec<EmittedLoopForkV2>,
    blocks: Vec<EmittedLoopBlockV2>,
}
impl Driver<'_, '_, '_, '_, '_> {
    fn frame(&mut self, builder: &mut MirBuilder, index: usize, entry: BasicBlockId, value: ValueId,
        condition: Option<Rc<ConditionMaterializationV1>>, targets: Option<(BasicBlockId, BasicBlockId)>,
    ) -> Result<(), String> {
        if self.frames.len() != index { return Err(reject("frame-order")); }
        let frame = &self.product.schedule().frames()[index];
        let canonical = *self.values.entry(frame.result()).or_insert(value);
        let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
        let relation = builder.function_state.local_ssa_correspondence.loan(function, canonical, value)
            .ok_or_else(|| reject("frame-alias-correspondence"))?;
        relation.verify(function)?;
        if let Prefix::BindingRebind { frame: original, .. } = frame.prefix() {
            if let Some(binding) = self.product.bindings.get(&original.target()) {
                if self.product.recipe.as_recipe().carriers.iter().any(|row| row.binding == *binding)
                    && self.logical.insert(*binding, frame.result()).is_none() {
                    return Err(reject("rebind-before-logical-entry"));
                }
            }
        }
        let end = builder.function_state.current_block.ok_or_else(|| reject("frame-end"))?;
        self.frames.push(EmittedLoopFrameV2 { index, entry, end, value, canonical: relation, condition, targets });
        Ok(())
    }
    fn condition(&mut self, builder: &mut MirBuilder, index: usize, targets: (BasicBlockId, BasicBlockId)) -> Result<(), String> {
        let entry = builder.function_state.current_block.ok_or_else(|| reject("condition-entry"))?;
        let materialization = super::source_loop_frames::emit_condition(builder, self.port, &self.product, index, targets)?;
        self.frame(builder, index, entry, materialization.values().0, Some(materialization), Some(targets))
    }
    fn scoped_body(&mut self, builder: &mut MirBuilder, key: LoopBlockKeyV1) -> Result<bool, String> {
        if self.blocks.iter().any(|row| row.key == key) { return Err(reject("schedule-block-repeat")); }
        let entry = builder.function_state.current_block.ok_or_else(|| reject("body-entry"))?;
        let (normal, scope_close) = try_with_lexical_scope_with_receipt_v2(builder, |builder| self.body(builder, key)).map_err(|error| error.to_string())?;
        let end = builder.function_state.current_block.ok_or_else(|| reject("body-end"))?;
        self.blocks.push(EmittedLoopBlockV2 { key, entry, end, normal, scope_close });
        Ok(normal)
    }
    fn body(&mut self, builder: &mut MirBuilder, key: LoopBlockKeyV1) -> Result<bool, String> {
        let steps = self.product.schedule().block(key).ok_or_else(|| reject("schedule-block"))?.to_vec();
        let mut position = 0;
        while position < steps.len() {
            match &steps[position] {
                Step::Call { site, observation, result, .. } => {
                    let entry = builder.function_state.current_block.ok_or_else(|| reject("call-entry"))?;
                    let (packet, actual) = super::source_loop_frames::emit_call(builder, self.port, &self.product, site, observation)?;
                    let result = match (result, actual) {
                        (Some(logical), Some((value, local))) => {
                            // Later source reads name the Local writer's result;
                            // retain initializer -> local separately as evidence.
                            if self.values.insert(*logical, local).is_some() { return Err(reject("call-result-repeat")); }
                            let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
                            let relation = builder.function_state.local_ssa_correspondence.loan(function, value, local)
                                .ok_or_else(|| reject("call-local-correspondence"))?;
                            relation.verify(function)?;
                            Some((*logical, relation))
                        }
                        (None, None) => None,
                        _ => return Err(reject("call-result-source")),
                    };
                    let end = builder.function_state.current_block.ok_or_else(|| reject("call-end"))?;
                    self.effects.push(EmittedLoopEffectV2 { site: site.clone(), entry, end, packet, result });
                    position += 1;
                }
                Step::OwnedArrayAppend { site, observation, .. } => {
                    let entry = builder.function_state.current_block.ok_or_else(|| reject("array-append-entry"))?;
                    let packet = super::source_loop_frames::emit_array_append(builder, self.port, &self.product, site, observation)?;
                    let end = builder.function_state.current_block.ok_or_else(|| reject("array-append-end"))?;
                    self.effects.push(EmittedLoopEffectV2 { site: site.clone(), entry, end, packet, result: None });
                    position += 1;
                }
                Step::Frame(index) => {
                    let prefix = self.product.schedule().frames()[*index].prefix();
                    if matches!(prefix, Prefix::BoolCondition { .. }) {
                        let Some(Step::If { condition, then_block, else_block, joins, .. }) = steps.get(position + 1) else {
                            return Err(reject("condition-without-original-if"));
                        };
                        if condition != index { return Err(reject("if-condition-frame")); }
                        if !self.branch(builder, *index, *then_block, *else_block, joins)? {
                            if position + 2 != steps.len() { return Err(reject("schedule-after-closed-if")); }
                            return Ok(false);
                        }
                        position += 2;
                    } else {
                        let entry = builder.function_state.current_block.ok_or_else(|| reject("binding-entry"))?;
                        let value = super::source_loop_frames::emit_binding(builder, self.port, &self.product, *index)?;
                        self.frame(builder, *index, entry, value, None, None)?;
                        position += 1;
                    }
                }
                Step::Return { source, exit, .. } => {
                    if position + 1 != steps.len() { return Err(reject("schedule-after-exit")); }
                    let crate::mir::loop_recipe_contract::LoopExitKindV2::Return { value: Some(logical) }
                        = self.product.recipe.as_recipe().exits[exit.raw() as usize].kind else { return Err(reject("exit-source-role")); };
                    let entry = builder.function_state.current_block.ok_or_else(|| reject("body-entry"))?;
                    let (value, read, bindings) = super::source_loop_frames::emit_return(builder, self.port, &self.product, source)?;
                    if let Some(previous) = self.values.insert(logical, value) {
                        if previous != value { return Err(reject("call-result-source")); }
                    }
                    let end = builder.function_state.current_block.ok_or_else(|| reject("body-end"))?;
                    let mut terminals = bindings.iter().filter(|(_, row)| matches!(row, MirInstruction::Return { .. }));
                    let terminal = terminals.next().cloned().ok_or_else(|| reject("exit-cursor"))?;
                    if terminals.next().is_some() || terminal.1 != (MirInstruction::Return { value: Some(value) }) {
                        return Err(reject("exit-source-role"));
                    }
                    self.returns.push(EmittedLoopReturnV2 { source: Rc::clone(source), entry, end, value, terminal, read, bindings });
                    return Ok(false);
                }
                Step::Exit { port, site, .. } => {
                    let original = self.product.source.ports.get(*port).ok_or_else(|| reject("exit-port"))?;
                    let edge = match &original.kind {
                        PortKind::Break(original) if original == site => Edge::Break(site.clone()),
                        PortKind::Continue(original) if original == site => Edge::Continue(site.clone()),
                        _ => return Err(reject("exit-source-role")),
                    };
                    if position + 1 != steps.len() { return Err(reject("schedule-after-exit")); }
                    self.edge(builder, edge, Some(*port))?;
                    return Ok(false);
                }
                Step::If { .. } => return Err(reject("if-without-condition-frame")),
            }
        }
        Ok(true)
    }
    fn branch(&mut self, builder: &mut MirBuilder, condition: usize, yes: LoopBlockKeyV1, no: Option<LoopBlockKeyV1>,
        source_joins: &[SourceLoopBranchPhiV2],
    )
        -> Result<bool, String> {
        use crate::mir::builder::if_form::IfBranchKindV1;
        let yes_bb = builder.next_block_id();
        let no_bb = builder.next_block_id();
        let join = builder.next_block_id();
        self.condition(builder, condition, (yes_bb, no_bb))?;
        let materialization = self.frames[condition].condition.as_ref().ok_or_else(|| reject("condition-entry"))?;
        let pending = crate::mir::builder::IfConditionMaterializationV1::from_materialization_v1(
            Rc::clone(materialization), (yes_bb, no_bb), join)?;
        let mut observer = super::source_loop_frames::prepare_if_observer(builder, self.port, &self.product, condition)?;
        if let Some(observer) = observer.as_deref_mut() {
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
            observer.before_branches(function, Rc::clone(&pending))?;
            let branch = function.blocks.get(&pending.block()).and_then(|body| body.terminator.clone())
                .ok_or_else(|| reject("condition-entry"))?;
            observer.emitted_branch(function, &pending, (pending.block(), branch))?;
        }
        let entry_values = self.ledger.borrow().source_values_snapshot();
        let no_cache = builder.snapshot_local_binding_state();
        let join_cache = builder.snapshot_local_binding_state();
        let entry_map = builder.function_state.variable_ctx.variable_map.clone();
        let entry_logical = self.logical.clone();
        let entry_canonical_values = self.values.clone();
        let dominating: BTreeSet<_> = entry_canonical_values.keys().copied().collect();
        builder.start_new_block(yes_bb)?;
        if let Some(observer) = observer.as_deref_mut() { observer.enter_branch(IfBranchKindV1::Then)?; }
        let yes_normal = self.scoped_body(builder, yes)?;
        if let Some(observer) = observer.as_deref_mut() { observer.finished_branch(IfBranchKindV1::Then)?; }
        let yes_map = builder.function_state.variable_ctx.variable_map.clone();
        let yes_logical = self.logical.clone();
        let yes_canonical_values = self.values.clone();
        let yes_values = self.ledger.borrow().source_values_snapshot();
        let yes_cache = builder.snapshot_local_binding_state();
        let yes_end = builder.function_state.current_block.ok_or_else(|| reject("then-end"))?;
        if yes_normal { crate::mir::builder::emission::branch::emit_jump(builder, join)?; }
        self.logical = entry_logical.clone();
        self.values = entry_canonical_values.clone();
        self.ledger.borrow_mut().restore_source_values(entry_values);
        builder.restore_local_binding_state(no_cache);
        builder.start_new_block(no_bb)?;
        if let Some(observer) = observer.as_deref_mut() { observer.enter_branch(IfBranchKindV1::Else)?; }
        let no_normal = match no { Some(key) => self.scoped_body(builder, key)?, None => true };
        if let Some(observer) = observer.as_deref_mut() { observer.finished_branch(IfBranchKindV1::Else)?; }
        let no_map = builder.function_state.variable_ctx.variable_map.clone();
        let no_end = builder.function_state.current_block.ok_or_else(|| reject("else-end"))?;
        if no_normal { crate::mir::builder::emission::branch::emit_jump(builder, join)?; }
        match (yes_normal, no_normal) {
            (false, false) => {},
            (true, false) => {
                self.logical = yes_logical;
                self.values = yes_canonical_values;
                self.ledger.borrow_mut().restore_source_values(yes_values);
                builder.restore_local_binding_state(yes_cache);
            }
            (false, true) => {},
            (true, true) => {
                self.logical = entry_logical.clone();
                self.values = entry_canonical_values.clone();
                builder.restore_local_binding_state(join_cache);
            }
        }
        let mut joins = Vec::new();
        if yes_normal || no_normal {
            builder.suppress_next_entry_pin_copy();
            builder.start_new_block(join)?;
            builder.merge_modified_vars(yes_bb, no_bb, yes_normal.then_some(yes_end), no_normal.then_some(no_end),
                &entry_map, &yes_map, &Some(no_map), None)?;
            if let Some(observer) = observer.as_deref_mut() {
                observer.completed_merge(builder, [yes_normal.then_some(yes_end), no_normal.then_some(no_end)])?;
            }
            let claims = self.port.ordinary_new_claim_ledger.as_ref().ok_or_else(|| reject("claims-missing"))?;
            let site = self.product.schedule().frames()[condition].prefix().statement();
            for source in claims.branch_scalar_sources_v1(self.source.owner(), site) {
                let state = self.ledger.borrow();
                let observation = state.observe_binding_value_v2(source.binding()).map_err(|error| error.to_string())?;
                let point = observation.loan_scalar_join_v1(&crate::mir::resolved_semantics::home_new_prefix::CurrentScalarProducerV1::BranchNormal(source))?;
                let branch = match point.branch() {
                    Some(branch) => branch,
                    None => {
                        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                            eprintln!("[loop-join-debug] branch-point-missing binding={:?} value={:?} normal=({}, {})",
                                point.binding(), point.value(), yes_normal, no_normal);
                        }
                        return Err(reject("if-logical-join-not-canonical"));
                    }
                };
                if !joins.iter().any(|old| Rc::ptr_eq(old, branch)) { joins.push(Rc::clone(branch)); }
            }
            if yes_normal && no_normal {
                if self.logical.values().any(|key| !dominating.contains(key)) {
                    if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                        eprintln!("[loop-join-debug] non-dominating logical={:?} entry={:?}", self.logical, entry_logical);
                    }
                    return Err(reject("if-logical-join-not-canonical"));
                }
                if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                    eprintln!("[loop-join-debug] scheduled={:?} observed={:?} logical={:?}",
                        source_joins.iter().map(|row| (row.source.binding(), row.result, row.binding, row.write_item)).collect::<Vec<_>>(),
                        joins.iter().map(|row| row.source().binding()).collect::<Vec<_>>(), self.logical);
                }
                let mut consumed = Vec::new();
                for scheduled in source_joins {
                    let mut matching = joins.iter().enumerate().filter(|(_, observed)|
                        Rc::ptr_eq(observed.source(), &scheduled.source));
                    let Some((index, observed)) = matching.next() else {
                        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                            eprintln!("[loop-join-debug] scheduled-source-unmatched binding={:?}", scheduled.source.binding());
                        }
                        return Err(reject("if-logical-join-not-canonical"));
                    };
                    if matching.next().is_some() || consumed.contains(&index) {
                        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                            eprintln!("[loop-join-debug] scheduled-source-duplicate binding={:?}", scheduled.source.binding());
                        }
                        return Err(reject("if-logical-join-not-canonical"));
                    }
                    consumed.push(index);
                    let relation = observed.relation();
                    let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
                    relation.verify(function)?;
                    if self.values.insert(scheduled.result, relation.values().0).is_some() {
                        if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                            eprintln!("[loop-join-debug] result-already-present result={:?}", scheduled.result);
                        }
                        return Err(reject("if-logical-join-not-canonical"));
                    }
                    if let Some(binding) = scheduled.binding {
                        let previous = self.logical.insert(binding, scheduled.result)
                            .ok_or_else(|| reject("if-logical-join-not-canonical"))?;
                        if entry_logical.get(&binding) != Some(&previous) {
                            if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                                eprintln!("[loop-join-debug] binding-prior-mismatch binding={binding:?} previous={previous:?} entry={:?}", entry_logical.get(&binding));
                            }
                            return Err(reject("if-logical-join-not-canonical"));
                        }
                    }
                }
            } else if !source_joins.is_empty() {
                if std::env::var_os("NYASH_DEBUG_LOCAL_CALLS").is_some() {
                    eprintln!("[loop-join-debug] joins-with-closed-arm normal=({}, {}) scheduled={}", yes_normal, no_normal, source_joins.len());
                }
                return Err(reject("if-logical-join-not-canonical"));
            }
        }
        self.forks.push(EmittedLoopForkV2 { condition, yes, no, starts: (yes_bb, no_bb), ends: (yes_end, no_end),
            normal: (yes_normal, no_normal), join: (yes_normal || no_normal).then_some(join),
            condition_materialization: pending, joins });
        Ok(yes_normal || no_normal)
    }
    fn edge(&mut self, builder: &mut MirBuilder, source: Edge, port: Option<usize>) -> Result<(), String> {
        if self.edges.iter().any(|row| row.source == source) { return Err(reject("source-edge-repeat")); }
        let target = match source { Edge::LoopEntry | Edge::Continue(_) | Edge::Backedge => self.header,
            Edge::ConditionFalse | Edge::Break(_) => self.after, _ => return Err(reject("source-edge-role")) };
        let cleanup = if let Some(index) = port {
            let claims = self.port.ordinary_new_claim_ledger.as_ref().ok_or_else(|| reject("claims-missing"))?;
            let plan = claims.prepare_source_loop_port_cleanup_v2(&self.product, index)?;
            Some(source_loop_cleanup::emit(builder, &mut self.ledger.borrow_mut(), plan)?)
        } else { None };
        let predecessor = builder.function_state.current_block.ok_or_else(|| reject("edge-block"))?;
        let mut values = Vec::new();
        let incoming = self.product.source.incoming.as_ref().map_err(|reason| reject(reason))?;
        let claims = self.port.ordinary_new_claim_ledger.as_ref().ok_or_else(|| reject("claims-missing"))?;
        let context = crate::mir::normal_callable_semantic_package::CallValidationContextV1::Lowering(claims);
        for row in incoming.iter() {
            let binding = row.basis.binding();
            let expected = row.header_incoming.iter().chain(row.after_incoming.iter()).find(|row| row.edge == source)
                .ok_or_else(|| reject("source-edge-carrier-missing"))?;
            let observation = self.ledger.borrow().observe_binding_value_v2(binding).map_err(|error| error.to_string())?;
            let before = observation.binding_value().1;
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
            let dependencies = match &expected.producer {
                crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Scalar(source) =>
                    crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::validate_scalar_binding_v2(
                        binding, source, &observation, context, function)?,
                crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Text(source) =>
                    crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::validate_text_binding_v2(
                        binding, source, &observation, context, function)?,
                crate::mir::resolved_semantics::home_new_prefix::SourceLoopValueProducerV1::Dynamic(source) =>
                    crate::mir::normal_callable_semantic_package::ExpressionReferenceReadV1::validate_dynamic_binding_v2(
                        binding, source, &observation, context, function)?,
            };
            let value = crate::mir::builder::ssa::local::try_ensure(builder, before, crate::mir::builder::ssa::local::LocalKind::Arg)?;
            let function = builder.function_state.current_function.as_ref().ok_or_else(|| reject("function-missing"))?;
            let localized = builder.function_state.local_ssa_correspondence.loan(function, before, value)
                .ok_or_else(|| reject("edge-localization"))?;
            localized.verify(function)?;
            let key = *self.product.bindings.get(&binding).ok_or_else(|| reject("edge-logical-binding"))?;
            let logical = *self.logical.get(&key).ok_or_else(|| reject("edge-logical-value"))?;
            let canonical = if matches!(source, Edge::LoopEntry) { None } else {
                let from = *self.values.get(&logical).ok_or_else(|| reject("edge-canonical-value"))?;
                let relation = builder.function_state.local_ssa_correspondence.loan(function, from, value)
                    .ok_or_else(|| reject("edge-canonical-correspondence"))?;
                relation.verify(function)?;
                Some(relation)
            };
            values.push(EmittedLoopIncomingV2 { binding, logical, observation, dependencies, localized, canonical });
        }
        crate::mir::builder::emission::branch::emit_jump(builder, target)?;
        self.edges.push(EmittedLoopEdgeV2 { source, predecessor, target, cleanup, values });
        Ok(())
    }
}
fn reject(reason: &str) -> String { format!("[freeze:contract][source-loop/driver/{reason}]") }
