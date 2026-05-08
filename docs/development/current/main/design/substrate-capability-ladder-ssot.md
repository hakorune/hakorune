---
Status: SSOT
Decision: provisional
Date: 2026-05-08
Scope: `.hako` kernel / collection / allocator owner を low-level substrate stop-line の先へ進めるため、capability ladder と native keep の境界を 1 枚で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29cm/README.md
  - docs/development/current/main/phases/phase-29ct/README.md
  - docs/development/current/main/phases/phase-293x/README.md
  - docs/development/current/main/design/abi-export-inventory.md
  - docs/development/current/main/design/handle-cache-metal-helper-contract-ssot.md
  - docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md
  - docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md
  - docs/development/current/main/design/minimal-capability-modules-ssot.md
  - docs/development/current/main/design/minimum-verifier-ssot.md
  - docs/development/current/main/design/raw-array-substrate-ssot.md
  - docs/development/current/main/design/raw-map-substrate-ssot.md
  - docs/development/current/main/design/gc-tls-atomic-capability-ssot.md
  - docs/development/current/main/design/final-metal-split-ssot.md
  - docs/development/current/main/design/collection-raw-substrate-contract-ssot.md
  - docs/development/current/main/design/de-rust-kernel-authority-cutover-ssot.md
  - docs/development/current/main/design/hako-runtime-c-abi-cutover-order-ssot.md
  - lang/src/runtime/collections/README.md
  - lang/src/hako_alloc/README.md
  - crates/nyash_kernel/src/plugin/handle_cache.rs
---

# Substrate Capability Ladder (SSOT)

## Goal

- end-state を「OCaml で止まる」でも「native metal を即 `.hako` 完全再現する」でもなく、capability ladder を経由して Rust-class の低層表現力へ近づく形で固定する。
- collection owner cutover の次を、perf micro-tuning ではなく substrate capability widening として読む。
- `hakozuna` のような allocator/runtime policy owner を将来 `.hako` に寄せるための、最小能力集合と fixed order を決める。
- `mimalloc-lite` と本物の allocator fast path を混同せず、`.hako` で allocator を深く書くための substrate prerequisite を固定する。

## Final Shape

この lane の最終形は 4 層で読む。

1. `hako_kernel`
   - user-visible collection/runtime/allocator policy
   - grow / split / append / probe / reclaim の algorithmic low-level
   - route/fallback/contract
2. `hako_substrate`
   - `RawArray` / `RawMap` / future allocator state machine
   - capability module を使う low-level control structure
   - PAL-like `.hako substrate` layer that owns low-level control shape while leaving final libc / OS glue below
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
  - `hako_substrate` owns the PAL-like low-level control role
  - capability substrate exposes the minimum unsafe power required to express low-level algorithms
  - native keeps only OS/ABI/GC metal
- do not introduce `hako.sys` as a monolithic unsafe shelf; keep the capability family split and name the two `.hako` owner layers as `hako_kernel` and `hako_substrate`.
- do not mirror a single `std::sys::pal`-style giant OS layer under `.hako`; split future OS-facing substrate work by capability family and keep final libc / platform glue below that seam.

## Allocator / Mimalloc Reading

This doc is the owner for allocator-grade substrate capability reading.
Do not create a separate mimalloc capability ladder. Add concrete rows here,
or to the narrower `hako_alloc` policy/state contract when the row is
allocator-specific.

Concrete implementation ordering now lives in:

- [`mimalloc-capability-taskboard-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/mimalloc-capability-taskboard-ssot.md)

The accepted design is capability-module based:

```text
hako.mem
hako.buf
hako.ptr
hako.atomic
hako.tls
hako.osvm
@rune Contract(...)
minimum verifier
```

Do not replace this with a broad C-style unsafe surface.

`mimalloc-lite` and allocator policy models are allowed to stay at the current
Box/VM policy level. They can prove size-class choices, page/free-list state,
statistics, and app-level behavior.

`mimalloc`-grade native fast paths are different. They require substrate
capabilities before `.hako` code can claim ownership of the hot path:

- numeric substrate: `usize` / fixed-width signed and unsigned integers,
  logical shift semantics, wrapping arithmetic, checked arithmetic
- memory substrate: `hako.mem`, `hako.buf`, `hako.ptr`, `RawBuf`,
  `RawArray`, `MaybeInit`
- layout substrate: fixed layout vocabulary, alignment, `sizeof`, `offsetof`,
  and repr-like contracts separate from semantic `box` objects
- verifier substrate: bounds, initialized range, ownership, double-free,
  `no_alloc`, and `no_safepoint`
- parallel/runtime substrate: TLS slots, atomics with memory order, GC barrier
  hooks, and OS VM page reserve/commit/decommit
- optimization/export substrate: `noalias`, `nocapture`, `nonnull`,
  `readonly`/`readnone`, alignment attributes, `clz`/`ctz`/`popcnt`,
  prefetch/assume, and static const tables

Current note: the export-attrs consistency guard is live for the narrow
`readonly` / `nocapture` / runtime-decl attr surface. It is a drift guard only;
it does not make `noalias`, `nonnull`, `dereferenceable`, or alignment export
backend-active.

Strong pointer attrs require a separate proof vocabulary before they can become
live. `handle_*` return classes and `native_ptr_*` return classes must not be
collapsed: `handle_owned` is a runtime value class, not an LLVM pointer attr
target. The next strong-attrs prerequisite is `M10c-pre pointer/handle return
proof vocabulary`.

Syntax alone does not make a row live. A capability row becomes live only when
the MIR/value representation, VM/LLVM/Stage0 consumer, fail-fast diagnostics,
fixture, and gate are all named. Until then, it remains reserved vocabulary.

`box` remains the semantic object mechanism. Allocator metadata must not rely
on dynamic user-box fields as its native layout. Fixed allocator metadata
belongs to the raw/layout substrate family, not to broad Box semantics.

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

Future OS-facing surface stays split by family as well:

- `hako.osvm` for page/virtual-memory capability
- `hako_std` families for file/process/time/env/net-facing facades
- no monolithic `.hako` OS shelf that mixes those responsibilities

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
It is the final libc / syscall / platform-difference absorber for Linux, Windows, and macOS leaf behavior.

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

### C0.5. Numeric substrate lock

- fixed-width integer and pointer-sized vocabulary is locked before allocator
  fast-path substrate.
- live narrow vocabulary:
  - `usize` / `isize`
  - `u8` / `u16` / `u32` / `u64`
  - `i8` / `i16` / `i32` / `i64`
- current live reading:
  - these names are MIR-classified type annotation text
  - typed-object storage planning may use them as inline i64 storage hints
  - runtime values still use the current dynamic `Integer(i64)` lane
  - current `>>` in that lane is signed i64 arithmetic shift
- reserved target vocabulary:
  - explicit wrapping arithmetic
  - explicit checked arithmetic
  - explicit logical right-shift surface distinct from current `>>`
- live acceptance requires:
  - docs/reference Decision
  - value representation row
  - MIR type/literal/op row
  - VM and LLVM lowering row
  - Stage0/JSON behavior row when exposed across that boundary
  - fixture and gate

Until the arithmetic half of this lock is live, allocator policy rows that need
width/overflow facts must stay as narrow `i64` models or helper-backed probes.
Do not introduce parser syntax first and leave backend meaning implicit.

### C0.75. Raw layout vocabulary

- `box` remains the semantic object surface.
- raw allocator metadata uses MIR-owned raw-layout facts, not dynamic Box
  fields.
- current live reading:
  - MIR `raw_layout` module owns `repr_c_v0`
  - fixed-width numeric fields are accepted:
    - `i8` / `i16` / `i32` / `i64`
    - `u8` / `u16` / `u32` / `u64`
  - natural alignment, field offsets, and final size are planned by MIR
  - pointer-sized fields, Box fields, and source syntax are still reserved
- live acceptance requires:
  - docs/reference Decision
  - MIR-owned vocabulary row
  - fail-fast unsupported type row
  - fixture and gate

### C1. Minimal memory capabilities

- `hako.mem`
- `hako.buf`
- `hako.ptr`
- current docs lock:
  - [`minimal-capability-modules-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimal-capability-modules-ssot.md)
- current implementation note:
  - `MemCoreBox.alloc_i64/realloc_i64/free_i64` is live
  - `BufCoreBox.len_i64/cap_i64/reserve_i64/grow_i64` is live
  - `BufCoreBox.cap_i64` routes through `PtrCoreBox.slot_cap_i64`; direct
    `nyash.array.slot_cap_h` ownership stays below the buf facade
  - `RawArrayCoreBox.slot_reserve_i64/slot_grow_i64` now route through `BufCoreBox`

ここでは native intrinsic lowering を許可する。

### C1.5. Minimum verifier

- bounds fail-fast
- initialized-range fail-fast
- ownership fail-fast
- current docs lock:
  - [`minimum-verifier-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/minimum-verifier-ssot.md)

current implementation note:
- `BoundsCoreBox.ensure_index_i64` is live
- `BoundsCoreBox.ensure_insert_index_i64` is live for RawArray insert
- `InitializedRangeCoreBox.ensure_initialized_index_i64` is live
- `OwnershipCoreBox.ensure_handle_readable_i64/ensure_handle_writable_i64/ensure_any_readable_i64` is live
  - RawArray remove/insert now compose these verifier gates before pointer
    substrate calls

これは `C1` と同時導入する。

### C2. `RawArray`

- `ptr/cap/len`
- reserve/grow
- slot load/store
- append-at-end policy
- current docs lock:
  - [`raw-array-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-array-substrate-ssot.md)

collection owner の次の本命はここ。

current implementation note:
- `RawArrayCoreBox.slot_cap_i64` is live as a readable ownership-gated
  capacity observer over `BufCoreBox.cap_i64`.
- `RawArrayCoreBox.slot_reserve_i64/slot_grow_i64` stay writable
  ownership-gated over `BufCoreBox`.

### C2.5. `RawBuf`

- raw byte-buffer allocation vocabulary
- alloc/realloc/free facade above `hako.mem`
- future home for verifier-backed raw buffer state
- current live surface:
  - [`raw_buf/README.md`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_buf/README.md)
  - [`raw_buf/raw_buf_core_box.hako`](/home/tomoaki/git/hakorune-selfhost/lang/src/runtime/substrate/raw_buf/raw_buf_core_box.hako)

This is an allocator-substrate consumer, not allocator policy/state owner.
`len/cap`, native layout, `MaybeInit`, `no_alloc`, `no_safepoint`,
TLS/atomic/OS VM, and double-free verification remain future rows.

### C3. `RawMap`

- probe
- tombstone
- rehash
- bucket walk
- current docs lock:
  - [`raw-map-substrate-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/raw-map-substrate-ssot.md)

`MapBox` semantics を `.hako` に deeper cut する土台。

current implementation note:
- `RawMapCoreBox.entry_count_i64/probe_*/slot_load_*/slot_store_*` are live
- `RawMapCoreBox.cap_i64` is the first truthful capacity observer
- `rehash/tombstone` remain parked until a truthful native seam exists

### C4. GC/TLS/atomics

- `hako.atomic`
- `hako.tls`
- `hako.gc`
- current docs lock:
  - [`gc-tls-atomic-capability-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/gc-tls-atomic-capability-ssot.md)

allocator/runtime policy owner を深くする前に必要。

### C5. Hakozuna portability layer

- size-class policy
- ptr/bin policy
- remote-free routing policy
- TLS cache policy
- telemetry/profile policy

ここでは policy を `.hako` に持ち、metal primitive は native keep する。
current reading:
- ladder-only and deferred for now

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
- current docs lock:
  - [`final-metal-split-ssot.md`](/home/tomoaki/git/hakorune-selfhost/docs/development/current/main/design/final-metal-split-ssot.md)

## Capability Foundation Pack

This pack is the substrate foundation order. It is not the active
`phase-293x` real-app task list; current active work remains owned by
`CURRENT_STATE.toml` and the phase docs.

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

## Allocator Follow-On Pack

This pack is not the active `phase-293x` real-app task by itself. It is the
docs-side order to use when the allocator substrate lane opens.

1. numeric substrate lock
   - fixed-width unsigned/pointer-sized vocabulary
   - current `>>` signed i64 arithmetic shift lock
   - wrapping/checked arithmetic semantics
   - MIR/VM/LLVM/Stage0 acceptance named before syntax expansion
2. raw layout vocabulary
   - fixed layout / alignment / `sizeof` / `offsetof`
   - `box` vs raw metadata boundary documented before implementation
   - current live row is MIR `repr_c_v0` fixed-width numeric field planning
   - source syntax and backend-active `sizeof` / `offsetof` remain future rows
3. minimal `RawBuf` + `RawArray` allocator fixture
   - `RawBuf` allocation facade is now live above `MemCoreBox`
   - no TLS
   - no atomics
   - no OS VM ownership
   - verifier-gated slot load/store and reserve/grow only for `RawArray`
4. `hako.mem` / `hako.buf` / `hako.ptr` widening
   - restricted unsafe only
   - bounds/initialized/ownership fail-fast
5. `no_alloc` / `no_safepoint` verifier row
   - contract must be checked before it is used as optimization metadata
6. TLS/atomic first rows
   - memory order vocabulary is required before remote-free style algorithms
7. native allocator fast-path proof
   - may use native keep for final metal
   - must not move OS VM or final allocator call ownership into `.hako`
     without the C5/C6 split

## Non-Goals

- native allocator/OS VM を即 `.hako` 実装すること
- current perf micro leaf を capability widening と混ぜること
- collection owner cutover を無効化して Rust method-shaped owner に戻すこと
- unrestricted unsafe surface を導入すること
- broad C shim / by-name app matcher で allocator-shaped user boxes を通すこと
