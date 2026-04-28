//! Loop route entrypoints for JoinIR lowering.
//!
//! Phase 2: Extracted from control_flow.rs
//! - loop_simple_while
//! - if_phi_join
//! - loop_continue_only (migrated to plan routing in Phase 29ap P8)
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
//! Phase 33-23: Route-Shape Analyzers (Stage 2)
//!
//! Stage 3 + Issue 1: Trim Route Extraction
//! - trim_validator.rs: Trim whitespace validation and helper generation (moved to plan/)
//! - trim_lowerer.rs: Trim-specific JoinIR lowering (moved to plan/)
//!
//! Phase 179-B: Generic Loop Framework
//! - route_prep_pipeline.rs: Unified preprocessing pipeline for simple loop families (moved to plan/)
//!
//! Phase 91 P5b: Escape Route-Shape Recognizer
//! - escape_shape_recognizer.rs: P5b (escape sequence handling) route-shape detection (thin wrapper)
//! - Extracted from ast_feature_extractor for improved modularity
//!
//! Phase 93/94: Route Policies
//! - route-policy compatibility re-exports have been retired
//! - Active route/policy owners are addressed by their physical modules
//!
//! Phase 255 P2: Common Utilities
//! - common/: Shared helper functions (moved to plan/, thin wrapper)

pub(in crate::mir::builder) mod registry;
pub(in crate::mir::builder) mod router;

// Re-export router for convenience
pub(in crate::mir::builder) use router::{route_loop, LoopRouteContext};

// Phase 140-P4-A: Re-export skip_whitespace shape detection for loop_canonicalizer
pub(crate) use crate::mir::builder::control_flow::facts::route_shape_recognizers::skip_whitespace::detect_skip_whitespace_shape;

// Phase 104: Re-export read_digits(loop(true)) shape detection for loop_canonicalizer
pub(crate) use crate::mir::builder::control_flow::facts::route_shape_recognizers::parse_number::detect_read_digits_loop_true_shape;

// Phase 142-P1: Re-export continue shape detection for loop_canonicalizer
pub(crate) use crate::mir::builder::control_flow::facts::route_shape_recognizers::parse_string::detect_continue_shape;

// Phase 143-P0: Re-export parse_number / parse_string shape detection for loop_canonicalizer
pub(crate) use crate::mir::builder::control_flow::facts::route_shape_recognizers::parse_number::detect_parse_number_shape;

// Phase 143-P1:
pub(crate) use crate::mir::builder::control_flow::facts::route_shape_recognizers::parse_string::detect_parse_string_shape;

// Phase 91 P5b: Re-export escape skip shape detection for loop_canonicalizer
pub(crate) use crate::mir::builder::control_flow::facts::escape_shape_recognizer::detect_escape_skip_shape;
