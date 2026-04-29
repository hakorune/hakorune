//! # Loop Route Lowering Module
//!
//! **Phase 33-12 Modularization**: split from the historical LoopForm route lowerer.
//!
//! ## Structure
//! LoopForm routes that still own lowering code live in separate sub-modules.
//! Route families owned by the plan/AST path are documented here but do not get
//! compatibility stubs.
//!
//! ### LoopSimpleWhile route (`simple_while.rs`)
//! No break/continue, straightforward loop transformation
//!
//! ### LoopBreak route
//! LoopForm-route lowering is not implemented here. Live LoopBreak lowering is
//! owned by the plan/composer route path and `loop_with_break_minimal.rs`.
//!
//! ### IfPhiJoin route
//! LoopForm-route lowering is not implemented here. Live IfPhiJoin lowering is
//! owned by the plan/AST route path and `loop_with_if_phi_if_sum.rs`.
//!
//! ### LoopContinueOnly route (`with_continue.rs`) [STUB]
//! Not yet implemented, deferred to Phase 195+
//!
//! ## Design Philosophy: Per-Route Modules
//! Benefits of this structure:
//! - **Testability**: Each route independently testable
//! - **Clarity**: live route ownership is explicit at the module boundary
//! - **Scalability**: Adding a new route is just creating a new file
//! - **Maintainability**: Changes to one route module do not touch unrelated routes
//!
//! ## Shared Utilities
//! Common helper functions are in `mod.rs` for all route modules to use.
//! Examples: constant folding, variable extraction, etc.
//!
//! ## Future: LoopContinueOnly Completion
//! When LoopContinueOnly is implemented:
//! 1. Modify `with_continue.rs` from stub to full implementation
//! 2. Update dispatch logic if needed
//! 3. Add tests to the route-specific lowering tests
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

pub mod nested_minimal;
pub mod simple_while;
pub mod with_continue;

pub use nested_minimal::lower_nested_loop_minimal_to_joinir;
pub use simple_while::lower_simple_while_to_joinir;
pub use with_continue::lower_loop_with_continue_to_joinir;

// ============================================================================
// Helper Functions (Shared Utilities)
// ============================================================================

// TODO: Implement helper functions for extraction and translation when a real
// LoopForm route needs them.
//
// 1. extract_carriers_from_header_phi(loop_form) -> Vec<CarrierVar>
// 2. extract_loop_condition_from_header(loop_form) -> ValueId
// 3. extract_body_instructions(loop_form) -> Vec<MirInstruction>
// 4. translate_mir_inst_to_joinir(inst, lowerer) -> JoinInst
// 5. find_if_else_block(loop_form) -> BasicBlockId
// 6. extract_if_condition(block) -> ValueId
// 7. extract_then_value(block) -> ValueId
// 8. extract_else_value(block) -> ValueId

#[cfg(test)]
mod tests {

    // ========================================================================
    // LoopSimpleWhile route tests
    // ========================================================================

    #[test]
    #[ignore] // route test body is still a stub; keep ignored until real JoinIR assertions are wired
    fn test_loop_simple_while_lowering_success() {
        // Placeholder: add integration test for LoopSimpleWhile route lowering
        // Step 1: Create mock LoopForm for LoopSimpleWhile route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_simple_while_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure
    }

    #[test]
    #[ignore] // route test body is still a stub; keep ignored until real JoinIR assertions are wired
    fn test_loop_simple_while_rejects_break() {
        // Placeholder: add test that rejects loop with break
        // Step 1: Create mock LoopForm with break
        // Step 2: Call lower_simple_while_to_joinir()
        // Step 3: Assert returns None (unsupported route shape)
    }

    // ========================================================================
    // LoopContinueOnly: Loop with Continue Tests
    // ========================================================================

    #[test]
    #[ignore] // continue route is still a stub; keep ignored until LoopContinueOnly lowering is implemented
    fn test_loop_continue_only_lowering_success() {
        // Placeholder: add integration test for LoopContinueOnly route lowering
        // Step 1: Create mock LoopForm for LoopContinueOnly route
        // Step 2: Create mock LoopToJoinLowerer
        // Step 3: Call lower_loop_with_continue_to_joinir()
        // Step 4: Assert returns Some(JoinInst)
        // Step 5: Verify generated JoinIR structure (Jump to loop_step on continue)
    }

    #[test]
    #[ignore] // continue route is still a stub; keep ignored until LoopContinueOnly lowering is implemented
    fn test_loop_continue_only_jump_correct() {
        // Placeholder: add test that verifies continue jumps to loop_step
        // Step 1: Create mock LoopForm for LoopContinueOnly route
        // Step 2: Call lower_loop_with_continue_to_joinir()
        // Step 3: Verify conditional Jump targets loop_step
        // Step 4: Verify Jump passes current carrier values as arguments
    }

    #[test]
    #[ignore] // continue route is still a stub; keep ignored until LoopContinueOnly lowering is implemented
    fn test_loop_continue_only_multiple_carriers() {
        // Placeholder: add test that verifies multiple carrier variables
        // Step 1: Create mock LoopForm with i + sum carriers
        // Step 2: Call lower_loop_with_continue_to_joinir()
        // Step 3: Verify loop_step params = [i, sum]
        // Step 4: Verify both tail Call and continue Jump use [i_next, sum_next]
    }
}
