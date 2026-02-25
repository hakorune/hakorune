use crate::mir::verification_types::VerificationError;
use crate::mir::{function::MirFunction, MirInstruction};

fn extern_name(inst: &MirInstruction) -> Option<&str> {
    match inst {
        MirInstruction::Call {
            callee: Some(crate::mir::Callee::Extern(name)),
            ..
        } => Some(name.as_str()),
        _ => None,
    }
}

/// Ensure that each Await instruction (or Call(callee=Extern(env.future.await))) is immediately
/// preceded and followed by a checkpoint.
/// A checkpoint is either MirInstruction::Safepoint or Call(callee=Extern("env.runtime.checkpoint")).
pub fn check_await_checkpoints(function: &MirFunction) -> Result<(), Vec<VerificationError>> {
    let mut errors = Vec::new();
    let is_cp = |inst: &MirInstruction| match inst {
        MirInstruction::Safepoint => true,
        _ => extern_name(inst) == Some("env.runtime.checkpoint"),
    };
    for (bid, block) in &function.blocks {
        let instrs = &block.instructions;
        for (idx, inst) in instrs.iter().enumerate() {
            let is_await_like = match inst {
                MirInstruction::Await { .. } => true,
                _ => extern_name(inst) == Some("env.future.await"),
            };
            if is_await_like {
                if idx == 0 || !is_cp(&instrs[idx - 1]) {
                    errors.push(VerificationError::MissingCheckpointAroundAwait {
                        block: *bid,
                        instruction_index: idx,
                        position: "before",
                    });
                }
                if idx + 1 >= instrs.len() || !is_cp(&instrs[idx + 1]) {
                    errors.push(VerificationError::MissingCheckpointAroundAwait {
                        block: *bid,
                        instruction_index: idx,
                        position: "after",
                    });
                }
            }
        }
    }
    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}
