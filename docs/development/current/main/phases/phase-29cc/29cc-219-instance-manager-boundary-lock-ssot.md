---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: plugin_loader_v2 instance_manager の責務を「解決 / invoke / handle組み立て」に分離し、execution-path-zero の thin 化順序を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-214-runtime-rust-thin-to-zero-execution-path-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-218-plugin-method-resolver-failfast-lock-ssot.md
  - src/runtime/plugin_loader_v2/enabled/instance_manager.rs
  - tools/checks/dev_gate.sh
---

# 29cc-219 Instance Manager Boundary Lock

## Purpose

`PluginLoaderV2::create_box` の責務混在を防ぐため、次の3段境界を固定する。

1. birth 契約の解決（type_id/birth_id/fini_id）
2. birth invoke と instance_id decode
3. `PluginBoxV2` handle 組み立て

## Contract

1. `create_box` は orchestrator とし、上記3段を順に呼ぶだけにする。
2. birth 契約解決は `config -> spec` の順で行い、未解決は fail-fast (`InvalidType` / `InvalidMethod`)。
3. `NYASH_FAIL_FAST=1`（default）で挙動を固定し、silent fallback を入れない。
4. 既存 ABI 契約（`invoke_alloc` / little-endian `instance_id`）は維持する。

## Acceptance

1. `cargo check --bin hakorune` が green。
2. `tools/checks/dev_gate.sh runtime-exec-zero` が green。
3. `tools/checks/dev_gate.sh portability` が green（節目/週次）。

## Not in this lock

1. `ffi_bridge.rs` の TLV encode/decode 責務切り出し
2. `loader/*` の lifecycle 切り出し
3. `.hako` 側への実装移植
