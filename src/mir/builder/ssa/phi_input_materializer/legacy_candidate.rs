//! P0's disconnected, clone-only legacy PHI-repair candidate.

use super::edge_verifier::{
    derive_terminator_cfg_view_v1, verify_phi_edges_v1, PhiEdgeVerificationErrorV1,
    PreparedUnusedPhiNormalizationV1, TerminatorCfgViewV1, UnusedPhiNormalizationErrorV1,
};
use super::legacy_candidate_cfg::{rebuild_cfg_caches_from_terminators_v1, sorted_block_ids};
use crate::mir::{
    BasicBlockId, Callee, ConstValue, EffectMask, MirFunction, MirInstruction, ValueId,
};
use hakorune_mir_defs::{CalleeBoxKind, TypeCertainty};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum PhiRepairPreflightErrorV1 {
    InvalidCfg(Vec<PhiEdgeVerificationErrorV1>),
    UnusedPhi(UnusedPhiNormalizationErrorV1),
    ExceptionInstruction {
        block: BasicBlockId,
    },
    DuplicateDefinition {
        value: ValueId,
        first_block: BasicBlockId,
        second_block: BasicBlockId,
    },
    AllocatorCursorCollision {
        value: ValueId,
        next_value_id: u32,
    },
    AllocatorOverflow {
        next_value_id: u32,
        planned: u32,
    },
    DuplicatePredecessor {
        block: BasicBlockId,
        phi_index: u32,
        predecessor: BasicBlockId,
    },
    PhantomPredecessor {
        block: BasicBlockId,
        phi_index: u32,
        predecessor: BasicBlockId,
    },
    UndefinedIncoming {
        block: BasicBlockId,
        phi_index: u32,
        predecessor: BasicBlockId,
        value: ValueId,
    },
    UndefinedRematerializationOperand {
        predecessor: BasicBlockId,
        value: ValueId,
    },
    UnrepairableMissingPredecessor {
        block: BasicBlockId,
        phi_index: u32,
        predecessor: BasicBlockId,
    },
    RematerializationCycle {
        predecessor: BasicBlockId,
        value: ValueId,
    },
    NonRematerializable {
        predecessor: BasicBlockId,
        value: ValueId,
    },
    ImpureSubstringCall {
        predecessor: BasicBlockId,
        value: ValueId,
    },
}

impl std::fmt::Display for PhiRepairPreflightErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_repair/preflight] {self:?}"
        )
    }
}

impl std::error::Error for PhiRepairPreflightErrorV1 {}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::mir::builder) enum LegacyPhiRepairCandidateErrorV1 {
    PlannedValueMissing {
        node: u32,
    },
    PlannedPhiMissing {
        block: BasicBlockId,
        phi_index: u32,
    },
    PlannedPhiInputMissing {
        block: BasicBlockId,
        phi_index: u32,
        input_index: u32,
    },
    AllocatorDrift {
        expected: ValueId,
        actual: ValueId,
    },
    FinalVerification(Vec<PhiEdgeVerificationErrorV1>),
}

impl std::fmt::Display for LegacyPhiRepairCandidateErrorV1 {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "[freeze:contract][phi_repair/candidate] {self:?}"
        )
    }
}

impl std::error::Error for LegacyPhiRepairCandidateErrorV1 {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
struct RematNodeIdV1(u32);

#[derive(Clone, Debug)]
enum PlannedValueV1 {
    Existing(ValueId),
    Node(RematNodeIdV1),
}

#[derive(Clone, Debug)]
enum PlannedRematInstructionV1 {
    Const(ConstValue),
    Copy {
        src: PlannedValueV1,
    },
    BinOp {
        op: crate::mir::BinaryOp,
        lhs: PlannedValueV1,
        rhs: PlannedValueV1,
    },
    Compare {
        op: crate::mir::CompareOp,
        lhs: PlannedValueV1,
        rhs: PlannedValueV1,
    },
    UnaryOp {
        op: crate::mir::UnaryOp,
        operand: PlannedValueV1,
    },
    Select {
        cond: PlannedValueV1,
        then_value: PlannedValueV1,
        else_value: PlannedValueV1,
    },
    SubstringCall {
        func: ValueId,
        box_name: String,
        method: String,
        certainty: TypeCertainty,
        box_kind: CalleeBoxKind,
        args: Box<[PlannedValueV1]>,
        receiver: Option<PlannedValueV1>,
        effects: EffectMask,
    },
}

#[derive(Clone, Debug)]
struct PreparedPhiRematerializationV1 {
    id: RematNodeIdV1,
    predecessor: BasicBlockId,
    instruction: PlannedRematInstructionV1,
}

#[derive(Clone, Debug)]
struct PreparedMissingPhiInputV1 {
    block: BasicBlockId,
    phi_index: u32,
    predecessor: BasicBlockId,
    incoming: ValueId,
}

#[derive(Clone, Debug)]
struct PreparedPhiInputRewriteV1 {
    block: BasicBlockId,
    phi_index: u32,
    input_index: u32,
    predecessor: BasicBlockId,
    replacement: PlannedValueV1,
}

#[derive(Clone, Debug)]
struct PreparedPhiRepairScheduleV1 {
    missing_inputs: Box<[PreparedMissingPhiInputV1]>,
    rematerializations: Box<[PreparedPhiRematerializationV1]>,
    rewrites: Box<[PreparedPhiInputRewriteV1]>,
    first_fresh_value: u32,
}

#[derive(Debug)]
pub(in crate::mir::builder) struct PreparedLegacyPhiRepairCandidateV1 {
    candidate: MirFunction,
    schedule: PreparedPhiRepairScheduleV1,
    _seal: LegacyPhiRepairCandidateSealV1,
}

#[derive(Debug)]
struct LegacyPhiRepairCandidateSealV1;

#[derive(Debug)]
pub(in crate::mir::builder) struct RepairedLegacyPhiFunctionCandidateV1 {
    candidate: MirFunction,
    _seal: RepairedLegacyPhiFunctionCandidateSealV1,
}

#[derive(Debug)]
struct RepairedLegacyPhiFunctionCandidateSealV1;

pub(in crate::mir::builder) fn prepare_legacy_phi_repair_candidate_v1(
    function: &MirFunction,
) -> Result<PreparedLegacyPhiRepairCandidateV1, PhiRepairPreflightErrorV1> {
    let unused = PreparedUnusedPhiNormalizationV1::collect(function)
        .map_err(PhiRepairPreflightErrorV1::UnusedPhi)?;
    unused
        .require_artifact_closure()
        .map_err(PhiRepairPreflightErrorV1::UnusedPhi)?;

    reject_exception_instructions(function)?;
    let cfg =
        derive_terminator_cfg_view_v1(function).map_err(PhiRepairPreflightErrorV1::InvalidCfg)?;
    let definitions = definition_catalog(function)?;
    verify_allocator_cursor(function, &definitions)?;

    let mut planner = PhiRepairPlannerV1::new(function, &cfg, &definitions);
    planner.plan()?;
    let schedule = planner.finish(function.next_value_id)?;
    Ok(PreparedLegacyPhiRepairCandidateV1 {
        candidate: function.clone(),
        schedule,
        _seal: LegacyPhiRepairCandidateSealV1,
    })
}

impl PreparedLegacyPhiRepairCandidateV1 {
    pub(in crate::mir::builder) fn execute(
        mut self,
    ) -> Result<RepairedLegacyPhiFunctionCandidateV1, LegacyPhiRepairCandidateErrorV1> {
        for missing in &self.schedule.missing_inputs {
            let block = self.candidate.get_block_mut(missing.block).ok_or(
                LegacyPhiRepairCandidateErrorV1::PlannedPhiMissing {
                    block: missing.block,
                    phi_index: missing.phi_index,
                },
            )?;
            let Some(MirInstruction::Phi { inputs, .. }) =
                block.instructions.get_mut(missing.phi_index as usize)
            else {
                return Err(LegacyPhiRepairCandidateErrorV1::PlannedPhiMissing {
                    block: missing.block,
                    phi_index: missing.phi_index,
                });
            };
            inputs.push((missing.predecessor, missing.incoming));
        }

        let mut values = BTreeMap::new();
        for remat in &self.schedule.rematerializations {
            let expected = ValueId::new(self.schedule.first_fresh_value + remat.id.0);
            let actual = self.candidate.next_value_id();
            if actual != expected {
                return Err(LegacyPhiRepairCandidateErrorV1::AllocatorDrift { expected, actual });
            }
            let instruction = materialize_instruction(remat, actual, &values)?;
            let block = self.candidate.get_block_mut(remat.predecessor).ok_or(
                LegacyPhiRepairCandidateErrorV1::PlannedPhiMissing {
                    block: remat.predecessor,
                    phi_index: 0,
                },
            )?;
            block.add_instruction_before_terminator(instruction);
            values.insert(remat.id, actual);
        }

        for rewrite in &self.schedule.rewrites {
            let replacement = resolve_planned_value(&rewrite.replacement, &values)?;
            let block = self.candidate.get_block_mut(rewrite.block).ok_or(
                LegacyPhiRepairCandidateErrorV1::PlannedPhiMissing {
                    block: rewrite.block,
                    phi_index: rewrite.phi_index,
                },
            )?;
            let Some(MirInstruction::Phi { inputs, .. }) =
                block.instructions.get_mut(rewrite.phi_index as usize)
            else {
                return Err(LegacyPhiRepairCandidateErrorV1::PlannedPhiMissing {
                    block: rewrite.block,
                    phi_index: rewrite.phi_index,
                });
            };
            let Some((_, incoming)) = inputs.get_mut(rewrite.input_index as usize) else {
                return Err(LegacyPhiRepairCandidateErrorV1::PlannedPhiInputMissing {
                    block: rewrite.block,
                    phi_index: rewrite.phi_index,
                    input_index: rewrite.input_index,
                });
            };
            *incoming = replacement;
        }

        rebuild_cfg_caches_from_terminators_v1(&mut self.candidate)
            .map_err(LegacyPhiRepairCandidateErrorV1::FinalVerification)?;
        verify_phi_edges_v1(&self.candidate)
            .map_err(LegacyPhiRepairCandidateErrorV1::FinalVerification)?;
        Ok(RepairedLegacyPhiFunctionCandidateV1 {
            candidate: self.candidate,
            _seal: RepairedLegacyPhiFunctionCandidateSealV1,
        })
    }

    #[cfg(test)]
    pub(super) fn schedule_counts(&self) -> (usize, usize) {
        (
            self.schedule.missing_inputs.len(),
            self.schedule.rematerializations.len(),
        )
    }
}

impl RepairedLegacyPhiFunctionCandidateV1 {
    #[cfg(test)]
    pub(super) fn function(&self) -> &MirFunction {
        &self.candidate
    }
}

struct PhiRepairPlannerV1<'a> {
    function: &'a MirFunction,
    cfg: &'a TerminatorCfgViewV1,
    definitions: &'a BTreeMap<ValueId, DefinitionV1>,
    missing_inputs: Vec<PreparedMissingPhiInputV1>,
    rematerializations: Vec<PreparedPhiRematerializationV1>,
    rewrites: Vec<PreparedPhiInputRewriteV1>,
    memo_by_predecessor: BTreeMap<BasicBlockId, BTreeMap<ValueId, PlannedValueV1>>,
    visiting_by_predecessor: BTreeMap<BasicBlockId, BTreeSet<ValueId>>,
}

impl<'a> PhiRepairPlannerV1<'a> {
    fn new(
        function: &'a MirFunction,
        cfg: &'a TerminatorCfgViewV1,
        definitions: &'a BTreeMap<ValueId, DefinitionV1>,
    ) -> Self {
        Self {
            function,
            cfg,
            definitions,
            missing_inputs: Vec::new(),
            rematerializations: Vec::new(),
            rewrites: Vec::new(),
            memo_by_predecessor: BTreeMap::new(),
            visiting_by_predecessor: BTreeMap::new(),
        }
    }

    fn plan(&mut self) -> Result<(), PhiRepairPreflightErrorV1> {
        for block_id in sorted_block_ids(self.function) {
            if !self.cfg.is_reachable(block_id) {
                continue;
            }
            let block = &self.function.blocks[&block_id];
            let expected_predecessors = self.cfg.predecessors(block_id);
            for (phi_index, instruction) in block.instructions.iter().enumerate() {
                let MirInstruction::Phi { dst, inputs, .. } = instruction else {
                    continue;
                };
                let phi_index = phi_index as u32;
                self.plan_one_phi(block_id, phi_index, *dst, inputs, &expected_predecessors)?;
            }
        }
        Ok(())
    }

    fn plan_one_phi(
        &mut self,
        block: BasicBlockId,
        phi_index: u32,
        dst: ValueId,
        inputs: &[(BasicBlockId, ValueId)],
        expected_predecessors: &BTreeSet<BasicBlockId>,
    ) -> Result<(), PhiRepairPreflightErrorV1> {
        let mut seen = BTreeSet::new();
        for (input_index, (predecessor, incoming)) in inputs.iter().enumerate() {
            if !seen.insert(*predecessor) {
                return Err(PhiRepairPreflightErrorV1::DuplicatePredecessor {
                    block,
                    phi_index,
                    predecessor: *predecessor,
                });
            }
            if !expected_predecessors.contains(predecessor) {
                return Err(PhiRepairPreflightErrorV1::PhantomPredecessor {
                    block,
                    phi_index,
                    predecessor: *predecessor,
                });
            }
            if !self.definitions.contains_key(incoming) {
                return Err(PhiRepairPreflightErrorV1::UndefinedIncoming {
                    block,
                    phi_index,
                    predecessor: *predecessor,
                    value: *incoming,
                });
            }
            let replacement = self.plan_value(*predecessor, *incoming)?;
            if !matches!(replacement, PlannedValueV1::Existing(value) if value == *incoming) {
                self.rewrites.push(PreparedPhiInputRewriteV1 {
                    block,
                    phi_index,
                    input_index: input_index as u32,
                    predecessor: *predecessor,
                    replacement,
                });
            }
        }

        for predecessor in expected_predecessors {
            if seen.contains(predecessor) {
                continue;
            }
            let incoming = if self.cfg.dominates(block, *predecessor) {
                dst
            } else {
                let mut candidates = inputs
                    .iter()
                    .filter_map(|(_, value)| {
                        self.definitions
                            .get(value)
                            .is_some_and(|definition| {
                                self.cfg.dominates(definition.block, *predecessor)
                            })
                            .then_some(*value)
                    })
                    .collect::<Vec<_>>();
                candidates.sort_by_key(|value| value.0);
                candidates.dedup();
                if candidates.len() != 1 {
                    return Err(PhiRepairPreflightErrorV1::UnrepairableMissingPredecessor {
                        block,
                        phi_index,
                        predecessor: *predecessor,
                    });
                }
                candidates[0]
            };
            self.missing_inputs.push(PreparedMissingPhiInputV1 {
                block,
                phi_index,
                predecessor: *predecessor,
                incoming,
            });
        }
        Ok(())
    }

    fn plan_value(
        &mut self,
        predecessor: BasicBlockId,
        value: ValueId,
    ) -> Result<PlannedValueV1, PhiRepairPreflightErrorV1> {
        if let Some(planned) = self
            .memo_by_predecessor
            .get(&predecessor)
            .and_then(|memo| memo.get(&value))
            .cloned()
        {
            return Ok(planned);
        }
        let Some(definition) = self.definitions.get(&value).cloned() else {
            return Err(
                PhiRepairPreflightErrorV1::UndefinedRematerializationOperand { predecessor, value },
            );
        };
        if self.cfg.dominates(definition.block, predecessor) {
            return Ok(PlannedValueV1::Existing(value));
        }
        if !self
            .visiting_by_predecessor
            .entry(predecessor)
            .or_default()
            .insert(value)
        {
            return Err(PhiRepairPreflightErrorV1::RematerializationCycle { predecessor, value });
        }
        let instruction = self.plan_instruction(predecessor, value, definition)?;
        self.visiting_by_predecessor
            .get_mut(&predecessor)
            .expect("visiting predecessor inserted")
            .remove(&value);
        let id = RematNodeIdV1(self.rematerializations.len() as u32);
        self.rematerializations
            .push(PreparedPhiRematerializationV1 {
                id,
                predecessor,
                instruction,
            });
        let planned = PlannedValueV1::Node(id);
        self.memo_by_predecessor
            .entry(predecessor)
            .or_default()
            .insert(value, planned.clone());
        Ok(planned)
    }

    fn plan_instruction(
        &mut self,
        predecessor: BasicBlockId,
        value: ValueId,
        definition: DefinitionV1,
    ) -> Result<PlannedRematInstructionV1, PhiRepairPreflightErrorV1> {
        let Some(instruction) = definition.instruction else {
            return Err(PhiRepairPreflightErrorV1::NonRematerializable { predecessor, value });
        };
        match instruction {
            MirInstruction::Const { value, .. } => Ok(PlannedRematInstructionV1::Const(value)),
            MirInstruction::Copy { src, .. } => Ok(PlannedRematInstructionV1::Copy {
                src: self.plan_value(predecessor, src)?,
            }),
            MirInstruction::BinOp { op, lhs, rhs, .. } => Ok(PlannedRematInstructionV1::BinOp {
                op,
                lhs: self.plan_value(predecessor, lhs)?,
                rhs: self.plan_value(predecessor, rhs)?,
            }),
            MirInstruction::Compare { op, lhs, rhs, .. } => {
                Ok(PlannedRematInstructionV1::Compare {
                    op,
                    lhs: self.plan_value(predecessor, lhs)?,
                    rhs: self.plan_value(predecessor, rhs)?,
                })
            }
            MirInstruction::UnaryOp { op, operand, .. } => Ok(PlannedRematInstructionV1::UnaryOp {
                op,
                operand: self.plan_value(predecessor, operand)?,
            }),
            MirInstruction::Select {
                cond,
                then_val,
                else_val,
                ..
            } => Ok(PlannedRematInstructionV1::Select {
                cond: self.plan_value(predecessor, cond)?,
                then_value: self.plan_value(predecessor, then_val)?,
                else_value: self.plan_value(predecessor, else_val)?,
            }),
            MirInstruction::Call {
                func,
                callee:
                    Some(Callee::Method {
                        box_name,
                        method,
                        receiver,
                        certainty,
                        box_kind,
                    }),
                args,
                effects,
                ..
            } if matches!(box_name.as_str(), "RuntimeDataBox" | "StringBox")
                && method == "substring" =>
            {
                if !effects.is_pure() {
                    return Err(PhiRepairPreflightErrorV1::ImpureSubstringCall {
                        predecessor,
                        value,
                    });
                }
                let args = args
                    .into_iter()
                    .map(|arg| self.plan_value(predecessor, arg))
                    .collect::<Result<Vec<_>, _>>()?;
                let receiver = receiver
                    .map(|receiver| self.plan_value(predecessor, receiver))
                    .transpose()?;
                Ok(PlannedRematInstructionV1::SubstringCall {
                    func,
                    box_name,
                    method,
                    certainty,
                    box_kind,
                    args: args.into_boxed_slice(),
                    receiver,
                    effects,
                })
            }
            _ => Err(PhiRepairPreflightErrorV1::NonRematerializable { predecessor, value }),
        }
    }

    fn finish(
        mut self,
        first_fresh_value: u32,
    ) -> Result<PreparedPhiRepairScheduleV1, PhiRepairPreflightErrorV1> {
        self.missing_inputs
            .sort_by_key(|row| (row.block.0, row.phi_index, row.predecessor.0));
        self.rewrites.sort_by_key(|row| {
            (
                row.predecessor.0,
                row.block.0,
                row.phi_index,
                row.input_index,
            )
        });
        let planned = u32::try_from(self.rematerializations.len()).expect("remat count fits u32");
        if first_fresh_value.checked_add(planned).is_none() {
            return Err(PhiRepairPreflightErrorV1::AllocatorOverflow {
                next_value_id: first_fresh_value,
                planned,
            });
        }
        Ok(PreparedPhiRepairScheduleV1 {
            missing_inputs: self.missing_inputs.into_boxed_slice(),
            rematerializations: self.rematerializations.into_boxed_slice(),
            rewrites: self.rewrites.into_boxed_slice(),
            first_fresh_value,
        })
    }
}

#[derive(Clone, Debug)]
struct DefinitionV1 {
    block: BasicBlockId,
    instruction: Option<MirInstruction>,
}

fn definition_catalog(
    function: &MirFunction,
) -> Result<BTreeMap<ValueId, DefinitionV1>, PhiRepairPreflightErrorV1> {
    let mut definitions = BTreeMap::new();
    for parameter in &function.params {
        insert_definition(
            &mut definitions,
            *parameter,
            DefinitionV1 {
                block: function.entry_block,
                instruction: None,
            },
        )?;
    }
    for block_id in sorted_block_ids(function) {
        let block = &function.blocks[&block_id];
        for instruction in block.all_instructions() {
            if let Some(value) = instruction.dst_value() {
                insert_definition(
                    &mut definitions,
                    value,
                    DefinitionV1 {
                        block: block_id,
                        instruction: Some(instruction.clone()),
                    },
                )?;
            }
        }
    }
    Ok(definitions)
}

fn insert_definition(
    definitions: &mut BTreeMap<ValueId, DefinitionV1>,
    value: ValueId,
    definition: DefinitionV1,
) -> Result<(), PhiRepairPreflightErrorV1> {
    if let Some(first) = definitions.get(&value) {
        return Err(PhiRepairPreflightErrorV1::DuplicateDefinition {
            value,
            first_block: first.block,
            second_block: definition.block,
        });
    }
    definitions.insert(value, definition);
    Ok(())
}

fn verify_allocator_cursor(
    function: &MirFunction,
    definitions: &BTreeMap<ValueId, DefinitionV1>,
) -> Result<(), PhiRepairPreflightErrorV1> {
    if let Some(value) = definitions
        .keys()
        .copied()
        .find(|value| value.0 >= function.next_value_id)
    {
        return Err(PhiRepairPreflightErrorV1::AllocatorCursorCollision {
            value,
            next_value_id: function.next_value_id,
        });
    }
    Ok(())
}

fn reject_exception_instructions(function: &MirFunction) -> Result<(), PhiRepairPreflightErrorV1> {
    for block_id in sorted_block_ids(function) {
        let block = &function.blocks[&block_id];
        if block.all_instructions().any(|instruction| {
            matches!(
                instruction,
                MirInstruction::Catch { .. } | MirInstruction::Throw { .. }
            )
        }) {
            return Err(PhiRepairPreflightErrorV1::ExceptionInstruction { block: block_id });
        }
    }
    Ok(())
}

fn resolve_planned_value(
    value: &PlannedValueV1,
    values: &BTreeMap<RematNodeIdV1, ValueId>,
) -> Result<ValueId, LegacyPhiRepairCandidateErrorV1> {
    match value {
        PlannedValueV1::Existing(value) => Ok(*value),
        PlannedValueV1::Node(node) => values
            .get(node)
            .copied()
            .ok_or(LegacyPhiRepairCandidateErrorV1::PlannedValueMissing { node: node.0 }),
    }
}

fn materialize_instruction(
    remat: &PreparedPhiRematerializationV1,
    destination: ValueId,
    values: &BTreeMap<RematNodeIdV1, ValueId>,
) -> Result<MirInstruction, LegacyPhiRepairCandidateErrorV1> {
    match &remat.instruction {
        PlannedRematInstructionV1::Const(value) => Ok(MirInstruction::Const {
            dst: destination,
            value: value.clone(),
        }),
        PlannedRematInstructionV1::Copy { src } => Ok(MirInstruction::Copy {
            dst: destination,
            src: resolve_planned_value(src, values)?,
        }),
        PlannedRematInstructionV1::BinOp { op, lhs, rhs } => Ok(MirInstruction::BinOp {
            dst: destination,
            op: *op,
            lhs: resolve_planned_value(lhs, values)?,
            rhs: resolve_planned_value(rhs, values)?,
        }),
        PlannedRematInstructionV1::Compare { op, lhs, rhs } => Ok(MirInstruction::Compare {
            dst: destination,
            op: *op,
            lhs: resolve_planned_value(lhs, values)?,
            rhs: resolve_planned_value(rhs, values)?,
        }),
        PlannedRematInstructionV1::UnaryOp { op, operand } => Ok(MirInstruction::UnaryOp {
            dst: destination,
            op: *op,
            operand: resolve_planned_value(operand, values)?,
        }),
        PlannedRematInstructionV1::Select {
            cond,
            then_value,
            else_value,
        } => Ok(MirInstruction::Select {
            dst: destination,
            cond: resolve_planned_value(cond, values)?,
            then_val: resolve_planned_value(then_value, values)?,
            else_val: resolve_planned_value(else_value, values)?,
        }),
        PlannedRematInstructionV1::SubstringCall {
            func,
            box_name,
            method,
            certainty,
            box_kind,
            args,
            receiver,
            effects,
        } => Ok(MirInstruction::Call {
            dst: Some(destination),
            func: *func,
            callee: Some(Callee::Method {
                box_name: box_name.clone(),
                method: method.clone(),
                receiver: receiver
                    .as_ref()
                    .map(|receiver| resolve_planned_value(receiver, values))
                    .transpose()?,
                certainty: *certainty,
                box_kind: *box_kind,
            }),
            args: args
                .iter()
                .map(|arg| resolve_planned_value(arg, values))
                .collect::<Result<Vec<_>, _>>()?,
            effects: *effects,
        }),
    }
}
