---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: runtime source-zero 継続のため、mainline が依存する Rust 経路を再棚卸しして cutover 順を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - CURRENT_TASK.md
---

# 29cc-245 Runtime Route Residue Relock

## Purpose

`.hako` 側移植が進んだ現在地で、実行主経路に残る Rust 依存点を再固定する。  
最適化作業と route cutover を混ぜないため、`1 boundary = 1 commit` を維持する。

## Decision Summary

- mainline route の健全性確認は `no_compat_mainline` と `runtime-exec-zero` を継続使用する。
- 次の境界は最適化ではなく route cutover を優先する。
- `no-delete-first` を維持し、source 削除は扱わない。

## Residue Inventory (2026-02-28 refresh)

1. `libnyash_kernel.a` 必須リンク境界（High）
   - `crates/nyash-llvm-compiler/src/main.rs`（emit-exe link path）
   - `src/runner/modes/common_util/exec.rs`（`verify_nyrt_dir`）
   - `src/runner/modes/mir.rs` / `src/runner/modes/llvm/harness_executor.rs`
2. runtime/plugin loader 境界（Medium）
   - `src/runtime/plugin_loader_v2/enabled/*`
   - `src/runtime/plugin_loader_unified.rs`
   - `src/runtime/semantics.rs` / `src/runtime/box_registry.rs` / `src/runtime/host_api/common.rs`
3. kernel plugin export 境界（Medium）
   - `crates/nyash_kernel/src/plugin/array.rs`（`nyash.array.get_hi/set_hii`）
   - `crates/nyash_kernel/src/plugin/invoke_core.rs`
   - `crates/nyash_kernel/src/plugin/future.rs`
   - `crates/nyash_kernel/src/plugin/value_codec/*`

## Fixed Next Order

1. RZ-LINK-min1  
   `emit-exe` 経路の `libnyash_kernel.a` 前提を optional route 化（既定は現行維持）
2. RZ-ARRAY-min1  
   `nyash.array.get_hi/set_hii` の mainline 呼び先を `.hako` 側 route へ寄せるための境界分離
3. RZ-LOADER-min1  
   loader/unified の route resolver 境界を `.hako` host facade 経由で固定

## Execution Status

- 2026-02-28: RZ-LINK-min1 (runner precheck split) 着手
  - `src/runner/modes/common_util/exec.rs` に `skip_nyrt_precheck()` を追加
  - `NYASH_LLVM_USE_HARNESS=1` のときのみ runner 側の `verify_nyrt_dir` をスキップ
  - 既定 (`NYASH_LLVM_USE_HARNESS!=1`) は従来挙動を維持
- 2026-02-28: RZ-ARRAY-min1 (route selector lock) 着手
  - `runtime_data_dispatch` の symbol 選択を `select_runtime_data_call_spec()` へ集約
  - default は現行維持（array mono-route 優先）
  - caller 明示時のみ runtime_data-only route を選べる境界を追加

## Gate Contract

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `tools/checks/dev_gate.sh runtime-exec-zero`

## Reopen Criteria

- 上記 gate fail
- mainline route で compat fallback が観測される
- `emit-exe` 経路で runtime route lock が崩れる
