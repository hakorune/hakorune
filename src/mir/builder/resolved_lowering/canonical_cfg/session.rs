use super::error::{CanonicalCfgBlockRoleV1, CanonicalCfgErrorV1};
use super::predecessors::derive_and_verify_predecessors;
use crate::mir::builder::MirBuilder;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::collections::BTreeMap;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedPredecessorsV1 {
    block: BasicBlockId,
    predecessors: Box<[BasicBlockId]>,
}

impl VerifiedPredecessorsV1 {
    pub(in crate::mir::builder) fn block(&self) -> BasicBlockId {
        self.block
    }

    pub(in crate::mir::builder) fn predecessors(&self) -> &[BasicBlockId] {
        &self.predecessors
    }

    #[cfg(test)]
    pub(in crate::mir::builder) fn from_test_parts(
        block: BasicBlockId,
        mut predecessors: Vec<BasicBlockId>,
    ) -> Self {
        predecessors.sort_unstable();
        predecessors.dedup();
        Self {
            block,
            predecessors: predecessors.into_boxed_slice(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub(in crate::mir::builder) struct VerifiedCanonicalCfgV1 {
    blocks: Box<[VerifiedPredecessorsV1]>,
}

impl VerifiedCanonicalCfgV1 {
    pub(in crate::mir::builder) fn blocks(&self) -> &[VerifiedPredecessorsV1] {
        &self.blocks
    }
}

#[derive(Debug, Default)]
pub(in crate::mir::builder) struct CanonicalCfgSessionV1 {
    sealed: BTreeMap<BasicBlockId, VerifiedPredecessorsV1>,
}

impl CanonicalCfgSessionV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self::default()
    }

    pub(in crate::mir::builder) fn create_block(
        &self,
        function: &mut MirFunction,
        block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if function.get_block(block).is_some() {
            return Err(CanonicalCfgErrorV1::BlockAlreadyExists { block });
        }
        function.add_block(BasicBlock::new(block));
        Ok(())
    }

    pub(in crate::mir::builder) fn select_block(
        &self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let function = builder.function_state.current_function.as_ref().ok_or(
            CanonicalCfgErrorV1::MissingBlock {
                block,
                role: CanonicalCfgBlockRoleV1::Source,
            },
        )?;
        if function.get_block(block).is_none() {
            return Err(CanonicalCfgErrorV1::MissingBlock {
                block,
                role: CanonicalCfgBlockRoleV1::Source,
            });
        }
        builder
            .start_new_block(block)
            .map_err(|_| CanonicalCfgErrorV1::MissingBlock {
                block,
                role: CanonicalCfgBlockRoleV1::Source,
            })
    }

    pub(in crate::mir::builder) fn emit_jump(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        target: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_edge(function, source, &[target])?;
        function
            .get_block_mut(source)
            .expect("source was checked")
            .set_terminator(MirInstruction::Jump {
                target,
                edge_args: None,
            });
        function
            .get_block_mut(target)
            .expect("target was checked")
            .add_predecessor(source);
        Ok(())
    }

    pub(in crate::mir::builder) fn emit_branch(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        condition: ValueId,
        then_block: BasicBlockId,
        else_block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if then_block == else_block {
            return Err(CanonicalCfgErrorV1::DuplicateEdge {
                source,
                target: then_block,
            });
        }
        self.preflight_edge(function, source, &[then_block, else_block])?;
        function
            .get_block_mut(source)
            .expect("source was checked")
            .set_terminator(MirInstruction::Branch {
                condition,
                then_bb: then_block,
                else_bb: else_block,
                then_edge_args: None,
                else_edge_args: None,
            });
        for target in [then_block, else_block] {
            function
                .get_block_mut(target)
                .expect("target was checked")
                .add_predecessor(source);
        }
        Ok(())
    }

    pub(in crate::mir::builder) fn emit_return(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        value: Option<ValueId>,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        function
            .get_block_mut(source)
            .expect("return source was checked")
            .set_terminator(MirInstruction::Return { value });
        Ok(())
    }

    pub(in crate::mir::builder) fn seal_block(
        &mut self,
        function: &mut MirFunction,
        block: BasicBlockId,
    ) -> Result<VerifiedPredecessorsV1, CanonicalCfgErrorV1> {
        let raw = function
            .get_block(block)
            .ok_or(CanonicalCfgErrorV1::MissingBlock {
                block,
                role: CanonicalCfgBlockRoleV1::Seal,
            })?;
        if raw.is_sealed() || self.sealed.contains_key(&block) {
            return Err(CanonicalCfgErrorV1::SealTwice { block });
        }

        let predecessors = derive_and_verify_predecessors(function)?;
        let witness = VerifiedPredecessorsV1 {
            block,
            predecessors: predecessors
                .get(&block)
                .expect("all function blocks have predecessor entries")
                .iter()
                .copied()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        };
        function
            .get_block_mut(block)
            .expect("seal block was checked")
            .seal();
        self.sealed.insert(block, witness.clone());
        Ok(witness)
    }

    pub(in crate::mir::builder) fn finish(
        self,
        function: &MirFunction,
    ) -> Result<VerifiedCanonicalCfgV1, CanonicalCfgErrorV1> {
        let predecessors = derive_and_verify_predecessors(function)?;

        for block in function.block_ids() {
            let Some(witness) = self.sealed.get(&block) else {
                return Err(CanonicalCfgErrorV1::UnsealedBlockAtFinish { block });
            };
            if !function
                .get_block(block)
                .expect("block id came from function")
                .is_sealed()
            {
                return Err(CanonicalCfgErrorV1::SealStateMismatch { block });
            }
            let current = predecessors
                .get(&block)
                .expect("all function blocks have predecessor entries")
                .iter()
                .copied()
                .collect::<Vec<_>>()
                .into_boxed_slice();
            if witness.predecessors.as_ref() != current.as_ref() {
                return Err(CanonicalCfgErrorV1::SealedPredecessorsChanged {
                    block,
                    sealed: witness.predecessors.clone(),
                    current,
                });
            }
        }

        for block in self.sealed.keys().copied() {
            if function.get_block(block).is_none() {
                return Err(CanonicalCfgErrorV1::SealedBlockRemoved { block });
            }
        }

        Ok(VerifiedCanonicalCfgV1 {
            blocks: self
                .sealed
                .into_values()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    fn preflight_edge(
        &self,
        function: &MirFunction,
        source: BasicBlockId,
        targets: &[BasicBlockId],
    ) -> Result<(), CanonicalCfgErrorV1> {
        let source_block = function
            .get_block(source)
            .ok_or(CanonicalCfgErrorV1::MissingBlock {
                block: source,
                role: CanonicalCfgBlockRoleV1::Source,
            })?;
        for target in targets.iter().copied() {
            function
                .get_block(target)
                .ok_or(CanonicalCfgErrorV1::MissingBlock {
                    block: target,
                    role: CanonicalCfgBlockRoleV1::Target,
                })?;
        }
        derive_and_verify_predecessors(function)?;
        if source_block.is_terminated() {
            return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source });
        }
        for target in targets.iter().copied() {
            let target_block = function.get_block(target).expect("target was checked");
            if target_block.is_sealed() || self.sealed.contains_key(&target) {
                return Err(CanonicalCfgErrorV1::EdgeAfterSeal { source, target });
            }
        }
        Ok(())
    }

    fn preflight_terminator(
        &self,
        function: &MirFunction,
        source: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let source_block = function
            .get_block(source)
            .ok_or(CanonicalCfgErrorV1::MissingBlock {
                block: source,
                role: CanonicalCfgBlockRoleV1::Source,
            })?;
        derive_and_verify_predecessors(function)?;
        if source_block.is_terminated() {
            return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source });
        }
        if source_block.is_sealed() || self.sealed.contains_key(&source) {
            return Err(CanonicalCfgErrorV1::SourceAfterSeal { source });
        }
        Ok(())
    }
}
