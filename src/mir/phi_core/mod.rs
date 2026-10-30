/*!
 * phi_core – Unified PHI management scaffold (Phase 1)
 *
 * Purpose:
 * - Provide a single, stable entry point for PHI-related helpers.
 * - Start with re-exports of existing, verified logic (zero behavior change).
 * - Prepare ground for gradual consolidation of loop PHI handling.
 */

pub mod common;
// Phase 84-5: if_phi 削除（レガシーフォールバック完全削除）
// Phase 30 F-2.1: loop_phi 削除（LoopFormBuilder が SSOT）
pub mod loop_snapshot_merge;

// Phase 191-193: LoopForm modularization - complete directory structure
pub mod loopform;
// Trio legacy boxes removed in Phase 70: LoopScopeShape now owns classification/liveness.

// Phase 26-B: Box-First Refactoring
// Phase 30 F-2.1: body_local_phi_builder 削除（LoopScopeShape で代替）
// Phase 62: phi_input_collector 削除（インライン化完了）

// Phase 26-C: Loop Snapshot Management
// Phase 30 F-2.1: header_phi_builder 削除（JoinIR loop_step で代替）
// Phase 30 F-2.2: loop_snapshot_manager 削除（テスト専用、外部呼び出しなし）

// Phase 26-D: Exit PHI Management
// Phase 30 F-2.1: exit_phi_builder 削除（JoinIR k_exit で代替、バイパス関数は loopform_builder に移動）

// Phase 84-2: Copy命令型伝播箱（ChatGPT Pro設計）
pub mod copy_type_propagator;

// Phase 84-3: PHI + Copy グラフ型推論箱（ChatGPT Pro設計）
pub mod phi_type_resolver;

// Phase 84-5: テスト専用ユーティリティ（if_phi.rs から移動）
pub mod test_utils;
