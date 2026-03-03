/// Macro to define PlanRuleId enum, PLAN_RULE_ORDER array, and rule_name() function.
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

        pub(in crate::mir::builder) fn rule_name(id: PlanRuleId) -> &'static str {
            match id {
                $(PlanRuleId::$variant => $name,)*
            }
        }
    };
}

// Define plan rules with unified enum, order array, and name function
define_plan_rules! {
    // Phase 273
    Pattern6 => "Pattern6_ScanWithInit (Phase 273)";
    Pattern7 => "Pattern7_SplitScan (Phase 273)";

    // Phase 286 P3.2
    Pattern5 => "Pattern5_InfiniteEarlyExit (Phase 286 P3.2)";

    // Phase 29bq P2
    LoopTrueBreak => "LoopTrueBreak (Phase 29bq P2)";
    LoopCondBreak => "LoopCondBreak (Phase 29bq P2)";
    LoopCondContinueOnly => "LoopCondContinueOnly (Phase 29bq P2.x)";
    LoopCondContinueWithReturn => "LoopCondContinueWithReturn (Phase 29bq P2.x)";
    LoopCondReturnInBody => "LoopCondReturnInBody (Phase 29bq P2.x)";

    // Phase 286 P2.4
    Pattern8 => "Pattern8_BoolPredicateScan (Phase 286 P2.4)";

    // Phase 286 P2.6
    Pattern3 => "Pattern3_IfPhi (Phase 286 P2.6)";

    // Phase 286 P2
    Pattern4 => "Pattern4_Continue (Phase 286 P2)";

    // Phase 286 P2.3
    Pattern9 => "Pattern9_AccumConstLoop (Phase 286 P2.3)";

    // Phase 286 P3.1
    Pattern2 => "Pattern2_Break (Phase 286 P3.1)";

    // Phase 286 P2.1
    Pattern1 => "Pattern1_SimpleWhile (Phase 286 P2.1)";
}

/// Semantic rule label (Pattern-number free).
///
/// This is the preferred vocabulary for UI/debug labels and docs.
pub(in crate::mir::builder) fn planner_rule_semantic_label(id: PlanRuleId) -> &'static str {
    match id {
        PlanRuleId::Pattern1 => "LoopSimpleWhile",
        PlanRuleId::Pattern2 => "LoopBreakRecipe",
        PlanRuleId::Pattern3 => "IfPhiJoin",
        PlanRuleId::Pattern4 => "LoopContinueOnly",
        PlanRuleId::Pattern5 => "LoopTrueEarlyExit",
        PlanRuleId::Pattern6 => "ScanWithInit",
        PlanRuleId::Pattern7 => "SplitScan",
        PlanRuleId::Pattern8 => "BoolPredicateScan",
        PlanRuleId::Pattern9 => "AccumConstLoop",
        PlanRuleId::LoopTrueBreak => "LoopTrueBreakContinue",
        PlanRuleId::LoopCondBreak => "LoopExitIfBreakContinue",
        PlanRuleId::LoopCondContinueOnly => "LoopContinueOnly",
        PlanRuleId::LoopCondContinueWithReturn => "LoopContinueWithReturn",
        PlanRuleId::LoopCondReturnInBody => "LoopReturnInBody",
    }
}
