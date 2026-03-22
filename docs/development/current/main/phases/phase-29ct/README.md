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

## Stop-Line

- do not reopen wide perf exploration before `docs/task lock` and `ABI/value manifest lock` land
- do not push allocator/TLS/queue policy into `.hako` before `hako.mem` / `hako.ptr` / minimum verifier exist
- do not treat current collection owner stop-line as end-state completion

## Non-Goals

- immediate native metal rewrite
- big-bang allocator migration
- unrestricted unsafe surface
- undoing the current collection owner cutover
