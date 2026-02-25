# Phase 21.6 — Core Numeric Boxes (Draft)

Status: proposal (to refine at 21.6 kickoff)

## Goal

Provide explicit, low‑level numeric boxes that:

- Give Nyash a “fair” core for int/f64 benchmarks against C.
- Stay compatible with the existing ArrayBox API (no breaking changes).
- Can be used both explicitly in `.hako` and (later) as conservative AotPrep targets.

This phase focuses on design + minimal implementation; aggressive auto‑rewrites stay behind opt‑in flags.

## Scope (21.6)

- Design and add **IntArrayCore** numeric core (NyRT + Hako wrapper):
  - NyRT: `IntArrayCore` box（Rust）with internal layout `Vec<i64>`（contiguous, row‑major semantics）。
  - Hako: `IntArrayCoreBox` in `nyash.core.numeric.intarray`, wrapping NyRT via externcall:
    - `static new(len: i64) -> IntArrayCoreBox` → `nyash.intarray.new_h`
    - `length(self) -> i64` → `nyash.intarray.len_h`
    - `get_unchecked(self, idx: i64) -> i64` → `nyash.intarray.get_hi`
    - `set_unchecked(self, idx: i64, v: i64)` → `nyash.intarray.set_hii`
  - Semantics: i64‑only、固定長（構造変更なし）。境界チェックは NyRT 側（Fail‑Fast）に限定し、Hako 側は数値カーネル専用の薄いラッパーに留める。

- Design and add **MatI64** (matrix box) on top of IntArrayCore:
  - Internal layout: `rows: i64`, `cols: i64`, `stride: i64`, `core: IntArrayCoreBox`.
  - Minimal API:
    - `new(rows: i64, cols: i64) -> MatI64`
    - `rows(self) -> i64`, `cols(self) -> i64`
    - `at(self, r: i64, c: i64) -> i64`
    - `set(self, r: i64, c: i64, v: i64)`
  - Provide one reference implementation:
    - `MatOps.matmul_naive(a: MatI64, b: MatI64) -> MatI64` (O(n³), clear structure, not tuned).

- Bench alignment:
  - Add `matmul_core` benchmark:
    - Nyash: MatI64 + IntArrayCore implementation.
    - C: struct `{ int64_t *ptr; int64_t rows; int64_t cols; int64_t stride; }` + helper `get/set`.
  - Keep existing `matmul` (ArrayBox vs raw `int*`) as “language‑level” benchmark.

Out of scope for 21.6:

- Auto‑rewrite from `ArrayBox` → `IntArrayCore` / `MatI64` in AotPrep (only sketched, not default).
- SIMD / blocked matmul / cache‑tuned kernels (can be separate optimization phases).
- f64/complex variants (only type skeletons, if any).

## Design Notes

- **Layering**
  - Core: IntArrayCore (and future F64ArrayCore) are “muscle” boxes: minimal, numeric‑only. NyRT では IntArrayCore（Rust）、Hako では IntArrayCoreBox として露出。
  - Matrix: MatI64 expresses 2D shape and indexing; it owns an IntArrayCoreBox.
  - High‑level: ArrayBox / MapBox / existing user APIs remain unchanged.

- **Hako ABI vs Nyash implementation**
  - IntArrayCore lives as a NyRT box (C/Rust implementation) exposed via Hako ABI (`nyash.intarray.*`).
  - IntArrayCoreBox, MatI64 and MatOps are written in Nyash, calling IntArrayCore via externcall while exposing boxcall APIs to user code.
  - This keeps heavy lifting in NyRT while keeping the 2D semantics in `.hako`.

- **Fair C comparison**
  - For `matmul_core`, C should mirror IntArrayCore/MatI64:
    - Same struct layout (ptr + len / rows + cols + stride).
    - Same naive O(n³) algorithm.
  - This separates:
    - “Nyash vs C as languages” → existing `matmul` (ArrayBox vs `int*`).
    - “Core numeric kernel parity” → new `matmul_core` (IntArrayCore vs equivalent C).

## AotPrep / Future Work (21.6+)

Not for default in 21.6, but to keep in mind:

- Add conservative patterns in Collections/AotPrep to detect:
  - `ArrayBox<i64>` with:
    - Fixed length.
    - No structural mutations after initialization.
    - Access patterns of the form `base + i*cols + j` (or similar linear forms).
  - Allow opt‑in rewrite from such patterns to IntArrayCore/MatI64 calls.

- Keep all auto‑rewrites:
  - Behind env toggles (e.g. `NYASH_AOT_INTARRAY_CORE=1`).
  - Semantics‑preserving by construction; fall back to ArrayBox path when unsure.

## Open Questions for 21.6 Kickoff

- Exact module names:
  - `nyash.core.intarray` / `nyash.core.matrix` vs `nyash.linalg.*`.
- Bounds checking policy for IntArrayCore:
  - Always on (fail‑fast) vs dev toggle for light checks in hot loops.
- Interop:
  - Whether MatI64 should expose its IntArrayCore (e.g. `as_core_row_major()`) for advanced users.
