//! Planner tag SSOT (stable log strings).
//!
//! Purpose:
//! - Keep `[joinir/planner_first rule=...]` tags stable across refactors.
//! - Prevent TSV expectations from drifting due to incidental formatting changes.

use crate::mir::builder::control_flow::plan::single_planner::{
    planner_rule_semantic_label, PlanRuleId,
};

fn planner_first_rule_name(rule_id: PlanRuleId) -> &'static str {
    match rule_id {
        // Keep these rule ids pinned for now because gate/TSV expectations still use them.
        PlanRuleId::LoopTrueBreak => "LoopTrueBreak",
        PlanRuleId::LoopCondBreak => "LoopCondBreak",
        PlanRuleId::LoopCondContinueOnly => "LoopCondContinueOnly",
        PlanRuleId::LoopCondContinueWithReturn => "LoopCondContinueWithReturn",
        PlanRuleId::LoopCondReturnInBody => "LoopCondReturnInBody",
        // Pattern-derived ids follow semantic labels by default.
        _ => planner_rule_semantic_label(rule_id),
    }
}

pub(in crate::mir::builder) fn planner_first_tag(rule_id: PlanRuleId) -> String {
    format!(
        "[joinir/planner_first rule={}]",
        planner_first_rule_name(rule_id)
    )
}

pub(in crate::mir::builder) fn planner_first_display_label(rule_id: PlanRuleId) -> &'static str {
    planner_rule_semantic_label(rule_id)
}

pub(in crate::mir::builder) fn planner_first_tag_with_label(rule_id: PlanRuleId) -> String {
    format!(
        "{} label={}",
        planner_first_tag(rule_id),
        planner_first_display_label(rule_id)
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn planner_first_tag_uses_semantic_name_for_pattern_rules() {
        assert_eq!(
            planner_first_tag(PlanRuleId::LoopBreakRecipe),
            "[joinir/planner_first rule=LoopBreakRecipe]"
        );
    }

    #[test]
    fn planner_first_tag_keeps_pinned_rule_name_for_loop_cond_break() {
        assert_eq!(
            planner_first_tag(PlanRuleId::LoopCondBreak),
            "[joinir/planner_first rule=LoopCondBreak]"
        );
    }
}
