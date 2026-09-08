//! Original-lane demand over the same borrowed graph as Map projection.
//!
//! This is physical use propagation, not operation or source admission. A Map
//! formal never gains an original lane merely because another formal has one.
use std::collections::{BTreeMap, BTreeSet, VecDeque};

use super::c_transport_v2::ProjectionAction;
use super::map_body_index::{MapBodyIndex, Producer, ValueKey};
use crate::mir::{ConstructionTarget, MirInstruction};

impl<'m> MapBodyIndex<'m> {
    pub(super) fn original_value_demands(
        &self,
        actions: &BTreeMap<ValueKey<'m>, ProjectionAction>,
    ) -> Result<BTreeSet<ValueKey<'m>>, String> {
        let map = self.map_value_demands()?;
        if map != actions.keys().copied().collect() {
            return Err("[freeze:contract][map-frame/projection-coverage-mismatch]".into());
        }
        let mut pending = VecDeque::new();
        for (site, instruction) in &self.instructions {
            if let Some(call) = self.calls.get(site) {
                // Undemanded formals keep their existing ABI. Demanded formals
                // propagate original needs from their body, never from the
                // first caller or the generic MIR parameter type.
                let formals = &self.functions[call.target].params;
                for (&formal, &actual) in formals.iter().zip(call.args) {
                    if !map.contains(&(call.target, formal)) {
                        pending.push_back((site.0, actual));
                    }
                }
                continue;
            }
            match instruction {
                MirInstruction::MapLiteralEntryWrite { receiver, key, .. } => {
                    pending.extend([(site.0, *receiver), (site.0, *key)]);
                }
                MirInstruction::Copy { dst, .. } | MirInstruction::Phi { dst, .. }
                    if map.contains(&(site.0, *dst)) => {}
                MirInstruction::Select { dst, cond, .. } if map.contains(&(site.0, *dst)) => {
                    pending.push_back((site.0, *cond));
                }
                MirInstruction::NewBox {
                    dst,
                    target: ConstructionTarget::Named(_),
                    ..
                } if map.contains(&(site.0, *dst)) => {
                    if self.named_alias_operand(*site)?.is_none() {
                        pending.extend(instruction.used_values().into_iter().map(|v| (site.0, v)));
                    }
                }
                MirInstruction::CopyOwned { dst, src } => {
                    // Keep the ownership operation and its original result.
                    // Its physical admission remains independently required.
                    pending.extend([(site.0, *dst), (site.0, *src)]);
                }
                _ => pending.extend(instruction.used_values().into_iter().map(|v| (site.0, v))),
            }
        }
        for (&key, action) in actions {
            if action.requires_original_producer() {
                pending.push_back(key);
            }
        }
        let mut original = BTreeSet::new();
        while let Some(key) = pending.pop_front() {
            if !original.insert(key) {
                continue;
            }
            match self.producer(key)? {
                Producer::Formal(ordinal) => {
                    pending.extend(self.incoming_actuals(key.0, ordinal)?);
                }
                Producer::Instruction { site, instruction } => {
                    // Exact-call actuals are decided by the same formal map
                    // above. A used result does not require unused old lanes.
                    if map.contains(&key)
                        && matches!(
                            instruction,
                            MirInstruction::NewBox {
                                target: ConstructionTarget::Named(_),
                                ..
                            }
                        )
                    {
                        if let Some(source) = self.named_alias_operand(site)? {
                            pending.push_back((key.0, source));
                        } else {
                            pending
                                .extend(instruction.used_values().into_iter().map(|v| (key.0, v)));
                        }
                    } else if !self.calls.contains_key(&site) {
                        pending.extend(instruction.used_values().into_iter().map(|v| (key.0, v)));
                    }
                }
            }
        }
        Ok(original)
    }
}

#[cfg(test)]
#[path = "map_original_demand_tests.rs"]
mod tests;
