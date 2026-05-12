---
Status: Complete
Date: 2026-05-12
Scope: first low-level byte-length `usize` facade aliases.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/reference/runtime/substrate-capabilities.md
---

# 294x-14a Low-Level Byte-Length Usize Facades

## Decision

The first low-level `usize` capability row is limited to byte-length facades
whose current execution can be made truthful over the dynamic `Integer(i64)`
lane.

Live v0 meaning:

```text
usize facade argument = non-negative current-lane i64 subset
```

This does not claim full `u64` or native pointer-sized slot behavior. It gives
allocator code a truthful `usize` spelling for sizes while preserving fail-fast
behavior for values that are invalid in the current lane.

## Live Surface

RawBuf:

```text
RawBufCoreBox.alloc_bytes_usize(size: usize)
RawBufCoreBox.realloc_bytes_usize(ptr: i64, new_size: usize)
```

OSVM:

```text
OsVmCoreBox.page_size_usize()
OsVmCoreBox.reserve_bytes_usize(len_bytes: usize)
OsVmCoreBox.commit_bytes_usize(base: i64, len_bytes: usize)
OsVmCoreBox.decommit_bytes_usize(base: i64, len_bytes: usize)
```

The wrappers delegate to the existing `*_i64` facades after rejecting negative
byte lengths. No new native leaf is added.

## Out Of Scope

- RawArray index/length/capacity `usize` variants.
- Bounds verifier `usize` variants.
- Native `usize` storage slots.
- Direct `hako_osvm_*_usize` extern leaves.
- hako_alloc production migration to `usize`.

## Verification

```text
cargo test -q --lib subset_accepts_boxcall_rawbufcore_alloc_bytes_usize
cargo test -q --lib subset_accepts_boxcall_rawbufcore_realloc_bytes_usize
cargo test -q --lib subset_accepts_boxcall_osvmcore_page_size_usize
cargo test -q --lib subset_accepts_boxcall_osvmcore_reserve_bytes_usize
cargo test -q --lib subset_accepts_boxcall_osvmcore_commit_bytes_usize
cargo test -q --lib subset_accepts_boxcall_osvmcore_decommit_bytes_usize
```
