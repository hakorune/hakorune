---
Status: SSOT
Decision: current
Date: 2026-04-05
Scope: `crates/nyash_kernel` の最終 owner graph を、Rust host microkernel / `.hako` semantic kernel / native accelerators の3 owner と、ABI facade / compat quarantine の2補助面で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/kernel-replacement-axis-ssot.md
  - docs/development/current/main/phases/phase-134x/README.md
  - docs/development/current/main/phases/phase-138x/README.md
  - crates/nyash_kernel/src/entry.rs
  - crates/nyash_kernel/src/ffi/mod.rs
  - crates/nyash_kernel/src/exports/string.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/plugin/array_substrate.rs
  - crates/nyash_kernel/src/plugin/map_aliases.rs
  - crates/nyash_kernel/src/plugin/future.rs
  - crates/nyash_kernel/src/plugin/invoke_core.rs
  - crates/nyash_kernel/src/hako_forward_bridge.rs
  - crates/nyash_kernel/src/plugin/module_string_dispatch
---

# nyash_kernel Semantic Owner SSOT

## Goal

- Rust を減らすことではなく、Rust から semantic ownership を外す
- Rust は `host microkernel` と `native accelerators` に寄せる
- `.hako` は collection / route / adapter semantics の owner になる
- `compat quarantine` を permanent owner にしない

## Final Owner Graph

### 1. Rust host microkernel

- process/bootstrap
- host service contracts
- FFI / handle / lifetime / unsafe boundary
- examples:
  - `crates/nyash_kernel/src/entry.rs`
  - `crates/nyash_kernel/src/ffi/**`
  - `crates/nyash_kernel/src/plugin/future.rs`
  - `crates/nyash_kernel/src/plugin/invoke_core.rs`
  - `crates/nyash_kernel/src/hako_forward_bridge.rs`

This bucket must not absorb new collection semantics.

### 2. `.hako` semantic kernel

- collection owner semantics
- adapter defaults
- route / method / module semantics
- target move order:
  1. `Array owner`
  2. `Map owner`
  3. `String semantic owner`

This bucket owns meaning, not raw unsafe leaf work.

### 3. native accelerators

- lifetime-sensitive substrate
- raw fast paths
- copy/search/cache leaves
- examples:
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`
  - map slot/probe/store leaves below runtime facade

This bucket provides capability only and must not become a semantic owner.

## Auxiliary Surfaces

### ABI facade

- thin export / alias shell only
- examples:
  - `crates/nyash_kernel/src/exports/string.rs`
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`

No new domain semantics belong here.

### compat quarantine

- surrogate / stop-gap / migration-only route surfaces
- non-owner by policy
- shrink-only until absorbed by either host microkernel or `.hako` semantic kernel
- current example:
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`

## Stop Lines

- keep `entry.rs` and `ffi/**` in Rust
- keep host service contracts in Rust
- keep lifetime-sensitive hot leaves in Rust until a source-backed replacement exists
- do not move `StringView` lifetime ownership to `.hako`
- do not broaden `compat quarantine`
- do not attach new semantics to ABI facades

## Migration Order

1. freeze the final owner graph
2. move `Array owner` to `.hako`
3. move `Map owner` to `.hako`
4. shrink `module_string_dispatch/**` toward `.hako` semantic ownership
5. review `String` last:
   - semantic owner can move
   - lifetime substrate stays in Rust unless proven safe

## Success Reading

- Rust remains thin but high-density
- `.hako` owns semantics without swallowing host/lifetime substrate
- native accelerators stay replaceable and non-owning
- `main kilo` reopens only after semantic ownership is clean enough to stay stable
