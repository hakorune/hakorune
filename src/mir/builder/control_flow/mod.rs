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
//! into focused submodules for better maintainability and clarity:
//!
//! ## Submodules
//!
//! - `debug` - Debug utilities and tracing
//! - `joinir` - JoinIR integration (route entry, routing, merge)
//!   - `route_entry` - Active module surface for route entry / registry
//!   - `routing` - Route routing and dispatch
//!   - `merge` - MIR block merging (5 phases)
//! - `exception` - Exception handling (try/catch/throw)
//!
//! ## Modularization History
//!
//! - Phase 1: Debug utilities (debug.rs) ✅
//! - Phase 2: Route entry layer (`joinir::route_entry`) ✅
//! - Phase 3: JoinIR routing (joinir/routing.rs) ✅
//! - Phase 4: Merge implementation (joinir/merge/) ✅
//! - Phase 5: Exception handling (exception/) ✅
//! - Phase 6: Documentation and cleanup ✅
//!
//! # Design Philosophy
//!
//! All control flow implementations follow a delegation pattern:
//! - Entry points in this file validate and route to submodules
//! - Submodules implement the actual logic
//! - Clear separation of concerns enables easier testing and modification

use super::ValueId;
use crate::ast::ASTNode;
use crate::mir::builder::control_flow::verify::diagnostics::planner_reject_detail;

// Phase 1: Debug utilities
pub(in crate::mir::builder) mod debug;

// Phase 2-4: JoinIR integration (route entry, routing, merge)
pub(in crate::mir::builder) mod joinir;

// Phase 5: Exception handling
pub(in crate::mir::builder) mod exception;

// Phase 134 P0: Normalization entry point consolidation
pub(in crate::mir::builder) mod normalization;

// Phase 264: EdgeCFG Fragment API (入口SSOT)
pub(in crate::mir::builder) mod edgecfg;

// Phase 29ca P1: top-level descriptive owner surface (folderization first cut)
pub(in crate::mir::builder) mod facts;

// Phase 29ca P1: top-level recipe/CorePlan owner surface (folderization first cut)
pub(in crate::mir::builder) mod recipes;

// Phase 29ca P1: top-level verifier/observability owner surface (folderization first cut)
pub(in crate::mir::builder) mod verify;

// Phase 29ca P1: top-level lowering/orchestration owner surface (folderization first cut)
pub(in crate::mir::builder) mod lower;

// Phase 29ca P1: top-level SSA/exit-binding owner surface (folderization first cut)
pub(in crate::mir::builder) mod ssa;

// Phase 29ca P1: top-level cleanup/policy owner surface (folderization first cut)
pub(in crate::mir::builder) mod cleanup;

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
pub(crate) use joinir::detect_escape_skip_shape;

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
        planner_reject_detail::clear_last_plan_reject_detail();

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
        let reject_detail = planner_reject_detail::take_last_plan_reject_detail();
        let detail_suffix = reject_detail
            .map(|detail| format!("\nDetail: [joinir/reject_detail] {}", detail))
            .unwrap_or_default();
        return Err(error_tags::freeze(&format!(
            "Loop lowering failed: JoinIR does not support this route shape, and LoopBuilder has been removed.\n\
             Function: {}\n\
             Hint: This loop route shape is not supported. All loops must use JoinIR lowering.{}",
            self.scope_ctx.current_function.as_ref().map(|f| f.signature.name.as_str()).unwrap_or("<unknown>")
            ,
            detail_suffix
        )));
    }

    /// Phase 49: Try JoinIR Frontend for mainline integration
    ///
    /// Returns `Ok(Some(value))` if the loop is successfully lowered via JoinIR,
    /// `Ok(None)` if no JoinIR route matched (unsupported loop structure).
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

    /// Control-flow: throw
    ///
    /// Delegates to exception::cf_throw for implementation.
    pub(super) fn cf_throw(&mut self, expression: ASTNode) -> Result<ValueId, String> {
        exception::cf_throw(self, expression)
    }
}
