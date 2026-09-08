//! Borrowed SSA and exact-call lookup for the static v2 frame planner.
//!
//! MIR remains the only operand/CFG graph. This index neither resolves source
//! names nor assigns representation kinds; it rejects ambiguous physical IDs.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::c_transport_v2::MapOperationKind;
use super::PublishedMirBackendView;
use crate::mir::{ConstructionTarget, MirFunction, MirInstruction, ValueId};

pub(super) type ValueKey<'m> = (&'m str, ValueId);
pub(super) type Site<'m> = (&'m str, u32, u32);

#[derive(Debug, Clone, Copy)]
pub(super) enum Producer<'m> {
    Formal(u32),
    Instruction {
        site: Site<'m>,
        instruction: &'m MirInstruction,
    },
}

#[derive(Debug, Clone, Copy)]
pub(super) struct ExactCall<'m> {
    pub target: &'m str,
    pub args: &'m [ValueId],
}

pub(super) struct MapBodyIndex<'m> {
    pub functions: BTreeMap<&'m str, &'m MirFunction>,
    pub values: BTreeMap<ValueKey<'m>, Producer<'m>>,
    pub instructions: BTreeMap<Site<'m>, &'m MirInstruction>,
    pub calls: BTreeMap<Site<'m>, ExactCall<'m>>,
    pub map_operations: BTreeMap<Site<'m>, MapOperationKind>,
}

fn reject(reason: &str, function: &str) -> String {
    format!("[freeze:contract][map-frame/{reason}] function={function}")
}

impl<'m> MapBodyIndex<'m> {
    pub(super) fn from_view(view: &PublishedMirBackendView<'m>) -> Result<Self, String> {
        let mut index = Self {
            functions: BTreeMap::new(),
            values: BTreeMap::new(),
            instructions: BTreeMap::new(),
            calls: BTreeMap::new(),
            map_operations: BTreeMap::new(),
        };
        for (name, function) in &view.module.functions {
            if name.contains('\0') {
                return Err(reject("function-name-nul", name));
            }
            index.functions.insert(name, function);
            for (ordinal, value) in function.params.iter().enumerate() {
                let ordinal =
                    u32::try_from(ordinal).map_err(|_| reject("formal-ordinal-overflow", name))?;
                index.insert_value(name, *value, Producer::Formal(ordinal))?;
            }
            for block_id in function.block_ids() {
                let block = &function.blocks[&block_id];
                for (ordinal, instruction) in block.all_instructions().enumerate() {
                    let ordinal = u32::try_from(ordinal)
                        .map_err(|_| reject("instruction-index-overflow", name))?;
                    let site = (name.as_str(), block_id.as_u32(), ordinal);
                    index.instructions.insert(site, instruction);
                    if let Some(dst) = instruction.dst_value() {
                        index.insert_value(
                            name,
                            dst,
                            Producer::Instruction { site, instruction },
                        )?;
                    }
                    match instruction {
                        MirInstruction::NewBox {
                            target: ConstructionTarget::IntrinsicMap,
                            args,
                            ..
                        } => {
                            if !args.is_empty() {
                                return Err(reject("intrinsic-map-arguments", name));
                            }
                            index
                                .map_operations
                                .insert(site, MapOperationKind::Allocate);
                        }
                        MirInstruction::MapLiteralEntryWrite { .. } => {
                            index.map_operations.insert(site, MapOperationKind::Write);
                        }
                        _ => {}
                    }
                }
            }
        }
        // Canonical keys were validated by the view. The definition table is
        // the relation owner; display names and signature spelling are not.
        for (name, block, instruction, key, args) in view
            .static_method_calls
            .iter()
            .map(|call| {
                (
                    call.function_name,
                    call.block_id,
                    call.instruction_index,
                    call.key,
                    call.args,
                )
            })
            .chain(view.free_function_calls.iter().map(|call| {
                (
                    call.function_name,
                    call.block_id,
                    call.instruction_index,
                    call.key,
                    call.args,
                )
            }))
        {
            let target = view
                .module
                .canonical_callable_definition_symbol(key)
                .ok_or_else(|| reject("call-definition-missing", name))?;
            let function = index
                .functions
                .get(target)
                .ok_or_else(|| reject("call-body-missing", name))?;
            if args.len() != function.params.len() {
                return Err(reject("physical-formal-arity", name));
            }
            let site = (name, block, instruction);
            if !index.instructions.contains_key(&site) {
                return Err(reject("call-site-missing", name));
            }
            if index
                .calls
                .insert(site, ExactCall { target, args })
                .is_some()
            {
                return Err(reject("duplicate-call-site", name));
            }
        }
        Ok(index)
    }

    /// Collect representation dependencies to a fixed point. This does not
    /// admit the leaf operations: the projection planner must validate every
    /// resulting producer against its existing physical contract.
    pub(super) fn map_value_demands(&self) -> Result<BTreeSet<ValueKey<'m>>, String> {
        let mut pending = VecDeque::new();
        for (site, instruction) in &self.instructions {
            if let MirInstruction::MapLiteralEntryWrite { value, .. } = instruction {
                pending.push_back((site.0, *value));
            }
        }
        let mut demanded = BTreeSet::new();
        while let Some(key) = pending.pop_front() {
            if !demanded.insert(key) {
                continue;
            }
            let mut local = Vec::new();
            match self.producer(key)? {
                Producer::Formal(ordinal) => {
                    pending.extend(self.incoming_actuals(key.0, ordinal)?);
                }
                Producer::Instruction { instruction, .. } => match instruction {
                    MirInstruction::Copy { src, .. } | MirInstruction::CopyOwned { src, .. } => {
                        local.push(*src);
                    }
                    MirInstruction::Phi { inputs, .. } => {
                        local.extend(inputs.iter().map(|(_, value)| *value));
                    }
                    MirInstruction::Select {
                        then_val, else_val, ..
                    } => {
                        local.extend([*then_val, *else_val]);
                    }
                    MirInstruction::BinOp { lhs, rhs, .. }
                    | MirInstruction::Compare { lhs, rhs, .. } => {
                        local.extend([*lhs, *rhs]);
                    }
                    MirInstruction::UnaryOp { operand, .. } => local.push(*operand),
                    // Constants, allocation and call results are leaves here;
                    // unknown operations are also left for explicit admission.
                    _ => {}
                },
            }
            pending.extend(local.into_iter().map(|value| (key.0, value)));
        }
        Ok(demanded)
    }

    fn insert_value(
        &mut self,
        function: &'m str,
        value: ValueId,
        producer: Producer<'m>,
    ) -> Result<(), String> {
        if value == ValueId::INVALID {
            return Err(reject("invalid-value-id", function));
        }
        if self.values.insert((function, value), producer).is_some() {
            return Err(reject("duplicate-value-definition", function));
        }
        Ok(())
    }

    pub(super) fn producer(&self, key: ValueKey<'m>) -> Result<Producer<'m>, String> {
        self.values
            .get(&key)
            .copied()
            .ok_or_else(|| reject("value-definition-missing", key.0))
    }

    /// Supplied PHI predecessors and Select arms are read from MIR by the
    /// planner. Incoming actuals are similarly borrowed from the exact call.
    pub(super) fn incoming_actuals(
        &self,
        function: &'m str,
        ordinal: u32,
    ) -> Result<Vec<ValueKey<'m>>, String> {
        if self
            .functions
            .get(function)
            .and_then(|body| body.params.get(ordinal as usize))
            .is_none()
        {
            return Err(reject("formal-ordinal-missing", function));
        }
        Ok(self
            .calls
            .iter()
            .filter_map(|(site, call)| {
                (call.target == function).then(|| (site.0, call.args[ordinal as usize]))
            })
            .collect())
    }
}

#[cfg(test)]
#[path = "map_body_index_tests.rs"]
mod tests;
