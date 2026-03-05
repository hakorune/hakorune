//! Loop route entrypoints for JoinIR lowering.
//!
//! Phase 2: Extracted from control_flow.rs
//! - loop_simple_while
//! - if_phi_join
//! - loop_continue_recipe (migrated to plan routing in Phase 29ap P8)
//!
//! Phase 29ap P12: Router delegates to plan/composer SSOT (legacy table removed)
//! - Router only coordinates planner/composer adoption
//! - rule-specific logic lives in plan layer
//!
//! Phase 193: AST Feature Extraction Modularization
//! - ast_feature_extractor.rs: Pure function module for analyzing loop AST (thin wrapper)
//! - High reusability for loop analysis tools
//!
//! Phase 193-4 / Phase 222.5-C: Exit Binding Builder
//! - exit_binding.rs: Fully boxified exit binding generation (orchestrator, thin wrapper)
//! - exit_binding_validator.rs: CarrierInfo and ExitMeta validation (thin wrapper)
//! - exit_binding_constructor.rs: Exit binding construction and ValueId allocation (thin wrapper)
//! - exit_binding_applicator.rs: Boundary application logic (thin wrapper)
//! - Eliminates hardcoded variable names and ValueId assumptions
//! - Supports both single and multi-carrier loop routes
//!
//! Phase 33-22: Common Loop Infrastructure
//! - common_init.rs: CommonPatternInitializer for unified initialization (thin wrapper)
//! - conversion_pipeline.rs: JoinIRConversionPipeline for unified conversion flow (moved to plan/)
//!
//! Phase 171-172: Refactoring Infrastructure
//! - loop_scope_shape_builder.rs: Unified LoopScopeShape initialization (Issue 4, thin wrapper)
//! - condition_env_builder.rs: Unified ConditionEnv construction (Issue 5, thin wrapper)
//!
//! Phase 33-23: Pattern-Specific Analyzers (Stage 2)
//!
//! Stage 3 + Issue 1: Trim Route Extraction
//! - trim_pattern_validator.rs: Trim pattern validation and whitespace check generation (moved to plan/)
//! - trim_pattern_lowerer.rs: Trim-specific JoinIR lowering (moved to plan/)
//!
//! Phase 179-B: Generic Loop Framework
//! - pattern_pipeline.rs: Unified preprocessing pipeline for simple loop families (moved to plan/)
//!
//! Phase 91 P5b: Escape Recognizer
//! - escape_pattern_recognizer.rs: P5b (escape sequence handling) detection (thin wrapper)
//! - Extracted from ast_feature_extractor for improved modularity
//!
//! Phase 93/94: Route Policies
//! - policies/: route recognition and routing decision (thin wrapper)
//! - Implementations moved to plan/
//!
//! Phase 255 P2: Common Utilities
//! - common/: Shared helper functions (moved to plan/, thin wrapper)

pub(in crate::mir::builder) mod router;
pub(in crate::mir::builder) mod registry;

pub(crate) mod ast_feature_extractor {
    pub(crate) use crate::mir::builder::control_flow::plan::ast_feature_extractor::*;
}
pub(in crate::mir::builder) mod policies {
    pub(in crate::mir::builder) use crate::mir::builder::control_flow::plan::policies::*;
}

// Re-export router for convenience
pub(in crate::mir::builder) use router::{route_loop, LoopPatternContext, LoopRouteContext};

// Phase 140-P4-A: Re-export for loop_canonicalizer SSOT (crate-wide visibility)
pub(crate) use ast_feature_extractor::detect_skip_whitespace_pattern;

// Phase 104: Re-export read_digits(loop(true)) detection for loop_canonicalizer
pub(crate) use ast_feature_extractor::detect_read_digits_loop_true_pattern;

// Phase 142-P1: Re-export continue pattern detection for loop_canonicalizer
pub(crate) use ast_feature_extractor::detect_continue_pattern;

// Phase 143-P0: Re-export parse_number pattern detection for loop_canonicalizer
pub(crate) use ast_feature_extractor::detect_parse_number_pattern;

// Phase 143-P1: Re-export parse_string pattern detection for loop_canonicalizer
pub(crate) use ast_feature_extractor::detect_parse_string_pattern;

// Phase 91 P5b: Re-export escape skip pattern detection for loop_canonicalizer
pub(crate) use ast_feature_extractor::detect_escape_skip_pattern;
