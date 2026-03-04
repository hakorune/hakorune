/// Macro to define PlanRuleId enum, PLAN_RULE_ORDER array, and legacy rule-name map.
///
/// This provides a single source of truth for rule variants, their order, and display names.
/// Eliminates manual sync across 3 locations (enum, array, rule_name function).
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

        pub(in crate::mir::builder) const PLAN_RULE_ORDER: &[PlanRuleId] = &[
            $(
                PlanRuleId::$variant
            ),*
        ];

        pub(in crate::mir::builder) fn planner_rule_legacy_name(id: PlanRuleId) -> &'static str {
            match id {
                $(PlanRuleId::$variant => $name,)*
            }
        }
    };
}

// Define plan rules with unified enum, order array, and name function
define_plan_rules! {
    // Phase 273
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

impl PlanRuleId {
    // Compatibility aliases kept during Phase D cleanup.
    // These will be removed when all callsites/tests stop using PatternN names.
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern1: Self = Self::LoopSimpleWhile;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern2: Self = Self::LoopBreakRecipe;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern3: Self = Self::IfPhiJoin;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern4: Self = Self::LoopContinueRecipe;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern5: Self = Self::LoopTrueEarlyExit;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern6: Self = Self::ScanWithInit;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern7: Self = Self::SplitScan;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern8: Self = Self::BoolPredicateScan;
    #[allow(non_upper_case_globals)]
    pub(in crate::mir::builder) const Pattern9: Self = Self::AccumConstLoop;
}

/// Preferred rule label used in planner entry logs.
///
/// D1 policy: human-facing labels are semantic (Pattern-number free) by default.
/// Legacy labels remain available via `planner_rule_legacy_name`.
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
    fn legacy_pattern_alias_constants_are_preserved() {
        assert_eq!(PlanRuleId::Pattern2, PlanRuleId::LoopBreakRecipe);
        assert_eq!(PlanRuleId::Pattern1, PlanRuleId::LoopSimpleWhile);
        assert_eq!(PlanRuleId::Pattern8, PlanRuleId::BoolPredicateScan);
    }
}
