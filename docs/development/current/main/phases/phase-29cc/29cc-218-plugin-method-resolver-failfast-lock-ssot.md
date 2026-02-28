---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: plugin_loader_v2 method resolver の legacy file fallback を compat-only に固定し、execution-path-zero の fail-fast 経路を維持する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-215-runtime-execution-path-observability-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/method_resolver.rs
  - tools/checks/dev_gate.sh
---

# 29cc-218 Plugin Method Resolver Fail-Fast Lock

## Purpose

`PluginLoaderV2::resolve_method_id` の最終段 fallback（legacy file-based resolution）を
mainline 既定から外し、de-rust 経路の fail-fast 契約を固定する。

## Contract

1. `config/spec/resolve_fn` で method を解決できない場合:
   - `NYASH_FAIL_FAST=1`（default）: `InvalidMethod` を返す。
   - `NYASH_FAIL_FAST=0`（compat bring-up）: legacy file fallback を許可する。
2. legacy file fallback は compat 専用挙動であり、mainline route の正本にしない。
3. `dev_provider_trace` 時は fallback reject を1行タグで観測可能にする。

## Acceptance

1. `cargo check --bin hakorune` が green。
2. `tools/checks/dev_gate.sh runtime-exec-zero` が green。
3. `tools/checks/dev_gate.sh portability` が green（週次/節目）。

## Not in this lock

1. `plugin_loader_v2/enabled/method_resolver.rs` の resolver 本体を `.hako` へ移植する作業
2. `instance_manager` / `ffi_bridge` の責務切り出し
