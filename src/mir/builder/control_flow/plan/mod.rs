//! Phase 273 P1: Facts/Recipe + CorePlan 二層構造 + PlanNormalizer + PlanVerifier
//!
//! This module provides a two-layer Plan architecture for loop lowering:
//!
//! # Architecture
//!
//! ```text
//! Facts/Recipe contract (ループ受理契約)
//!     ↓ PlanNormalizer (SSOT)
//! CorePlan (固定語彙 - 構造ノードのみ)
//!     ↓ PlanLowerer
//! MIR (block/value/phi)
//! ```
//!
//! - **Facts/Recipe contract**: planner-produced loop acceptance contract
//! - **PlanNormalizer**: Facts/Recipe contract → CorePlan conversion (SSOT, scan knowledge here)
//! - **CorePlan**: Fixed vocabulary, expressions as ValueId references (no String parsing)
//! - **PlanVerifier**: Fail-fast validation for CorePlan invariants
//! - **PlanLowerer**: Processes CorePlan only (no string interpretation)
//!
//! # Key Design Decision (String式禁止)
//!
//! CorePlan expressions use **ValueId references only** (String expressions forbidden).
//! This prevents "second language processor" from growing inside Lowerer.

#![allow(dead_code)]

// ============================================================================
// Module Declarations (Layer-based Organization)
// Design: docs/development/current/main/design/plan-mod-layout-ssot.md
// ============================================================================

// Layer 0: Core Data Structures (New from Refactoring)
// 分割されたデータ構造定義
pub(in crate::mir::builder) mod core;
pub(in crate::mir::builder) mod domain;
pub(in crate::mir::builder) mod effect;
pub(in crate::mir::builder) mod exit;

// Layer 1: Core Infrastructure (基盤)
// MIR lowering の中核インフラ
pub(in crate::mir::builder) mod branchn;
// Phase 29ai P0: Facts SSOT + Single Planner skeleton (parallel footing, unused in P0)
pub(in crate::mir::builder) mod edgecfg_facade;
pub(in crate::mir::builder) mod facts;
pub(in crate::mir::builder) mod lowerer;
pub(in crate::mir::builder) mod normalizer;
pub(in crate::mir::builder) mod step_mode;
// Phase 29bq+: Recipe-first base types (Facts→Lower contract)
pub(in crate::mir::builder) mod recipes;
pub(in crate::mir::builder) mod trace;

// Layer 2: Analysis Layer (観測)
// AST分析のみ、変更なし
// Phase 29bq P0: Canon (analysis-only view)
pub(in crate::mir::builder) mod canon;

// Layer 3: Skeleton/Feature Layer (分解スロット)
// route 形状認識の分解スロット
// Phase 29bt P0: Skeletons + Features (decomposition slots)
pub(in crate::mir::builder) mod features;
pub(in crate::mir::builder) mod skeletons;

// Layer 4: Route-Specific (形状固有)
// 各 route の固有処理（一部 helper には historical file 名が残る）
// Phase 29ca P1: Core loop body effect contract (SSOT)
pub(in crate::mir::builder) mod coreloop_body_contract;
// Phase 29ca P1: Generic loop v0 module (facts/normalizer SSOT)
pub(in crate::mir::builder) mod generic_loop;
// Phase 29bq+: loop_break module moved to plan side
pub(in crate::mir::builder) mod loop_break;
// Phase 29bq+: loop-break condition policy router moved to plan side
pub(in crate::mir::builder) mod loop_break_condition_policy_router;
// Phase 29bq+: loop_break input facts box moved to plan side
pub(in crate::mir::builder) mod loop_break_prep_box;
// Phase 29bq+: loop-break policy router moved to plan side
pub(in crate::mir::builder) mod loop_break_policy_router;
// Phase 29bq+: loop_break steps moved to plan side
pub(in crate::mir::builder) mod loop_break_steps;
// Layer 5: Loop-Specific (ループ固有)
// 各ループタイプの固有処理
// Phase 29bq+: body local policy moved to plan side
pub(in crate::mir::builder) mod body_local_policy;
mod body_local_policy_helpers;
mod body_local_policy_inputs;
mod body_local_policy_runner;
mod body_local_policy_types;
// Phase 29bq+: Exit binding utilities moved to plan side
pub(in crate::mir::builder) mod exit_binding;
pub(in crate::mir::builder) mod exit_binding_applicator;
pub(in crate::mir::builder) mod exit_binding_constructor;
pub(in crate::mir::builder) mod exit_binding_validator;
// Phase 29bq+: bundle/using resolver loop(i<n) with i=next_i + nested return (BoxCount)
pub(in crate::mir::builder) mod loop_bundle_resolver_v0;
// Phase 29bq+: Stage1UsingResolverBox._collect_using_entries loop (BoxCount)
pub(in crate::mir::builder) mod loop_collect_using_entries_v0;
// Phase 29bq P2.x: loop_cond* shared helpers
pub(in crate::mir::builder) mod loop_cond_shared;
// Phase 29bq P2.x: unified loop_cond helpers (moved from loop_cond_unified/helpers.rs)
pub(in crate::mir::builder) mod loop_cond_unified_helpers;
// Phase 29bq P2.x: unified loop_cond facts (variants)
// Note: loop_cond_continue_only, loop_cond_continue_with_return, loop_cond_return_in_body,
//       loop_cond_break_continue are now unified into loop_cond/
pub(in crate::mir::builder) mod loop_cond;
// Phase 29bq+: scan_methods outer loop coverage (block-wrapped inner loop, BoxCount)
pub(in crate::mir::builder) mod loop_scan_methods_block_v0;
// Phase 29bq+: scan_methods outer loop coverage (BoxCount)
pub(in crate::mir::builder) mod loop_scan_methods_v0;
// Phase 29bq+: selfhost _collect_phi_vars outer loop coverage (BoxCount)
pub(in crate::mir::builder) mod loop_scan_phi_vars_v0;
// Phase 29bq+: one-shape scan loop coverage (BoxCount)
pub(in crate::mir::builder) mod loop_scan_v0;
// Phase 29bq+: Loop scope shape builder moved to plan side
pub(in crate::mir::builder) mod loop_scope_shape_builder;
// Phase 29bq P2: loop(true) break/continue coverage
pub(in crate::mir::builder) mod loop_true_break_continue;
// Phase 29bq+: loop(true) counter extractor moved to plan side
pub(in crate::mir::builder) mod loop_true_counter_extractor;
// Phase 12: Unified nested loop depth1 module (consolidates 4 variants)
pub(in crate::mir::builder) mod nested_loop_depth1;
// Phase 29bq+: Shared nested loop plan lowering helper
pub(in crate::mir::builder) mod nested_loop_plan;
pub(in crate::mir::builder) mod nested_loop_plan_break_continue;
pub(in crate::mir::builder) mod nested_loop_plan_bridge;
pub(in crate::mir::builder) mod nested_loop_plan_continue_with_return;
pub(in crate::mir::builder) mod nested_loop_plan_recipe_fallback;
pub(in crate::mir::builder) mod nested_loop_plan_recipe_fallback_policy;
// Phase 29bq+: read_digits break condition box moved to plan side
pub(in crate::mir::builder) mod read_digits_break_condition_box;
// Phase W6: shared scan loop segment vocabulary (SSOT)
pub(in crate::mir::builder) mod scan_loop_segments;
// Phase 29bq+: Trim utilities moved to plan side
pub(in crate::mir::builder) mod trim_loop_lowering;
pub(in crate::mir::builder) mod trim_lowerer;
pub(in crate::mir::builder) mod trim_validator;

// Layer 6: Data Structures (データ構造)
// CorePlan の構成要素
// M1 scaffold: RecipeTree vocabulary + Parts dispatch entry (no calls from existing pipeline)
pub(in crate::mir::builder) mod parts;
pub(in crate::mir::builder) mod recipe_tree;
// M5f: Neutral re-export layer for parts (parts should not depend on features)
pub(in crate::mir::builder) mod steps;

// Layer 7: Orchestration (オーケストレーション)
// 全体の制御・調整
// Phase 29ao P0: CorePlan composer scaffold (unused)
pub(in crate::mir::builder) mod composer;
pub(in crate::mir::builder) mod emit;
pub(in crate::mir::builder) mod planner;
// Phase 29ai P5: JoinIR router → single plan extraction entrypoint
pub(in crate::mir::builder) mod single_planner;

// Layer 8: Utilities (ユーティリティ)
// 共通機能・ポリシー
// Phase 29bq+: Common route helpers moved to plan side
// Phase 29bq+: Common route initializer moved to plan side
pub(in crate::mir::builder) mod common_init;
// Phase 29ai P6: Extractors moved into plan layer
pub(in crate::mir::builder) mod extractors;
// Phase 29av P1: FlowBox observability tags (strict/dev only)
// Phase 29bq+: Route recognizers moved to plan side
pub(in crate::mir::builder) mod route_shape_recognizers;
// Phase 29ao P21: loop_simple_while subset policy (SSOT gate)
// Phase 29bq+: Route policies moved to plan side
pub(in crate::mir::builder) mod policies;

// Layer 9: Legacy/Scaffolding (残骸・足場)
// 歴史的経緯で残存、将来的には整理予定
// Phase 29bq+: AST feature extractor moved to plan side
pub(in crate::mir::builder) mod ast_feature_extractor;
// Phase 29bq+: Condition env builder moved to plan side
pub(in crate::mir::builder) mod condition_env_builder;
// Phase 29bq+: Conversion pipeline moved to plan side
pub(in crate::mir::builder) mod conversion_pipeline;
// Phase 29bq+: Escape route recognizer moved to plan side
pub(in crate::mir::builder) mod escape_shape_recognizer;
// Phase 29bq+: Structural lock (join_key/session)
// sealing は edgecfg/api/frag_emit_session.rs に統合（Phase 29bq+ 骨格拡大）
pub(in crate::mir::builder) mod join_key;
// Phase 29bq+: Route prep pipeline moved to plan side (patterns layer thin)
pub(in crate::mir::builder) mod plan_build_session;
pub(in crate::mir::builder) mod route_prep_pipeline;

// ============================================================================
// Entrypoints (SSOT)
// ============================================================================
//
// Plan pipeline entrypoints:
// - planner::build_plan_with_facts* (Facts + recipe contract)
// - PlanVerifier::verify (CorePlan invariants)
// - PlanLowerer::lower (CorePlan → MIR)
#[allow(unused_imports)]
pub(in crate::mir::builder) use lowerer::PlanLowerer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use normalizer::PlanNormalizer;
#[allow(unused_imports)]
pub(in crate::mir::builder) use planner::{
    build_plan_with_facts, build_plan_with_facts_ctx, PlanBuildOutcome,
};
#[allow(unused_imports)]
pub(in crate::mir::builder) use crate::mir::builder::control_flow::verify::PlanVerifier;

pub(in crate::mir::builder) use branchn::CoreBranchNPlan;
pub(in crate::mir::builder) use plan_build_session::PlanBuildSession;

// ============================================================================
// Re-exports (New Modules)
// ============================================================================

// Core types
pub(in crate::mir::builder) use self::core::{
    CoreIfJoin, CoreIfPlan, CoreLoopPlan, CorePhiInfo, CorePlan, LoopStepMode, LoweredRecipe,
};
pub(in crate::mir::builder) use self::effect::CoreEffectPlan;
pub(in crate::mir::builder) use self::exit::CoreExitPlan;

// Domain types
#[cfg(test)]
pub(in crate::mir::builder) use self::domain::scan_direction_from_step_lit;
pub(in crate::mir::builder) use self::domain::LoopBreakStepPlacement;
