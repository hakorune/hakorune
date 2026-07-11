# Phase 161x: hot-path capability seam freeze

- Status: Landed
- 目的: `phase-160x` の棚卸し結果を hot path 単位で固定し、`phase-137x` の perf front を capability seam 経由で読めるようにする。

## Planned Scope

- `store.array.str`
  - current executor detail: `array_string_store_handle_at(...)`
  - future capability seam: `RawArray` consumer + lower `hako.value_repr` / `hako.mem` / `hako.ptr`
- `const_suffix`
  - current executor detail: `concat_const_suffix_fallback(...)`
  - future capability seam: string borrow/span + freeze/materialize family
- observer backend
  - current executor detail: `observe/backend/tls.rs`
  - future seam: runtime mechanics keep under final native metal boundary, with canonical identity only aligned upward

## Planned Deliverable

1. source-backed mapping for `store.array.str`
   - current executor detail
   - `RawArray` consumer seam
   - lower value-repr / memory seam
2. source-backed mapping for `const_suffix`
   - current executor detail
   - string borrow/span seam
   - string freeze/materialize seam
3. source-backed mapping for observer exact counter
   - canonical identity
   - runtime TLS backend
   - final native keep reading

## Exit

- hot path ごとの capability family reading が source-backed に読める
- `phase-137x` current front が helper 名だけでなく seam 名でも追える
- perf reopen can now treat capability seam names as the primary reading and helper names as executor detail
