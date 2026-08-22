//! Detached pinned-Text Finish writer.
//!
//! This is still part of the canonical CFG substrate: the detached DraftSeal
//! image is allowed one lifecycle marker before the existing Return writer,
//! but no second CFG session or projection-side MIR write is introduced.

use super::error::{CanonicalCfgBlockRoleV1, CanonicalCfgErrorV1};
use crate::mir::pinned_text_residence_lifecycle::TextFormalResidenceIdV1;
use crate::mir::{BasicBlockId, MirFunction, MirInstruction};

pub(in crate::mir::builder::resolved_lowering) fn emit_detached(
    function: &mut MirFunction,
    source: BasicBlockId,
    residence: TextFormalResidenceIdV1,
) -> Result<(), CanonicalCfgErrorV1> {
    let block = function
        .get_block(source)
        .ok_or(CanonicalCfgErrorV1::MissingBlock {
            block: source,
            role: CanonicalCfgBlockRoleV1::Source,
        })?;
    if block.terminator.is_some() {
        return Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { source });
    }
    if !block.is_sealed() {
        return Err(CanonicalCfgErrorV1::PinnedTextResidence(
            "detached Residence Finish requires a sealed exit block".to_owned(),
        ));
    }
    if block.instructions.iter().any(|instruction| {
        matches!(
            instruction,
            MirInstruction::PinnedTextResidenceFinish {
                residence: existing,
            } if *existing == residence
        )
    }) {
        return Err(CanonicalCfgErrorV1::PinnedTextResidence(
            "detached Residence Finish was already emitted for this exit".to_owned(),
        ));
    }
    function
        .get_block_mut(source)
        .expect("detached Residence exit was checked")
        .add_instruction(MirInstruction::PinnedTextResidenceFinish { residence });
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::builder::resolved_lowering::canonical_cfg::CanonicalCfgSessionV1;
    use crate::mir::compiler::pinned_text_backend_frame::PinnedTextBackendFrameContractV1;
    use crate::mir::function::{FunctionSignature, MirFunction};
    use crate::mir::pinned_text_access_plan::PinnedTextAccessPlanTableV1;
    use crate::mir::pinned_text_residence_lifecycle::PreparedPinnedTextResidenceLifecycleV1;
    use crate::mir::resolved_semantics::FunctionOwnerIssuerV1;
    use crate::mir::{BasicBlock, EffectMask, MirType, ValueId};

    fn function() -> MirFunction {
        MirFunction::new(
            FunctionSignature {
                name: "detached_finish_test".to_owned(),
                params: vec![],
                return_type: MirType::Void,
                effects: EffectMask::READ,
            },
            BasicBlockId::new(0),
        )
    }

    fn residence(
        function_owner: crate::mir::resolved_semantics::FunctionOwnerIdV1,
    ) -> TextFormalResidenceIdV1 {
        let plans = PinnedTextAccessPlanTableV1::new(17);
        let frame = PinnedTextBackendFrameContractV1::from_test(function_owner, 17, 1);
        PreparedPinnedTextResidenceLifecycleV1::issue_from_frame(
            function_owner,
            &plans,
            frame.borrow(),
            BasicBlockId::new(1),
            BasicBlockId::new(2),
        )
        .expect("same-cohort carrier")
        .residence()
    }

    #[test]
    fn writes_finish_before_the_existing_return_on_a_detached_sealed_exit() {
        let mut mir = function();
        mir.add_block(BasicBlock::new(BasicBlockId::new(1)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        mir.get_block_mut(BasicBlockId::new(1))
            .expect("exit")
            .seal();

        emit_detached(&mut mir, BasicBlockId::new(1), residence(owner)).expect("Finish");
        mir.get_block_mut(BasicBlockId::new(1))
            .expect("exit")
            .set_terminator(MirInstruction::Return {
                value: Some(ValueId::new(9)),
            });

        let exit = mir.get_block(BasicBlockId::new(1)).expect("exit");
        assert!(matches!(
            exit.instructions.as_slice(),
            [MirInstruction::PinnedTextResidenceFinish { .. }]
        ));
        assert!(matches!(
            exit.terminator,
            Some(MirInstruction::Return { .. })
        ));
    }

    #[test]
    fn rejects_unsealed_terminated_and_duplicate_detached_finish() {
        let mut mir = function();
        mir.add_block(BasicBlock::new(BasicBlockId::new(1)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        let id = residence(owner);
        assert!(matches!(
            emit_detached(&mut mir, BasicBlockId::new(1), id),
            Err(CanonicalCfgErrorV1::PinnedTextResidence(_))
        ));

        mir.get_block_mut(BasicBlockId::new(1))
            .expect("exit")
            .seal();
        emit_detached(&mut mir, BasicBlockId::new(1), id).expect("first Finish");
        assert!(matches!(
            emit_detached(&mut mir, BasicBlockId::new(1), id),
            Err(CanonicalCfgErrorV1::PinnedTextResidence(_))
        ));
        mir.get_block_mut(BasicBlockId::new(1))
            .expect("exit")
            .set_terminator(MirInstruction::Return { value: None });
        assert!(matches!(
            emit_detached(&mut mir, BasicBlockId::new(1), id),
            Err(CanonicalCfgErrorV1::SourceAlreadyTerminated { .. })
        ));
    }

    #[test]
    fn canonical_session_forwards_to_the_detached_writer() {
        let mut mir = function();
        mir.add_block(BasicBlock::new(BasicBlockId::new(1)));
        let owner = FunctionOwnerIssuerV1::new_for_compilation()
            .expect("compilation brand")
            .issue()
            .expect("function owner");
        mir.get_block_mut(BasicBlockId::new(1))
            .expect("exit")
            .seal();
        CanonicalCfgSessionV1::emit_pinned_text_residence_finish_detached(
            &mut mir,
            BasicBlockId::new(1),
            residence(owner),
        )
        .expect("forwarded Finish");
        assert!(matches!(
            mir.get_block(BasicBlockId::new(1))
                .expect("exit")
                .instructions
                .as_slice(),
            [MirInstruction::PinnedTextResidenceFinish { .. }]
        ));
    }
}
