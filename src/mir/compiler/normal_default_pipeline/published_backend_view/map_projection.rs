//! One physical leaf mapping for domain closure and complete projection actions.
//! Existing exact site plans supply boxed ABI; this never infers source types.
use std::collections::{BTreeMap, BTreeSet};

use super::c_transport_v2::ProjectionAction as Action;
use super::map_body_index::{MapBodyIndex, Producer, ValueKey};
use super::map_value_domains::ValueDomain as Domain;
use crate::mir::boxed_sum_abi_plan::BoxedSumPayloadStorage as Storage;
use crate::mir::{ConstValue, ConstructionTarget, MirInstruction, MirType};

impl<'m> MapBodyIndex<'m> {
    /// None is a graph/operation or unsupported producer, not an admitted unknown.
    /// Alias domains still follow the sole original operand graph.
    pub(super) fn map_leaf_projection(
        &self,
        key: ValueKey<'m>,
    ) -> Result<Option<(Domain, Action)>, String> {
        let Producer::Instruction { site, instruction } = self.producer(key)? else {
            return Ok(None);
        };
        Ok(match instruction {
            MirInstruction::Const { value, .. } => Some(match value {
                ConstValue::Integer(value) => (Domain::I64, Action::ExactI64(*value)),
                ConstValue::Bool(value) => (Domain::Bool, Action::ExactBool(*value)),
                ConstValue::Float(value) => (Domain::F64, Action::ExactF64(value.to_bits())),
                ConstValue::Null | ConstValue::Void => (Domain::Void, Action::ExactVoid),
                ConstValue::String(_) => (Domain::String, Action::OriginalHandle),
            }),
            MirInstruction::NewBox {
                target: ConstructionTarget::IntrinsicArray | ConstructionTarget::IntrinsicMap,
                ..
            } => Some((Domain::Handle, Action::OriginalHandle)),
            MirInstruction::NewBox {
                target: ConstructionTarget::Named(_),
                ..
            } => match self.named_projection_action(key)? {
                Action::OriginalHandle => Some((Domain::Handle, Action::OriginalHandle)),
                Action::NamedAliasOperandZero => None,
                _ => unreachable!("Named binding has only allocating and alias outcomes"),
            },
            // These are only the view's checked Integer-result canonical calls.
            MirInstruction::Call(_) if self.calls.contains_key(&site) => {
                Some((Domain::I64, Action::OriginalI64))
            }
            MirInstruction::VariantMake { .. } if self.boxed_sum_sites.contains_key(&site) => {
                return Err("[freeze:contract][map-frame/boxed-object-escape-unavailable]".into());
            }
            MirInstruction::VariantTag { .. } if self.boxed_sum_sites.contains_key(&site) => {
                Some((Domain::I64, Action::OriginalI64))
            }
            MirInstruction::VariantProject { payload_type, .. } => {
                match (
                    payload_type.as_ref(),
                    self.boxed_sum_sites
                        .get(&site)
                        .and_then(|plan| plan.payload_storage.as_ref()),
                ) {
                    (Some(MirType::Integer), Some(Storage::I64)) => {
                        Some((Domain::I64, Action::OriginalI64))
                    }
                    (Some(MirType::Bool), Some(Storage::I64)) => {
                        Some((Domain::Bool, Action::OriginalBoolI64))
                    }
                    (Some(MirType::String), Some(Storage::Handle)) => {
                        Some((Domain::String, Action::OriginalHandle))
                    }
                    (
                        Some(
                            MirType::Box(_)
                            | MirType::Array(_)
                            | MirType::Future(_)
                            | MirType::WeakRef,
                        ),
                        Some(Storage::Handle),
                    ) => {
                        return Err(
                            "[freeze:contract][map-frame/boxed-payload-representation-unavailable]"
                                .into(),
                        )
                    }
                    _ => None,
                }
            }
            _ => None,
        })
    }

    /// Complete demanded action map, never a caller-supplied subset. C admission
    /// and execution still must consume each original/materialized producer.
    #[cfg(test)]
    pub(super) fn map_projection_actions(&self) -> Result<BTreeMap<ValueKey<'m>, Action>, String> {
        Ok(self.map_frame_projection()?.0)
    }

    /// One analysis for this bound index; no reuse across Named bindings or graphs.
    pub(super) fn map_frame_projection(
        &self,
    ) -> Result<(BTreeMap<ValueKey<'m>, Action>, BTreeSet<ValueKey<'m>>), String> {
        let demands = self.map_value_demands()?;
        let (domains, operations) = self.map_domains_and_operations(&demands)?;
        let mut actions = BTreeMap::new();
        for &key in &demands {
            let action = if let Some((_, action)) = self.map_leaf_projection(key)? {
                action
            } else {
                match self.producer(key)? {
                    Producer::Formal(ordinal) => Action::Formal(ordinal),
                    Producer::Instruction { instruction, .. } => match instruction {
                        MirInstruction::Copy { .. } => Action::Copy,
                        MirInstruction::Phi { .. } => Action::Phi,
                        MirInstruction::Select { .. } => Action::Select,
                        MirInstruction::NewBox { target: ConstructionTarget::Named(_), .. } => {
                            self.named_projection_action(key)?
                        }
                        MirInstruction::BinOp { .. } | MirInstruction::Compare { .. }
                        | MirInstruction::UnaryOp { .. } => Action::Operation(operations[&key]),
                        _ => return Err(format!(
                            "[freeze:contract][map-frame/producer-projection-unavailable] function={} value={}",
                            key.0, key.1.as_u32())),
                    },
                }
            };
            actions.insert(key, action);
        }
        // A representation cycle with no physical seed cannot gain tags by
        // carrying Formal/Phi rows. Mixed domains remain valid for transport.
        if domains
            .values()
            .any(|set| set.contains(&Domain::Unresolved))
        {
            return Err("[freeze:contract][map-frame/value-domain-unresolved]".into());
        }
        let original = self.original_demands_for_map(&demands, &actions)?;
        for (key, action) in &actions {
            if matches!(action, Action::ExactF64(_)) && original.contains(key) {
                return Err(format!(
                    "[freeze:contract][map-frame/original-float-unavailable] function={} value={}",
                    key.0,
                    key.1.as_u32()
                ));
            }
        }
        Ok((actions, original))
    }
}
