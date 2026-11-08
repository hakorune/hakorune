use super::error::{require_block, CanonicalCfgBlockRoleV1, CanonicalCfgErrorV1};
use super::open_instruction_target::{
    CanonicalCfgCreationStateV1, CanonicalOpenInstructionTargetErrorV1,
    VerifiedCanonicalOpenInstructionTargetV1,
};
use super::predecessors::{derive_and_verify_predecessors, TerminatorPredecessorsV1};
use crate::mir::builder::MirBuilder;
use crate::mir::checked_callout::CheckedCallOutSiteIdV1;
use crate::mir::pinned_text_residence_lifecycle::{
    PinnedTextResidenceFinishCapabilityV1, PreparedPinnedTextResidenceLifecycleV1,
    TextFormalResidenceIdV1,
};
use crate::mir::resolved_semantics::FunctionOwnerIdV1;
use crate::mir::{BasicBlock, BasicBlockId, MirFunction, MirInstruction, ValueId};
use std::cell::RefCell;
use std::collections::BTreeMap;

#[path = "session/invoke.rs"]
mod invoke;

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
    pub(super) sealed: BTreeMap<BasicBlockId, VerifiedPredecessorsV1>,
    creation: CanonicalCfgCreationStateV1,
    pub(super) forward_predecessors: RefCell<Option<TerminatorPredecessorsV1>>,
}

impl CanonicalCfgSessionV1 {
    pub(in crate::mir::builder) fn new() -> Self {
        Self::default()
    }

    pub(in crate::mir::builder::resolved_lowering) fn new_for_owner(
        owner: FunctionOwnerIdV1,
    ) -> Self {
        Self {
            sealed: BTreeMap::new(),
            creation: CanonicalCfgCreationStateV1::new_for_owner(owner),
            forward_predecessors: RefCell::default(),
        }
    }

    pub(in crate::mir::builder) fn create_block(
        &mut self,
        function: &mut MirFunction,
        block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if function.get_block(block).is_some() {
            return Err(CanonicalCfgErrorV1::BlockAlreadyExists { block });
        }
        let incoming = self.forward_predecessors(function).remove(&block);
        let mut created = BasicBlock::new(block);
        if let Some(incoming) = incoming {
            created.predecessors = incoming;
        }
        function.add_block(created);
        self.creation.record_created(block);
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn prepare_created_open_instruction_target(
        &self,
        function: &MirFunction,
        owner: FunctionOwnerIdV1,
        block: BasicBlockId,
    ) -> Result<VerifiedCanonicalOpenInstructionTargetV1, CanonicalOpenInstructionTargetErrorV1>
    {
        self.creation
            .prepare_open_target(function, self.sealed.contains_key(&block), owner, block)
    }

    pub(in crate::mir::builder) fn select_block(
        &self,
        builder: &mut MirBuilder,
        block: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        let missing = || CanonicalCfgErrorV1::MissingBlock {
            block,
            role: CanonicalCfgBlockRoleV1::Source,
        };
        let function = builder
            .function_state
            .current_function
            .as_ref()
            .ok_or_else(&missing)?;
        require_block(function, block, CanonicalCfgBlockRoleV1::Source)?;
        builder.start_new_block(block).map_err(|_| missing())
    }

    pub(in crate::mir::builder) fn emit_jump(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        target: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_edge(function, source, &[target])?;
        self.commit_instruction(
            function,
            source,
            MirInstruction::Jump {
                target,
                edge_args: None,
            },
            None,
        );
        Ok(())
    }

    pub(in crate::mir::builder) fn emit_return(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        value: Option<ValueId>,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        self.commit_instruction(function, source, MirInstruction::Return { value }, None);
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
        self.commit_instruction(
            function,
            source,
            MirInstruction::CheckedCallOut {
                site_id,
                receiver,
                arguments,
                normal_landing,
                fault_landing,
                effects,
            },
            None,
        );
        Ok(())
    }

    /// Sole canonical CFG writer for the physical pinned-Text Residence
    /// entry.  The affine carrier has already co-sealed the function-local
    /// frame/plan provenance; this method only installs its two edges.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_enter(
        &mut self,
        function: &mut MirFunction,
        source: BasicBlockId,
        carrier: PreparedPinnedTextResidenceLifecycleV1,
    ) -> Result<PinnedTextResidenceFinishCapabilityV1, CanonicalCfgErrorV1> {
        let (plan, residence, normal_landing, trap_landing) = carrier.into_parts();
        if source == normal_landing || source == trap_landing {
            return Err(CanonicalCfgErrorV1::PinnedTextResidence(
                "source and landing blocks must be distinct".to_owned(),
            ));
        }
        self.preflight_edge(function, source, &[normal_landing, trap_landing])?;
        let trap_block = function
            .get_block(trap_landing)
            .expect("Residence trap target was checked");
        if !trap_block.instructions.is_empty()
            || trap_block.terminator.is_some()
            || !trap_block.predecessors.is_empty()
            || trap_block.is_sealed()
            || self.sealed.contains_key(&trap_landing)
        {
            return Err(CanonicalCfgErrorV1::PinnedTextResidence(
                "Residence trap landing must be empty, unterminated, unsealed, and predecessor-free"
                    .to_owned(),
            ));
        }
        self.commit_instruction(
            function,
            source,
            MirInstruction::PinnedTextResidenceEnter {
                plan,
                normal_landing,
                trap_landing,
            },
            None,
        );
        self.commit_instruction(
            function,
            trap_landing,
            MirInstruction::PinnedTextResidenceTrap { plan },
            None,
        );
        self.seal_block(function, trap_landing)?;
        Ok(PinnedTextResidenceFinishCapabilityV1::from_parts(residence))
    }

    /// Sole canonical instruction writer for the success-only Residence
    /// finish marker.  The caller has already validated the explicit exit
    /// set; this writer only requires the selected block to be unterminated.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_finish(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        residence: TextFormalResidenceIdV1,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        function
            .get_block_mut(source)
            .expect("Residence normal landing was checked")
            .add_instruction(MirInstruction::PinnedTextResidenceFinish { residence });
        Ok(())
    }

    /// Sole canonical writer for a success-only Residence marker on the
    /// detached DraftSeal image. The image has already passed the live CFG
    /// close, so a sealed block is expected; only the absence of a terminator
    /// and the Residence marker itself are admitted here. No second CFG
    /// session or direct MIR writer is allowed to place this instruction.
    pub(in crate::mir::builder::resolved_lowering) fn emit_pinned_text_residence_finish_detached(
        function: &mut MirFunction,
        source: BasicBlockId,
        residence: TextFormalResidenceIdV1,
    ) -> Result<(), CanonicalCfgErrorV1> {
        super::pinned_text_finish::emit_detached(function, source, residence)
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
        let fault_target_matches = function.blocks.values().any(|block| {
            matches!(
                block.terminator.as_ref(),
                Some(MirInstruction::CheckedCallOut {
                    site_id: actual_site,
                    fault_landing,
                    ..
                }) if *actual_site == site_id && *fault_landing == source
            )
        });
        if !fault_target_matches {
            return Err(CanonicalCfgErrorV1::CheckedCallOut(
                "checked callout Fault source is not the admitted site landing".to_owned(),
            ));
        }
        self.commit_instruction(
            function,
            source,
            MirInstruction::CheckedCallOutFault { site_id },
            None,
        );
        Ok(())
    }

    pub(in crate::mir::builder) fn seal_block(
        &mut self,
        function: &mut MirFunction,
        block: BasicBlockId,
    ) -> Result<VerifiedPredecessorsV1, CanonicalCfgErrorV1> {
        let raw = require_block(function, block, CanonicalCfgBlockRoleV1::Seal)?;
        if raw.is_sealed() || self.sealed.contains_key(&block) {
            return Err(CanonicalCfgErrorV1::SealTwice { block });
        }

        let witness = self
            .predecessor_witnesses(function)?
            .into_iter()
            .find(|witness| witness.block == block)
            .expect("seal block was checked");
        function
            .get_block_mut(block)
            .expect("seal block was checked")
            .seal();
        self.sealed.insert(block, witness.clone());
        Ok(witness)
    }

    /// Observe actual edges without closing a block against later writers.
    /// The function owner rechecks these witnesses at its final close.
    pub(in crate::mir::builder) fn predecessor_witnesses(
        &self,
        function: &MirFunction,
    ) -> Result<Vec<VerifiedPredecessorsV1>, CanonicalCfgErrorV1> {
        Ok(derive_and_verify_predecessors(function)?
            .into_iter()
            .map(|(block, incoming)| VerifiedPredecessorsV1 {
                block,
                predecessors: incoming.into_iter().collect(),
            })
            .collect())
    }

    /// Adopt completed prefix/checked-packet blocks and seal open blocks once
    /// all function edges are known. Existing seals must keep their witnesses.
    pub(in crate::mir::builder) fn close_function_blocks(
        &mut self,
        function: &mut MirFunction,
    ) -> Result<Vec<VerifiedPredecessorsV1>, CanonicalCfgErrorV1> {
        let witnesses = self.predecessor_witnesses(function)?;
        self.validate_recorded_seals(function, &witnesses, false)?;
        for witness in &witnesses {
            function
                .get_block_mut(witness.block)
                .expect("witness block")
                .seal();
            self.sealed.insert(witness.block, witness.clone());
        }
        Ok(witnesses)
    }

    pub(in crate::mir::builder) fn finish(
        self,
        function: &MirFunction,
    ) -> Result<VerifiedCanonicalCfgV1, CanonicalCfgErrorV1> {
        let witnesses = self.predecessor_witnesses(function)?;
        self.validate_recorded_seals(function, &witnesses, true)?;

        Ok(VerifiedCanonicalCfgV1 {
            blocks: self
                .sealed
                .into_values()
                .collect::<Vec<_>>()
                .into_boxed_slice(),
        })
    }

    fn validate_recorded_seals(
        &self,
        function: &MirFunction,
        current: &[VerifiedPredecessorsV1],
        require_all: bool,
    ) -> Result<(), CanonicalCfgErrorV1> {
        for witness in current {
            let block = witness.block;
            let Some(previous) = self.sealed.get(&block) else {
                if require_all {
                    return Err(CanonicalCfgErrorV1::UnsealedBlockAtFinish { block });
                }
                continue;
            };
            if !function
                .get_block(block)
                .expect("witness block")
                .is_sealed()
            {
                return Err(CanonicalCfgErrorV1::SealStateMismatch { block });
            }
            if previous != witness {
                return Err(CanonicalCfgErrorV1::SealedPredecessorsChanged {
                    block,
                    sealed: previous.predecessors.clone(),
                    current: witness.predecessors.clone(),
                });
            }
        }
        for block in self.sealed.keys().copied() {
            if function.get_block(block).is_none() {
                return Err(CanonicalCfgErrorV1::SealedBlockRemoved { block });
            }
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering::canonical_cfg) fn preflight_edge(
        &self,
        function: &MirFunction,
        source: BasicBlockId,
        targets: &[BasicBlockId],
    ) -> Result<(), CanonicalCfgErrorV1> {
        let source_block = require_block(function, source, CanonicalCfgBlockRoleV1::Source)?;
        for target in targets.iter().copied() {
            require_block(function, target, CanonicalCfgBlockRoleV1::Target)?;
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
        let source_block = require_block(function, source, CanonicalCfgBlockRoleV1::Source)?;
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
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::module_invocation_identity::ModuleInvocationBrandV1;
    use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
    use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
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
                CheckedCallOutSiteIdV1::from_test(6),
                CheckedCallOutEntryIdV1::from_test(17),
                CheckedCallOutNormalShapeV1::EndAuthorizedHandle {
                    lease_slot: crate::mir::checked_callout::CheckedCallOutLeaseSlotIdV1::from_test(
                        1,
                    ),
                },
                EffectMask::READ,
                ModuleInvocationBrandV1::legacy_test(),
            ))
            .expect("plan admission");
        CanonicalCfgSessionV1::new()
            .emit_checked_callout(
                &mut function,
                BasicBlockId::new(0),
                CheckedCallOutSiteIdV1::from_test(6),
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
                CheckedCallOutSiteIdV1::from_test(7),
                CheckedCallOutEntryIdV1::from_test(18),
                CheckedCallOutNormalShapeV1::ImmediateI64,
                EffectMask::READ,
                ModuleInvocationBrandV1::legacy_test(),
            ))
            .expect("plan admission");
        let error = CanonicalCfgSessionV1::new().emit_checked_callout(
            &mut function,
            BasicBlockId::new(0),
            CheckedCallOutSiteIdV1::from_test(7),
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

    fn residence_carrier(
        owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
        normal: BasicBlockId,
        trap: BasicBlockId,
    ) -> PreparedPinnedTextResidenceLifecycleV1 {
        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame_contract = PinnedTextBackendFrameContractV1::from_test(owner, 17, 1);
        let frame = frame_contract.borrow();
        PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(owner, &plans, frame, normal, trap)
            .expect("test Residence carrier")
    }

    #[test]
    fn pinned_text_residence_enter_and_finish_are_canonical_cfg_writes() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function.add_block(BasicBlock::new(BasicBlockId::new(2)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        let carrier = residence_carrier(owner, BasicBlockId::new(1), BasicBlockId::new(2));
        let mut cfg = CanonicalCfgSessionV1::new();
        let capability = cfg
            .emit_pinned_text_residence_enter(&mut function, BasicBlockId::new(0), carrier)
            .expect("Enter");
        let source = function.get_block(BasicBlockId::new(0)).unwrap();
        assert!(matches!(
            source.terminator,
            Some(MirInstruction::PinnedTextResidenceEnter { .. })
        ));
        assert_eq!(source.successors.len(), 2);
        assert!(source.effects.contains(crate::mir::Effect::Control));
        assert!(source.effects.contains(crate::mir::Effect::Barrier));
        assert!(source.effects.contains(crate::mir::Effect::WriteHeap));

        let trap = function.get_block(BasicBlockId::new(2)).unwrap();
        assert!(matches!(
            trap.terminator,
            Some(MirInstruction::PinnedTextResidenceTrap { .. })
        ));
        assert!(trap.instructions.is_empty());
        assert!(trap.successors.is_empty());
        assert!(trap.is_sealed());
        assert!(trap.effects.contains(crate::mir::Effect::Control));
        assert!(trap.effects.contains(crate::mir::Effect::Panic));

        cfg.emit_pinned_text_residence_finish(
            &mut function,
            BasicBlockId::new(1),
            capability.into_residence(),
        )
        .expect("Finish");
        let normal = function.get_block(BasicBlockId::new(1)).unwrap();
        assert!(matches!(
            normal.instructions.as_slice(),
            [MirInstruction::PinnedTextResidenceFinish { .. }]
        ));
        assert!(normal.effects.contains(crate::mir::Effect::Panic));
        assert!(normal.effects.contains(crate::mir::Effect::Barrier));
    }

    #[test]
    fn pinned_text_residence_rejects_return_before_finish() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function.add_block(BasicBlock::new(BasicBlockId::new(2)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        let carrier = residence_carrier(owner, BasicBlockId::new(1), BasicBlockId::new(2));
        let mut cfg = CanonicalCfgSessionV1::new();
        let capability = cfg
            .emit_pinned_text_residence_enter(&mut function, BasicBlockId::new(0), carrier)
            .expect("Enter");
        cfg.emit_return(&mut function, BasicBlockId::new(1), Some(ValueId::new(9)))
            .expect("Return");
        assert!(matches!(
            cfg.emit_pinned_text_residence_finish(
                &mut function,
                BasicBlockId::new(1),
                capability.into_residence(),
            ),
            Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { .. })
        ));
    }

    #[test]
    fn pinned_text_residence_rejects_nonempty_trap_before_cfg_mutation() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function.add_block(BasicBlock::new(BasicBlockId::new(2)));
        function
            .get_block_mut(BasicBlockId::new(2))
            .unwrap()
            .add_instruction(MirInstruction::KeepAlive {
                values: vec![ValueId::new(9)],
            });
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        let carrier = residence_carrier(owner, BasicBlockId::new(1), BasicBlockId::new(2));
        let mut cfg = CanonicalCfgSessionV1::new();
        let result =
            cfg.emit_pinned_text_residence_enter(&mut function, BasicBlockId::new(0), carrier);
        assert!(matches!(
            result,
            Err(CanonicalCfgErrorV1::PinnedTextResidence(_))
        ));
        assert!(function
            .get_block(BasicBlockId::new(0))
            .unwrap()
            .terminator
            .is_none());
        assert!(function
            .get_block(BasicBlockId::new(2))
            .unwrap()
            .terminator
            .is_none());
    }

    #[test]
    fn pinned_text_residence_trap_is_sealed_against_rejoin_edges() {
        let mut function = function();
        function.add_block(BasicBlock::new(BasicBlockId::new(1)));
        function.add_block(BasicBlock::new(BasicBlockId::new(2)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        let carrier = residence_carrier(owner, BasicBlockId::new(1), BasicBlockId::new(2));
        let mut cfg = CanonicalCfgSessionV1::new();
        let _capability = cfg
            .emit_pinned_text_residence_enter(&mut function, BasicBlockId::new(0), carrier)
            .expect("Enter and Trap");
        let result = cfg.emit_jump(&mut function, BasicBlockId::new(1), BasicBlockId::new(2));
        assert!(matches!(
            result,
            Err(CanonicalCfgErrorV1::EdgeAfterSeal { .. })
        ));
    }
}
