---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: execution-path-zero 運用のため、runtime route drift を観測・監査する最小契約を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-217-runtime-vm-aot-route-lock-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - tools/checks/phase29cc_runtime_execution_path_zero_guard.sh
  - tools/checks/dev_gate.sh
  - src/runner_plugin_init.rs
---

# 29cc-215 Runtime Execution Path Observability Lock

## Purpose

`execution-path-zero` を「主観」ではなくログ契約と guard で監査できる状態に固定する。

## Term definition

`adapter route contract`:
- registry mapping が存在する
- handler route + trace tag 契約が存在する
- runtime core box が ABI symbol へ接続される

## Route drift observability lock

1. 起動時に `runner_plugin_init` が次の1行タグを出せること:
   - `[runtime/exec-path] plugin_loader_backend=<...> plugin_exec_mode=<...> box_factory_policy=<...>`
2. 既存の plugin loader backend 表示（`backend_kind()`）契約を維持する。
3. 観測は既定静音を維持し、`NYASH_CLI_VERBOSE=1` または `NYASH_DEBUG_PLUGIN=1` のときのみ出力する。

## Guard contract

1. `tools/checks/phase29cc_runtime_execution_path_zero_guard.sh` を正本 guard とする。
2. guard は次を監査する:
   - lock doc / runtime lock / cutover SSOT の存在
   - `[runtime/exec-path]` タグの実装存在
   - `plugin_exec_mode` / `box_factory_policy` 観測フック存在
   - `backend_kind()` 契約存在
   - `dev_gate` への配線存在
3. 日常実行は `tools/checks/dev_gate.sh runtime-exec-zero` で行う。
4. Step-3 adapter fixture は `tools/smokes/v2/profiles/integration/apps/phase29cc_runtime_v0_adapter_fixtures_vm.sh` を正本 smoke とする。

## Acceptance

1. `phase29cc_runtime_execution_path_zero_guard.sh` が green。
2. `dev_gate.sh runtime-exec-zero` が green。
3. `phase29cc_runtime_v0_adapter_fixtures_vm.sh` で
   `[vm/adapter/string_core:len_i64]` タグ契約（handler source）が監査される（adapter route drift 検知）。
4. `CURRENT_TASK.md` / `10-Now.md` / `phase-29cc/README.md` に lock 参照が同期されている。

## Not in this lock

1. ABI 語彙追加（`string_len` / `array_get_i64` / `array_set_i64` 実装本体）
2. plugin loader の機能置換
3. Rust source 削除
