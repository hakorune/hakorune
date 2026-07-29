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

// Phase 29ca P1: top-level cleanup/policy owner surface (folderization first cut)
pub(in crate::mir::builder) mod cleanup;

// MIR-CLEAN-007: grouped generic-loop canon owner surface.
pub(in crate::mir::builder) mod generic_loop_canon;

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

    /// Lower an If while retaining the caller's raw child-descent port.
    pub(in crate::mir::builder) fn cf_if_with_port_v1<Port>(
        &mut self,
        port: &mut Port,
        condition: ASTNode,
        then_branch: ASTNode,
        else_branch: Option<ASTNode>,
    ) -> Result<ValueId, String>
    where
        Port: crate::mir::builder::recursive_child_lowering::RawAstChildLoweringPortV1,
    {
        use crate::mir::builder::if_form::IfBranchKindV1;
        use crate::mir::builder::recursive_child_lowering::drive_legacy_expression_v1;

        let condition_value = drive_legacy_expression_v1(self, port, condition)?;
        let has_explicit_else = else_branch.is_some();
        let mut then_branch = Some(then_branch);
        let mut else_branch = else_branch;
        self.lower_if_form_with_condition_value_and_branch_lowerer(
            condition_value,
            None,
            has_explicit_else,
            move |builder, branch| match branch {
                IfBranchKindV1::Then => drive_legacy_expression_v1(
                    builder,
                    port,
                    then_branch
                        .take()
                        .ok_or_else(|| "[if-form/raw-then-demanded-twice]".to_string())?,
                ),
                IfBranchKindV1::Else => drive_legacy_expression_v1(
                    builder,
                    port,
                    else_branch
                        .take()
                        .ok_or_else(|| "[if-form/raw-else-demanded-without-input]".to_string())?,
                ),
            },
        )
    }
}
