---
Status: Active
Decision: provisional
Date: 2026-03-23
Scope: collection owner stop-line の次として、`.hako` kernel を capability ladder 経由で deeper substrate へ進めるための final shape と implementation order を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/design/substrate-capability-ladder-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
---

# Phase 29ct: Substrate Capability Ladder

## Goal

- final shape を「`.hako` owner + thin native metal keep」として固定する。
- collection owner stop-line の次を、perf micro-tuning ではなく substrate capability widening の順番で読む。
- `hakozuna` を含む future low-level kernel/allocator policy owner を `.hako` に寄せるための fixed order を決める。

## Final Shape

1. `.hako semantic owner`
   - collection/runtime/allocator policy
   - visible contract/fallback
2. `.hako algorithm substrate`
   - `RawArray`
   - `RawMap`
   - future allocator state machine
3. capability substrate
   - `hako.abi`
   - `hako.value_repr`
   - `hako.mem`
   - `hako.buf`
   - `hako.ptr`
   - `hako.atomic`
   - `hako.tls`
   - `hako.gc`
   - `hako.osvm`
4. native metal keep
   - OS VM
   - final allocator calls
   - GC hooks
   - final ABI stubs
   - platform-specific TLS/atomic fallback

## Fixed Order

1. docs/task lock
   - `substrate-capability-ladder-ssot.md`
   - `value-repr-and-abi-manifest-ssot.md`
2. ABI/value manifest lock
   - current `nyash.array.*` / `nyash.map.*` / `nyash.runtime_data.*` symbol inventory
   - canonical value classes and ownership
3. minimal capability modules
   - `hako.mem`
   - `hako.buf`
   - `hako.ptr`
   - minimum verifier
4. `RawArray`
5. `RawMap`
6. GC/TLS/atomic capability widening
7. Hakozuna portability layer
8. final metal split

## Exact First Tasks

1. manifest inventory for current collection/kernel exports
   - `docs/development/current/main/design/abi-export-inventory.md`
   - `crates/nyash_kernel/src/plugin/array.rs`
   - `crates/nyash_kernel/src/plugin/map.rs`
   - `crates/nyash_kernel/src/plugin/runtime_data.rs`
   - `lang/src/vm/boxes/abi_adapter_registry.hako`
2. value representation lock
   - `crates/nyash_kernel/src/plugin/value_codec/mod.rs`
   - `crates/nyash_kernel/src/plugin/value_codec/decode.rs`
   - `crates/nyash_kernel/src/plugin/value_codec/encode.rs`
3. metal helper contract lock
   - `crates/nyash_kernel/src/plugin/handle_cache.rs`
4. future substrate module root lock
   - `lang/src/runtime/substrate/` for physical staging

## Landed Slice

- `V0 ABI export inventory` landed
  - docs-side truth lives in [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
  - `AbiAdapterRegistryBox` is fixed as runtime consumer/default-row registry, not manifest SSOT
  - current export surface is classified as:
    - `mainline substrate`
    - `runtime-facade`
    - `compat-only`
    - `adapter-default consumer`

- `V1 value representation lock` landed
  - canonical classes are fixed as:
    - `imm_i64`
    - `imm_bool`
    - `handle_owned`
    - `handle_borrowed_string`
    - `boxed_local`
  - `value_public` stays inventory-only umbrella
  - `BorrowedHandleBox` is fixed as the current concrete borrowed-string alias carrier
  - `CodecProfile` is fixed as helper policy, not public ABI schema

- `V2 metal helper contract lock` landed
  - [`handle-cache-metal-helper-contract-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md) fixes `handle_cache.rs` as:
    - typed handle cache
    - typed dispatch helper
    - array i64 re-encode helper
  - non-goals are fixed:
    - not ABI manifest truth
    - not value representation owner
    - not array/map policy owner

- `V3 future substrate module root lock` landed
  - physical staging root is now reserved at:
    - [`lang/src/runtime/substrate/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/README.md)
    - [`lang/src/runtime/substrate/hako_module.toml`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/hako_module.toml)
  - current non-goals are fixed:
    - no `hako.mem` / `hako.buf` / `hako.ptr` implementation yet
    - no `RawArray` / `RawMap` yet
    - no allocator/TLS/atomic/GC policy move yet
  - next active slice is `V4 minimal capability modules`
    - first targets:
      - `hako.mem`
      - `hako.buf`
      - `hako.ptr`

- `V4 minimal capability modules` landed
  - [`minimal-capability-modules-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimal-capability-modules-ssot.md) fixes:
    - `mem -> buf -> ptr -> verifier` order
    - per-module responsibilities
    - non-goals for this wave
  - physical staging docs now exist at:
    - [`lang/src/runtime/substrate/mem/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/mem/README.md)
    - [`lang/src/runtime/substrate/buf/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/buf/README.md)
    - [`lang/src/runtime/substrate/ptr/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/ptr/README.md)

- `V5 minimum verifier lock` landed
  - [`minimum-verifier-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimum-verifier-ssot.md) fixes:
    - `bounds -> initialized-range -> ownership` order
    - current non-goals for this wave
    - docs-first reading only
  - physical staging root now exists at:
    - [`lang/src/runtime/substrate/verifier/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/verifier/README.md)
  - next active slice is `C2 RawArray`

- `C2 RawArray` landed as docs/task lock
  - [`raw-array-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-array-substrate-ssot.md) fixes:
    - `RawArray` is the first algorithm-substrate consumer of `mem/buf/ptr/verifier`
    - `ptr/cap/len`, reserve/grow, slot load/store, append-at-end are the current owned roles
    - current non-goals for this wave
  - physical staging root now exists at:
    - [`lang/src/runtime/substrate/raw_array/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_array/README.md)
  - next active slice is `C3 RawMap`

- `C3 RawMap` landed as docs/task lock
  - [`raw-map-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-map-substrate-ssot.md) fixes:
    - `RawMap` is the next algorithm-substrate consumer after `RawArray`
    - bucket/probe/tombstone/rehash are the current owned roles
    - current non-goals for this wave
  - physical staging root now exists at:
    - [`lang/src/runtime/substrate/raw_map/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_map/README.md)
  - next active slice is `C4 GC/TLS/atomic capability widening`

- `C4 GC/TLS/atomic capability widening` landed as docs/task lock
  - [`gc-tls-atomic-capability-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/gc-tls-atomic-capability-ssot.md) fixes:
    - `atomic -> tls -> gc` order
    - per-module owned roles
    - current non-goals for this wave
  - physical staging roots now exist at:
    - [`lang/src/runtime/substrate/atomic/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/atomic/README.md)
    - [`lang/src/runtime/substrate/tls/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/tls/README.md)
    - [`lang/src/runtime/substrate/gc/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/gc/README.md)
  - next active slice is `C6 final metal split detail lock`

- `C6 final metal split detail lock` landed
  - [`final-metal-split-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/final-metal-split-ssot.md) fixes:
    - `.hako owner` / `native metal keep` final split table
    - fail-fast reading for the current lane
    - current non-goals for this wave
  - `C5 Hakozuna portability layer` remains ladder-only and deferred

## Stop-Line

- do not reopen wide perf exploration before `docs/task lock` and `ABI/value manifest lock` land
- do not push allocator/TLS/queue policy into `.hako` before `hako.mem` / `hako.ptr` / minimum verifier exist
- do not treat current collection owner stop-line as end-state completion

## Non-Goals

- immediate native metal rewrite
- big-bang allocator migration
- unrestricted unsafe surface
- undoing the current collection owner cutover
