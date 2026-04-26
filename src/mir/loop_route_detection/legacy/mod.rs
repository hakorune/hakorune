//! Legacy loop route detector support modules.
//!
//! Route-shape function entry points were removed from this module. Current
//! route selection is owned by `LoopFeatures -> classify() -> LoopRouteKind`.

#[cfg(test)]
mod tests;

// Phase 170-D: Loop Condition Scope Analysis Boxes
pub mod condition_var_analyzer;
pub mod loop_condition_scope;

// Phase 171-C: LoopBodyLocal Carrier Promotion
pub mod loop_body_carrier_promoter;

// Phase 223-3: LoopBodyLocal Condition Promotion
// (for LoopContinueOnly)
pub mod loop_body_cond_promoter;

// Phase 224: A-4 DigitPos route promotion
pub mod loop_body_digitpos_promoter;

// Phase 171-C-5: Trim route helper
pub mod trim_loop_helper;

// Phase 200-A: Function Scope Capture Infrastructure
pub mod function_scope_capture;

// Phase 79: Pure Detection Logic (Detector/Promoter separation)
pub mod digitpos_detector;
pub mod trim_detector;
