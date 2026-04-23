# Nyash Strings: UTF‑8 First, Bytes Separate

Status: Design committed. This document defines how Nyash treats text vs bytes and the minimal APIs we expose in each layer.

## Principles
- UTF‑8 is the only in‑memory encoding for `StringBox`.
- Text operations are defined in terms of Unicode code points (CP). Grapheme cluster (GC) helpers may be added on top.
- Bytes are not text. Byte operations live in a separate `ByteCursorBox` and byte‑level instructions.
- Conversions are explicit.

## Implementation Note (Current Runtime)

The Rust runtime currently supports a legacy performance/compatibility switch:

- Default: **byte** indexing for `length/indexOf/lastIndexOf/substring`
- `NYASH_STR_CP=1`: **code point** indexing for the same APIs

This applies to both primitive `String` and `StringBox` paths. Long-term, the language-level SSOT is CP semantics; the env toggle exists to keep migrations reversible while the VM/runner paths are being unified.

## Model
- `StringBox`: immutable UTF‑8 string value. Public text APIs are CP‑indexed.
- `Utf8CursorBox`: delegated implementation for scanning and slicing `StringBox` as CPs.
- `ByteCursorBox`: independent binary view/holder for byte sequences.

## Invariants
- Indices are zero‑based. Slices use half‑open intervals `[i, j)`.
- CP APIs never intermix with byte APIs. GC APIs are explicitly suffixed (e.g., `*_gc`).
- Conversions must be explicit. No implicit transcoding.

## Core APIs (MVP)

Text (UTF‑8/CP): implemented by `StringBox` delegating to `Utf8CursorBox`.
- `length() -> i64` — number of code points.
- `substring(i,j) -> StringBox` — CP slice.
- `indexOf(substr, from=0) -> i64` — CP index or `-1`.
- `lastIndexOf(substr, from=length()) -> i64` — last CP index at or before `from`, or `-1`.
- Optional helpers: `startsWith/endsWith/replace/split/trim` as sugar.

Bytes: handled by `ByteCursorBox`.
- `len_bytes() -> i64`
- `slice_bytes(i,j) -> ByteCursorBox`
- `find_bytes(pattern, from=0) -> i64`
- `to_string_utf8(strict=true) -> StringBox | Error` — strict throws on invalid UTF‑8 (MVP may replace with U+FFFD when `strict=false`).

## Errors
- CP APIs clamp out‑of‑range indices (dev builds may enable strict). Byte APIs mirror the same behavior for byte indices.
- `to_string_utf8(strict=true)` fails on invalid input; `strict=false` replaces invalid sequences by U+FFFD.

## Interop
- FFI/ABI boundaries use UTF‑8. Non‑UTF‑8 sources must enter via `ByteCursorBox` + explicit transcoding.
- Other encodings (e.g., UTF‑16) are future work via separate cursor boxes; `StringBox` remains UTF‑8.

## Roadmap
1) Provide Nyash‑level MVP boxes: `Utf8CursorBox`, `ByteCursorBox`.
2) Route `StringBox` public methods through `Utf8CursorBox`.
3) Migrate Mini‑VM and macro scanners to use `Utf8CursorBox` helpers.
4) Add CP/byte parity smokes; later add GC helpers and normalizers.

## Proposed Convenience (design only)

Parsing helpers (sugar; documented during feature‑pause, not implemented):
- `toDigitOrNull(base=10) -> i64 | null`
  - Returns 0..9 when the code point is a decimal digit (or base subset), otherwise `null`.
  - CP based; delegates to `Utf8CursorBox` to read the leading code point.
- `toIntOrNull() -> i64 | null`
  - Parses the leading consecutive decimal digits into an integer; returns `null` when no digit at head.
  - Pure function; does not move any external cursor (callers decide how to advance).

Notes
- Zero new runtime opcodes; compiled as comparisons and simple arithmetic.
- `Option/Maybe` may replace `null` in a future revision; documenting `null` keeps MVP simple.
