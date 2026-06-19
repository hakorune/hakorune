/*!
 * Narrow string corridor relation layer.
 *
 * This module consumes the generic MIR PHI base-relation seam and records
 * string-corridor continuity as metadata. It does not own PHI semantics, and
 * it does not emit placement/effect candidates itself.
 */

use super::phi_query::{collect_phi_carry_relations, PhiBaseRelation};
use super::string_corridor_recognizer::{
    match_add_in_block, match_len_call, match_substring_call, match_substring_concat3_helper_call,
    string_source_identity,
};
use super::value_origin::{build_value_def_map, resolve_value_origin, ValueDefMap};
use super::{MirFunction, MirInstruction, MirModule, ValueId};
use std::collections::BTreeSet;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorRelationKind {
    PhiCarryBase,
    StableLengthScalar,
}

impl std::fmt::Display for StringCorridorRelationKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PhiCarryBase => f.write_str("phi_carry_base"),
            Self::StableLengthScalar => f.write_str("stable_length_scalar"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum StringCorridorWindowContract {
    PreservePlanWindow,
    StopAtMerge,
}

impl StringCorridorWindowContract {
    pub fn preserves_plan_window(self) -> bool {
        matches!(self, Self::PreservePlanWindow)
    }
}

impl std::fmt::Display for StringCorridorWindowContract {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PreservePlanWindow => f.write_str("preserve_plan_window"),
            Self::StopAtMerge => f.write_str("stop_at_merge"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StringCorridorRelation {
    pub kind: StringCorridorRelationKind,
    pub base_value: ValueId,
    pub window_contract: StringCorridorWindowContract,
    pub witness_value: Option<ValueId>,
    pub reason: &'static str,
}

impl StringCorridorRelation {
    pub fn summary(&self) -> String {
        match self.witness_value {
            Some(witness) => format!(
                "{} base=%{} witness=%{} window={} {}",
                self.kind, self.base_value.0, witness.0, self.window_contract, self.reason
            ),
            None => format!(
                "{} base=%{} window={} {}",
                self.kind, self.base_value.0, self.window_contract, self.reason
            ),
        }
    }
}

fn find_phi_inputs(
    function: &MirFunction,
    phi_value: ValueId,
) -> Option<Vec<(super::BasicBlockId, ValueId)>> {
    for block in function.blocks.values() {
        for inst in &block.instructions {
            if let MirInstruction::Phi { dst, inputs, .. } = inst {
                if *dst == phi_value {
                    return Some(inputs.clone());
                }
            }
        }
    }
    None
}

fn value_is_const_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
    expected: i64,
) -> bool {
    let root = resolve_value_origin(function, &def_map, value);
    let Some((bbid, idx)) = def_map.get(&root).copied() else {
        return false;
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return false;
    };
    matches!(
        block.instructions.get(idx),
        Some(MirInstruction::Const {
            value: super::ConstValue::Integer(actual),
            ..
        }) if *actual == expected
    )
}

fn same_or_same_const_i64(
    function: &MirFunction,
    def_map: &ValueDefMap,
    lhs: ValueId,
    rhs: ValueId,
) -> bool {
    if lhs == rhs {
        return true;
    }

    let lhs_root = resolve_value_origin(function, &def_map, lhs);
    let rhs_root = resolve_value_origin(function, &def_map, rhs);
    if lhs_root == rhs_root {
        return true;
    }

    let Some((lhs_bbid, lhs_idx)) = def_map.get(&lhs_root).copied() else {
        return false;
    };
    let Some((rhs_bbid, rhs_idx)) = def_map.get(&rhs_root).copied() else {
        return false;
    };
    let Some(lhs_block) = function.blocks.get(&lhs_bbid) else {
        return false;
    };
    let Some(rhs_block) = function.blocks.get(&rhs_bbid) else {
        return false;
    };
    matches!(
        (lhs_block.instructions.get(lhs_idx), rhs_block.instructions.get(rhs_idx)),
        (
            Some(MirInstruction::Const {
                value: super::ConstValue::Integer(lhs_val),
                ..
            }),
            Some(MirInstruction::Const {
                value: super::ConstValue::Integer(rhs_val),
                ..
            })
        ) if lhs_val == rhs_val
    )
}

fn entry_length_value_for_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    phi_value: ValueId,
) -> Option<ValueId> {
    let inputs = find_phi_inputs(function, phi_value)?;
    let (entry_bbid, entry_value) = inputs.iter().min_by_key(|(bbid, _)| bbid.0).copied()?;
    let entry_identity = string_source_identity(function, &def_map, entry_value)?;
    let block = function.blocks.get(&entry_bbid)?;

    for inst in &block.instructions {
        let Some((dst, receiver, _effects)) = match_len_call(inst) else {
            continue;
        };
        let Some(receiver_identity) = string_source_identity(function, &def_map, receiver) else {
            continue;
        };
        if receiver_identity == entry_identity {
            return Some(resolve_value_origin(function, &def_map, dst));
        }
    }

    None
}

fn plan_window_preserves_length_value(
    function: &MirFunction,
    def_map: &ValueDefMap,
    start: ValueId,
    end: ValueId,
    length_value: ValueId,
) -> bool {
    let start_root = resolve_value_origin(function, &def_map, start);
    let end_root = resolve_value_origin(function, &def_map, end);
    let length_root = resolve_value_origin(function, &def_map, length_value);

    if end_root == length_root {
        return value_is_const_i64(function, def_map, start_root, 0);
    }

    let Some((end_bbid, _)) = def_map.get(&end_root).copied() else {
        return false;
    };
    let Some(add_shape) = match_add_in_block(function, end_bbid, &def_map, end_root) else {
        return false;
    };
    let lhs_root = resolve_value_origin(function, &def_map, add_shape.lhs);
    let rhs_root = resolve_value_origin(function, &def_map, add_shape.rhs);
    (same_or_same_const_i64(function, def_map, lhs_root, start_root) && rhs_root == length_root)
        || (lhs_root == length_root
            && same_or_same_const_i64(function, def_map, rhs_root, start_root))
}

fn stable_length_relation_for_phi(
    function: &MirFunction,
    def_map: &ValueDefMap,
    phi_value: ValueId,
    base_value: ValueId,
) -> Option<StringCorridorRelation> {
    let length_value = entry_length_value_for_phi(function, def_map, phi_value)?;
    let base_root = resolve_value_origin(function, &def_map, base_value);
    let (bbid, idx) = def_map.get(&base_root).copied()?;
    let block = function.blocks.get(&bbid)?;
    let inst = block.instructions.get(idx)?;
    let (start, end) = if let Some(shape) = match_substring_concat3_helper_call(inst) {
        (shape.start, shape.end)
    } else if let Some((_dst, _receiver, start, end, _effects)) = match_substring_call(inst) {
        (start, end)
    } else {
        return None;
    };
    if !plan_window_preserves_length_value(function, def_map, start, end, length_value) {
        return None;
    }

    Some(StringCorridorRelation {
        kind: StringCorridorRelationKind::StableLengthScalar,
        base_value,
        witness_value: Some(length_value),
        window_contract: StringCorridorWindowContract::StopAtMerge,
        reason:
            "merged phi route keeps the entry scalar source length stable even while the proof-bearing plan window stops at the merge",
    })
}

fn is_raw_substring_view_call(inst: &MirInstruction) -> bool {
    matches!(
        inst,
        MirInstruction::Call {
            callee:
                Some(super::Callee::Method {
                    method,
                    receiver: Some(receiver),
                    ..
                }),
            args,
            ..
        } if args.len() == 3
            && matches!(method.as_str(), "substring" | "slice")
            && args.first().is_some_and(|arg| arg == receiver)
    ) || matches!(
        inst,
        MirInstruction::Call {
            callee: Some(super::Callee::Extern(name)),
            args,
            ..
        } if args.len() == 3 && name == "nyash.string.substring_hii"
    )
}

fn stable_length_relation_for_direct_length_call(
    function: &MirFunction,
    def_map: &ValueDefMap,
    length_value: ValueId,
    receiver_value: ValueId,
) -> Option<StringCorridorRelation> {
    let receiver_root = resolve_value_origin(function, def_map, receiver_value);
    if function
        .metadata
        .string_corridor_relations
        .get(&receiver_root)
        .is_some_and(|relations| {
            relations
                .iter()
                .any(|relation| relation.kind == StringCorridorRelationKind::StableLengthScalar)
        })
    {
        return None;
    }

    let Some((bbid, idx)) = def_map.get(&receiver_root).copied() else {
        return Some(StringCorridorRelation {
            kind: StringCorridorRelationKind::StableLengthScalar,
            base_value: receiver_root,
            witness_value: Some(resolve_value_origin(function, def_map, length_value)),
            window_contract: StringCorridorWindowContract::PreservePlanWindow,
            reason:
                "direct source length call keeps the source scalar length stable without a merge",
        });
    };
    let Some(block) = function.blocks.get(&bbid) else {
        return None;
    };
    let Some(inst) = block.instructions.get(idx) else {
        return None;
    };

    let is_retained_substring_view =
        is_raw_substring_view_call(inst) || match_substring_concat3_helper_call(inst).is_some();
    let is_direct_source = matches!(
        inst,
        MirInstruction::Const {
            value: super::ConstValue::String(_),
            ..
        } | MirInstruction::Copy { .. }
            | MirInstruction::Phi { .. }
    );
    if !is_retained_substring_view && !is_direct_source {
        return None;
    }

    Some(StringCorridorRelation {
        kind: StringCorridorRelationKind::StableLengthScalar,
        base_value: receiver_root,
        witness_value: Some(resolve_value_origin(function, def_map, length_value)),
        window_contract: StringCorridorWindowContract::PreservePlanWindow,
        reason: if is_retained_substring_view {
            "retained substring view length stays stable without a merge"
        } else {
            "direct source length call keeps the source scalar length stable without a merge"
        },
    })
}

fn push_relation_if_absent(
    relations: &mut Vec<StringCorridorRelation>,
    relation: StringCorridorRelation,
) {
    if relations.iter().any(|existing| {
        existing.kind == relation.kind
            && existing.base_value == relation.base_value
            && existing.witness_value == relation.witness_value
            && existing.window_contract == relation.window_contract
            && existing.reason == relation.reason
    }) {
        return;
    }
    relations.push(relation);
}

pub fn refresh_module_string_corridor_relations(module: &mut MirModule) {
    for function in module.functions.values_mut() {
        refresh_function_string_corridor_relations(function);
    }
}

pub fn refresh_function_string_corridor_relations(function: &mut MirFunction) {
    let preserved_stable_length = preserved_stable_length_relations(function);
    function.metadata.string_corridor_relations.clear();
    let anchors = function
        .metadata
        .string_corridor_facts
        .keys()
        .copied()
        .collect::<BTreeSet<_>>();
    let def_map = build_value_def_map(function);

    collect_phi_carry_relations_into(function, &def_map, &anchors);
    collect_direct_length_relations_into(function, &def_map);
    collect_preserved_stable_length_relations_into(function, preserved_stable_length);
}

fn preserved_stable_length_relations(function: &MirFunction) -> Vec<StringCorridorRelation> {
    let def_map = build_value_def_map(function);
    function
        .metadata
        .string_corridor_relations
        .values()
        .flatten()
        .copied()
        .filter(|relation| relation.kind == StringCorridorRelationKind::StableLengthScalar)
        .filter(|relation| value_is_defined_or_param(function, &def_map, relation.base_value))
        .filter(|relation| {
            relation
                .witness_value
                .is_some_and(|witness| value_is_defined_or_param(function, &def_map, witness))
        })
        .collect()
}

fn value_is_defined_or_param(
    function: &MirFunction,
    def_map: &ValueDefMap,
    value: ValueId,
) -> bool {
    function.params.contains(&value) || def_map.contains_key(&value)
}

/// Phase 1: phi-carry base relations and stable-length witnesses for merged phis.
fn collect_phi_carry_relations_into(
    function: &mut MirFunction,
    def_map: &ValueDefMap,
    anchors: &BTreeSet<ValueId>,
) {
    for relation in collect_phi_carry_relations(function, anchors) {
        let PhiBaseRelation::SameBase(base_value) = relation.relation else {
            continue;
        };
        if base_value == relation.phi_value {
            continue;
        }
        function
            .metadata
            .string_corridor_relations
            .entry(relation.phi_value)
            .or_default()
            .push(StringCorridorRelation {
                kind: StringCorridorRelationKind::PhiCarryBase,
                base_value,
                window_contract: if relation.window_safe {
                    StringCorridorWindowContract::PreservePlanWindow
                } else {
                    StringCorridorWindowContract::StopAtMerge
                },
                witness_value: None,
                reason: if relation.window_safe {
                    "single-input phi continuity keeps the current string corridor lane and preserves the proof-bearing plan window"
                } else {
                    "merged phi continuity keeps the current string corridor lane but stops the proof-bearing plan window at the merge"
                },
            });

        if !relation.window_safe {
            if let Some(stable_length) =
                stable_length_relation_for_phi(function, def_map, relation.phi_value, base_value)
            {
                function
                    .metadata
                    .string_corridor_relations
                    .entry(relation.phi_value)
                    .or_default()
                    .push(stable_length);
            }
        }
    }
}

/// Phase 2: stable-length relations from direct `length()` calls on corridor sources.
fn collect_direct_length_relations_into(function: &mut MirFunction, def_map: &ValueDefMap) {
    for block in function.blocks.values() {
        for inst in &block.instructions {
            let Some((dst, receiver, _effects)) = match_len_call(inst) else {
                continue;
            };
            if let Some(relation) =
                stable_length_relation_for_direct_length_call(function, def_map, dst, receiver)
            {
                function
                    .metadata
                    .string_corridor_relations
                    .entry(relation.base_value)
                    .or_default()
                    .push(relation);
            }
        }
    }
}

/// Phase 3: carry forward typed stable-length relations that survived refresh.
fn collect_preserved_stable_length_relations_into(
    function: &mut MirFunction,
    preserved: Vec<StringCorridorRelation>,
) {
    for relation in preserved {
        let relations = function
            .metadata
            .string_corridor_relations
            .entry(relation.base_value)
            .or_default();
        push_relation_if_absent(relations, relation);
    }
}

#[cfg(test)]
mod tests;
