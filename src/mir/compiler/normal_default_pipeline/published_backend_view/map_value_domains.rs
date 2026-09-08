//! Physical representation closure over the frame's borrowed body index.
//!
//! These observations do not admit producers or change formal ABI. In particular,
//! a provisional Bool result does not prove that its comparison inputs are valid.
use std::collections::{BTreeMap, BTreeSet};

use super::c_transport_v2::PhysicalOperation;
use super::map_body_index::{MapBodyIndex, Producer, ValueKey};
use crate::mir::{BinaryOp, ConstValue, ConstructionTarget, MirInstruction, UnaryOp};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub(super) enum ValueDomain {
    I64,
    Bool,
    F64,
    Void,
    String,
    Handle,
    /// A missing physical contract must never disappear as an empty union.
    Unresolved,
}

type Domains<'m> = BTreeMap<ValueKey<'m>, BTreeSet<ValueDomain>>;

impl<'m> MapBodyIndex<'m> {
    pub(super) fn map_value_domains(&self) -> Result<Domains<'m>, String> {
        let mut domains: Domains<'m> = self
            .map_value_demands()?
            .into_iter()
            .map(|key| (key, BTreeSet::new()))
            .collect();
        self.close_domains(&mut domains)?;
        // Empty means no seed reached this SCC, not an admitted unknown kind.
        // A seeded loop has already converged before this step.
        for domain in domains.values_mut().filter(|domain| domain.is_empty()) {
            domain.insert(ValueDomain::Unresolved);
        }
        self.close_domains(&mut domains)?;
        Ok(domains)
    }

    fn close_domains(&self, domains: &mut Domains<'m>) -> Result<(), String> {
        loop {
            let mut changed = false;
            for key in domains.keys().copied().collect::<Vec<_>>() {
                let next = self.producer_domains(key, domains)?;
                let entry = domains.get_mut(&key).unwrap();
                let before = entry.len();
                entry.extend(next);
                changed |= before != entry.len();
            }
            if !changed {
                return Ok(());
            }
        }
    }

    fn producer_domains(
        &self,
        key: ValueKey<'m>,
        domains: &Domains<'m>,
    ) -> Result<BTreeSet<ValueDomain>, String> {
        use ValueDomain::*;
        let one = |domain| BTreeSet::from([domain]);
        let local = |value| {
            domains
                .get(&(key.0, value))
                .cloned()
                .unwrap_or_else(|| one(Unresolved))
        };
        Ok(match self.producer(key)? {
            Producer::Formal(ordinal) => {
                let actuals = self.incoming_actuals(key.0, ordinal)?;
                if actuals.is_empty() {
                    one(Unresolved)
                } else {
                    actuals
                        .into_iter()
                        .flat_map(|actual| {
                            domains
                                .get(&actual)
                                .cloned()
                                .unwrap_or_else(|| one(Unresolved))
                        })
                        .collect()
                }
            }
            Producer::Instruction { site, instruction } => match instruction {
                MirInstruction::Const { value, .. } => one(match value {
                    ConstValue::Integer(_) => I64,
                    ConstValue::Bool(_) => Bool,
                    ConstValue::Float(_) => F64,
                    ConstValue::String(_) => String,
                    ConstValue::Null | ConstValue::Void => Void,
                }),
                MirInstruction::NewBox {
                    target: ConstructionTarget::IntrinsicMap | ConstructionTarget::IntrinsicArray,
                    ..
                } => one(Handle),
                MirInstruction::Copy { src, .. } => local(*src),
                MirInstruction::Phi { inputs, .. } => {
                    if inputs.is_empty() {
                        one(Unresolved)
                    } else {
                        inputs.iter().flat_map(|(_, value)| local(*value)).collect()
                    }
                }
                MirInstruction::Select {
                    then_val, else_val, ..
                } => local(*then_val).union(&local(*else_val)).copied().collect(),
                MirInstruction::Compare { .. }
                | MirInstruction::UnaryOp {
                    op: UnaryOp::Not, ..
                } => one(Bool),
                MirInstruction::BinOp { op, lhs, rhs, .. } => {
                    let lhs = local(*lhs);
                    let rhs = local(*rhs);
                    lhs.iter()
                        .flat_map(|left| {
                            rhs.iter().map(move |right| {
                                match binary_operation(*op, *left, *right) {
                                    Some(PhysicalOperation::I64Binary) => I64,
                                    Some(PhysicalOperation::StringConcat) => String,
                                    _ => Unresolved,
                                }
                            })
                        })
                        .collect()
                }
                MirInstruction::Call(_) if self.calls.contains_key(&site) => one(I64),
                // Named allocation needs actual selected allocation admission,
                // not a matching typed plan or a builtin-looking name. Boxed sums
                // need exact site/ABI binding. CopyOwned keeps its capability Stop.
                _ => one(Unresolved),
            },
        })
    }

    /// Selection cannot be called with a provisional or caller-supplied domain map.
    /// This does not validate unrelated leaf producers or close external ingress.
    pub(super) fn map_physical_operations(
        &self,
    ) -> Result<BTreeMap<ValueKey<'m>, PhysicalOperation>, String> {
        let domains = self.map_value_domains()?;
        let mut selected = BTreeMap::new();
        for &key in domains.keys() {
            if let Producer::Instruction { instruction, .. } = self.producer(key)? {
                if matches!(
                    instruction,
                    MirInstruction::BinOp { .. }
                        | MirInstruction::Compare { .. }
                        | MirInstruction::UnaryOp { .. }
                ) {
                    selected.insert(key, self.map_physical_operation(key, &domains)?);
                }
            }
        }
        Ok(selected)
    }

    fn map_physical_operation(
        &self,
        key: ValueKey<'m>,
        domains: &Domains<'m>,
    ) -> Result<PhysicalOperation, String> {
        use PhysicalOperation as Op;
        use ValueDomain as D;
        let single = |value| {
            let domain = domains.get(&(key.0, value))?;
            (domain.len() == 1).then(|| *domain.first().unwrap())
        };
        let operation = match self.producer(key)? {
            Producer::Instruction { instruction, .. } => match instruction {
                MirInstruction::BinOp { op, lhs, rhs, .. } => single(*lhs)
                    .zip(single(*rhs))
                    .and_then(|(left, right)| binary_operation(*op, left, right)),
                MirInstruction::Compare { lhs, rhs, .. } => match (single(*lhs), single(*rhs)) {
                    (Some(D::I64), Some(D::I64)) => Some(Op::I64Compare),
                    (Some(D::Bool), Some(D::Bool)) => Some(Op::BoolCompare),
                    (Some(D::String), Some(D::String)) => Some(Op::StringCompare),
                    _ => None,
                },
                MirInstruction::UnaryOp {
                    op: UnaryOp::Not,
                    operand,
                    ..
                } => match single(*operand) {
                    Some(D::I64) => Some(Op::I64Not),
                    Some(D::Bool) => Some(Op::BoolNot),
                    _ => None,
                },
                _ => None,
            },
            Producer::Formal(_) => None,
        };
        operation.ok_or_else(|| {
            format!(
                "[freeze:contract][map-frame/operation-input-domain] function={} value={}",
                key.0,
                key.1.as_u32(),
            )
        })
    }
}

// One admitted binary domain/opcode mapping serves propagation and selection.
fn binary_operation(op: BinaryOp, lhs: ValueDomain, rhs: ValueDomain) -> Option<PhysicalOperation> {
    use ValueDomain as D;
    match (op, lhs, rhs) {
        (BinaryOp::Add, D::String, D::String) => Some(PhysicalOperation::StringConcat),
        (
            BinaryOp::Add | BinaryOp::Sub | BinaryOp::Mul | BinaryOp::Div | BinaryOp::Mod,
            D::I64,
            D::I64,
        ) => Some(PhysicalOperation::I64Binary),
        _ => None,
    }
}

#[cfg(test)]
#[path = "map_value_domains_tests.rs"]
mod tests;
