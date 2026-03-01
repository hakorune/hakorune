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
- target model は「Rust source を保存したまま、mainline の runtime/plugin 意味論依存を 0 行化（`.hako` 主経路化）」とする。

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

1. RZ-LC-min1 (docs sync)
   - `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` を Lane-C reopen + source-keep で同期する。
2. RZ-LC-min2 (loader boundary)
   - `plugin_loader_v2` の mainline 判定源を `route_resolver` へ集約し、compat/dev 分岐を隔離する。
3. RZ-LC-min3 (kernel entry boundary)
   - `invoke/by_name` / `future` / `exports` の mainline 呼び先を `.hako forward bridge` 優先へ固定し、Rust 側は thin compat を維持する。
4. RZ-LC-min4 (closeout)
   - `runtime-exec-zero` + `no_compat_mainline` + `portability` 緑で Lane-C を monitor-only へ戻し、最適化 lane へ handoff する。

## Execution Status

- 2026-02-28: RZ-LINK-min1 (runner precheck split) 着手
  - `src/runner/modes/common_util/exec.rs` に `skip_nyrt_precheck()` を追加
  - `NYASH_LLVM_USE_HARNESS=1` のときのみ runner 側の `verify_nyrt_dir` をスキップ
  - 既定 (`NYASH_LLVM_USE_HARNESS!=1`) は従来挙動を維持
- 2026-02-28: RZ-ARRAY-min1 (route selector lock) 着手
  - `runtime_data_dispatch` の symbol 選択を `select_runtime_data_call_spec()` へ集約
  - default は現行維持（array mono-route 優先）
  - caller 明示時のみ runtime_data-only route を選べる境界を追加
- 2026-02-28: RZ-ARRAY-min2 (route policy lock) 着手
  - `NYASH_RUNTIME_DATA_ARRAY_ROUTE_POLICY`（`array_mono|runtime_data_only`）を追加
  - default は `array_mono` に固定し、`29cc-217` route lock と整合
  - 無効値は fail-fast（RuntimeError）
- 2026-03-01: RZ-LOADER-min1 (route contract box lock) 着手
  - `route_resolver` に `MethodRouteContract` / `BirthRouteContract` を追加
  - `resolve_method_contract()` / `resolve_birth_contract()` を追加し resolver/instance の重複判定を集約
  - 既定挙動（fail-fast/compat 条件）は不変
- 2026-03-01: RZ-LOADER-min2 (ffi/host route contract lock) 着手
  - `route_resolver` に `InvokeRouteContract` を追加
  - `ffi_bridge` / `instance_manager` の invoke route 解決を contract 箱経由へ統一
  - `host_bridge::invoke_alloc_with_route()` 呼び出し点の判定源を集約（挙動不変）
- 2026-03-01: RZ-LOADER-min3 (compat/dev branch isolation) 着手
  - `ffi_bridge` の compat/dev trace/probe 分岐を `compat_ffi_bridge` へ移設
  - mainline invoke フローを route解決・invoke・decode へ縮退
  - ENV契約とログタグは維持（挙動不変）
- 2026-03-01: RZ-LOADER-min4 (loader/types route contract reuse) 着手
  - `route_resolver` に `resolve_birth_contract_for_lib()` を追加
  - `loader/singletons` / `instance_manager` / `ffi_bridge` / `types` の invoke route 解決を contract 経由へ統一
  - global 直lookup での route 判定を縮退（挙動不変）
- 2026-03-01: RZ-KERNEL-min1 (entry invoke helper thin lock) 着手
  - `crates/nyash_kernel/src/plugin/invoke_core.rs` に `resolve_named_method_for_handle()` / `invoke_named_receiver_to_i64()` を追加
  - `crates/nyash_kernel/src/plugin/invoke/by_name.rs` の receiver+method 解決 / invoke+decode 重複を helper 経由へ統一
  - `crates/nyash_kernel/src/plugin/future.rs` の `spawn_instance3` method 解決を helper 経由へ統一
  - hook優先・fallback policy（`NYASH_VM_USE_FALLBACK`）の挙動は不変
- 2026-03-01: RZ-LC-min4 (closeout gate refresh) 実施
  - `tools/checks/dev_gate.sh runtime-exec-zero` green
  - `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` green
  - `tools/checks/dev_gate.sh portability` green

## Gate Contract

- `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh`
- `tools/checks/dev_gate.sh runtime-exec-zero`
- `cargo test route_resolver::tests -- --nocapture`

## Reopen Criteria

- 上記 gate fail
- mainline route で compat fallback が観測される
- `emit-exe` 経路で runtime route lock が崩れる
