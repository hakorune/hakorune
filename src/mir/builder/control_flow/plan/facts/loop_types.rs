//! Core types for LoopFacts
//!
//! Contains the main structs: LoopFacts, ScanWithInitFacts, SplitScanFacts

use super::scan_shapes::{ConditionShape, SplitScanShape, StepShape};
use super::skeleton_facts::SkeletonFacts;
use super::feature_facts::LoopFeatureFacts;
use super::pattern1_simplewhile_facts::Pattern1SimpleWhileFacts;
use super::pattern1_char_map_facts::Pattern1CharMapFacts;
use super::pattern1_array_join_facts::Pattern1ArrayJoinFacts;
use super::pattern_is_integer_facts::PatternIsIntegerFacts;
use super::pattern_starts_with_facts::PatternStartsWithFacts;
use super::pattern_int_to_str_facts::PatternIntToStrFacts;
use super::pattern_escape_map_facts::PatternEscapeMapFacts;
use super::pattern_split_lines_facts::PatternSplitLinesFacts;
use super::pattern_skip_ws_facts::PatternSkipWsFacts;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use super::pattern3_ifphi_facts::Pattern3IfPhiFacts;
use super::pattern4_continue_facts::Pattern4ContinueFacts;
use super::pattern5_infinite_early_exit_facts::Pattern5InfiniteEarlyExitFacts;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::LoopTrueBreakContinueFacts;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::plan::loop_scan_v0::LoopScanV0Facts;
use crate::mir::builder::control_flow::plan::loop_scan_methods_v0::LoopScanMethodsV0Facts;
use crate::mir::builder::control_flow::plan::loop_scan_methods_block_v0::LoopScanMethodsBlockV0Facts;
use crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::LoopScanPhiVarsV0Facts;
use crate::mir::builder::control_flow::plan::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Facts;
use crate::mir::builder::control_flow::plan::loop_bundle_resolver_v0::LoopBundleResolverV0Facts;
use super::pattern6_nested_minimal_facts::Pattern6NestedMinimalFacts;
use super::pattern8_bool_predicate_scan_facts::Pattern8BoolPredicateScanFacts;
use super::pattern9_accum_const_loop_facts::Pattern9AccumConstLoopFacts;
use super::pattern2_break_types::Pattern2BreakFacts;
use super::pattern2_loopbodylocal_facts::Pattern2LoopBodyLocalFacts;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopFacts {
    pub condition_shape: ConditionShape,
    pub step_shape: StepShape,
    pub skeleton: SkeletonFacts,
    pub features: LoopFeatureFacts,
    pub scan_with_init: Option<ScanWithInitFacts>,
    pub split_scan: Option<SplitScanFacts>,
    pub pattern1_simplewhile: Option<Pattern1SimpleWhileFacts>,
    pub pattern1_char_map: Option<Pattern1CharMapFacts>,
    pub pattern1_array_join: Option<Pattern1ArrayJoinFacts>,
    pub pattern_is_integer: Option<PatternIsIntegerFacts>,
    pub pattern_starts_with: Option<PatternStartsWithFacts>,
    pub pattern_int_to_str: Option<PatternIntToStrFacts>,
    pub pattern_escape_map: Option<PatternEscapeMapFacts>,
    pub pattern_split_lines: Option<PatternSplitLinesFacts>,
    pub pattern_skip_ws: Option<PatternSkipWsFacts>,
    pub generic_loop_v0: Option<GenericLoopV0Facts>,
    pub generic_loop_v1: Option<GenericLoopV1Facts>,
    pub pattern3_ifphi: Option<Pattern3IfPhiFacts>,
    pub pattern4_continue: Option<Pattern4ContinueFacts>,
    pub pattern5_infinite_early_exit: Option<Pattern5InfiniteEarlyExitFacts>,
    pub loop_true_break_continue: Option<LoopTrueBreakContinueFacts>,
    /// Note: cluster3/4/5 are selected via nested_loop_profile table
    pub loop_cond_break_continue: Option<LoopCondBreakContinueFacts>,
    pub loop_cond_continue_only: Option<LoopCondContinueOnlyFacts>,
    pub loop_cond_continue_with_return: Option<LoopCondContinueWithReturnFacts>,
    pub loop_cond_return_in_body: Option<LoopCondReturnInBodyFacts>,
    pub loop_scan_v0: Option<LoopScanV0Facts>,
    pub loop_scan_methods_block_v0: Option<LoopScanMethodsBlockV0Facts>,
    pub loop_scan_methods_v0: Option<LoopScanMethodsV0Facts>,
    pub loop_scan_phi_vars_v0: Option<LoopScanPhiVarsV0Facts>,
    pub loop_collect_using_entries_v0: Option<LoopCollectUsingEntriesV0Facts>,
    pub loop_bundle_resolver_v0: Option<LoopBundleResolverV0Facts>,
    pub pattern6_nested_minimal: Option<Pattern6NestedMinimalFacts>,
    pub pattern8_bool_predicate_scan: Option<Pattern8BoolPredicateScanFacts>,
    pub pattern9_accum_const_loop: Option<Pattern9AccumConstLoopFacts>,
    pub pattern2_break: Option<Pattern2BreakFacts>,
    pub pattern2_loopbodylocal: Option<Pattern2LoopBodyLocalFacts>,
}

impl LoopFacts {
    pub fn scan_with_init(&self) -> Option<&ScanWithInitFacts> {
        self.scan_with_init.as_ref()
    }

    pub fn split_scan(&self) -> Option<&SplitScanFacts> {
        self.split_scan.as_ref()
    }

    pub fn loop_simple_while(&self) -> Option<&Pattern1SimpleWhileFacts> {
        self.pattern1_simplewhile.as_ref()
    }

    pub fn loop_char_map(&self) -> Option<&Pattern1CharMapFacts> {
        self.pattern1_char_map.as_ref()
    }

    pub fn loop_array_join(&self) -> Option<&Pattern1ArrayJoinFacts> {
        self.pattern1_array_join.as_ref()
    }

    pub fn string_is_integer(&self) -> Option<&PatternIsIntegerFacts> {
        self.pattern_is_integer.as_ref()
    }

    pub fn if_phi_join(&self) -> Option<&Pattern3IfPhiFacts> {
        self.pattern3_ifphi.as_ref()
    }

    pub fn loop_continue_recipe(&self) -> Option<&Pattern4ContinueFacts> {
        self.pattern4_continue.as_ref()
    }

    pub fn loop_true_early_exit(&self) -> Option<&Pattern5InfiniteEarlyExitFacts> {
        self.pattern5_infinite_early_exit.as_ref()
    }

    pub fn nested_loop_minimal(&self) -> Option<&Pattern6NestedMinimalFacts> {
        self.pattern6_nested_minimal.as_ref()
    }

    pub fn bool_predicate_scan(&self) -> Option<&Pattern8BoolPredicateScanFacts> {
        self.pattern8_bool_predicate_scan.as_ref()
    }

    pub fn accum_const_loop(&self) -> Option<&Pattern9AccumConstLoopFacts> {
        self.pattern9_accum_const_loop.as_ref()
    }

    pub fn loop_break(&self) -> Option<&Pattern2BreakFacts> {
        self.pattern2_break.as_ref()
    }

    pub fn loop_break_body_local(&self) -> Option<&Pattern2LoopBodyLocalFacts> {
        self.pattern2_loopbodylocal.as_ref()
    }

    pub fn loop_cond_break_continue(&self) -> Option<&LoopCondBreakContinueFacts> {
        self.loop_cond_break_continue.as_ref()
    }

    pub fn loop_cond_continue_only(&self) -> Option<&LoopCondContinueOnlyFacts> {
        self.loop_cond_continue_only.as_ref()
    }

    pub fn loop_cond_continue_with_return(
        &self,
    ) -> Option<&LoopCondContinueWithReturnFacts> {
        self.loop_cond_continue_with_return.as_ref()
    }

    pub fn loop_cond_return_in_body(&self) -> Option<&LoopCondReturnInBodyFacts> {
        self.loop_cond_return_in_body.as_ref()
    }

    pub fn generic_loop_v0(&self) -> Option<&GenericLoopV0Facts> {
        self.generic_loop_v0.as_ref()
    }

    pub fn generic_loop_v1(&self) -> Option<&GenericLoopV1Facts> {
        self.generic_loop_v1.as_ref()
    }

    pub fn loop_scan_methods_v0(&self) -> Option<&LoopScanMethodsV0Facts> {
        self.loop_scan_methods_v0.as_ref()
    }

    pub fn loop_scan_methods_block_v0(&self) -> Option<&LoopScanMethodsBlockV0Facts> {
        self.loop_scan_methods_block_v0.as_ref()
    }

    pub fn loop_scan_phi_vars_v0(&self) -> Option<&LoopScanPhiVarsV0Facts> {
        self.loop_scan_phi_vars_v0.as_ref()
    }

    pub fn loop_scan_v0(&self) -> Option<&LoopScanV0Facts> {
        self.loop_scan_v0.as_ref()
    }

    pub fn loop_collect_using_entries_v0(
        &self,
    ) -> Option<&LoopCollectUsingEntriesV0Facts> {
        self.loop_collect_using_entries_v0.as_ref()
    }

    pub fn loop_bundle_resolver_v0(&self) -> Option<&LoopBundleResolverV0Facts> {
        self.loop_bundle_resolver_v0.as_ref()
    }
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct ScanWithInitFacts {
    pub loop_var: String,
    pub haystack: String,
    pub needle: String,
    pub step_lit: i64,
    pub dynamic_needle: bool,
}

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct SplitScanFacts {
    pub s_var: String,
    pub sep_var: String,
    pub result_var: String,
    pub i_var: String,
    pub start_var: String,
    pub shape: SplitScanShape,
}
