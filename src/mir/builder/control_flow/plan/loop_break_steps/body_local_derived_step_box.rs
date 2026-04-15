//! BodyLocalDerivedStepBox (Phase 94, extracted in Phase 5)
//!
//! Responsibility:
//! - Apply the Phase 94 P5b escape-derived routing to loop_break route inputs.
//! - No JoinIR generation. Purely sets `inputs.body_local_derived_recipe` or fails fast.

use crate::ast::ASTNode;

use super::super::loop_break_prep_box::{LoopBreakDebugLog, LoopBreakPrepInputs};
use crate::mir::builder::control_flow::cleanup::policies::p5b_escape_derived_policy::classify_p5b_escape_derived;
use crate::mir::builder::control_flow::cleanup::policies::PolicyDecision;

pub(crate) struct BodyLocalDerivedStepBox;

impl BodyLocalDerivedStepBox {
    pub(crate) fn apply(
        analysis_body: &[ASTNode],
        inputs: &mut LoopBreakPrepInputs,
        verbose: bool,
    ) -> Result<(), String> {
        match classify_p5b_escape_derived(analysis_body, &inputs.loop_var_name) {
            PolicyDecision::Use(recipe) => {
                LoopBreakDebugLog::new(verbose).log(
                    "phase94",
                    format!(
                        "Phase 94: Enabled BodyLocalDerived for '{}' (counter='{}', pre_delta={}, post_delta={})",
                        recipe.name, recipe.loop_counter_name, recipe.pre_delta, recipe.post_delta
                    ),
                );
                inputs.body_local_derived_recipe = Some(recipe);
                Ok(())
            }
            PolicyDecision::Reject(reason) => Err(format!("[cf_loop/loop_break] {}", reason)),
            PolicyDecision::None => Ok(()),
        }
    }
}
