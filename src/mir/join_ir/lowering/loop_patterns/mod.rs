//! # Loop Route Lowering Module
//!
//! **Phase 33-12 Modularization**: Split from single 735-line `loop_patterns.rs`
//!
//! ## Structure
//! Each route-focused lowering is a separate sub-module for single responsibility:
//!
//! ### LoopSimpleWhile route (`simple_while.rs`)
//! No break/continue, straightforward loop transformation
//!
//! ### LoopBreak route (`with_break.rs`)
//! Conditional break → two exit paths (natural + break)
//!
//! ### IfPhiJoin route (`with_if_phi.rs`)
//! If-expressions inside loops → PHI merging at exit
//!
//! ### LoopContinueOnly route (`with_continue.rs`) [STUB]
//! Not yet implemented, deferred to Phase 195+
//!
//! ## Design Philosophy: Per-Route Modules
//! Benefits of this structure:
//! - **Testability**: Each route independently testable
//! - **Clarity**: Code for LoopBreak route is in `with_break.rs`, not buried in 735 lines
//! - **Scalability**: Adding a new route is just creating a new file
//! - **Maintainability**: Changes to LoopSimpleWhile don't touch IfPhiJoin
//!
//! ## Shared Utilities
//! Common helper functions are in `mod.rs` for all route modules to use.
//! Examples: constant folding, variable extraction, etc.
//!
//! ## Future: LoopContinueOnly Completion
//! When LoopContinueOnly is implemented (legacy Pattern 4; traceability-only):
//! 1. Modify `with_continue.rs` from stub to full implementation
//! 2. Update dispatch logic if needed
//! 3. Add tests to `loop_patterns/tests/`
//! 4. No changes to other routes needed!
//!
//! # Design Philosophy (Implementation)
//!
//! Route lowering functions are "thin boxes":
//! - Takes input (LoopForm, builder)
//! - Returns Result (success/error)
//! - No side effects outside the builder
//!
//! # Reference
//!
//! Design document: `docs/private/roadmap2/phases/phase-188-joinir-loop-pattern-expansion/design.md`

pub mod simple_while;
pub mod with_break;
pub mod with_continue;
pub mod with_if_phi;
pub mod nested_minimal;  // Phase 188.1

pub use simple_while::lower_simple_while_to_joinir;
pub use with_break::lower_loop_with_break_to_joinir;
pub use with_continue::lower_loop_with_continue_to_joinir;
pub use with_if_phi::lower_loop_with_conditional_phi_to_joinir;
pub use nested_minimal::lower_nested_loop_minimal_to_joinir;  // Phase 188.1

// ============================================================================
// Helper Functions (Shared Utilities)
// ============================================================================

// TODO: Implement helper functions for extraction and translation
// These will be shared across all 4 route modules:
//
// 1. extract_carriers_from_header_phi(loop_form) -> Vec<CarrierVar>
// 2. extract_loop_condition_from_header(loop_form) -> ValueId
// 3. extract_body_instructions(loop_form) -> Vec<MirInstruction>
// 4. translate_mir_inst_to_joinir(inst, lowerer) -> JoinInst
// 5. find_break_block(loop_form) -> BasicBlockId
// 6. extract_break_condition(block) -> ValueId
// 7. find_if_else_block(loop_form) -> BasicBlockId
// 8. extract_if_condition(block) -> ValueId
// 9. extract_then_value(block) -> ValueId
// 10. extract_else_value(block) -> ValueId

#[cfg(test)]
mod tests {

    // ========================================================================
    // LoopSimpleWhile route tests
    // ========================================================================

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_simple_while_lowering_success() {
        // TODO: Add integration test for LoopSimpleWhile route lowering
        // Step 1: Create mock LoopForm for LoopSimpleWhile route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_simple_while_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_simple_while_rejects_break() {
        // TODO: Add test that rejects loop with break
        // Step 1: Create mock LoopForm with break
        // Step 2: Call lower_simple_while_to_joinir()
        // Step 3: Assert returns None (unsupported route shape)
    }

    // ========================================================================
    // LoopBreak route tests
    // ========================================================================

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_break_lowering_success() {
        // TODO: Add integration test for LoopBreak route lowering
        // Step 1: Create mock LoopForm for LoopBreak route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_loop_with_break_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure (two Jumps to k_exit)
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_break_exit_phi_correct() {
        // TODO: Add test that verifies k_exit receives correct exit values
        // Step 1: Create mock LoopForm for LoopBreak route
        // Step 2: Call lower_loop_with_break_to_joinir()
        // Step 3: Verify k_exit params = [i_exit]
        // Step 4: Verify both Jumps pass current i as argument
    }

    // ========================================================================
    // IfPhiJoin: Loop with If-Else PHI Tests
    // ========================================================================

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_if_phi_join_lowering_success() {
        // TODO: Add integration test for IfPhiJoin route lowering
        // Step 1: Create mock LoopForm for IfPhiJoin route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_loop_with_conditional_phi_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure (Select instruction)
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_if_phi_join_multiple_carriers() {
        // TODO: Add test that verifies multiple carrier variables
        // Step 1: Create mock LoopForm with i + sum carriers
        // Step 2: Call lower_loop_with_conditional_phi_to_joinir()
        // Step 3: Verify loop_step params = [i, sum]
        // Step 4: Verify tail Call args = [i_next, sum_new]
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_if_phi_join_lowering_integration() {
        // TODO: Add test that verifies If lowering integration
        // Step 1: Create mock LoopForm with if-else
        // Step 2: Call lower_loop_with_conditional_phi_to_joinir()
        // Step 3: Verify Select instruction is generated
        // Step 4: Verify Select has correct cond/then_val/else_val
    }

    // ========================================================================
    // LoopContinueOnly: Loop with Continue Tests
    // ========================================================================

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_continue_only_lowering_success() {
        // TODO: Add integration test for LoopContinueOnly route lowering
        // Step 1: Create mock LoopForm for LoopContinueOnly route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_loop_with_continue_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure (Jump to loop_step on continue)
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_continue_only_jump_correct() {
        // TODO: Add test that verifies continue jumps to loop_step
        // Step 1: Create mock LoopForm for LoopContinueOnly route
        // Step 2: Call lower_loop_with_continue_to_joinir()
        // Step 3: Verify conditional Jump targets loop_step
        // Step 4: Verify Jump passes current carrier values as arguments
    }

    #[test]
    #[ignore] // TODO: Implement test after lowering logic is complete
    fn test_loop_continue_only_multiple_carriers() {
        // TODO: Add test that verifies multiple carrier variables
        // Step 1: Create mock LoopForm with i + sum carriers
        // Step 2: Call lower_loop_with_continue_to_joinir()
        // Step 3: Verify loop_step params = [i, sum]
        // Step 4: Verify both tail Call and continue Jump use [i_next, sum_next]
    }
}
