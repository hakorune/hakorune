---
Status: Active
Decision: accepted
Date: 2026-02-28
Scope: source-zero cutover のため、runtime/plugin Rust residue を責務単位で棚卸しし、no-delete-first の経路切替順を固定する。
Related:
  - docs/development/current/main/phases/phase-29cc/29cc-220-runtime-source-zero-cutover-lock-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - src/runtime/plugin_loader_v2/enabled/
  - crates/nyash_kernel/src/plugin/
---

# 29cc-221 Runtime/Plugin Rust Residue Inventory Lock

## Purpose

source-zero 移行で迷走しないため、残存 Rust 実装を「責務」「経路切替順」「受け入れ gate」で固定する。

## A. Runtime plugin loader residue (`src/runtime/plugin_loader_v2/enabled/*`)

### A1. Resolver/dispatch boundary

- `method_resolver.rs`
- `instance_manager.rs`

Role:
- method id 解決
- instance lifecycle と create/invoke の橋渡し

Route cutover order:
1. `method_resolver.rs`
2. `instance_manager.rs`

Acceptance:
- fail-fast lock（29cc-218/219）を維持
- route drift guard green

### A2. Bridge boundary

- `ffi_bridge.rs`
- `host_bridge.rs`

Role:
- host/plugin 境界変換
- FFI 呼び出し橋渡し

Route cutover order:
1. `ffi_bridge.rs`
2. `host_bridge.rs`

Acceptance:
- Core C ABI/TypeBox ABI v2 呼び出し点が維持される
- mainline route に Rust bridge 分岐がない

### A3. Loader/config boundary

- `loader/*`
- `types.rs`
- `globals.rs`
- `errors.rs`
- `extern_functions.rs`

Role:
- loader 設定・spec・library metadata
- global loader state

Route cutover order:
1. `loader/*`
2. `types.rs` / `globals.rs`
3. `errors.rs` / `extern_functions.rs`

Acceptance:
- plugin config/load 経路が `.hako + ABI` で成立
- compat default-off で daily gate green

## B. Kernel plugin residue (`crates/nyash_kernel/src/plugin/*`)

### B1. Core invoke/runtime boundary

- `invoke_core.rs`
- `birth.rs`
- `runtime_data.rs`
- `semantics.rs`
- `instance.rs`

Role:
- birth/invoke のコア制御
- runtime state/semantics

Route cutover order:
1. `invoke_core.rs` / `birth.rs`
2. `runtime_data.rs` / `semantics.rs`
3. `instance.rs`

Acceptance:
- TypeBox ABI v2 resolve/invoke 契約が維持
- runtime behavior smoke green

### B2. Codec boundary

- `value_codec/mod.rs`
- `value_codec/encode.rs`
- `value_codec/decode.rs`
- `value_codec/borrowed_handle.rs`

Role:
- TLV encode/decode
- borrowed/owned handle 契約の維持

Route cutover order:
1. encode/decode
2. borrowed_handle
3. mod wiring

Acceptance:
- borrowed/owned matrix guard green
- ABI conformance smoke green

### B3. Async/entry boundary

- `future.rs`
- `invoke.rs`
- `mod.rs`

Role:
- async plugin invoke
- entrypoint 配線

Route cutover order:
1. `future.rs`
2. `invoke.rs`
3. `mod.rs`

Acceptance:
- mainline で legacy/compat entry が不在（Rust source は残置可）
- plugin gate pack green

## Non-target (keep for now)

- `array.rs` / `string.rs` / `map.rs` / `console.rs` / `intarray.rs` / `module_string_dispatch.rs` / `handle_helpers.rs`
  - 理由: 現時点では ABI 契約の境界面を先に固定するため。
  - 扱い: source-zero final wave で再評価（独立 lock で扱う）。
