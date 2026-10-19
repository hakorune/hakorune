//! Corroborate the SAME emitted schedule's forks and normal joins.
//! Child expression/source validation and whole physical closure remain separate.
use super::source_loop_driver::{EmittedSourceLoopGraphV2, EmittedLoopBlockV2};
use crate::mir::loop_recipe_contract::{LoopBlockKeyV1, SourceLoopScheduledStepV2 as Step};
use crate::mir::resolved_semantics::home_new_prefix::LifecyclePrefixSourceV1 as Prefix;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};
use std::collections::BTreeSet;

impl EmittedSourceLoopGraphV2 {
    pub(in crate::mir) fn verify_control_v2(&self, function: &MirFunction) -> Result<(), String> {
        if self.result.original().0 != self.after || !matches!(self.result.original().1,
            MirInstruction::Const { value: crate::mir::ConstValue::Void, .. }) { return Err(reject("loop-result")); }
        if self.returns.len() != self.product.schedule().returns_v2().count()
            || self.returns.iter().any(|row| self.product.schedule().returns_v2()
                .filter(|source| std::rc::Rc::ptr_eq(source, &row.source)).count() != 1)
            || self.returns.iter().enumerate().any(|(index, row)| self.returns[..index].iter()
                .any(|old| std::rc::Rc::ptr_eq(&old.source, &row.source))) { return Err(reject("exit-outcome")); }
        if self.frames.len() != self.product.schedule().frames().len()
            || self.effects.len() != self.product.schedule().blocks_array_appends_v2().count() + self.product.schedule().calls_v2().count()
            || self.frames.iter().enumerate().any(|(index, frame)| frame.index != index) {
            return Err(reject("frame-census"));
        }
        for (frame, source) in self.frames.iter().zip(self.product.schedule().frames()) {
            frame.canonical.verify(function)?;
            if frame.canonical.values().1 != frame.value { return Err(reject("frame-result")); }
            match (source.prefix(), &frame.condition, frame.targets) {
                (Prefix::BoolCondition { .. }, Some(condition), Some((yes, no))) => {
                    condition.relation().verify(function)?;
                    if condition.values().0 != frame.value || yes == no
                        || !matches!(function.blocks.get(&condition.block()).and_then(|block| block.terminator.as_ref()),
                            Some(MirInstruction::Branch { condition: value, then_bb, else_bb,
                                then_edge_args: Some(a), else_edge_args: Some(b) })
                            if *value == condition.values().1 && (*then_bb, *else_bb) == (yes, no)
                                && a.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && a.values.is_empty()
                                && b.layout == crate::mir::edge_args::JumpArgsLayout::CarriersOnly && b.values.is_empty()) {
                        return Err(reject("condition-branch"));
                    }
                }
                (Prefix::InitializerExpression { .. } | Prefix::BindingRebind { .. }, None, None) => {},
                _ => return Err(reject("frame-purpose")),
            }
        }
        let root = self.product.recipe.as_recipe().loops.first().ok_or_else(|| reject("root-loop"))?;
        let (_, targets) = self.frames.first().and_then(|row| row.condition.as_ref().zip(row.targets))
            .ok_or_else(|| reject("header-condition"))?;
        if self.frames[0].entry != self.header {
            return Err(reject("header-condition-block"));
        }
        let false_edge = self.edges.iter().find(|row| matches!(row.source,
            crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::ConditionFalse))
            .ok_or_else(|| reject("false-edge"))?;
        if false_edge.predecessor != targets.1 { return Err(reject("false-condition-target")); }
        let mut seen = BTreeSet::new();
        let mut forks = BTreeSet::new();
        self.verify_control_block_v2(root.body, targets.0, function, &mut seen, &mut forks)?;
        let body = self.control_block_v2(root.body)?;
        let backedges = self.edges.iter().filter(|row| matches!(row.source,
            crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::Backedge)).collect::<Vec<_>>();
        if body.normal {
            let [edge] = backedges.as_slice() else { return Err(reject("backedge-census")); };
            if edge.cleanup.as_ref().is_none_or(|cleanup| cleanup.from != body.end) { return Err(reject("body-backedge")); }
        } else if !backedges.is_empty() { return Err(reject("terminated-backedge")); }
        if seen.len() != self.blocks.len() || forks.len() != self.forks.len() {
            return Err(reject("control-census"));
        }
        Ok(())
    }
    fn control_block_v2(&self, key: LoopBlockKeyV1) -> Result<&EmittedLoopBlockV2, String> {
        let mut rows = self.blocks.iter().filter(|row| row.key == key);
        let row = rows.next().ok_or_else(|| reject("block-missing"))?;
        if rows.next().is_some() { return Err(reject("block-repeat")); }
        Ok(row)
    }
    fn verify_control_block_v2(&self, key: LoopBlockKeyV1, entry: BasicBlockId, function: &MirFunction,
        seen: &mut BTreeSet<LoopBlockKeyV1>, forks: &mut BTreeSet<usize>,
    ) -> Result<(), String> {
        if !seen.insert(key) { return Err(reject("block-repeat")); }
        let block = self.control_block_v2(key)?;
        if block.entry != entry || !function.blocks.contains_key(&block.entry)
            || !function.blocks.contains_key(&block.end) { return Err(reject("block-coordinates")); }
        if let Some((index, (id, instruction))) = block.scope_close.occurrence() {
            if *id != block.end || !block.normal || function.blocks[id].instructions.get(index) != Some(instruction) {
                return Err(reject("scope-close"));
            }
        }
        let steps = self.product.schedule().block(key).ok_or_else(|| reject("schedule-block"))?;
        let mut cursor = entry;
        for step in steps {
            if let Step::Call { site, observation, result, .. } = step {
                let mut rows = self.effects.iter().filter(|row| &row.site == site);
                let row = rows.next().ok_or_else(|| reject("call-missing"))?;
                if rows.next().is_some() || row.entry != cursor
                    || row.packet.site() != observation.site()
                    || row.result.as_ref().map(|(key, _)| key) != result.as_ref() {
                    return Err(reject("call-sequence"));
                }
                if let Some((_, relation)) = &row.result { relation.verify(function)?; }
                // The packet owner validates the original continuation and
                // ordered arguments in both Lowering and final projection.
                cursor = row.end;
            }
            if let Step::OwnedArrayAppend { site, observation, .. } = step {
                let mut rows = self.effects.iter().filter(|row| &row.site == site);
                let row = rows.next().ok_or_else(|| reject("array-append-missing"))?;
                let crate::mir::resolved_semantics::home_new_prefix::NamedArrayHomeObservationV1::Append {
                    source, call, candidate, ..
                } = observation else { return Err(reject("array-append-source")); };
                let packet = row.packet.owned_array_append().ok_or_else(|| reject("array-append-source"))?;
                let continuation = packet.normal_continuation_binding();
                if rows.next().is_some() || row.result.is_some() || row.entry != cursor || packet.source().site() != call
                    || !packet.source().matches_original_v1(source, candidate)
                    || !matches!(&continuation.1, MirInstruction::Jump { target, edge_args: None } if *target == row.end)
                    || function.blocks.get(&continuation.0).and_then(|block| block.terminator.as_ref()) != Some(&continuation.1)
                    || !crate::mir::verification::utils::compute_dominators(function)
                        .dominates(row.entry, packet.invoke_binding().0)
                    || !crate::mir::verification::utils::compute_dominators(function)
                        .dominates(row.entry, packet.candidate_entry_binding().0) {
                    return Err(reject("array-append-sequence"));
                }
                cursor = row.end;
            }
            if let Step::Frame(index) = step {
                let frame = self.frames.get(*index).ok_or_else(|| reject("block-frame"))?;
                if frame.entry != cursor { return Err(reject("frame-sequence")); }
                cursor = frame.end;
            }
            if let Step::If { site, condition, then_block, else_block, joins, .. } = step {
                let mut rows = self.forks.iter().filter(|row| row.condition == *condition);
                let row = rows.next().ok_or_else(|| reject("fork-missing"))?;
                if rows.next().is_some() || !forks.insert(*condition)
                    || row.yes != *then_block || row.no != *else_block
                    || self.frames.get(*condition).and_then(|row| row.targets) != Some(row.starts)
                    || row.join.is_some() != (row.normal.0 || row.normal.1) {
                    return Err(reject("fork-source"));
                }
                if self.frames[*condition].end != cursor { return Err(reject("fork-entry")); }
                if self.frames[*condition].condition.as_ref().is_none_or(|frame|
                    !std::rc::Rc::ptr_eq(frame, row.condition_materialization.materialization()))
                    || row.condition_materialization.arms() != row.starts
                    || row.join.is_some_and(|join| join != row.condition_materialization.merge())
                    || joins.len() != row.joins.len()
                    || joins.iter().any(|expected| row.joins.iter()
                        .filter(|actual| actual.source() == &expected.source).count() != 1)
                    || row.joins.iter().any(|point| !std::rc::Rc::ptr_eq(point.pending(), &row.condition_materialization)
                        || point.source().site() != site || !point.source().corroborates_original()) {
                    return Err(reject("fork-source"));
                }
                cursor = row.join.unwrap_or(row.ends.1);
                self.verify_control_block_v2(row.yes, row.starts.0, function, seen, forks)?;
                let yes = self.control_block_v2(row.yes)?;
                if (yes.end, yes.normal) != (row.ends.0, row.normal.0) { return Err(reject("then-end")); }
                match row.no {
                    Some(no) => {
                        self.verify_control_block_v2(no, row.starts.1, function, seen, forks)?;
                        let no = self.control_block_v2(no)?;
                        if (no.end, no.normal) != (row.ends.1, row.normal.1) { return Err(reject("else-end")); }
                    }
                    None if row.starts.1 == row.ends.1 && row.normal.1 => {},
                    _ => return Err(reject("implicit-else")),
                }
                for (end, normal) in [(row.ends.0, row.normal.0), (row.ends.1, row.normal.1)] {
                    if normal && function.blocks.get(&end).and_then(|block| block.terminator.as_ref())
                        != row.join.as_ref().map(|target| MirInstruction::Jump { target: *target, edge_args: None }).as_ref() {
                        return Err(reject("normal-join"));
                    }
                }
                if let Some(join) = row.join {
                    if row.starts.0 == row.starts.1 || [row.starts.0, row.starts.1, row.ends.0, row.ends.1].contains(&join)
                        || !function.blocks.contains_key(&join) { return Err(reject("join-coordinates")); }
                }
            }
            if let Step::Return { source, .. } = step {
                let mut rows = self.returns.iter().filter(|row| std::rc::Rc::ptr_eq(&row.source, source));
                let row = rows.next().ok_or_else(|| reject("exit-cursor"))?;
                if rows.next().is_some() || row.entry != cursor
                    || row.terminal.1 != (MirInstruction::Return { value: Some(row.value) })
                    || function.blocks.get(&row.terminal.0).and_then(|block| block.terminator.as_ref()) != Some(&row.terminal.1)
                    || row.end != row.entry || !row.bindings.last().is_some_and(|binding|
                        binding.0 == row.end && matches!(binding.1, MirInstruction::Jump { edge_args: None, .. })
                            && function.blocks.get(&row.end).and_then(|block| block.terminator.as_ref()) == Some(&binding.1)) {
                    return Err(reject("exit-entry"));
                }
                cursor = row.end;
            }
            if let Step::Exit { port, .. } = step {
                let edge = self.edges.iter().find(|row| row.cleanup.as_ref().is_some_and(|cleanup| cleanup.plan.port() == *port))
                    .ok_or_else(|| reject("exit-cursor"))?;
                if edge.cleanup.as_ref().unwrap().from != cursor { return Err(reject("exit-entry")); }
                cursor = edge.predecessor;
            }
        }
        if block.end != cursor { return Err(reject("block-sequence-end")); }
        match steps.last() {
            Some(Step::Return { source, .. }) => {
                if block.normal || self.returns.iter().filter(|row| std::rc::Rc::ptr_eq(&row.source, source)
                    && row.end == block.end).count() != 1 { return Err(reject("exit-outcome")); }
            }
            Some(Step::Exit { site, port, .. }) => {
                let mut edges = self.edges.iter().filter(|edge| match &edge.source {
                    crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::Break(original)
                    | crate::mir::resolved_semantics::source_value_flow::SourceValueEdgeV1::Continue(original) => original == site,
                    _ => false,
                });
                let edge = edges.next().ok_or_else(|| reject("exit-edge"))?;
                if edges.next().is_some() || block.normal || edge.predecessor != block.end
                    || edge.cleanup.as_ref().is_none_or(|cleanup| cleanup.plan.port() != *port) {
                    return Err(reject("exit-outcome"));
                }
            }
            Some(Step::If { condition, .. }) => {
                let fork = self.forks.iter().find(|row| row.condition == *condition).ok_or_else(|| reject("final-fork"))?;
                if block.normal != fork.join.is_some()
                    || block.end != fork.join.unwrap_or(fork.ends.1) { return Err(reject("final-fork-outcome")); }
            }
            _ if !block.normal => return Err(reject("normal-outcome")),
            _ => {},
        }
        Ok(())
    }
}
fn reject(reason: &str) -> String { format!("[freeze:contract][source-loop/control/{reason}]") }
