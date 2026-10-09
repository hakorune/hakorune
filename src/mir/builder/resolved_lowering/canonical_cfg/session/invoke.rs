//! Canonical CFG placement for one source-backed fallible I64 call.

use super::{CanonicalCfgErrorV1, CanonicalCfgSessionV1};
use crate::mir::definitions::MirCall;
use crate::mir::instruction::{InvokeCallResultKind, InvokeOperation};
use crate::mir::{BasicBlockId, MirFunction, MirInstruction, ValueId};

impl CanonicalCfgSessionV1 {
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
