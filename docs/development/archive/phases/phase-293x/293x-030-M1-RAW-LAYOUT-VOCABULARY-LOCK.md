# 293x-030 M1 RAW-LAYOUT-VOCABULARY-LOCK

Status: Landed
Date: 2026-05-08

## Decision

The first raw-layout substrate slice is accepted as MIR-owned vocabulary and a
narrow `repr_c_v0` planner for fixed-width numeric fields.

This is not `.hako struct` syntax and not backend-active native allocation. It
exists so future parser and allocator rows can target one MIR-owned layout
contract instead of rediscovering field offsets in `.inc`, runtime helpers, or
application-specific code.

## Scope

Accepted in this card:

- MIR `raw_layout` vocabulary.
- `repr_c_v0` layout kind.
- fixed-width numeric field storage:
  - `i8`, `i16`, `i32`, `i64`
  - `u8`, `u16`, `u32`, `u64`
- natural alignment / padding / final size calculation for the narrow fixed
  field set.
- fail-fast diagnostics for:
  - empty layout names
  - empty field sets
  - duplicate field names
  - pointer-sized fields such as `usize` / `isize`
  - semantic Box fields such as `StringBox`

Deferred:

- `.hako struct` syntax
- `@rune Repr(C)` parser metadata
- `@rune Align(N)` syntax or metadata
- `sizeof`, `offsetof`, `alignof` source operators
- pointer / handle fields
- target-ABI-dependent `usize` / `isize`
- backend-active native allocation or field load/store
- Stage0 / MIR JSON exposure

## Owner Boundary

- `.hako` source owns no new syntax in this card.
- MIR owns the raw-layout vocabulary and narrow planning contract.
- Backends must not infer raw layout from semantic `box` declarations.
- `.inc` may only read a future MIR/JSON raw-layout plan; it must not compute
  field offsets by source names or app-specific types.
- Runtime helpers remain allocation/storage owners when a future row makes raw
  layout executable.

## Gates

```bash
cargo test -q raw_layout --lib
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Continue with `M4 minimum verifier hardening` before exposing broader
`hako.mem` / `hako.buf` / `hako.ptr` allocator-facing rows.
