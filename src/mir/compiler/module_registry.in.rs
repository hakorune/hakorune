// Compiler module registry.
//
// This file contains only Cargo/module topology. Semantic authorities and
// compiler entry points remain in the sibling modules and in `mod.rs`.

#[cfg(test)]
pub(crate) mod loop_source_expression_shapes;
pub(crate) mod canonical_source_identity;
pub(in crate::mir) mod dynamic_full_body_source;
#[cfg(test)]
mod dynamic_full_body_source_tests;
pub(in crate::mir) mod function_input;
pub(in crate::mir) mod located;
pub(in crate::mir) mod loop_cond_break_continue_projection;
// Caller-zero Main0 Continue co-seal issuer; sole owner is this row's source
// front. Recheck at the Main0 canonical-root handoff / production caller
// switch slice (active card generic-loop-mainline-source-front-d0).
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice. Sole sealer of the source map product.
// Caller-zero until the same Main0 production caller switch; recheck at the
// canonical-root handoff slice.
// Second bounded Main0 profile (in-body-step); same canonical-root handoff
// and production caller switch as the continue profile.
// Third bounded Main0 profile (derived predicate); same canonical-root
// handoff and production caller switch as the landed profiles.
mod lowering_input;
mod module_session;
pub(in crate::mir) mod pinned_text_backend_frame;
pub(in crate::mir) mod pinned_text_residence_backend_carrier;
pub(crate) mod pinned_text_residence_backend_projection;
#[cfg(test)]
mod loop_cond_break_continue_projection_tests;
#[cfg(test)]
pub(crate) use loop_cond_break_continue_projection_tests::positive_function as loop_cond_function_for_test;
mod normal_default_pipeline;
pub(crate) mod normal_source_plan;
pub(in crate::mir) mod selected_dynamic_w6_activation;
pub(in crate::mir) mod source_projection;
pub(in crate::mir) mod source_view;
pub(crate) mod target_capability;
