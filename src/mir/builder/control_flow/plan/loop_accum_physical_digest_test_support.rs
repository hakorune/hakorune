//! Immutable alpha-digest observer shared by DirectAccum parity adapters.
//!
//! The observer knows only MIR and caller-supplied witnesses.  It deliberately
//! has no recipe, CorePlan, route, Builder mutation, PHI, or SSA authority.

#![cfg(test)]

use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AlphaPhysicalMirDigestV1 {
    pub(crate) cfg: Box<[String]>,
    pub(crate) instructions: Box<[String]>,
    pub(crate) phis: Box<[String]>,
    pub(crate) results: Box<[String]>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct MirRoleWitnessV1 {
    pub(crate) rows: Box<[(String, BasicBlockId)]>,
}

impl MirRoleWitnessV1 {
    pub(crate) fn new(rows: Vec<(impl Into<String>, BasicBlockId)>) -> Result<Self, String> {
        let rows = rows
            .into_iter()
            .map(|(role, block)| (role.into(), block))
            .collect::<Vec<_>>();
        if rows.is_empty() {
            return Err("MIR role witness must not be empty".to_owned());
        }
        if rows
            .windows(2)
            .any(|pair| pair[0].0 == pair[1].0 || pair[0].1 == pair[1].1)
        {
            return Err("MIR role witness contains a duplicate role or block".to_owned());
        }
        Ok(Self {
            rows: rows.into_boxed_slice(),
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AlphaFinalBindingWitnessV1 {
    pub(crate) name: String,
    pub(crate) value: ValueId,
    pub(crate) provenance: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct AlphaFunctionResultWitnessV1 {
    pub(crate) value: Option<ValueId>,
    pub(crate) provenance: String,
    pub(crate) expected_type: MirType,
}

fn role_maps(
    witness: &MirRoleWitnessV1,
) -> (BTreeMap<BasicBlockId, &str>, BTreeMap<&str, BasicBlockId>) {
    let by_block = witness
        .rows
        .iter()
        .map(|(role, block)| (*block, role.as_str()))
        .collect();
    let by_role = witness
        .rows
        .iter()
        .map(|(role, block)| (role.as_str(), *block))
        .collect();
    (by_block, by_role)
}

fn label<'a>(labels: &'a BTreeMap<ValueId, String>, value: ValueId) -> Result<&'a str, String> {
    labels
        .get(&value)
        .map(String::as_str)
        .ok_or_else(|| format!("uncredited MIR value {value:?}"))
}

fn role_for<'a>(
    by_block: &'a BTreeMap<BasicBlockId, &'a str>,
    block: BasicBlockId,
) -> Result<&'a str, String> {
    by_block
        .get(&block)
        .copied()
        .ok_or_else(|| format!("uncredited MIR block {block:?}"))
}

fn instruction_row(
    role: &str,
    instruction: &MirInstruction,
    by_block: &BTreeMap<BasicBlockId, &str>,
    labels: &BTreeMap<ValueId, String>,
) -> Result<String, String> {
    match instruction {
        MirInstruction::Phi { dst, inputs, .. } => {
            let inputs = inputs
                .iter()
                .map(|(block, value)| {
                    Ok(format!(
                        "{}={}",
                        role_for(by_block, *block)?,
                        label(labels, *value)?
                    ))
                })
                .collect::<Result<Vec<_>, String>>()?
                .join(",");
            Ok(format!("{role}:phi:{}=[{inputs}]", label(labels, *dst)?))
        }
        MirInstruction::Const { dst, value } => {
            Ok(format!("{role}:const:{}={value:?}", label(labels, *dst)?))
        }
        MirInstruction::Copy { src, .. } => Ok(format!("{role}:copy:{}", label(labels, *src)?)),
        MirInstruction::BinOp { op, lhs, rhs, .. } => Ok(format!(
            "{role}:bin:{op:?}:{}:{}",
            label(labels, *lhs)?,
            label(labels, *rhs)?
        )),
        MirInstruction::Compare { op, lhs, rhs, .. } => Ok(format!(
            "{role}:compare:{op:?}:{}:{}",
            label(labels, *lhs)?,
            label(labels, *rhs)?
        )),
        MirInstruction::KeepAlive { values } => Ok(format!(
            "{role}:keepalive:{}",
            values
                .iter()
                .map(|value| label(labels, *value))
                .collect::<Result<Vec<_>, String>>()?
                .join(",")
        )),
        other => Err(format!("unexpected DirectAccum MIR instruction: {other:?}")),
    }
}

fn terminator_row(
    role: &str,
    terminator: Option<&MirInstruction>,
    by_block: &BTreeMap<BasicBlockId, &str>,
    labels: &BTreeMap<ValueId, String>,
) -> Result<String, String> {
    match terminator {
        Some(MirInstruction::Branch {
            condition,
            then_bb,
            else_bb,
            ..
        }) => Ok(format!(
            "{role}:branch:{}:{}:{}",
            label(labels, *condition)?,
            role_for(by_block, *then_bb)?,
            role_for(by_block, *else_bb)?
        )),
        Some(MirInstruction::Jump { target, .. }) => {
            Ok(format!("{role}:jump:{}", role_for(by_block, *target)?))
        }
        Some(MirInstruction::Return { value }) => Ok(format!(
            "{role}:return:{}",
            value
                .map(|value| label(labels, value).map(str::to_owned))
                .transpose()?
                .unwrap_or_else(|| "unit".to_owned())
        )),
        None => Ok(format!("{role}:open")),
        other => Err(format!("unexpected DirectAccum MIR terminator: {other:?}")),
    }
}

pub(crate) fn observe_mir(
    function: &MirFunction,
    roles: &MirRoleWitnessV1,
    labels: &BTreeMap<ValueId, String>,
    final_bindings: &[AlphaFinalBindingWitnessV1],
    result: &AlphaFunctionResultWitnessV1,
    value_types: &BTreeMap<ValueId, MirType>,
) -> Result<AlphaPhysicalMirDigestV1, String> {
    let (by_block, by_role) = role_maps(roles);
    let mut cfg = Vec::new();
    let mut instructions = Vec::new();
    let mut phis = Vec::new();
    for (role, block) in roles.rows.iter() {
        let block_data = function
            .blocks
            .get(block)
            .ok_or_else(|| format!("MIR role {role} references missing block {block:?}"))?;
        let predecessors = block_data
            .predecessors
            .iter()
            .map(|pred| role_for(&by_block, *pred))
            .collect::<Result<Vec<_>, String>>()?
            .join(",");
        let successors = block_data
            .successors
            .iter()
            .map(|succ| role_for(&by_block, *succ))
            .collect::<Result<Vec<_>, String>>()?
            .join(",");
        let terminator = terminator_row(role, block_data.terminator.as_ref(), &by_block, labels)?;
        cfg.push(format!(
            "{role}:pred=[{predecessors}]:succ=[{successors}]:{terminator}"
        ));
        for instruction in &block_data.instructions {
            let row = instruction_row(role, instruction, &by_block, labels)
                .map_err(|error| format!("{error} at role={role} instruction={instruction:?}"))?;
            if matches!(instruction, MirInstruction::Phi { .. }) {
                phis.push(row.clone());
            }
            instructions.push(row);
        }
    }

    let mut final_rows = final_bindings.to_vec();
    final_rows.sort_by(|left, right| left.name.cmp(&right.name));
    let mut results = final_rows
        .iter()
        .map(|binding| {
            let ty = value_types.get(&binding.value).ok_or_else(|| {
                format!(
                    "missing type fact for final {}: {:?}",
                    binding.name, binding.value
                )
            })?;
            Ok(format!(
                "final:{}:{}:{ty:?}",
                binding.name, binding.provenance
            ))
        })
        .collect::<Result<Vec<_>, String>>()?;
    if let Some(value) = result.value {
        let actual = value_types
            .get(&value)
            .ok_or_else(|| format!("missing type fact for result {value:?}"))?;
        if *actual != result.expected_type {
            return Err(format!(
                "function result type mismatch: expected {:?}, got {:?}",
                result.expected_type, actual
            ));
        }
        results.push(format!("result:{}:{actual:?}", result.provenance));
    } else {
        results.push(format!(
            "result:{}:{:?}",
            result.provenance, result.expected_type
        ));
    }
    if by_role
        .values()
        .any(|block| !function.blocks.contains_key(block))
    {
        return Err("MIR role witness contains an absent block".to_owned());
    }
    Ok(AlphaPhysicalMirDigestV1 {
        cfg: cfg.into_boxed_slice(),
        instructions: instructions.into_boxed_slice(),
        phis: phis.into_boxed_slice(),
        results: results.into_boxed_slice(),
    })
}
