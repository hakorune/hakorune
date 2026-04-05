---
Status: SSOT
Decision: current
Date: 2026-04-05
Scope: `crates/nyash_kernel` の最終 owner graph を、Rust host microkernel / `.hako` semantic kernel / native accelerators の3 owner と、ABI facade / compat quarantine の2補助面で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
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
- optimization authority は `.hako owner / policy -> MIR canonical contract -> Rust executor` に従う

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

Keep the following files here explicitly:
- `crates/nyash_kernel/src/plugin/future.rs`
- `crates/nyash_kernel/src/plugin/invoke_core.rs`
- `crates/nyash_kernel/src/hako_forward_bridge.rs`

These are host service contracts / runtime glue, not compat quarantine.

### 2. `.hako` semantic kernel

- collection owner semantics
- adapter defaults
- route / method / module semantics
- target move order:
  1. `Array owner`
  2. `Map owner`
  3. `String semantic owner`

This bucket owns meaning, not raw unsafe leaf work.

#### Array semantic owner seam

- visible owner:
  - `lang/src/runtime/collections/array_core_box.hako`
  - `lang/src/runtime/collections/array_state_core_box.hako`
- substrate below the owner:
  - `lang/src/runtime/substrate/raw_array/raw_array_core_box.hako`
  - `lang/src/runtime/substrate/ptr/ptr_core_box.hako`
- ABI facade:
  - `crates/nyash_kernel/src/plugin/array_substrate.rs`
- compat alias surface:
  - `crates/nyash_kernel/src/plugin/array_runtime_aliases.rs`
- compat/runtime forwarding:
  - `crates/nyash_kernel/src/plugin/array_runtime_facade.rs`
- native accelerators kept in Rust:
  - `crates/nyash_kernel/src/plugin/array_handle_cache.rs`
  - `crates/nyash_kernel/src/plugin/array_string_slot.rs`

The first pilot moves visible `ArrayBox.{get,set,push,len,length,size}` semantics,
fallback, and state bookkeeping to `.hako` owner authority. It does not move raw
slot implementation or cache/fast-path substrate out of Rust.
Historical runtime aliases remain a separate shrink-only surface and must not
pull owner logic back into Rust forwarding code.

#### Map semantic owner seam

- visible owner:
  - `lang/src/runtime/collections/map_core_box.hako`
  - `lang/src/runtime/collections/map_state_core_box.hako`
- substrate below the owner:
  - `lang/src/runtime/substrate/raw_map/raw_map_core_box.hako`
- ABI facade:
  - `crates/nyash_kernel/src/plugin/map_aliases.rs`
- observer shim:
  - `crates/nyash_kernel/src/plugin/map_substrate.rs`
- compat/runtime forwarding:
  - `crates/nyash_kernel/src/plugin/map_runtime_facade.rs`
- native accelerators kept in Rust:
  - `crates/nyash_kernel/src/plugin/map_probe.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_load.rs`
  - `crates/nyash_kernel/src/plugin/map_slot_store.rs`

The second pilot moves visible `MapBox.{get,set,has,len,length,size}` semantics,
key normalization, and state bookkeeping to `.hako` owner authority. It does not
move raw probe/load/store substrate out of Rust.

#### String semantic boundary seam

- `.hako` semantic owner:
  - `lang/src/runtime/kernel/string/README.md`
  - `lang/src/runtime/kernel/string/chain_policy.hako`
  - `lang/src/runtime/kernel/string/search.hako`
- VM-facing runtime wrapper:
  - `lang/src/runtime/collections/string_core_box.hako`
- Rust thin ABI facade:
  - `crates/nyash_kernel/src/exports/string.rs`
- Rust native/lifetime substrate:
  - `crates/nyash_kernel/src/exports/string_view.rs`
  - `crates/nyash_kernel/src/exports/string_helpers.rs`
  - `crates/nyash_kernel/src/exports/string_plan.rs`
- compat quarantine:
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`

String is not an owner cutover like Array/Map. The clean stop-line is:
- semantic owner lives in `.hako` string-kernel policy/control modules
- search wrappers such as `indexOf` / `lastIndexOf` read through `.hako` search owner helpers
- VM runtime wrapper remains separate from final semantic ownership
- borrowed view/span ownership, materialize boundaries, and raw copy/search fast
  paths stay in Rust
- quarantine dispatch code does not become a permanent string owner

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

For `Array`, the thin facade ends at `nyash.array.slot_*` aliases. Historical
runtime aliases such as `nyash.array.get_hh` / `set_hhh` / `has_hh` remain
compat forwarding only and must not regain owner logic.

For `Map`, the thin facade ends at `nyash.map.slot_*`, `probe_*`, and
`entry_count_i64`. Historical aliases such as `entry_count_h`, `cap_h`,
`clear_h`, and `delete_hh` remain compat-only and must not regain owner logic.

For `String`, the thin facade ends at exported `nyash.string.*` entrypoints.
Borrowed substring/view policy and materialize/search/copy substrate stay below
that facade in Rust.

### compat quarantine

- surrogate / stop-gap / migration-only route surfaces
- non-owner by policy
- shrink-only until absorbed by either host microkernel or `.hako` semantic kernel
- current example:
  - `crates/nyash_kernel/src/plugin/module_string_dispatch/**`

Do not classify `future.rs`, `invoke_core.rs`, or `hako_forward_bridge.rs` as quarantine.
They are Rust host microkernel glue.

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
4. review `String` last:
   - semantic owner can move
   - lifetime substrate stays in Rust unless proven safe
5. shrink `module_string_dispatch/**` toward either `.hako` semantic ownership
   or non-owner quarantine removal

## Success Reading

- Rust remains thin but high-density
- `.hako` owns semantics without swallowing host/lifetime substrate
- native accelerators stay replaceable and non-owning
- `main kilo` reopens only after semantic ownership is clean enough to stay stable
