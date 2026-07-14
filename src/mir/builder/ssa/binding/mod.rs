//! Function-owned sealed-block SSA construction for canonical bindings.
//!
//! SSA-I1-T connects this box to exactly one admitted trivial whole-owner
//! lowering route. Other canonical families remain outside this authority.

mod adapter;
mod error;
mod mir_adapter;

#[cfg(test)]
mod mir_adapter_tests;
#[cfg(test)]
mod tests;

pub(in crate::mir::builder) use adapter::BindingSsaIrV1;
pub(in crate::mir::builder) use error::BindingSsaErrorV1;
pub(in crate::mir::builder) use mir_adapter::MirBindingSsaAdapterV1;

use crate::mir::builder::resolved_lowering::canonical_cfg::VerifiedPredecessorsV1;
use crate::mir::resolved_semantics::{BindingRefV1, FunctionOwnerIdV1};
use crate::mir::{BasicBlockId, ValueId};
use std::collections::BTreeMap;

#[derive(Debug)]
struct IncompletePhiV1<Token> {
    token: Token,
}

#[derive(Debug, Default)]
struct BlockStateV1 {
    predecessors: Option<Box<[BasicBlockId]>>,
}

#[derive(Debug)]
struct PhiCleanupV1<Token> {
    token: Token,
}

/// One reaching-definition owner for one canonical function.
#[derive(Debug)]
pub(in crate::mir::builder) struct BindingSsaBuilderV1<Token> {
    owner: FunctionOwnerIdV1,
    definitions: BTreeMap<(BasicBlockId, BindingRefV1), ValueId>,
    blocks: BTreeMap<BasicBlockId, BlockStateV1>,
    incomplete: BTreeMap<(BasicBlockId, BindingRefV1), IncompletePhiV1<Token>>,
    poisoned: bool,
}

impl<Token: Copy + Eq + std::fmt::Debug> BindingSsaBuilderV1<Token> {
    pub(in crate::mir::builder) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self {
            owner,
            definitions: BTreeMap::new(),
            blocks: BTreeMap::new(),
            incomplete: BTreeMap::new(),
            poisoned: false,
        }
    }

    pub(in crate::mir::builder) fn define(
        &mut self,
        binding: BindingRefV1,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(binding)?;
        self.blocks.entry(block).or_default();
        self.definitions.insert((block, binding), value);
        Ok(())
    }

    pub(in crate::mir::builder) fn read<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        binding: BindingRefV1,
        block: BasicBlockId,
    ) -> Result<ValueId, BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(binding)?;
        self.blocks.entry(block).or_default();
        let mut created = Vec::new();
        match self.read_recursive(ir, binding, block, &mut created) {
            Ok(value) => Ok(value),
            Err(primary) => Err(self.abort(ir, primary, created)),
        }
    }

    pub(in crate::mir::builder) fn seal<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        if witness.block() != block {
            return Err(BindingSsaErrorV1::WitnessBlockMismatch {
                expected: block,
                actual: witness.block(),
            });
        }
        let state = self.blocks.entry(block).or_default();
        if state.predecessors.is_some() {
            return Err(BindingSsaErrorV1::BlockSealedTwice { block });
        }
        state.predecessors = Some(witness.predecessors().into());

        let bindings = self
            .incomplete
            .keys()
            .filter_map(|(candidate, binding)| (*candidate == block).then_some(*binding))
            .collect::<Vec<_>>();
        let mut affected = Vec::new();
        for binding in bindings {
            let incomplete = self
                .incomplete
                .get(&(block, binding))
                .expect("binding came from incomplete index");
            affected.push(PhiCleanupV1 {
                token: incomplete.token,
            });
            if let Err(primary) = self.complete_incomplete(ir, binding, block, &mut affected) {
                return Err(self.abort(ir, primary, affected));
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn finish(self) -> Result<(), BindingSsaErrorV1> {
        if self.poisoned {
            return Err(BindingSsaErrorV1::Poisoned);
        }
        if !self.incomplete.is_empty() {
            return Err(BindingSsaErrorV1::IncompleteAtFinish {
                count: self.incomplete.len(),
            });
        }
        let open = self
            .blocks
            .into_iter()
            .filter_map(|(block, state)| state.predecessors.is_none().then_some(block))
            .collect::<Vec<_>>();
        if !open.is_empty() {
            return Err(BindingSsaErrorV1::UnsealedAtFinish {
                blocks: open.into_boxed_slice(),
            });
        }
        Ok(())
    }

    fn read_recursive<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        binding: BindingRefV1,
        block: BasicBlockId,
        created: &mut Vec<PhiCleanupV1<Token>>,
    ) -> Result<ValueId, BindingSsaErrorV1> {
        if let Some(value) = self.definitions.get(&(block, binding)).copied() {
            return Ok(value);
        }
        let predecessors = self.blocks.entry(block).or_default().predecessors.clone();
        let Some(predecessors) = predecessors else {
            let (value, token) = ir.define_provisional_phi(block).map_err(|detail| {
                BindingSsaErrorV1::PhiOperation {
                    operation: "define",
                    detail,
                }
            })?;
            self.definitions.insert((block, binding), value);
            self.incomplete
                .insert((block, binding), IncompletePhiV1 { token });
            created.push(PhiCleanupV1 { token });
            return Ok(value);
        };

        match predecessors.as_ref() {
            [] => Err(BindingSsaErrorV1::MissingDefinition { block, binding }),
            [predecessor] => {
                let value = self.read_recursive(ir, binding, *predecessor, created)?;
                self.definitions.insert((block, binding), value);
                Ok(value)
            }
            _ => {
                let (value, token) = ir.define_provisional_phi(block).map_err(|detail| {
                    BindingSsaErrorV1::PhiOperation {
                        operation: "define",
                        detail,
                    }
                })?;
                self.definitions.insert((block, binding), value);
                created.push(PhiCleanupV1 { token });
                let inputs = predecessors
                    .iter()
                    .map(|predecessor| {
                        self.read_recursive(ir, binding, *predecessor, created)
                            .map(|input| (*predecessor, input))
                    })
                    .collect::<Result<Vec<_>, _>>()?;
                self.verify_inputs(ir, &inputs)?;
                ir.patch_phi_inputs(token, &inputs).map_err(|detail| {
                    BindingSsaErrorV1::PhiOperation {
                        operation: "patch",
                        detail,
                    }
                })?;
                forget_cleanup(created, token);
                Ok(value)
            }
        }
    }

    fn complete_incomplete<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        binding: BindingRefV1,
        block: BasicBlockId,
        affected: &mut Vec<PhiCleanupV1<Token>>,
    ) -> Result<(), BindingSsaErrorV1> {
        let token = self
            .incomplete
            .get(&(block, binding))
            .expect("completion requires an incomplete PHI")
            .token;
        let predecessors = self.blocks[&block]
            .predecessors
            .as_ref()
            .expect("seal installed predecessors")
            .clone();
        if predecessors.is_empty() {
            return Err(BindingSsaErrorV1::MissingDefinition { block, binding });
        }
        let inputs = predecessors
            .iter()
            .map(|predecessor| {
                self.read_recursive(ir, binding, *predecessor, affected)
                    .map(|value| (*predecessor, value))
            })
            .collect::<Result<Vec<_>, _>>()?;
        self.verify_inputs(ir, &inputs)?;
        ir.patch_phi_inputs(token, &inputs)
            .map_err(|detail| BindingSsaErrorV1::PhiOperation {
                operation: "patch",
                detail,
            })?;
        forget_cleanup(affected, token);
        self.incomplete.remove(&(block, binding));
        Ok(())
    }

    fn verify_inputs<I: BindingSsaIrV1<PhiToken = Token>>(
        &self,
        ir: &I,
        inputs: &[(BasicBlockId, ValueId)],
    ) -> Result<(), BindingSsaErrorV1> {
        for (predecessor, value) in inputs.iter().copied() {
            ir.verify_phi_input(predecessor, value).map_err(|detail| {
                BindingSsaErrorV1::PhiOperation {
                    operation: "verify_input",
                    detail,
                }
            })?;
        }
        Ok(())
    }

    fn abort<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        primary: BindingSsaErrorV1,
        phis: Vec<PhiCleanupV1<Token>>,
    ) -> BindingSsaErrorV1 {
        self.poisoned = true;
        let mut cleanup_failures = Vec::new();
        for phi in phis.into_iter().rev() {
            if let Err(error) = ir.rollback_phi(phi.token) {
                cleanup_failures.push(error);
            }
        }
        if cleanup_failures.is_empty() {
            primary
        } else {
            BindingSsaErrorV1::DuringPhiCleanup {
                primary: Box::new(primary),
                cleanup_failures: cleanup_failures.into_boxed_slice(),
            }
        }
    }

    fn require_usable(&self) -> Result<(), BindingSsaErrorV1> {
        (!self.poisoned)
            .then_some(())
            .ok_or(BindingSsaErrorV1::Poisoned)
    }

    fn require_binding(&self, binding: BindingRefV1) -> Result<(), BindingSsaErrorV1> {
        if binding.owner() == self.owner {
            Ok(())
        } else {
            Err(BindingSsaErrorV1::ForeignBinding {
                expected: self.owner,
                actual: binding.owner(),
            })
        }
    }
}

fn forget_cleanup<Token: Copy + Eq>(pending: &mut Vec<PhiCleanupV1<Token>>, token: Token) {
    if let Some(index) = pending.iter().rposition(|phi| phi.token == token) {
        pending.remove(index);
    }
}
