---
Status: Done
Decision: accepted
Date: 2026-02-27
Scope: WSM-P4-min7 として `.hako` wasm binary writer 用の BufferBox/FileBox バイナリ I/O 契約を追加し、VM route を lock する。
Related:
  - docs/development/current/main/design/wasm-hako-only-output-roadmap-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-154-wsm-p4-min1-binary-writer-doc-lock-ssot.md
  - docs/development/current/main/phases/phase-29cc/29cc-155-wsm-p4-min2-binary-writer-skeleton-lock-ssot.md
  - src/boxes/buffer/mod.rs
  - src/boxes/file/mod.rs
  - src/backend/mir_interpreter/handlers/boxes_buffer.rs
  - src/backend/mir_interpreter/handlers/boxes_file.rs
---

# 29cc-177 WSM-P4-min7 Buffer/File Binary Contract Lock

## Purpose
`.hako` 側で wasm binary writer を実装する前提として、バイト列の読み書き API を runtime/VM で固定する。

## Decision
1. FileBox に `readBytes` / `writeBytes` を追加する（非破壊追加）。
2. BufferBox に typed binary API を追加する（LE 固定）。
   - write: `writeU8/U16/U32/U64/F32/F64`
   - read: `readU8/U16/U32/U64/F32/F64`
3. VM handler に BufferBox/FileBox の route を追加し、`BoxCall/MethodCall` どちらでも fail-fast 契約を維持する。
4. provider SSOT を壊さず、FileBox binary I/O は `FileIo` trait 拡張（`read_bytes`/`write_bytes`）で統一する。

## Contract
1. FileBox.writeBytes 入力:
   - `ArrayBox<Integer 0..255>` または `BufferBox`
   - 範囲外/非整数は error 文字列で fail-fast
2. BufferBox typed API:
   - 数値範囲は型に合わせて fail-fast
   - エンディアンは little-endian 固定
   - 読み取りは offset 指定、境界外は fail-fast
3. `readU64` は VMValue(i64) へ返すため `i64::MAX` 超過時は overflow fail-fast

## Acceptance
1. `cargo check --bin hakorune`
2. `cargo test --lib bufferbox_numeric_rw_contract -- --nocapture`
3. `cargo test --lib bufferbox_numeric_bounds_contract -- --nocapture`
4. `cargo test --lib bufferbox_boxcall_typed_rw_contract -- --nocapture`
5. `cargo test --lib test_filebox_read_write_bytes_roundtrip -- --nocapture`
6. `cargo test --lib filebox_read_close_arg_contract_is_enforced_per_mode -- --nocapture`

## Next
- P4 lane で `.hako` wasm writer 実装時に、この API 群を SSOT 前提として利用する。
