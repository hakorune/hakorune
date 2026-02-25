//! Tail Call Lowering Policy Box
//!
//! Phase 131 Task 2: Extracted from instruction_rewriter.rs
//!
//! This box encapsulates the policy for lowering tail calls to *skippable continuation functions*.
//! Continuation functions are declared by contract (JoinInlineBoundary), and only those that are
//! structurally skippable (pure exit stubs) are lowered to Jump(exit_block_id).
//!
//! # Responsibility
//!
//! - Detect tail calls to skippable continuation functions
//! - Convert them to Jump(exit_block_id) instructions
//! - This is "exit edge normalization" - ensuring continuation exits become proper jumps
//!
//! # Non-Responsibility (handled by caller)
//!
//! - Exit value collection (handled by ExitArgsCollectorBox)
//! - Remapping of ValueIds (handled by JoinIrIdRemapper)
//! - Instruction rewriting of non-k_exit instructions (handled by instruction_rewriter)

#![allow(dead_code)]

use crate::mir::{BasicBlockId, MirInstruction, ValueId};
use std::collections::BTreeSet;

/// Policy box for tail call lowering (k_exit special case)
///
/// # Phase 131 Task 2 Contract
///
/// - **Input**: Call instruction, function name, args
/// - **Output**: `Option<LoweringDecision>` - None if not k_exit, Some(decision) if k_exit
/// - **Invariant**: Stateless (no mutable state)
pub struct TailCallLoweringPolicyBox {
    skippable_continuation_func_names: BTreeSet<String>,
}

/// Decision for how to lower a tail call
#[derive(Debug, Clone)]
pub enum LoweringDecision {
    /// This is a k_exit tail call - normalize to exit jump
    NormalizeToExitJump {
        /// Arguments passed to k_exit (will be used for exit value collection)
        args: Vec<ValueId>,
    },
    /// Not a k_exit call - handle normally (convert to Jump to target block)
    NormalTailCall,
}

impl TailCallLoweringPolicyBox {
    /// Create new policy box
    pub fn new(skippable_continuation_func_names: BTreeSet<String>) -> Self {
        Self {
            skippable_continuation_func_names,
        }
    }

    /// Classify a tail call instruction
    ///
    /// # Arguments
    ///
    /// - `callee_name`: Name of the called function
    /// - `args`: Arguments passed to the call
    ///
    /// # Returns
    ///
    /// - `Some(LoweringDecision)` if this is a special tail call requiring policy decision
    /// - `None` if this is not a tail call or not relevant to this policy
    pub fn classify_tail_call(
        &self,
        callee_name: &str,
        args: &[ValueId],
    ) -> Option<LoweringDecision> {
        if self.skippable_continuation_func_names.contains(callee_name) {
            // This is a skippable continuation tail call - normalize to exit jump
            Some(LoweringDecision::NormalizeToExitJump {
                args: args.to_vec(),
            })
        } else {
            // Not a skippable continuation - caller will handle as normal tail call (Jump to entry block)
            None
        }
    }

    /// Generate exit jump instruction for k_exit tail call
    ///
    /// # Arguments
    ///
    /// - `exit_block_id`: The shared exit block ID (from block_allocator)
    ///
    /// # Returns
    ///
    /// - MirInstruction::Jump targeting the exit block
    ///
    /// # Contract
    ///
    /// - This is the ONLY instruction type that should be emitted for k_exit tail calls
    /// - Caller must collect exit values separately (via ExitArgsCollectorBox)
    pub fn rewrite_to_exit_jump(&self, exit_block_id: BasicBlockId) -> MirInstruction {
        MirInstruction::Jump {
            target: exit_block_id,
            edge_args: None,
        }
    }

    /// Verify that the generated terminator matches the expected contract (strict mode only)
    ///
    /// # Arguments
    ///
    /// - `terminator`: The terminator instruction to verify
    /// - `exit_block_id`: Expected exit block ID
    ///
    /// # Returns
    ///
    /// - `Ok(())` if verification passes
    /// - `Err(msg)` if contract violation detected
    ///
    /// # Contract
    ///
    /// In strict mode, skippable continuation tail calls MUST become Jump(exit_block_id).
    /// Any other terminator is a contract violation.
    pub fn verify_exit_jump(
        &self,
        terminator: &MirInstruction,
        exit_block_id: BasicBlockId,
    ) -> Result<(), String> {
        match terminator {
            MirInstruction::Jump { target, .. } if *target == exit_block_id => Ok(()),
            MirInstruction::Jump { target, .. } => Err(crate::mir::join_ir::lowering::error_tags::freeze_with_hint(
                "phase131/k_exit/wrong_jump_target",
                &format!(
                    "skippable continuation tail call lowered to Jump {:?}, expected exit_block_id {:?}",
                    target, exit_block_id
                ),
                "skippable continuation blocks are not merged; ensure calls to skipped continuations become an exit jump to the shared exit block",
            )),
            other => Err(crate::mir::join_ir::lowering::error_tags::freeze_with_hint(
                "phase131/k_exit/not_jump",
                &format!(
                    "skippable continuation tail call resulted in unexpected terminator: {:?}",
                    other
                ),
                "skippable continuations must be lowered to Jump(exit_block_id)",
            )),
        }
    }
}

impl Default for TailCallLoweringPolicyBox {
    fn default() -> Self {
        Self::new(BTreeSet::new())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mir::join_ir_vm_bridge::join_func_name;
    use crate::mir::join_ir::JoinFuncId;

    #[test]
    fn test_classify_skippable_continuation_call() {
        let k_exit_name = join_func_name(JoinFuncId::new(2));
        let policy = TailCallLoweringPolicyBox::new(BTreeSet::from([k_exit_name.clone()]));
        let args = vec![ValueId(1), ValueId(2)];

        let decision = policy.classify_tail_call(&k_exit_name, &args);
        assert!(matches!(
            decision,
            Some(LoweringDecision::NormalizeToExitJump { .. })
        ));
    }

    #[test]
    fn test_classify_normal_call() {
        let policy = TailCallLoweringPolicyBox::new(BTreeSet::new());
        let args = vec![ValueId(1)];

        let decision = policy.classify_tail_call("some_other_function", &args);
        assert!(decision.is_none());
    }

    #[test]
    fn test_rewrite_to_exit_jump() {
        let policy = TailCallLoweringPolicyBox::new(BTreeSet::new());
        let exit_block = BasicBlockId(42);

        let jump = policy.rewrite_to_exit_jump(exit_block);
        assert!(matches!(
            jump,
            MirInstruction::Jump { target, .. } if target == exit_block
        ));
    }

    #[test]
    fn test_verify_exit_jump_success() {
        let policy = TailCallLoweringPolicyBox::new(BTreeSet::new());
        let exit_block = BasicBlockId(42);
        let jump = MirInstruction::Jump {
            target: exit_block,
            edge_args: None,
        };

        let result = policy.verify_exit_jump(&jump, exit_block);
        assert!(result.is_ok());
    }

    #[test]
    fn test_verify_exit_jump_wrong_target() {
        let policy = TailCallLoweringPolicyBox::new(BTreeSet::new());
        let exit_block = BasicBlockId(42);
        let wrong_jump = MirInstruction::Jump {
            target: BasicBlockId(99),
            edge_args: None,
        };

        let result = policy.verify_exit_jump(&wrong_jump, exit_block);
        assert!(result.is_err());
    }
}
