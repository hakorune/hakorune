//! Stable producer identity shared by the legacy registry and portable artifact.
//!
//! This identity is provenance only. Semantic recipe verification and lowering
//! must not dispatch on it.

use serde::{Deserialize, Serialize};

pub(crate) mod entry_keys {
    pub(crate) const LOOP_BREAK_RECIPE: &str = "loop_break_recipe";
    pub(crate) const IF_PHI_JOIN: &str = "if_phi_join";
    pub(crate) const LOOP_CONTINUE_ONLY: &str = "loop_continue_only";
    pub(crate) const LOOP_TRUE_EARLY_EXIT: &str = "loop_true_early_exit";
    pub(crate) const LOOP_SIMPLE_WHILE: &str = "loop_simple_while";
    pub(crate) const LOOP_CHAR_MAP: &str = "loop_char_map";
    pub(crate) const LOOP_ARRAY_JOIN: &str = "loop_array_join";
    pub(crate) const SCAN_WITH_INIT: &str = "scan_with_init";
    pub(crate) const SPLIT_SCAN: &str = "split_scan";
    pub(crate) const BOOL_PREDICATE_SCAN: &str = "bool_predicate_scan";
    pub(crate) const ACCUM_CONST_LOOP: &str = "accum_const_loop";
    pub(crate) const NESTED_LOOP_MINIMAL: &str = "nested_loop_minimal";
    pub(crate) const LOOP_TRUE_BREAK_CONTINUE: &str = "loop_true_break_continue";
    pub(crate) const LOOP_COND_BREAK_CONTINUE: &str = "loop_cond_break_continue";
    pub(crate) const LOOP_COND_CONTINUE_ONLY: &str = "loop_cond_continue_only";
    pub(crate) const LOOP_COND_CONTINUE_WITH_RETURN: &str = "loop_cond_continue_with_return";
    pub(crate) const LOOP_COND_RETURN_IN_BODY: &str = "loop_cond_return_in_body";
    pub(crate) const GENERIC_LOOP_V0: &str = "generic_loop_v0";
    pub(crate) const GENERIC_LOOP_V1: &str = "generic_loop_v1";
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub(crate) enum LoopRouteId {
    LoopBreakRecipe,
    IfPhiJoin,
    LoopContinueOnly,
    LoopTrueEarlyExit,
    LoopSimpleWhile,
    LoopCharMap,
    LoopArrayJoin,
    ScanWithInit,
    SplitScan,
    BoolPredicateScan,
    AccumConstLoop,
    NestedLoopMinimal,
    LoopTrueBreakContinue,
    LoopCondBreakContinue,
    LoopCondContinueOnly,
    LoopCondContinueWithReturn,
    LoopCondReturnInBody,
    GenericLoopV0,
    GenericLoopV1,
}

impl LoopRouteId {
    pub(crate) const fn as_str(self) -> &'static str {
        match self {
            Self::LoopBreakRecipe => entry_keys::LOOP_BREAK_RECIPE,
            Self::IfPhiJoin => entry_keys::IF_PHI_JOIN,
            Self::LoopContinueOnly => entry_keys::LOOP_CONTINUE_ONLY,
            Self::LoopTrueEarlyExit => entry_keys::LOOP_TRUE_EARLY_EXIT,
            Self::LoopSimpleWhile => entry_keys::LOOP_SIMPLE_WHILE,
            Self::LoopCharMap => entry_keys::LOOP_CHAR_MAP,
            Self::LoopArrayJoin => entry_keys::LOOP_ARRAY_JOIN,
            Self::ScanWithInit => entry_keys::SCAN_WITH_INIT,
            Self::SplitScan => entry_keys::SPLIT_SCAN,
            Self::BoolPredicateScan => entry_keys::BOOL_PREDICATE_SCAN,
            Self::AccumConstLoop => entry_keys::ACCUM_CONST_LOOP,
            Self::NestedLoopMinimal => entry_keys::NESTED_LOOP_MINIMAL,
            Self::LoopTrueBreakContinue => entry_keys::LOOP_TRUE_BREAK_CONTINUE,
            Self::LoopCondBreakContinue => entry_keys::LOOP_COND_BREAK_CONTINUE,
            Self::LoopCondContinueOnly => entry_keys::LOOP_COND_CONTINUE_ONLY,
            Self::LoopCondContinueWithReturn => entry_keys::LOOP_COND_CONTINUE_WITH_RETURN,
            Self::LoopCondReturnInBody => entry_keys::LOOP_COND_RETURN_IN_BODY,
            Self::GenericLoopV0 => entry_keys::GENERIC_LOOP_V0,
            Self::GenericLoopV1 => entry_keys::GENERIC_LOOP_V1,
        }
    }
}

impl std::fmt::Display for LoopRouteId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}
