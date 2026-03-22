---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の V4 として、`hako.mem` / `hako.buf` / `hako.ptr` の最小責務と staging order を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - lang/src/runtime/substrate/README.md
  - lang/src/runtime/substrate/mem/README.md
  - lang/src/runtime/substrate/buf/README.md
  - lang/src/runtime/substrate/ptr/README.md
---

# Minimal Capability Modules (SSOT)

## Goal

- `hako.mem` / `hako.buf` / `hako.ptr` を一気に実装せず、docs-first の capability boxes として責務だけ固定する。
- `RawArray` / `RawMap` の前に、low-level algorithm が依存する最小 substrate を順番に切る。
- unrestricted unsafe ではなく、restricted capability surface を前提にする。

## Fixed Order

1. `hako.mem`
2. `hako.buf`
3. `hako.ptr`
4. minimum verifier

この順番を current lane の正本とする。

## Module Roles

### `hako.mem`

- lowest staged capability
- native intrinsic backed memory primitive facade
- owns:
  - alloc / realloc / free
  - memcpy / memmove / memset / memcmp
  - alignment request vocabulary
- does not own:
  - len/cap policy
  - typed pointer facade
  - verifier policy

### `hako.buf`

- buffer-shape capability
- owns:
  - len / cap
  - reserve / grow / shrink
  - set_len
- depends on:
  - `hako.mem`
- does not own:
  - raw allocation primitive itself
  - typed pointer dereference semantics
  - collection policy

### `hako.ptr`

- typed pointer/span facade
- owns:
  - typed pointer naming
  - span/slice-like view vocabulary
  - inbounds/raw read-write entry vocabulary
- depends on:
  - `hako.mem`
  - `hako.buf`
- does not own:
  - allocator policy
  - TLS/atomic/GC policy
  - unrestricted pointer arithmetic

## Minimum Verifier Follow-Up

minimum verifier はこの sliceでは未実装だが、次の lock は次で固定する。

1. bounds
2. initialized-range
3. ownership

`double free` / `use-after-free` は後続 widening に送る。

docs-side truth is:

- [`minimum-verifier-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimum-verifier-ssot.md)

## First Consumer

`hako.mem` / `hako.buf` / `hako.ptr` / minimum verifier の最初の consumer は `RawArray` で固定する。

- docs-side truth:
  - [`raw-array-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-array-substrate-ssot.md)
- reading:
  - `RawArray` is above these capability modules
  - `RawArray` is not `RawMap`
  - `RawArray` does not collapse `runtime/collections/` owner boxes into `runtime/substrate/`

## Physical Staging

current physical staging path は次。

- [`lang/src/runtime/substrate/mem/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/mem/README.md)
- [`lang/src/runtime/substrate/buf/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/buf/README.md)
- [`lang/src/runtime/substrate/ptr/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/ptr/README.md)

この wave では README だけを置く。
current implementation note:
- the first live subset now exists as `MemCoreBox.alloc_i64/realloc_i64/free_i64`
- `BufCoreBox.len_i64/cap_i64/reserve_i64/grow_i64` is the first live buffer-shaped facade
- `shrink` / `set_len` remain future widening targets

## Non-Goals

- `RawArray` / `RawMap` implementation
- allocator state machine
- TLS / atomic / GC capability implementation
- OS VM / final allocator / final ABI stubs
- `runtime/collections/` owner migration
- broad implementation tree under `runtime/substrate/`
