# Phase 161x: hot-path capability seam freeze

- Status: Next
- 目的: `phase-160x` の棚卸し結果を hot path 単位で固定し、`phase-137x` の perf front を capability seam 経由で読めるようにする。

## Planned Scope

- `store.array.str`
  - current executor detail: `array_string_store_handle_at(...)`
  - future capability seam: `RawArray` + lower memory/buffer family
- `const_suffix`
  - current executor detail: `concat_const_suffix_fallback(...)`
  - future capability seam: string borrow/freeze family
- observer backend
  - current executor detail: `observe/backend/tls.rs`
  - future seam: runtime mechanics keep under final native metal boundary

## Exit

- hot path ごとの capability family reading が source-backed に読める
- `phase-137x` current front が helper 名だけでなく seam 名でも追える
