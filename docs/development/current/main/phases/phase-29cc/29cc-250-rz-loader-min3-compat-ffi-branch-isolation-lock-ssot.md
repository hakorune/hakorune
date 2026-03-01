---
Status: Active
Decision: accepted
Date: 2026-03-01
Scope: RZ-LOADER-min3 として ffi_bridge の compat/dev 分岐を専用モジュールへ隔離し、mainline責務を縮退する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-249-rz-loader-min2-ffi-host-route-contract-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/ffi_bridge.rs
  - src/runtime/plugin_loader_v2/enabled/compat_ffi_bridge.rs
---

# 29cc-250 RZ-LOADER-min3 Compat FFI Branch Isolation Lock

## Purpose

`ffi_bridge` に混在していた compat/dev 向け trace/probe 分岐を分離し、  
mainline invoke フローの責務を「route解決 + invoke + decode」に縮退する。

## Decision

- 新規: `compat_ffi_bridge.rs` を追加。
- 下記の分岐を `compat_ffi_bridge` へ移設:
  - C wrap probe
  - C core probe
  - call trace
  - tlv shim trace
- `ffi_bridge` は上記を `maybe_*` 呼び出しで利用するのみ。
- 挙動は不変（ENV名/ログタグ/default値は維持）。

## Acceptance

- `cargo check --bin hakorune` green
- `tools/checks/dev_gate.sh runtime-exec-zero` green
- `phase29y_no_compat_mainline_vm.sh` green
