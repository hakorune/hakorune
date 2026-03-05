/// Macro to define PlanRuleId enum.
///
/// `PLAN_RULE_ORDER` is intentionally declared separately so single_planner can
/// list only active recipe-entry rules while still keeping extra IDs for
/// planner-first tag emission at router level.
macro_rules! define_plan_rules {
    (
        $(
            $(#[$variant_meta:meta])*
            $variant:ident;
        )*
    ) => {
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub(in crate::mir::builder) enum PlanRuleId {
            $(
                $(#[$variant_meta])*
                $variant
            ),*
        }
    };
}

// Define plan rules with unified enum, order array, and name function
define_plan_rules! {
    // Phase 273 (tag/label compatibility; planner order excludes these)
    ScanWithInit;
    SplitScan;

    // Phase 286 P3.2
    LoopTrueEarlyExit;

    // Phase 29bq P2
    LoopTrueBreak;
    LoopCondBreak;
    LoopCondContinueOnly;
    LoopCondContinueWithReturn;
    LoopCondReturnInBody;

    // Phase 286 P2.4
    BoolPredicateScan;

    // Phase 286 P2.6
    IfPhiJoin;

    // Phase 286 P2
    LoopContinueRecipe;

    // Phase 286 P2.3
    AccumConstLoop;

    // Phase 286 P3.1
    LoopBreakRecipe;

    // Phase 286 P2.1
    LoopSimpleWhile;
}

/// Rule order used by single_planner recipe-entry selection.
///
/// Keep this list limited to rules that can currently be matched as recipe-entry.
pub(in crate::mir::builder) const PLAN_RULE_ORDER: &[PlanRuleId] =
    &[PlanRuleId::LoopCondContinueWithReturn];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(in crate::mir::builder) struct PlannerRuleLabels {
    pub tag_rule: &'static str,
    pub display_label: &'static str,
    pub route_label: &'static str,
}

/// SSOT mapping for planner rule vocabulary.
///
/// Keep tag-rule names stable for gate compatibility, while display/route labels
/// remain semantic and pattern-number free.
pub(in crate::mir::builder) const fn planner_rule_labels(
    id: PlanRuleId,
) -> PlannerRuleLabels {
    match id {
        PlanRuleId::LoopSimpleWhile => PlannerRuleLabels {
            tag_rule: "LoopSimpleWhile",
            display_label: "LoopSimpleWhile",
            route_label: "loop_simple_while",
        },
        PlanRuleId::LoopBreakRecipe => PlannerRuleLabels {
            tag_rule: "LoopBreakRecipe",
            display_label: "LoopBreakRecipe",
            route_label: "loop_break",
        },
        PlanRuleId::IfPhiJoin => PlannerRuleLabels {
            tag_rule: "IfPhiJoin",
            display_label: "IfPhiJoin",
            route_label: "if_phi_join",
        },
        PlanRuleId::LoopContinueRecipe => PlannerRuleLabels {
            tag_rule: "LoopContinueOnly",
            display_label: "LoopContinueOnly",
            route_label: "loop_continue_only",
        },
        PlanRuleId::LoopTrueEarlyExit => PlannerRuleLabels {
            tag_rule: "LoopTrueEarlyExit",
            display_label: "LoopTrueEarlyExit",
            route_label: "loop_true_early_exit",
        },
        PlanRuleId::ScanWithInit => PlannerRuleLabels {
            tag_rule: "ScanWithInit",
            display_label: "ScanWithInit",
            route_label: "scan_with_init",
        },
        PlanRuleId::SplitScan => PlannerRuleLabels {
            tag_rule: "SplitScan",
            display_label: "SplitScan",
            route_label: "split_scan",
        },
        PlanRuleId::BoolPredicateScan => PlannerRuleLabels {
            tag_rule: "BoolPredicateScan",
            display_label: "BoolPredicateScan",
            route_label: "bool_predicate_scan",
        },
        PlanRuleId::AccumConstLoop => PlannerRuleLabels {
            tag_rule: "AccumConstLoop",
            display_label: "AccumConstLoop",
            route_label: "accum_const_loop",
        },
        PlanRuleId::LoopTrueBreak => PlannerRuleLabels {
            tag_rule: "LoopTrueBreak",
            display_label: "LoopTrueBreakContinue",
            route_label: "loop_true_break_continue",
        },
        PlanRuleId::LoopCondBreak => PlannerRuleLabels {
            tag_rule: "LoopCondBreak",
            display_label: "LoopExitIfBreakContinue",
            route_label: "loop_cond_break_continue",
        },
        PlanRuleId::LoopCondContinueOnly => PlannerRuleLabels {
            tag_rule: "LoopCondContinueOnly",
            display_label: "LoopContinueOnly",
            route_label: "loop_cond_continue_only",
        },
        PlanRuleId::LoopCondContinueWithReturn => PlannerRuleLabels {
            tag_rule: "LoopCondContinueWithReturn",
            display_label: "LoopContinueWithReturn",
            route_label: "loop_cond_continue_with_return",
        },
        PlanRuleId::LoopCondReturnInBody => PlannerRuleLabels {
            tag_rule: "LoopCondReturnInBody",
            display_label: "LoopReturnInBody",
            route_label: "loop_cond_return_in_body",
        },
    }
}

pub(in crate::mir::builder) const fn planner_rule_tag_name(id: PlanRuleId) -> &'static str {
    planner_rule_labels(id).tag_rule
}

pub(in crate::mir::builder) const fn planner_rule_route_label(
    id: PlanRuleId,
) -> &'static str {
    planner_rule_labels(id).route_label
}

/// Preferred rule label used in planner entry logs.
///
/// D1 policy: human-facing labels are semantic (Pattern-number free) by default.
/// Legacy labels are intentionally kept in test-only compatibility checks.
pub(in crate::mir::builder) fn rule_name(id: PlanRuleId) -> &'static str {
    planner_rule_semantic_label(id)
}

/// Semantic rule label (Pattern-number free).
///
/// This is the preferred vocabulary for UI/debug labels and docs.
pub(in crate::mir::builder) fn planner_rule_semantic_label(id: PlanRuleId) -> &'static str {
    planner_rule_labels(id).display_label
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rule_name_uses_semantic_label() {
        assert_eq!(rule_name(PlanRuleId::LoopBreakRecipe), "LoopBreakRecipe");
        assert_eq!(
            rule_name(PlanRuleId::LoopCondReturnInBody),
            "LoopReturnInBody"
        );
    }

    #[test]
    fn planner_rule_order_is_single_plan_only() {
        assert_eq!(PLAN_RULE_ORDER, &[PlanRuleId::LoopCondContinueWithReturn]);
    }

    #[test]
    fn planner_rule_labels_keep_tag_display_route_for_loop_continue_recipe() {
        let labels = planner_rule_labels(PlanRuleId::LoopContinueRecipe);
        assert_eq!(labels.tag_rule, "LoopContinueOnly");
        assert_eq!(labels.display_label, "LoopContinueOnly");
        assert_eq!(labels.route_label, "loop_continue_only");
    }
}
