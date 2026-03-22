---
Status: SSOT
Decision: provisional
Date: 2026-03-23
Scope: `.hako` kernel を collection owner stop-line の先へ進めるため、capability ladder と native keep の境界を 1 枚で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - lang/src/runtime/collections/README.md
  - crates/nyash_kernel/src/plugin/handle_cache.rs
---

# Substrate Capability Ladder (SSOT)

## Goal

- end-state を「OCaml で止まる」でも「native metal を即 `.hako` 完全再現する」でもなく、capability ladder を経由して Rust-class の低層表現力へ近づく形で固定する。
- collection owner cutover の次を、perf micro-tuning ではなく substrate capability widening として読む。
- `hakozuna` のような allocator/runtime policy owner を将来 `.hako` に寄せるための、最小能力集合と fixed order を決める。

## Final Shape

この lane の最終形は 4 層で読む。

1. `.hako semantic owner`
   - user-visible collection/runtime/allocator policy
   - grow / split / append / probe / reclaim の algorithmic low-level
   - route/fallback/contract
2. `.hako algorithm substrate`
   - `RawArray` / `RawMap` / future allocator state machine
   - capability module を使う low-level control structure
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
   - ここは `.hako` surface を持つが、初期段階では native intrinsic へ lowering してよい
4. native metal keep
   - OS virtual memory
   - final allocator call
   - final GC barrier / root / pin hook
   - final ABI entry stub
   - platform fallback TLS / atomics

## Reading Rule

- preferred target is not `Rust line count zero`.
- preferred target is not `native metal zero`.
- preferred target is:
  - `.hako` owns meaning/policy/control
  - capability substrate exposes the minimum unsafe power required to express low-level algorithms
  - native keeps only OS/ABI/GC metal

## Capability Modules

logical module family は次で固定する。

- `hako.abi`
  - extern declaration
  - symbol name
  - ownership contract
  - callback / struct layout manifest
- `hako.value_repr`
  - runtime value classes
  - borrowed/owned handle distinction
  - bridge-visible representation rules
- `hako.mem`
  - alloc / realloc / free
  - memcpy / memmove / memset / memcmp
  - alignment
- `hako.buf`
  - len / cap
  - reserve / grow / shrink
  - set_len
- `hako.ptr`
  - typed pointer/span facade
  - inbounds/raw read/write
- `hako.atomic`
  - load/store
  - CAS
  - fetch_add
  - fence
- `hako.tls`
  - thread/task local storage
  - cache slot primitive
- `hako.gc`
  - write_barrier
  - root_scope
  - pin/unpin
- `hako.osvm`
  - page reserve/commit/decommit facade

physical staging path は、当面 `lang/src/runtime/substrate/` に置く読みで固定する。
logical `hako.*` 名が先で、directory 名はそれに従わせる。

current root lock is:

- [`lang/src/runtime/substrate/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/README.md)
- [`lang/src/runtime/substrate/hako_module.toml`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/hako_module.toml)

## Native Keep Lock

次は native keep に固定する。

- OS VM syscall glue
- final allocator backend
- final ABI symbol stub
- platform-specific TLS fallback
- platform-specific atomics fallback
- final GC integration points

`native keep` は policy owner ではなく、metal service provider として扱う。

## Safety Lock

- unrestricted raw pointer は入れない。
- capability token 付きの restricted unsafe を原則にする。
- minimum verifier は capability 導入と同時に必要。
- full sanitizer は後続でよいが、次は初期段階から fail-fast にする。
  - bounds
  - initialized range
  - ownership mismatch
  - double free

## Fixed Order

### C0. Value/ABI lock

- `hako.value_repr` と ABI manifest を先に固定する。
- current hand-written export surface はこの lock より先に増やさない。

### C1. Minimal memory capabilities

- `hako.mem`
- `hako.buf`
- `hako.ptr`
- current docs lock:
  - [`minimal-capability-modules-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimal-capability-modules-ssot.md)

ここでは native intrinsic lowering を許可する。

### C1.5. Minimum verifier

- bounds fail-fast
- initialized-range fail-fast
- ownership fail-fast
- current docs lock:
  - [`minimum-verifier-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimum-verifier-ssot.md)

これは `C1` と同時導入する。

### C2. `RawArray`

- `ptr/cap/len`
- reserve/grow
- slot load/store
- append-at-end policy

collection owner の次の本命はここ。

### C3. `RawMap`

- probe
- tombstone
- rehash
- bucket walk

`MapBox` semantics を `.hako` に deeper cut する土台。

### C4. GC/TLS/atomics

- `hako.atomic`
- `hako.tls`
- `hako.gc`

allocator/runtime policy owner を深くする前に必要。

### C5. Hakozuna portability layer

- size-class policy
- ptr/bin policy
- remote-free routing policy
- TLS cache policy
- telemetry/profile policy

ここでは policy を `.hako` に持ち、metal primitive は native keep する。

### C6. Hakozuna metal split

- `.hako` owner:
  - allocator state machine
  - bin policy
  - reclaim heuristics
  - queue policy
- native keep:
  - OS VM
  - final allocator call
  - final platform stubs

## Immediate Task Pack

いま先に切るべき実作業は次。

1. ABI export surface を manifest 化する
   - docs-side inventory truth:
     - [`abi-export-inventory.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/abi-export-inventory.md)
   - `crates/nyash_kernel/src/plugin/array.rs`
   - `crates/nyash_kernel/src/plugin/map.rs`
   - `crates/nyash_kernel/src/plugin/runtime_data.rs`
   - `lang/src/vm/boxes/abi_adapter_registry.hako`
2. `value_codec` を representation owner として棚卸しする
   - `crates/nyash_kernel/src/plugin/value_codec/`
3. handle cache/downcast を metal helper contract として固定する
   - `crates/nyash_kernel/src/plugin/handle_cache.rs`
   - [`handle-cache-metal-helper-contract-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md)
4. `RawArray` 用の substrate module layout を docs で確定する
   - future physical home: `lang/src/runtime/substrate/`

## Non-Goals

- native allocator/OS VM を即 `.hako` 実装すること
- current perf micro leaf を capability widening と混ぜること
- collection owner cutover を無効化して Rust method-shaped owner に戻すこと
- unrestricted unsafe surface を導入すること
