# 293x-028 M0 NUMERIC-TYPE-NAME-STORAGE-LOCK

Status: Landed
Date: 2026-05-08

## Decision

The first mimalloc-grade numeric substrate slice is accepted as a
type-name/storage lock, not as full exact-width integer semantics.

The live vocabulary is:

```text
i8 i16 i32 i64 isize
u8 u16 u32 u64 usize
```

These names are owned by MIR `numeric_substrate` metadata. Current execution
still uses the existing dynamic `Integer(i64)` lane. Typed-object layout may use
the names as inline i64 storage hints while preserving the original declared
type name for later exact-width rows.

## Scope

Accepted in this card:

- MIR-owned numeric type-name classifier.
- typed-object field storage inference reads the classifier instead of carrying
  a local string list for numeric substrate names.
- `x: u32`, `x: u64`, `x: usize`, etc. may produce an inline-i64 typed-object
  slot in the current EXE route.
- Rust parser and `.hako` parser require no token widening for this slice
  because these names are ordinary `TYPE_REF` identifiers today.

Deferred:

- numeric literal suffixes such as `1u64`
- static range checks
- `u64` values outside signed i64
- wrapping arithmetic syntax
- checked arithmetic syntax
- logical vs arithmetic shift distinction
- MIR JSON exact-width const/type tags

## Owner Boundary

- `.hako` source owns ordinary type annotation text.
- MIR owns numeric type-name classification and typed-object storage planning.
- VM/EXE keep current i64 runtime behavior.
- Backends must not infer exact unsigned or fixed-width semantics from these
  names yet.

## Gates

```bash
cargo test -q numeric_substrate --lib
cargo test -q typed_object_plan --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Continue M0 as a separate row for exact arithmetic/shift semantics, or move to
M1 raw layout vocabulary if allocator substrate work only needs the current
type-name storage lock first.
