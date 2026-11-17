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
pub(in crate::mir::builder) mod runtime_adjacent_shadow_guard;

// Re-export router for convenience
pub(in crate::mir::builder) use router::{route_loop, LoopRouteContext};
