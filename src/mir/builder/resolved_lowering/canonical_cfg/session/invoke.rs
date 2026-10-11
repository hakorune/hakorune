//! Canonical CFG placement for one source-backed fallible I64 call.

use super::{CanonicalCfgErrorV1, CanonicalCfgSessionV1};
use crate::mir::definitions::MirCall;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

impl CanonicalCfgSessionV1 {
    /// Root Home graphs may populate their landing Jumps before the Invoke.
    /// Only the source-backed root algorithm uses this shape; the ordinary
    /// I64 Invoke entry keeps its empty-landing contract.
    pub(in crate::mir::builder::resolved_lowering) fn emit_root_invoke(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        operation: InvokeOperation,
        frame: ValueId,
        normal: BasicBlockId,
        fault: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if source == normal || source == fault || normal == fault {
            return Err(CanonicalCfgErrorV1::Invoke(
                "invalid root landing shape".into(),
            ));
        }
        let cleanup = matches!(
            &operation,
            InvokeOperation::HomeRelease { .. }
                | InvokeOperation::HomeReleaseIfLive { .. }
                | InvokeOperation::OwnedFieldResidenceRelease { .. }
                | InvokeOperation::OwnedObjectFieldRelease { .. }
                | InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::End { .. })
        );
        let ingress = matches!(
            &operation,
            InvokeOperation::Call { call, .. } if call.dst.is_none()
        ) || matches!(
            &operation,
            InvokeOperation::Map(crate::mir::instruction::MapInvokeOperation::CheckedGetI64 { .. })
        );
        if !cleanup && !ingress {
            return Err(CanonicalCfgErrorV1::Invoke(
                "unsupported root operation".into(),
            ));
        }
        self.preflight_edge(function, source, &[normal, fault])?;
        if function
            .get_block(source)
            .is_some_and(|block| block.is_sealed())
            || self.sealed.contains_key(&source)
        {
            return Err(CanonicalCfgErrorV1::SourceAfterSeal { source });
        }
        if !root_frame_is_exact(function, frame) {
            return Err(CanonicalCfgErrorV1::Invoke(
                "root frame is not exactly defined at entry".into(),
            ));
        }
        for (landing, is_normal) in [(normal, true), (fault, false)] {
            let block = function
                .get_block(landing)
                .expect("preflight checked landing");
            if !matches!(
                block.terminator,
                Some(MirInstruction::Jump {
                    edge_args: None,
                    ..
                })
            ) || !block.predecessors.is_empty()
                || (is_normal && cleanup && !block.instructions.is_empty())
                || (!is_normal && !block.instructions.is_empty())
                || (is_normal
                    && ingress
                    && (block.instructions.len() != 1
                        || !matches!(
                            block.instructions.first(),
                            Some(MirInstruction::InvokeNormalResult { invoke_block, .. })
                                if *invoke_block == source
                        )))
            {
                return Err(CanonicalCfgErrorV1::Invoke(
                    "root landing is occupied or foreign".into(),
                ));
            }
        }
        function
            .get_block_mut(source)
            .expect("preflight checked source")
            .set_terminator(MirInstruction::Invoke {
                operation,
                fault_frame: frame,
                normal_landing: normal,
                fault_landing: fault,
            });
        for landing in [normal, fault] {
            function
                .get_block_mut(landing)
                .expect("preflight checked landing")
                .add_predecessor(source);
        }
        Ok(())
    }

    /// The root Fault terminal is emitted before its suffix Invokes. Its
    /// frame and eventual reachability are checked again by the root ledger.
    pub(in crate::mir::builder::resolved_lowering) fn emit_root_fault(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        frame: ValueId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        if !root_frame_is_exact(function, frame) {
            return Err(CanonicalCfgErrorV1::Invoke(
                "root frame is not exactly defined at entry".into(),
            ));
        }
        function
            .get_block_mut(source)
            .expect("preflight checked root Fault")
            .set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn emit_i64_invoke(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        call: MirCall,
        frame: ValueId,
        normal: BasicBlockId,
        fault: BasicBlockId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        if call.dst.is_some() || source == normal || source == fault || normal == fault {
            return Err(CanonicalCfgErrorV1::Invoke(
                "invalid call or landing shape".into(),
            ));
        }
        self.preflight_edge(function, source, &[normal, fault])?;
        for landing in [normal, fault] {
            let block = function
                .get_block(landing)
                .expect("preflight checked landing");
            if !block.instructions.is_empty()
                || block.terminator.is_some()
                || !block.predecessors.is_empty()
            {
                return Err(CanonicalCfgErrorV1::Invoke("landing is not empty".into()));
            }
        }
        function
            .get_block_mut(source)
            .expect("preflight checked source")
            .set_terminator(MirInstruction::Invoke {
                operation: InvokeOperation::Call {
                    call,
                    result: InvokeCallResultKind::I64,
                },
                fault_frame: frame,
                normal_landing: normal,
                fault_landing: fault,
            });
        for landing in [normal, fault] {
            function
                .get_block_mut(landing)
                .expect("preflight checked landing")
                .add_predecessor(source);
        }
        Ok(())
    }

    pub(in crate::mir::builder::resolved_lowering) fn emit_invoke_fault(
        &self,
        function: &mut MirFunction,
        source: BasicBlockId,
        frame: ValueId,
    ) -> Result<(), CanonicalCfgErrorV1> {
        self.preflight_terminator(function, source)?;
        if !function.blocks.values().any(|block| {
            matches!(
                block.terminator.as_ref(),
                Some(MirInstruction::Invoke { fault_frame, fault_landing, .. })
                    if *fault_frame == frame && *fault_landing == source
            )
        }) {
            return Err(CanonicalCfgErrorV1::Invoke("foreign Fault landing".into()));
        }
        function
            .get_block_mut(source)
            .expect("preflight checked Fault")
            .set_terminator(MirInstruction::ReturnFault { fault_frame: frame });
        Ok(())
    }
}

fn root_frame_is_exact(function: &MirFunction, frame: ValueId) -> bool {
    let mut definitions = function.blocks.values().flat_map(|block| {
        block.all_instructions().filter_map(move |instruction| {
            if let MirInstruction::FaultFrameEnter { dst, .. } = instruction {
                Some((block.id, *dst))
            } else {
                None
            }
        })
    });
    matches!(definitions.next(), Some((block, value)) if block == function.entry_block && value == frame)
        && definitions.next().is_none()
}
