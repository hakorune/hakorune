//! 3-Stage Pipeline: Scan → Plan → Apply
//!
//! Phase 287 P3: Modularize instruction_rewriter.rs 3-stage pipeline
//! Extracted from instruction_rewriter.rs to separate physical files.
//!
//! This module contains the pipeline stages of JoinIR instruction rewriting:
//! 1. **Plan**: Build rewritten blocks (pure transformation)
//! 2. **Apply**: Apply rewritten blocks to MirBuilder
//!
//! # Phase 287 P5: Facade Pattern (Re-export SSOT)
//!
//! Stage functions are re-exported through this module for unified API access.
//! Implementation files use `pub(super)` visibility, keeping this module as the
//! single entry point for the pipeline.

// Module declarations (implementation files)
mod apply;
mod plan;

// Phase 287 P5: Re-export stage functions (facade pattern)
pub(in crate::mir::builder::control_flow::joinir::merge) use apply::apply_rewrites;
pub(in crate::mir::builder::control_flow::joinir::merge) use plan::plan_rewrites;
