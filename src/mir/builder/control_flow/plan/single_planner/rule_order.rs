/// Macro to define PlanRuleId enum.
///
/// `PLAN_RULE_ORDER` is intentionally declared separately so single_planner can
/// list only DomainPlan-carrying rules while still keeping extra IDs for
/// planner-first tag emission at router level.
macro_rules! define_plan_rules {
    (
        $(
            $(#[$variant_meta:meta])*
            $variant:ident => $name:expr;
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
    ScanWithInit => "Pattern6_ScanWithInit (Phase 273)";
    SplitScan => "Pattern7_SplitScan (Phase 273)";

    // Phase 286 P3.2
    LoopTrueEarlyExit => "Pattern5_InfiniteEarlyExit (Phase 286 P3.2)";

    // Phase 29bq P2
    LoopTrueBreak => "LoopTrueBreak (Phase 29bq P2)";
    LoopCondBreak => "LoopCondBreak (Phase 29bq P2)";
    LoopCondContinueOnly => "LoopCondContinueOnly (Phase 29bq P2.x)";
    LoopCondContinueWithReturn => "LoopCondContinueWithReturn (Phase 29bq P2.x)";
    LoopCondReturnInBody => "LoopCondReturnInBody (Phase 29bq P2.x)";

    // Phase 286 P2.4
    BoolPredicateScan => "Pattern8_BoolPredicateScan (Phase 286 P2.4)";

    // Phase 286 P2.6
    IfPhiJoin => "Pattern3_IfPhi (Phase 286 P2.6)";

    // Phase 286 P2
    LoopContinueRecipe => "Pattern4_Continue (Phase 286 P2)";

    // Phase 286 P2.3
    AccumConstLoop => "Pattern9_AccumConstLoop (Phase 286 P2.3)";

    // Phase 286 P3.1
    LoopBreakRecipe => "Pattern2_Break (Phase 286 P3.1)";

    // Phase 286 P2.1
    LoopSimpleWhile => "Pattern1_SimpleWhile (Phase 286 P2.1)";
}

/// Rule order used by single_planner DomainPlan selection.
///
/// Keep this list limited to rules that can be produced by `DomainPlanKind`.
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
fn planner_rule_legacy_name(id: PlanRuleId) -> &'static str {
    match id {
        PlanRuleId::ScanWithInit => "Pattern6_ScanWithInit (Phase 273)",
        PlanRuleId::SplitScan => "Pattern7_SplitScan (Phase 273)",
        PlanRuleId::LoopTrueEarlyExit => "Pattern5_InfiniteEarlyExit (Phase 286 P3.2)",
        PlanRuleId::LoopTrueBreak => "LoopTrueBreak (Phase 29bq P2)",
        PlanRuleId::LoopCondBreak => "LoopCondBreak (Phase 29bq P2)",
        PlanRuleId::LoopCondContinueOnly => "LoopCondContinueOnly (Phase 29bq P2.x)",
        PlanRuleId::LoopCondContinueWithReturn => "LoopCondContinueWithReturn (Phase 29bq P2.x)",
        PlanRuleId::LoopCondReturnInBody => "LoopCondReturnInBody (Phase 29bq P2.x)",
        PlanRuleId::BoolPredicateScan => "Pattern8_BoolPredicateScan (Phase 286 P2.4)",
        PlanRuleId::IfPhiJoin => "Pattern3_IfPhi (Phase 286 P2.6)",
        PlanRuleId::LoopContinueRecipe => "Pattern4_Continue (Phase 286 P2)",
        PlanRuleId::AccumConstLoop => "Pattern9_AccumConstLoop (Phase 286 P2.3)",
        PlanRuleId::LoopBreakRecipe => "Pattern2_Break (Phase 286 P3.1)",
        PlanRuleId::LoopSimpleWhile => "Pattern1_SimpleWhile (Phase 286 P2.1)",
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
    fn legacy_rule_name_alias_is_preserved() {
        assert_eq!(
            planner_rule_legacy_name(PlanRuleId::LoopBreakRecipe),
            "Pattern2_Break (Phase 286 P3.1)"
        );
        assert_eq!(
            planner_rule_legacy_name(PlanRuleId::LoopSimpleWhile),
            "Pattern1_SimpleWhile (Phase 286 P2.1)"
        );
    }

    #[test]
    fn planner_rule_order_is_domain_plan_only() {
        assert_eq!(PLAN_RULE_ORDER, &[PlanRuleId::LoopCondContinueWithReturn]);
    }
}
