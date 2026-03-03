//! Planner tag SSOT (stable log strings).
//!
//! Purpose:
//! - Keep `[joinir/planner_first rule=...]` tags stable across refactors.
//! - Prevent TSV expectations from drifting due to incidental formatting changes.

use crate::mir::builder::control_flow::plan::single_planner::{
    planner_rule_semantic_label, PlanRuleId,
};

pub(in crate::mir::builder) fn planner_first_tag(rule_id: PlanRuleId) -> &'static str {
    match rule_id {
        PlanRuleId::Pattern1 => "[joinir/planner_first rule=Pattern1]",
        PlanRuleId::Pattern2 => "[joinir/planner_first rule=Pattern2]",
        PlanRuleId::Pattern3 => "[joinir/planner_first rule=Pattern3]",
        PlanRuleId::Pattern4 => "[joinir/planner_first rule=Pattern4]",
        PlanRuleId::Pattern5 => "[joinir/planner_first rule=Pattern5]",
        PlanRuleId::Pattern6 => "[joinir/planner_first rule=Pattern6]",
        PlanRuleId::Pattern7 => "[joinir/planner_first rule=Pattern7]",
        PlanRuleId::Pattern8 => "[joinir/planner_first rule=Pattern8]",
        PlanRuleId::Pattern9 => "[joinir/planner_first rule=Pattern9]",
        PlanRuleId::LoopTrueBreak => "[joinir/planner_first rule=LoopTrueBreak]",
        PlanRuleId::LoopCondBreak => "[joinir/planner_first rule=LoopCondBreak]",
        PlanRuleId::LoopCondContinueOnly => "[joinir/planner_first rule=LoopCondContinueOnly]",
        PlanRuleId::LoopCondContinueWithReturn => {
            "[joinir/planner_first rule=LoopCondContinueWithReturn]"
        }
        PlanRuleId::LoopCondReturnInBody => "[joinir/planner_first rule=LoopCondReturnInBody]",
    }
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
