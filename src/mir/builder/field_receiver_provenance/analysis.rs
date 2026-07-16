use super::cfg::EphemeralReceiverCfgV1;
use super::definitions::{DefinitionKindV1, ExactDefinitionIndexV1};
use super::{
    MirBuilder, SameRootReceiverProofErrorV1, SameRootReceiverValueSealV1,
    VerifiedCurrentReceiverIdentityV1, VerifiedSameRootReceiverValueV1,
};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, MirType, ValueId};
use std::collections::{BTreeMap, BTreeSet};

#[derive(Clone, Copy, PartialEq, Eq)]
enum VisitStateV1 {
    Visiting,
    Proven,
}

enum ProofFrameV1 {
    Enter(ValueId),
    ExitCopy {
        value: ValueId,
        source: ValueId,
        use_block: BasicBlockId,
        use_order: Option<usize>,
    },
    ExitPhi {
        value: ValueId,
        inputs: Vec<(BasicBlockId, ValueId)>,
    },
}

pub(super) struct ConstructionResultV1 {
    pub(super) proof: VerifiedSameRootReceiverValueV1,
    pub(super) normalized: Option<String>,
}

pub(super) fn verify(
    builder: &MirBuilder,
    seed: ValueId,
    capture_normalized: bool,
) -> Result<ConstructionResultV1, SameRootReceiverProofErrorV1> {
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .ok_or(SameRootReceiverProofErrorV1::NoCurrentFunction)?;
    let use_block = builder
        .current_block
        .ok_or(SameRootReceiverProofErrorV1::MissingUseSite)?;
    let use_order = function
        .blocks
        .get(&use_block)
        .ok_or(SameRootReceiverProofErrorV1::MissingCfgBlock)?
        .instructions
        .len();
    verify_at(builder, seed, capture_normalized, use_block, use_order)
}

pub(super) fn verify_at(
    builder: &MirBuilder,
    seed: ValueId,
    capture_normalized: bool,
    use_block: BasicBlockId,
    use_order: usize,
) -> Result<ConstructionResultV1, SameRootReceiverProofErrorV1> {
    let receiver = VerifiedCurrentReceiverIdentityV1::verify(builder)?;
    let function = builder
        .scope_ctx
        .current_function
        .as_ref()
        .ok_or(SameRootReceiverProofErrorV1::NoCurrentFunction)?;
    let definitions = ExactDefinitionIndexV1::build(function)?;
    let cfg = EphemeralReceiverCfgV1::new(function)?;
    if !available_at_instruction(&definitions, &cfg, seed, use_block, Some(use_order))? {
        return Err(SameRootReceiverProofErrorV1::SeedUnavailable);
    }
    let mut states = BTreeMap::new();
    let mut normalized = BTreeMap::<ValueId, String>::new();
    let mut worklist = vec![ProofFrameV1::Enter(seed)];
    let mut budget = definitions.traversal_budget();

    while let Some(frame) = worklist.pop() {
        if budget == 0 {
            return Err(SameRootReceiverProofErrorV1::TraversalBudgetExceeded);
        }
        budget -= 1;
        match frame {
            ProofFrameV1::Enter(value) => {
                match states.get(&value) {
                    Some(VisitStateV1::Proven) => continue,
                    Some(VisitStateV1::Visiting) => {
                        return Err(SameRootReceiverProofErrorV1::ValueDefinitionCycle)
                    }
                    None => {}
                }
                validate_value(builder, value, &receiver)?;
                let definition = definitions.get(value)?;
                match &definition.kind {
                    DefinitionKindV1::Parameter(0) if value == receiver.receiver_parameter() => {
                        states.insert(value, VisitStateV1::Proven);
                        if capture_normalized {
                            normalized.insert(value, "R".to_string());
                        }
                    }
                    DefinitionKindV1::Parameter(_) => {
                        return Err(SameRootReceiverProofErrorV1::ForeignParameter)
                    }
                    DefinitionKindV1::Instruction { .. } => match definition
                        .instruction(function)?
                    {
                        MirInstruction::Copy { src, .. } => {
                            states.insert(value, VisitStateV1::Visiting);
                            worklist.push(ProofFrameV1::ExitCopy {
                                value,
                                source: *src,
                                use_block: definition.block,
                                use_order: definition.order,
                            });
                            worklist.push(ProofFrameV1::Enter(*src));
                        }
                        MirInstruction::Phi { inputs, .. } => {
                            validate_phi_shape(function, &cfg, definition.block, inputs)?;
                            states.insert(value, VisitStateV1::Visiting);
                            worklist.push(ProofFrameV1::ExitPhi {
                                value,
                                inputs: inputs.clone(),
                            });
                            for child in inputs.iter().rev().map(|(_, input)| *input) {
                                worklist.push(ProofFrameV1::Enter(child));
                            }
                        }
                        _ => return Err(SameRootReceiverProofErrorV1::UnsupportedDefinitionKind),
                    },
                }
            }
            ProofFrameV1::ExitCopy {
                value,
                source,
                use_block,
                use_order,
            } => {
                require_proven(&states, source)?;
                if !available_at_instruction(&definitions, &cfg, source, use_block, use_order)? {
                    return Err(SameRootReceiverProofErrorV1::CopySourceUnavailable);
                }
                states.insert(value, VisitStateV1::Proven);
                if capture_normalized {
                    let shape = normalized
                        .get(&source)
                        .cloned()
                        .ok_or(SameRootReceiverProofErrorV1::TraversalBudgetExceeded)?;
                    normalized.insert(value, shape);
                }
            }
            ProofFrameV1::ExitPhi { value, inputs } => {
                for (_, child) in &inputs {
                    require_proven(&states, *child)?;
                }
                validate_phi_availability(&definitions, &cfg, &inputs)?;
                states.insert(value, VisitStateV1::Proven);
                if capture_normalized {
                    let mut shapes = Vec::with_capacity(inputs.len());
                    for (_, child) in inputs {
                        shapes.push(
                            normalized
                                .get(&child)
                                .cloned()
                                .ok_or(SameRootReceiverProofErrorV1::TraversalBudgetExceeded)?,
                        );
                    }
                    shapes.sort();
                    normalized.insert(value, format!("P[{}]", shapes.join(",")));
                }
            }
        }
    }

    if states.get(&seed) != Some(&VisitStateV1::Proven) {
        return Err(SameRootReceiverProofErrorV1::TraversalBudgetExceeded);
    }
    let normalized_seed = capture_normalized
        .then(|| normalized.remove(&seed))
        .flatten();
    Ok(ConstructionResultV1 {
        proof: VerifiedSameRootReceiverValueV1 {
            value: seed,
            receiver,
            _seal: SameRootReceiverValueSealV1,
        },
        normalized: normalized_seed,
    })
}

fn validate_value(
    builder: &MirBuilder,
    value: ValueId,
    receiver: &VerifiedCurrentReceiverIdentityV1,
) -> Result<(), SameRootReceiverProofErrorV1> {
    let ty = builder
        .type_ctx
        .value_types
        .get(&value)
        .ok_or(SameRootReceiverProofErrorV1::SeedTypeMissing)?;
    if ty != &MirType::Box(receiver.owner_box().to_string()) {
        return Err(SameRootReceiverProofErrorV1::SeedTypeMismatch);
    }
    if builder
        .type_ctx
        .value_origin_newbox
        .get(&value)
        .is_some_and(|origin| origin != receiver.owner_box())
    {
        return Err(SameRootReceiverProofErrorV1::ForeignOrigin);
    }
    Ok(())
}

fn require_proven(
    states: &BTreeMap<ValueId, VisitStateV1>,
    value: ValueId,
) -> Result<(), SameRootReceiverProofErrorV1> {
    if states.get(&value) != Some(&VisitStateV1::Proven) {
        return Err(SameRootReceiverProofErrorV1::ValueDefinitionCycle);
    }
    Ok(())
}

fn available_at_instruction(
    definitions: &ExactDefinitionIndexV1,
    cfg: &EphemeralReceiverCfgV1,
    value: ValueId,
    use_block: BasicBlockId,
    use_order: Option<usize>,
) -> Result<bool, SameRootReceiverProofErrorV1> {
    let definition = definitions.get(value)?;
    if !cfg.is_reachable(definition.block) || !cfg.is_reachable(use_block) {
        return Ok(false);
    }
    if definition.block == use_block {
        return Ok(match (definition.order, use_order) {
            (None, _) => true,
            (Some(definition_order), Some(use_order)) => definition_order < use_order,
            _ => false,
        });
    }
    Ok(cfg.dominates(definition.block, use_block))
}

fn validate_phi_shape(
    function: &MirFunction,
    cfg: &EphemeralReceiverCfgV1,
    phi_block: BasicBlockId,
    inputs: &[(BasicBlockId, ValueId)],
) -> Result<(), SameRootReceiverProofErrorV1> {
    if !cfg.is_reachable(phi_block) {
        return Err(SameRootReceiverProofErrorV1::PhiUnreachable);
    }
    if inputs.len() < 2 {
        return Err(SameRootReceiverProofErrorV1::PhiTooFewInputs);
    }
    let mut attached = BTreeSet::new();
    for (predecessor, _) in inputs {
        if !attached.insert(*predecessor) {
            return Err(SameRootReceiverProofErrorV1::DuplicatePhiPredecessor);
        }
        if !cfg.is_reachable(*predecessor) {
            return Err(SameRootReceiverProofErrorV1::UnreachablePhiPredecessor);
        }
    }
    let expected = cfg.reachable_predecessors(phi_block);
    if attached
        .iter()
        .any(|predecessor| !expected.contains(predecessor))
    {
        return Err(SameRootReceiverProofErrorV1::PhantomPhiPredecessor);
    }
    if expected
        .iter()
        .any(|predecessor| !attached.contains(predecessor))
    {
        return Err(SameRootReceiverProofErrorV1::MissingPhiPredecessor);
    }
    for (predecessor, _incoming) in inputs {
        if cfg
            .edge_participates_in_cycle(function, phi_block, *predecessor)
            .map_err(|_| SameRootReceiverProofErrorV1::TraversalBudgetExceeded)?
        {
            return Err(SameRootReceiverProofErrorV1::CfgCycleOrBackedge);
        }
    }
    Ok(())
}

fn validate_phi_availability(
    definitions: &ExactDefinitionIndexV1,
    cfg: &EphemeralReceiverCfgV1,
    inputs: &[(BasicBlockId, ValueId)],
) -> Result<(), SameRootReceiverProofErrorV1> {
    for (predecessor, incoming) in inputs {
        let definition = definitions.get(*incoming)?;
        if !cfg.is_reachable(definition.block)
            || (definition.block != *predecessor && !cfg.dominates(definition.block, *predecessor))
        {
            return Err(SameRootReceiverProofErrorV1::PhiIncomingUnavailable);
        }
    }
    Ok(())
}
