---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: source-zero 直前の残存Rust境界（AOT link / loader / C ABI entry）を固定し、静的結合方針を明文化する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-242-kernel-residue-closeout-lock-ssot.md
  - docs/reference/abi/ABI_BOUNDARY_MATRIX.md
---

# 29cc-253 Source-Zero Static-Link Boundary Lock

## Purpose

「残りを .hako へ移す」時の迷走を防ぐため、残存境界を 3 本に固定し、  
core runtime の結合方針を static-first で確定する。

## Static-Link Policy (fixed)

1. Core runtime（host/kernel）は static-first を維持する。
2. AOT 実行は `libnyash_kernel.a` を正本にし、dynamic 既定への切替は行わない。
3. 外部 plugin（ユーザー配布 plugin）は dynamic load を許可するが、mainline route の正本にはしない。
4. source-zero 完了後に残す Rust は portability/build scaffolding のみ。

## Fixed Remaining Boundaries (1 boundary = 1 commit)

1. AOT link boundary
   - target:
     - `crates/nyash-llvm-compiler/src/main.rs`
     - `src/runner/modes/common_util/exec.rs`
   - contract:
     - `--emit-exe` は static archive (`libnyash_kernel.a`) で成立
     - harness route (`NYASH_LLVM_USE_HARNESS=1`) は補助経路として維持（既定切替しない）

2. runtime/plugin loader boundary
   - target:
     - `src/runtime/plugin_loader_v2/enabled/mod.rs`（配下一式）
   - contract:
     - loader/route 判定は `.hako + ABI` 側へ段階移管
     - mainline fail-fast / compat default-off を維持

3. C ABI entry boundary（plugin invoke/future + exports）
   - target:
     - `crates/nyash_kernel/src/exports/string.rs`
     - `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
     - `crates/nyash_kernel/src/plugin/future.rs`
   - contract:
     - C ABI symbol surface は維持（互換破壊しない）
     - 実体は `.hako` 実装へ寄せ、Rust 側は薄い境界に縮退

## Implementation Sync (2026-03-01)

1. C ABI entry thin化（boundary-3）
   - `crates/nyash_kernel/src/plugin/invoke_core.rs`
     - `NamedReceiver` と named receiver/method 解決 helper を追加。
   - `crates/nyash_kernel/src/plugin/invoke/by_name.rs`
     - receiver 解決・method 解決・TLV decode の重複を `invoke_core` 経由へ統一。
   - `crates/nyash_kernel/src/plugin/future.rs`
     - spawn entry の receiver route 解決を `invoke_core` 経由へ統一。

2. runtime/plugin loader compat境界の集約（boundary-2）
   - `src/runtime/plugin_loader_v2/enabled/compat_method_resolver.rs`
     - `resolve_method_id_with_compat_policy()` を追加し、fail-fast/trace 判定を compat層へ集約。
   - `src/runtime/plugin_loader_v2/enabled/method_resolver.rs`
     - compat fallback 分岐を 1 呼び出しへ縮退。

3. AOT link boundary の重複撤去（boundary-1）
   - `src/runner/modes/common_util/exec.rs`
     - `default_nyrt_dir()` / `apply_nyrt_arg()` を追加し、`ny_llvmc_emit_exe_lib/bin` の static-first precheck 適用を共通化。

## Acceptance

1. `tools/checks/dev_gate.sh runtime-exec-zero` green
2. `bash tools/smokes/v2/profiles/integration/apps/phase29y_no_compat_mainline_vm.sh` green
3. `CURRENT_TASK.md` / `phase-29cc/README.md` が本 lock を参照し、固定順が一致
4. static-first 方針（core static / external plugin dynamic optional）が docs 間で矛盾しない

## Not in this lock

1. Rust source の物理削除（Deletion Gate 条件達成後に別 lock）
2. perf 最適化の再開
3. ABI 面の追加
