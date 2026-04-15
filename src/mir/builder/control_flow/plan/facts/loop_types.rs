//! Core types for LoopFacts
//!
//! Contains the main structs: LoopFacts, ScanWithInitFacts, SplitScanFacts

use super::accum_const_loop_facts::AccumConstLoopFacts;
use super::bool_predicate_scan_facts::BoolPredicateScanFacts;
use super::escape_map_facts::EscapeMapFacts;
use super::feature_facts::LoopFeatureFacts;
use super::int_to_str_facts::IntToStrFacts;
use super::loop_array_join_facts::LoopArrayJoinFacts;
use super::loop_char_map_facts::LoopCharMapFacts;
use super::loop_simple_while_facts::LoopSimpleWhileFacts;
use super::loop_true_early_exit_facts::LoopTrueEarlyExitFacts;
use super::nested_loop_minimal_facts::NestedLoopMinimalFacts;
use super::scan_shapes::{ConditionShape, SplitScanShape, StepShape};
use super::skeleton_facts::SkeletonFacts;
use super::skip_whitespace_facts::SkipWhitespaceFacts;
use super::split_lines_facts::SplitLinesFacts;
use super::starts_with_facts::StartsWithFacts;
use super::string_is_integer_facts::StringIsIntegerFacts;
use super::{IfPhiJoinFacts, LoopContinueOnlyFacts};
use crate::mir::builder::control_flow::facts::loop_scan_methods_block_v0::LoopScanMethodsBlockV0Facts;
use crate::mir::builder::control_flow::facts::loop_scan_methods_v0::LoopScanMethodsV0Facts;
use crate::mir::builder::control_flow::plan::generic_loop::facts_types::{
    GenericLoopV0Facts, GenericLoopV1Facts,
};
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakBodyLocalFacts;
use crate::mir::builder::control_flow::plan::loop_break::facts::LoopBreakFacts;
use crate::mir::builder::control_flow::plan::loop_bundle_resolver_v0::LoopBundleResolverV0Facts;
use crate::mir::builder::control_flow::plan::loop_collect_using_entries_v0::LoopCollectUsingEntriesV0Facts;
use crate::mir::builder::control_flow::plan::loop_cond::break_continue_types::LoopCondBreakContinueFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_only_facts::LoopCondContinueOnlyFacts;
use crate::mir::builder::control_flow::plan::loop_cond::continue_with_return_facts::LoopCondContinueWithReturnFacts;
use crate::mir::builder::control_flow::plan::loop_cond::return_in_body_facts::LoopCondReturnInBodyFacts;
use crate::mir::builder::control_flow::plan::loop_scan_phi_vars_v0::LoopScanPhiVarsV0Facts;
use crate::mir::builder::control_flow::plan::loop_scan_v0::LoopScanV0Facts;
use crate::mir::builder::control_flow::plan::loop_true_break_continue::facts::LoopTrueBreakContinueFacts;

#[derive(Debug, Clone)]
pub(in crate::mir::builder) struct LoopFacts {
    pub condition_shape: ConditionShape,
    pub step_shape: StepShape,
    pub skeleton: SkeletonFacts,
    pub features: LoopFeatureFacts,
    pub scan_with_init: Option<ScanWithInitFacts>,
    pub split_scan: Option<SplitScanFacts>,
    pub loop_simple_while: Option<LoopSimpleWhileFacts>,
    pub loop_char_map: Option<LoopCharMapFacts>,
    pub loop_array_join: Option<LoopArrayJoinFacts>,
    pub string_is_integer: Option<StringIsIntegerFacts>,
    pub starts_with: Option<StartsWithFacts>,
    pub int_to_str: Option<IntToStrFacts>,
    pub escape_map: Option<EscapeMapFacts>,
    pub split_lines: Option<SplitLinesFacts>,
    pub skip_whitespace: Option<SkipWhitespaceFacts>,
    pub generic_loop_v0: Option<GenericLoopV0Facts>,
    pub generic_loop_v1: Option<GenericLoopV1Facts>,
    pub if_phi_join: Option<IfPhiJoinFacts>,
    pub loop_continue_only: Option<LoopContinueOnlyFacts>,
    pub loop_true_early_exit: Option<LoopTrueEarlyExitFacts>,
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
    pub nested_loop_minimal: Option<NestedLoopMinimalFacts>,
    pub bool_predicate_scan: Option<BoolPredicateScanFacts>,
    pub accum_const_loop: Option<AccumConstLoopFacts>,
    pub loop_break: Option<LoopBreakFacts>,
    pub loop_break_body_local: Option<LoopBreakBodyLocalFacts>,
}

impl LoopFacts {
    pub fn scan_with_init(&self) -> Option<&ScanWithInitFacts> {
        self.scan_with_init.as_ref()
    }

    pub fn split_scan(&self) -> Option<&SplitScanFacts> {
        self.split_scan.as_ref()
    }

    pub fn loop_simple_while(&self) -> Option<&LoopSimpleWhileFacts> {
        self.loop_simple_while.as_ref()
    }

    pub fn loop_char_map(&self) -> Option<&LoopCharMapFacts> {
        self.loop_char_map.as_ref()
    }

    pub fn loop_array_join(&self) -> Option<&LoopArrayJoinFacts> {
        self.loop_array_join.as_ref()
    }

    pub fn string_is_integer(&self) -> Option<&StringIsIntegerFacts> {
        self.string_is_integer.as_ref()
    }

    pub fn starts_with(&self) -> Option<&StartsWithFacts> {
        self.starts_with.as_ref()
    }

    pub fn int_to_str(&self) -> Option<&IntToStrFacts> {
        self.int_to_str.as_ref()
    }

    pub fn escape_map(&self) -> Option<&EscapeMapFacts> {
        self.escape_map.as_ref()
    }

    pub fn split_lines(&self) -> Option<&SplitLinesFacts> {
        self.split_lines.as_ref()
    }

    pub fn skip_whitespace(&self) -> Option<&SkipWhitespaceFacts> {
        self.skip_whitespace.as_ref()
    }

    pub fn if_phi_join(&self) -> Option<&IfPhiJoinFacts> {
        self.if_phi_join.as_ref()
    }

    pub fn loop_continue_only(&self) -> Option<&LoopContinueOnlyFacts> {
        self.loop_continue_only.as_ref()
    }

    pub fn loop_true_early_exit(&self) -> Option<&LoopTrueEarlyExitFacts> {
        self.loop_true_early_exit.as_ref()
    }

    pub fn nested_loop_minimal(&self) -> Option<&NestedLoopMinimalFacts> {
        self.nested_loop_minimal.as_ref()
    }

    pub fn bool_predicate_scan(&self) -> Option<&BoolPredicateScanFacts> {
        self.bool_predicate_scan.as_ref()
    }

    pub fn accum_const_loop(&self) -> Option<&AccumConstLoopFacts> {
        self.accum_const_loop.as_ref()
    }

    pub fn loop_break(&self) -> Option<&LoopBreakFacts> {
        self.loop_break.as_ref()
    }

    pub fn loop_break_body_local(&self) -> Option<&LoopBreakBodyLocalFacts> {
        self.loop_break_body_local.as_ref()
    }

    pub fn loop_cond_break_continue(&self) -> Option<&LoopCondBreakContinueFacts> {
        self.loop_cond_break_continue.as_ref()
    }

    pub fn loop_cond_continue_only(&self) -> Option<&LoopCondContinueOnlyFacts> {
        self.loop_cond_continue_only.as_ref()
    }

    pub fn loop_cond_continue_with_return(&self) -> Option<&LoopCondContinueWithReturnFacts> {
        self.loop_cond_continue_with_return.as_ref()
    }

    pub fn loop_cond_return_in_body(&self) -> Option<&LoopCondReturnInBodyFacts> {
        self.loop_cond_return_in_body.as_ref()
    }

    pub fn loop_true_break_continue(&self) -> Option<&LoopTrueBreakContinueFacts> {
        self.loop_true_break_continue.as_ref()
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

    pub fn loop_collect_using_entries_v0(&self) -> Option<&LoopCollectUsingEntriesV0Facts> {
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
