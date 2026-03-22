---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `phase-29ct` の C2 として、`RawArray` を capability substrate の最初の consumer として docs-first で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - lang/src/runtime/substrate/README.md
  - lang/src/runtime/substrate/raw_array/README.md
---

# RawArray Substrate (SSOT)

## Goal

- `RawArray` を `.hako algorithm substrate` の最初の concrete box として固定する。
- `hako.mem` / `hako.buf` / `hako.ptr` / minimum verifier の上に乗る最初の consumer を明確にする。
- `ArrayCoreBox` の visible owner を崩さず、future low-level array policy の受け皿だけを docs-first で決める。

## Reading

- `RawArray` は semantic owner ではない。
- `RawArray` は capability substrate を使う algorithm substrate である。
- current phase では docs-first only とし、まだ `.hako` 実装本体は作らない。

## Fixed Dependencies

`RawArray` の前提は次で固定する。

1. `hako.mem`
2. `hako.buf`
3. `hako.ptr`
4. minimum verifier
   - `bounds`
   - `initialized-range`
   - `ownership`

`RawArray` はこの 4 箱の consumer であり、provider ではない。

## RawArray Roles

- owns:
  - `ptr/cap/len` shape
  - reserve/grow
  - slot load/store vocabulary
  - append-at-end policy
- does not own:
  - user-visible `ArrayBox` semantics
  - ABI manifest truth
  - final allocator backend
  - GC/TLS/atomic policy

## Relationship To Current Owner Boxes

- current semantic owner remains:
  - `runtime/collections/array_core_box.hako`
- current native metal helpers remain:
  - `crates/nyash_kernel/src/plugin/array*.rs`
  - `crates/nyash_kernel/src/plugin/handle_cache.rs`
- `RawArray` is the future algorithm substrate box that may sit between those layers later.

## Physical Staging

current staging root is reserved at:

- [`lang/src/runtime/substrate/raw_array/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_array/README.md)

This phase places README/docs only.

## Non-Goals

- `.hako` implementation body for `RawArray`
- `RawMap`
- allocator state machine
- TLS / atomic / GC implementation
- OS VM / final allocator / final ABI stub
- unrestricted raw pointer
- perf lane reopen

## Follow-Up

After this docs lock, the next widening remains:

1. `RawArray` implementation planning
2. `RawMap` docs/task lock
