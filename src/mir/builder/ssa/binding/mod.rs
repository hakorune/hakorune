//! Function-owned sealed-block SSA construction for canonical bindings.
//!
//! Resolver bindings and explicitly issued physical PHI destinations share
//! this algorithm; only resolver bindings carry source ownership authority.

mod adapter;
mod error;
mod key;
pub(in crate::mir::builder) use key::{FunctionSsaKeyV1, SsaKeyV1};
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

/// One reaching-definition owner for one canonical function.
#[derive(Debug)]
pub(in crate::mir::builder) struct BindingSsaBuilderV1<Token, Key = BindingRefV1> {
    owner: Option<FunctionOwnerIdV1>,
    definitions: BTreeMap<(BasicBlockId, Key), ValueId>,
    blocks: BTreeMap<BasicBlockId, Option<Box<[BasicBlockId]>>>,
    incomplete: BTreeMap<(BasicBlockId, Key), Token>,
    edge_definitions: BTreeMap<(BasicBlockId, BasicBlockId, Key), ValueId>,
    poisoned: bool,
}

impl<Token: Copy + Eq + std::fmt::Debug> BindingSsaBuilderV1<Token> {
    pub(in crate::mir::builder) fn new(owner: FunctionOwnerIdV1) -> Self {
        Self::with_keys(Some(owner))
    }
}
impl<Token: Copy + Eq + std::fmt::Debug, Key: SsaKeyV1> BindingSsaBuilderV1<Token, Key> {
    pub(in crate::mir::builder) fn with_keys(owner: Option<FunctionOwnerIdV1>) -> Self {
        Self {
            owner,
            definitions: BTreeMap::new(),
            blocks: BTreeMap::new(),
            incomplete: BTreeMap::new(),
            edge_definitions: BTreeMap::new(),
            poisoned: false,
        }
    }

    pub(in crate::mir::builder) fn define(
        &mut self,
        binding: Key,
        block: BasicBlockId,
        value: ValueId,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(binding)?;
        self.blocks.entry(block).or_default();
        self.definitions.insert((block, binding), value);
        Ok(())
    }

    pub(in crate::mir::builder) fn define_edge(
        &mut self,
        binding: Key,
        source: BasicBlockId,
        target: BasicBlockId,
        value: ValueId,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(binding)?;
        if self.blocks.get(&target).is_some_and(Option::is_some) {
            return Err(BindingSsaErrorV1::BlockSealedTwice { block: target });
        }
        self.edge_definitions
            .insert((source, target, binding), value);
        Ok(())
    }

    fn read_edge<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        binding: Key,
        source: BasicBlockId,
        target: BasicBlockId,
        created: &mut Vec<Token>,
    ) -> Result<ValueId, BindingSsaErrorV1> {
        match self
            .edge_definitions
            .get(&(source, target, binding))
            .copied()
        {
            Some(value) => Ok(value),
            None => self.read_recursive(ir, binding, source, created),
        }
    }

    /// Adopt a typed destination issued by the same function's PHI writer.
    /// Its incoming values are still derived by the common sealed-block SSA.
    pub(in crate::mir::builder) fn adopt_provisional_phi(
        &mut self,
        binding: Key,
        block: BasicBlockId,
        value: ValueId,
        token: Token,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(binding)?;
        let state = self.blocks.entry(block).or_default();
        if state.is_some() {
            return Err(BindingSsaErrorV1::BlockSealedTwice { block });
        }
        if self.definitions.contains_key(&(block, binding)) {
            return Err(BindingSsaErrorV1::PhiOperation {
                operation: "adopt",
                detail: "binding already has a definition".to_owned(),
            });
        }
        self.definitions.insert((block, binding), value);
        self.incomplete.insert((block, binding), token);
        Ok(())
    }

    /// Close the function only after every final predecessor witness exists.
    /// A locally completed block must retain its original incoming edge set.
    pub(in crate::mir::builder) fn close<I: BindingSsaIrV1<PhiToken = Token>>(
        mut self,
        ir: &mut I,
        witnesses: &[VerifiedPredecessorsV1],
    ) -> Result<(), BindingSsaErrorV1> {
        for witness in witnesses {
            match self.blocks.get(&witness.block()).and_then(Option::as_ref) {
                Some(previous) if previous.as_ref() == witness.predecessors() => {}
                Some(_) => {
                    return Err(BindingSsaErrorV1::PhiOperation {
                        operation: "close",
                        detail: format!("predecessors changed for {}", witness.block()),
                    })
                }
                None => self.seal(ir, witness.block(), witness)?,
            }
        }
        self.finish()
    }

    pub(in crate::mir::builder) fn read<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        binding: Key,
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
        self.record_predecessors(block, witness, false)?;

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
            affected.push(*incomplete);
            if let Err(primary) = self.complete_incomplete(ir, binding, block, &mut affected) {
                return Err(self.abort(ir, primary, affected));
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn key_for_definition(
        &self,
        block: BasicBlockId,
        value: ValueId,
    ) -> Option<Key> {
        self.definitions
            .iter()
            .find_map(|(&(candidate, key), &definition)| {
                (candidate == block && definition == value).then_some(key)
            })
    }

    pub(in crate::mir::builder) fn complete_adopted<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        key: Key,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
    ) -> Result<Vec<(BasicBlockId, ValueId)>, BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(key)?;
        self.record_predecessors(block, witness, true)?;
        for &source in witness.predecessors() {
            if !self.edge_definitions.contains_key(&(source, block, key)) {
                return Err(self.missing_definition(source, key));
            }
        }
        let token =
            *self
                .incomplete
                .get(&(block, key))
                .ok_or_else(|| BindingSsaErrorV1::PhiOperation {
                    operation: "complete-adopted",
                    detail: "destination is not pending".to_owned(),
                })?;
        let mut affected = vec![token];
        self.complete_incomplete(ir, key, block, &mut affected)
            .map_err(|error| self.abort(ir, error, affected))
    }

    pub(in crate::mir::builder) fn discard_adopted<I: BindingSsaIrV1<PhiToken = Token>>(
        &mut self,
        ir: &mut I,
        key: Key,
        block: BasicBlockId,
    ) -> Result<(), BindingSsaErrorV1> {
        self.require_usable()?;
        self.require_binding(key)?;
        let token =
            *self
                .incomplete
                .get(&(block, key))
                .ok_or_else(|| BindingSsaErrorV1::PhiOperation {
                    operation: "discard-adopted",
                    detail: "destination is not pending".to_owned(),
                })?;
        ir.rollback_phi(token)
            .map_err(|detail| BindingSsaErrorV1::PhiOperation {
                operation: "discard-adopted",
                detail,
            })?;
        self.incomplete.remove(&(block, key));
        self.definitions.remove(&(block, key));
        Ok(())
    }

    fn record_predecessors(
        &mut self,
        block: BasicBlockId,
        witness: &VerifiedPredecessorsV1,
        allow_same: bool,
    ) -> Result<(), BindingSsaErrorV1> {
        if witness.block() != block {
            return Err(BindingSsaErrorV1::WitnessBlockMismatch {
                expected: block,
                actual: witness.block(),
            });
        }
        let state = self.blocks.entry(block).or_default();
        if let Some(previous) = state.as_ref() {
            if allow_same && previous.as_ref() == witness.predecessors() {
                return Ok(());
            }
            return Err(BindingSsaErrorV1::BlockSealedTwice { block });
        }
        *state = Some(witness.predecessors().into());
        Ok(())
    }

    pub(in crate::mir::builder) fn finish(self) -> Result<(), BindingSsaErrorV1> {
        self.validate_finish()
    }

    pub(in crate::mir::builder) fn validate_finish(&self) -> Result<(), BindingSsaErrorV1> {
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
            .iter()
            .filter_map(|(block, state)| state.is_none().then_some(*block))
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
        binding: Key,
        block: BasicBlockId,
        created: &mut Vec<Token>,
    ) -> Result<ValueId, BindingSsaErrorV1> {
        if let Some(value) = self.definitions.get(&(block, binding)).copied() {
            return Ok(value);
        }
        let predecessors = self.blocks.entry(block).or_default().clone();
        let Some(predecessors) = predecessors else {
            let (value, token) = ir.define_provisional_phi(block).map_err(|detail| {
                BindingSsaErrorV1::PhiOperation {
                    operation: "define",
                    detail,
                }
            })?;
            self.definitions.insert((block, binding), value);
            self.incomplete.insert((block, binding), token);
            created.push(token);
            return Ok(value);
        };

        match predecessors.as_ref() {
            [] => Err(self.missing_definition(block, binding)),
            [predecessor] => {
                let value = self.read_edge(ir, binding, *predecessor, block, created)?;
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
                created.push(token);
                let inputs = predecessors
                    .iter()
                    .map(|predecessor| {
                        self.read_edge(ir, binding, *predecessor, block, created)
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
        binding: Key,
        block: BasicBlockId,
        affected: &mut Vec<Token>,
    ) -> Result<Vec<(BasicBlockId, ValueId)>, BindingSsaErrorV1> {
        let token = *self
            .incomplete
            .get(&(block, binding))
            .expect("completion requires an incomplete PHI");
        let predecessors = self.blocks[&block]
            .as_ref()
            .expect("seal installed predecessors")
            .clone();
        if predecessors.is_empty() {
            return Err(self.missing_definition(block, binding));
        }
        let inputs = predecessors
            .iter()
            .map(|predecessor| {
                self.read_edge(ir, binding, *predecessor, block, affected)
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
        Ok(inputs)
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
        phis: Vec<Token>,
    ) -> BindingSsaErrorV1 {
        self.poisoned = true;
        let mut cleanup_failures = Vec::new();
        for phi in phis.into_iter().rev() {
            if let Err(error) = ir.rollback_phi(phi) {
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

    fn missing_definition(&self, block: BasicBlockId, binding: Key) -> BindingSsaErrorV1 {
        match binding.source_binding() {
            Some(binding) => BindingSsaErrorV1::MissingDefinition { block, binding },
            None => BindingSsaErrorV1::PhiOperation {
                operation: "missing-plan-definition",
                detail: format!("{block}: {binding:?}"),
            },
        }
    }
    fn require_binding(&self, binding: Key) -> Result<(), BindingSsaErrorV1> {
        let Some(source) = binding.source_binding() else {
            return Ok(());
        };
        let expected = self.owner.ok_or_else(|| BindingSsaErrorV1::PhiOperation {
            operation: "binding-owner",
            detail: "source binding has no resolver owner".to_owned(),
        })?;
        if source.owner() == expected {
            Ok(())
        } else {
            Err(BindingSsaErrorV1::ForeignBinding {
                expected,
                actual: source.owner(),
            })
        }
    }
}

fn forget_cleanup<Token: Copy + Eq>(pending: &mut Vec<Token>, token: Token) {
    if let Some(index) = pending.iter().rposition(|phi| *phi == token) {
        pending.remove(index);
    }
}
