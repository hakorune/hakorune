use super::error::{CanonicalCfgBlockRoleV1, CanonicalCfgErrorV1};
use super::predecessors::derive_and_verify_predecessors;
use crate::mir::builder::MirBuilder;
use crate::mir::checked_callout::CheckedCallOutSiteIdV1;
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

    /// Sole canonical CFG writer for the neutral CheckedCallOut terminator.
    /// The function-local site plan is the authority for effect/ABI/shape;
    /// this method only installs the terminator and its two CFG edges.
    pub(in crate::mir::builder) fn emit_checked_callout(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        site_id: CheckedCallOutSiteIdV1,
        receiver: ValueId,
        arguments: Vec<ValueId>,
        normal_landing: BasicBlockId,
        fault_landing: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if source == normal_landing || source == fault_landing {
            return Err(CanonicalCfgErrorV1::CheckedCallOut(
                "source and landing blocks must be distinct".to_owned(),
            ));
        }
        let effects = function
            .metadata
            .checked_callout_plan(site_id)
            .ok_or_else(|| {
                CanonicalCfgErrorV1::CheckedCallOut(format!(
                    "missing function-local site plan for {site_id:?}"
                ))
            })?
            .effects();
        self.preflight_edge(function, source, &[normal_landing, fault_landing])?;
        function
            .metadata
            .checked_callout_plan(site_id)
            .expect("site plan remained present")
            .validate_instruction(site_id, normal_landing, fault_landing, effects)
            .map_err(|error| CanonicalCfgErrorV1::CheckedCallOut(format!("{error:?}")))?;
        function
            .get_block_mut(source)
            .expect("source was checked")
            .set_terminator(MirInstruction::CheckedCallOut {
                site_id,
                receiver,
                arguments,
                normal_landing,
                fault_landing,
                effects,
            });
        for target in [normal_landing, fault_landing] {
            function
                .get_block_mut(target)
                .expect("target was checked")
                .add_predecessor(source);
        }
        Ok(())
    }

    /// Sole canonical CFG writer for a checked-call Fault landing.  Fault is
    /// a terminal with no successor; it cannot silently rejoin `After` or a
    /// shared normal cleanup block.
    pub(in crate::mir::builder) fn emit_checked_callout_fault(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        site_id: CheckedCallOutSiteIdV1,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        if function.metadata.checked_callout_plan(site_id).is_none() {
            return Err(CanonicalCfgErrorV1::CheckedCallOut(
                "checked callout Fault has no admitted site plan".to_owned(),
            ));
        }
        let block = function
            .get_block_mut(source)
            .expect("Fault source was checked");
        block.set_terminator(MirInstruction::CheckedCallOutFault { site_id });
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::checked_callout::{
        CheckedCallOutEntryIdV1, CheckedCallOutNormalShapeV1, CheckedCallOutSiteIdV1,
        CheckedCallOutSitePlanV1,
    };
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::{EffectMask, MirType};

    fn function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "checked_callout_test".to_owned(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        )
    }

    #[test]
    fn checked_callout_is_the_only_two_edge_cfg_write() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function.add_block(BasicBlock::new(BasicBlockId::new(2)));
        function
            .metadata
            .admit_checked_callout_plan(CheckedCallOutSitePlanV1::from_test(
                CheckedCallOutSiteIdV1(6),
                CheckedCallOutEntryIdV1(17),
                CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1(1),
                },
                EffectMask::READ,
                ModuleInvocationBrandV1::legacy_test(),
            ))
            .expect("plan admission");
        CanonicalCfgSessionV1::new()
            .emit_checked_callout(
                &mut function,
                BasicBlockId::new(0),
                CheckedCallOutSiteIdV1(6),
                ValueId::new(0),
                vec![ValueId::new(1), ValueId::new(2)],
                BasicBlockId::new(1),
                BasicBlockId::new(2),
            )
            .expect("checked callout");
        let source = function.get_block(BasicBlockId::new(0)).unwrap();
        assert_eq!(source.successors.len(), 2);
        assert!(matches!(
            source.terminator,
            Some(MirInstruction::CheckedCallOut { .. })
        ));
        assert_eq!(source.terminator.as_ref().unwrap().dst_value(), None);
    }

    #[test]
    fn checked_callout_rejects_same_normal_and_fault_landing() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function
            .metadata
            .admit_checked_callout_plan(CheckedCallOutSitePlanV1::from_test(
                CheckedCallOutSiteIdV1(7),
                CheckedCallOutEntryIdV1(18),
                CheckedCallOutNormalShapeV1::ImmediateI64,
                EffectMask::READ,
                ModuleInvocationBrandV1::legacy_test(),
            ))
            .expect("plan admission");
        let error = CanonicalCfgSessionV1::new().emit_checked_callout(
            &mut function,
            BasicBlockId::new(0),
            CheckedCallOutSiteIdV1(7),
            ValueId::new(0),
            vec![],
            BasicBlockId::new(1),
            BasicBlockId::new(1),
        );
        assert!(matches!(error, Err(CanonicalCfgErrorV1::CheckedCallOut(_))));
        assert!(function
            .get_block(BasicBlockId::new(0))
            .unwrap()
            .terminator
            .is_none());
    }
}
