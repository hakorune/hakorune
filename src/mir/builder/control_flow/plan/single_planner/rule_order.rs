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
    match id {
        PlanRuleId::LoopSimpleWhile => "LoopSimpleWhile",
        PlanRuleId::LoopBreakRecipe => "LoopBreakRecipe",
        PlanRuleId::IfPhiJoin => "IfPhiJoin",
        PlanRuleId::LoopContinueRecipe => "LoopContinueOnly",
        PlanRuleId::LoopTrueEarlyExit => "LoopTrueEarlyExit",
        PlanRuleId::ScanWithInit => "ScanWithInit",
        PlanRuleId::SplitScan => "SplitScan",
        PlanRuleId::BoolPredicateScan => "BoolPredicateScan",
        PlanRuleId::AccumConstLoop => "AccumConstLoop",
        PlanRuleId::LoopTrueBreak => "LoopTrueBreakContinue",
        PlanRuleId::LoopCondBreak => "LoopExitIfBreakContinue",
        PlanRuleId::LoopCondContinueOnly => "LoopContinueOnly",
        PlanRuleId::LoopCondContinueWithReturn => "LoopContinueWithReturn",
        PlanRuleId::LoopCondReturnInBody => "LoopReturnInBody",
    }
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
}
