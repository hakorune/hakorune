---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: no-delete-first source-zero lane の A3-min3 として、globals/errors/extern_functions の境界失敗を fail-fast 契約で固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-221-runtime-plugin-rust-residue-inventory-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-227-runtime-a3-min2-types-handle-route-cutover-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/globals.rs
  - src/runtime/plugin_loader_v2/enabled/errors.rs
  - src/runtime/plugin_loader_v2/enabled/extern_functions.rs
---

# 29cc-228 Runtime A3-min3 Globals/Errors/Extern Fail-Fast Lock

## Purpose

`loader` 周辺で `unwrap` や暗黙panicに依存していた境界を `BidError::PluginError` に揃え、mainline の失敗モードを決定的にする。

## Fixed Contract

1. `globals::init_global_loader_v2` / `shutdown_plugins_v2` の `RwLock` 取得失敗は panic せず `BidError::PluginError` を返す。
2. `errors.rs` は `RwLock` read/write 失敗変換を SSOT として提供する。
3. `extern_functions::extern_call` の unknown interface/method は `BidError::PluginError` で fail-fast する。
4. unknown interface/method の reject 契約は unit test で固定する。

## Acceptance

1. `cargo check --bin hakorune` green
2. `tools/checks/dev_gate.sh runtime-exec-zero` green
3. `phase29y_no_compat_mainline_vm.sh` green

## Not in this lock

1. `PluginBoxMetadata` の route-aware 型縮退（A3-min4で扱う）
2. source削除（no-delete-first を維持）
