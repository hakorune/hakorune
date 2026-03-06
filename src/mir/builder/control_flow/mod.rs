//! Control-flow entrypoints for MIR builder.
//!
//! This module provides the main entry points for control flow constructs:
//! - Block expressions
//! - If/else conditionals
//! - Loops
//! - Try/catch/finally exception handling
//! - Throw statements
//!
//! # Architecture
//!
//! Originally a monolithic 1,632-line file, this module has been modularized
//! into 19 focused submodules for better maintainability and clarity:
//!
//! ## Submodules
//!
//! - `debug` - Debug utilities and tracing
//! - `joinir` - JoinIR integration (patterns, routing, merge)
//!   - `patterns` - Loop pattern implementations (3 patterns)
//!   - `routing` - Pattern routing and dispatch
//!   - `merge` - MIR block merging (5 phases)
//! - `exception` - Exception handling (try/catch/throw)
//! - `utils` - Utility functions (loop variable extraction)
//!
//! ## Modularization History
//!
//! - Phase 1: Debug utilities (debug.rs) ✅
//! - Phase 2: Pattern lowerers (joinir/patterns/) ✅
//! - Phase 3: JoinIR routing (joinir/routing.rs) ✅
//! - Phase 4: Merge implementation (joinir/merge/) ✅
//! - Phase 5: Exception handling (exception/) ✅
//! - Phase 6: Utility functions (utils.rs) ✅
//! - Phase 7: Documentation and cleanup ✅
//!
//! # Design Philosophy
//!
//! All control flow implementations follow a delegation pattern:
//! - Entry points in this file validate and route to submodules
//! - Submodules implement the actual logic
//! - Clear separation of concerns enables easier testing and modification

use super::ValueId;
use crate::ast::ASTNode;

// Phase 1: Debug utilities
pub(in crate::mir::builder) mod debug;

// Phase 2-4: JoinIR integration (patterns, routing, merge)
pub(in crate::mir::builder) mod joinir;

// Phase 5: Exception handling
pub(in crate::mir::builder) mod exception;

// Phase 6: Utility functions
pub(in crate::mir::builder) mod utils;

// Phase 134 P0: Normalization entry point consolidation
pub(in crate::mir::builder) mod normalization;

// Phase 264: EdgeCFG Fragment API (入口SSOT)
pub(in crate::mir::builder) mod edgecfg;

// Phase 273 P0: Plan Extractor (Pure) + PlanLowerer SSOT
pub(in crate::mir::builder) mod plan;

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use joinir::detect_skip_whitespace_shape;

// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use joinir::detect_read_digits_loop_true_shape;

// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use joinir::detect_continue_shape;

// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use joinir::detect_parse_number_shape;
pub(crate) use joinir::detect_parse_string_shape;

// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub(crate) use joinir::detect_escape_skip_pattern;

impl super::MirBuilder {
    /// Control-flow: block
    pub(super) fn cf_block(&mut self, statements: Vec<ASTNode>) -> Result<ValueId, String> {
        // identical to build_block; kept here for future policy hooks
        self.build_block(statements)
    }

    /// Control-flow: if
    ///
    /// # Phase 124: JoinIR-Only (hako_check専用化完了)
    ///
    /// If statements are now always routed through the canonical lowering path
    /// (lower_if_form), which internally uses JoinIR-based PHI generation.
    ///
    /// Phase 123 の環境変数による分岐は削除済み。
    pub(super) fn cf_if(
        &mut self,
        condition: ASTNode,
        then_branch: ASTNode,
        else_branch: Option<ASTNode>,
    ) -> Result<ValueId, String> {
        // Phase 124: JoinIR-only path (環境変数分岐削除)
        // lower_if_form は JoinIR ベースの PHI 生成を使用
        self.lower_if_form(condition, then_branch, else_branch)
    }

    /// Control-flow: loop
    ///
    /// # Phase 49: JoinIR Frontend Mainline Integration
    ///
    /// This is the unified entry point for all loop lowering. All loops are processed
    /// via JoinIR Frontend (Phase 187-2: LoopBuilder removed).
    /// Specific functions are enabled via dev flags (Phase 49):
    ///
    /// - Dev フラグ（既存）:
    ///   - `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`: JsonTokenizer.print_tokens/0
    ///   - `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`: ArrayExtBox.filter/2
    ///
    /// Note: Arity does NOT include implicit `me` receiver.
    pub(super) fn cf_loop(
        &mut self,
        condition: ASTNode,
        body: Vec<ASTNode>,
    ) -> Result<ValueId, String> {
        crate::mir::builder::control_flow::plan::facts::reject_reason::clear_last_plan_reject_detail();

        // Phase 49/80: Try JoinIR Frontend route for mainline targets
        if let Some(result) = self.try_cf_loop_joinir(&condition, &body)? {
            return Ok(result);
        }

        if crate::config::env::builder_loopform_debug() {
            let ring0 = crate::runtime::get_global_ring0();
            ring0.log.debug("[cf_loop] CALLED from somewhere");
            ring0.log.debug("[cf_loop] Current stack (simulated): check build_statement vs build_expression_impl");
        }

        // Phase 186: LoopBuilder Hard Freeze - Legacy path disabled
        // Phase 187-2: LoopBuilder module removed - all loops must use JoinIR
        use crate::mir::join_ir::lowering::error_tags;
        let reject_detail =
            crate::mir::builder::control_flow::plan::facts::reject_reason::take_last_plan_reject_detail();
        let detail_suffix = reject_detail
            .map(|detail| format!("\nDetail: [joinir/reject_detail] {}", detail))
            .unwrap_or_default();
        return Err(error_tags::freeze(&format!(
            "Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.\n\
             Function: {}\n\
             Hint: This loop pattern is not supported. All loops must use JoinIR lowering.{}",
            self.scope_ctx.current_function.as_ref().map(|f| f.signature.name.as_str()).unwrap_or("<unknown>")
            ,
            detail_suffix
        )));
    }

    /// Phase 49: Try JoinIR Frontend for mainline integration
    ///
    /// Returns `Ok(Some(value))` if the loop is successfully lowered via JoinIR,
    /// `Ok(None)` if no JoinIR pattern matched (unsupported loop structure).
    /// Phase 187-2: Legacy LoopBuilder removed - all loops must use JoinIR.
    ///
    /// # Phase 49-4: Multi-target support
    ///
    /// Targets are enabled via separate dev flags:
    /// - `HAKO_JOINIR_PRINT_TOKENS_MAIN=1`: JsonTokenizer.print_tokens/0
    /// - `HAKO_JOINIR_ARRAY_FILTER_MAIN=1`: ArrayExtBox.filter/2
    ///
    /// Note: Arity in function names does NOT include implicit `me` receiver.
    /// - Instance method `print_tokens()` → `/0` (no explicit params)
    /// - Static method `filter(arr, pred)` → `/2` (two params)

    /// Phase 49-3.2: Merge JoinIR-generated MIR blocks into current_function
    ///
    /// **Phase 4 Refactoring Complete**: This function now delegates to the modular
    /// merge implementation in `joinir::merge::merge_joinir_mir_blocks()`.
    ///
    /// The original 714-line implementation has been broken down into 6 focused modules:
    /// 1. block_allocator.rs - Block ID allocation
    /// 2. value_collector.rs - Value collection
    /// 3. ID remapping (using JoinIrIdRemapper)
    /// 4. instruction_rewriter.rs - Instruction rewriting
    /// 5. exit_phi_builder.rs - Exit PHI construction
    /// 6. Boundary reconnection (in merge/mod.rs)
    fn merge_joinir_mir_blocks(
        &mut self,
        mir_module: &crate::mir::MirModule,
        boundary: Option<&crate::mir::join_ir::lowering::inline_boundary::JoinInlineBoundary>,
        debug: bool,
    ) -> Result<Option<ValueId>, String> {
        // Phase 4: Delegate to modular implementation
        joinir::merge::merge_joinir_mir_blocks(self, mir_module, boundary, debug)
    }

    /// Control-flow: try/catch/finally
    ///
    /// Delegates to exception::cf_try_catch for implementation.
    pub(super) fn cf_try_catch(
        &mut self,
        try_body: Vec<ASTNode>,
        catch_clauses: Vec<crate::ast::CatchClause>,
        finally_body: Option<Vec<ASTNode>>,
    ) -> Result<ValueId, String> {
        exception::cf_try_catch(self, try_body, catch_clauses, finally_body)
    }

    /// Phase 188-Impl-2: Extract loop variable name from condition
    ///
    /// For `i < 3`, extracts `i`.
    /// For `arr.length() > 0`, extracts `arr`.
    ///
    /// Delegates to utils::extract_loop_variable_from_condition for implementation.
    fn extract_loop_variable_from_condition(&self, condition: &ASTNode) -> Result<String, String> {
        utils::extract_loop_variable_from_condition(condition)
    }

    /// Control-flow: throw
    ///
    /// Delegates to exception::cf_throw for implementation.
    pub(super) fn cf_throw(&mut self, expression: ASTNode) -> Result<ValueId, String> {
        exception::cf_throw(self, expression)
    }
}
