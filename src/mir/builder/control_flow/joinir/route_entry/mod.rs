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
//! Phase 193 / 291x: Route-shape analyzer ownership
//! - aggregate LoopFeatures extraction lives in facts/ast_feature_extractor.rs
//! - shape-specific detector exports below point at their recognizer owners
//!
//! Phase 193-4 / Phase 222.5-C: Exit Binding Builder
//! - exit_binding.rs: Fully boxified exit binding generation (orchestrator, thin wrapper)
//! - exit_binding_validator.rs: CarrierInfo and ExitMeta validation (thin wrapper)
//! - exit_binding_constructor.rs: Exit binding construction and ValueId allocation (thin wrapper)
//! - exit_binding_applicator.rs: Boundary application logic (thin wrapper)
//! - Eliminates hardcoded variable names and ValueId assumptions
//! - Supports both single and multi-carrier loop routes
//!
//! Phase 33-23: Route-Shape Analyzers (Stage 2)
//!
//! Retired helper shelves:
//! - condition_env_builder.rs was deleted in 291x-751 after becoming self-test only.
//! - trim_validator.rs / trim_lowerer.rs were deleted in 291x-751 after becoming self-test only.
//!
//! Phase 91 P5b: Escape Route-Shape Recognizer
//! - escape_shape_recognizer.rs: P5b (escape sequence handling) route-shape detection
//! - Owned outside ast_feature_extractor to keep route-shape entrances explicit
//!
//! Phase 93/94: Route Policies
//! - route-policy compatibility re-exports have been retired
//! - Active route/policy owners are addressed by their physical modules
//!
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
