# Phase 160x: capability-family inventory

- Status: Active
- 目的: current Rust helper 群を future capability family 名で棚卸しし、hot path がどの seam に属するかを perf 再開前に source-backed に固定する。
- 対象:
  - `CURRENT_TASK.md`
  - `docs/development/current/main/05-Restart-Quick-Resume.md`
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/15-Workstream-Map.md`
  - `docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md`
  - `docs/development/current/main/design/semantic-optimization-authority-ssot.md`
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - `crates/nyash_kernel/src/plugin/map_probe.rs`
  - `crates/nyash_kernel/src/observe/backend/tls.rs`

## Goal

- helper 名ではなく capability family 名で current substrate を読めるようにする
- `store.array.str` / `const_suffix` / observer backend が将来どの seam に乗るかを固定する
- capability family 自体は final seam として残し、その下の native metal keep を長期に薄くする読みを崩さない

## Fixed Reading

- short-term:
  - Rust から authority を外す
- mid-term:
  - Rust helper を capability family の consumer に再定義する
- long-term:
  - capability seam を残したまま native metal keep を `OS / ABI / GC` の最終葉まで縮める

## Inventory Targets

1. string family
   - `string_view.rs`
   - `string_helpers.rs`
2. collection accelerator family
   - `array_handle_cache.rs`
   - `array_string_slot.rs`
   - `map_probe.rs`
3. observer/runtime mechanics family
   - `observe/backend/tls.rs`

## First Inventory Table

| Current surface | Capability-family reading | Current seam reading |
| --- | --- | --- |
| `string_view.rs` | string borrow/span family above `hako.ptr` / `hako.value_repr` | lifetime-sensitive native substrate keep |
| `string_helpers.rs` | string freeze/copy/search family above `hako.mem` / `hako.ptr` / `hako.buf` | native accelerator leaf |
| `array_handle_cache.rs` | `RawArray` consumer helper with lower `hako.tls`-like cache mechanics | native runtime mechanics below `RawArray` |
| `array_string_slot.rs` | `RawArray` + string-store consumer over memory / pointer / value-repr family | native accelerator leaf under `store.array.str` |
| `map_probe.rs` | `RawMap` consumer over key/probe substrate | native accelerator leaf |
| `observe/backend/tls.rs` | observer runtime backend; identity above, mechanics below | out-of-band native runtime mechanics keep |
| `src/runtime/host_handles.rs` | host handle/runtime mechanics keep | final native metal boundary side |

## Planned Granularity

1. inventory each hot file by capability family
2. freeze `store.array.str` / `const_suffix` / observer exact counter as seam consumers
3. resume `phase-137x` only after the seam reading is stable in docs

## Next

1. `phase-161x hot-path capability seam freeze`
2. `phase-137x main kilo reopen selection`
