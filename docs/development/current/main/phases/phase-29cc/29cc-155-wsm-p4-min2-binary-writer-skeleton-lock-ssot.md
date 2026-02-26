---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min2（.hako-only roadmap P4）として wasm binary writer skeleton を実装し、contract test/smoke を lock する。
Related:
  - src/backend/wasm/binary_writer.rs
  - src/backend/wasm/mod.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh
  - tools/checks/dev_gate.sh
  - docs/development/current/main/phases/phase-29cc/29cc-154-wsm-p4-min1-binary-writer-doc-lock-ssot.md
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
---

# 29cc-155 WSM-P4-min2 Binary Writer Skeleton Lock

## Purpose
P4-min1 で固定した docs 契約に沿って、wasm binary writer の最小実装を Rust 側に追加し、WSM-P4 の実装境界を fail-fast で固定する。

## Implemented
1. `src/backend/wasm/binary_writer.rs` を追加。
2. 最小 wasm binary 生成関数を実装。
   - magic/version
   - type section
   - function section
   - export section（`main`）
   - code section（`i32.const <value>; end`）
3. `LEB128` の最小 encode（u32/i32）を実装。
4. `src/backend/wasm/mod.rs` に `binary_writer` を配線し、契約ヘルパー `build_minimal_i32_const_wasm` を追加。

## Contract tests / smoke lock
1. unit test lock:
   - `wasm_binary_writer_magic_version_contract`
   - `wasm_binary_writer_section_order_contract`
   - `wasm_binary_writer_leb128_contract`
   - `wasm_binary_writer_main_export_contract`
   - `wasm_binary_writer_minimal_module_contract`
2. smoke lock:
   - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min2_binary_writer_lock_vm.sh`
3. gate integration:
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に P4-min2 lock を追加。

## Decision notes
1. まだ `.hako` 側 writer 本体は未実装。P4-min2 は Rust 側 skeleton lock に限定する。
2. WAT fallback の追加は禁止。P4 系は fail-fast の境界を維持する。

## Next
- `WSM-P4-min3`: `.hako` 側 writer 入口（最小 fixture 1件）と parity gate 設計を docs-first で固定する。
