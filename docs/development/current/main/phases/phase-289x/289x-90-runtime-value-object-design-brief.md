---
Status: Active Planning
Date: 2026-04-19
Scope: phase-289x の runtime-wide `value world / object world` 設計を、実装前に 1 枚で読める形へ圧縮する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
---

# Phase 289x Runtime Value/Object Design Brief

## Purpose

phase-289x は、string lane で見えてきた設計を runtime 全体へ広げるための
successor planning だよ。

ただし、この文書は実装許可ではない。
phase-137x の active string read-side lane が keeper/reject に達するまで、
runtime-wide storage rewrite / `TextLane` / MIR legality lift / allocator work は開かない。

## One Sentence

内部では値を値のまま運び、public / host / identity 境界でだけ object / handle にする。

```text
Internal:
  value world

Boundary:
  publish / promote effect

External:
  object world / handle world
```

## Layer Contract

| Layer | Owns | Must not own |
| --- | --- | --- |
| `.hako` language | semantic values, identity containers, escape meaning | handle class, registry, storage lane |
| canonical MIR / lowering | demand facts, publication boundary, sink capability | helper-name legality, runtime re-recognition |
| Rust runtime | executor, residence, objectization mechanics, caches | semantic legality, public ABI widening |
| LLVM/native | scalarization and specialization after contract proof | new language truth |

Rule:

- language decides meaning
- MIR/lowering decides boundary demand
- runtime executes the demanded boundary
- helper names never become the proof that publication was legal

## Semantic Families

| Family | Language reading | Internal target | Object boundary | First allowed action |
| --- | --- | --- | --- | --- |
| `String` | immutable value | `VerifiedTextSource`, `TextPlan`, `OwnedBytes`, `KernelTextSlot`, read alias lane | `publish` + `freeze.str` | phase-137x only |
| `Bytes` | byte value | future `BytesRef`, `OwnedBytes`, `BytesCell` | explicit bytes publish / host demand | docs vocabulary only |
| `Int` | scalar value | immediate | scalar box only on object demand | audit boxed transitions |
| `Bool` | scalar value | immediate | scalar box only on object demand | audit boxed transitions |
| `Array` | identity container | lane host for element residence | array handle stays identity | design after string judge |
| `Map` | identity container | lane host for key/value residence | map handle stays identity | key/value boundary map |
| `View/Slice` | borrowed read view | `Ref` / read session | stable object only on escape | docs after text proof |
| small aggregate | value | `agg_local` / immediate fields | box only on escape | out of phase unless evidence demands |

Important distinction:

- `String`, `Bytes`, `Int`, `Bool`, and small aggregates are semantic values.
- `Array` and `Map` are identity containers.
- Array/Map lane-hosting is only internal residence specialization, not a rewrite of container semantics.

## Lifecycle Vocabulary

Use the same lifecycle shape across families only where it has real meaning.

| State | Meaning | Current string analogue | Non-string reading |
| --- | --- | --- | --- |
| `Ref` | borrowed/read-only view | `VerifiedTextSource`, read session | bytes/view borrow, array/map read view |
| `Plan` | deferred construction | `TextPlan`, deferred `const_suffix` | bytes builder only if proven |
| `Owned` | unpublished owned payload | `OwnedBytes` | owned bytes / local aggregate payload |
| `Cell` | container residence | `KernelTextSlot`, future `TextCell` | array element cell, map key/value cell |
| `Immediate` | unboxed scalar payload | `i64` encoded scalar paths | int/bool/future small scalar |
| `Stable` | object-capable public representation | `StringBox`/handle after `freeze.str` | scalar box, published bytes, generic object |

Rule:

- Do not force every family to implement every state.
- Do not rename current mechanics into a new state unless the family has an owner, tests, and a boundary demand.

## Demand Vocabulary

phase-289x should split current `CodecProfile`-style names into explicit demand names before code APIs are invented.

| Demand | Question it answers | Examples today |
| --- | --- | --- |
| `ValueDemand::ReadRef` | can the caller read without owning/publishing? | string read session, array string len/indexof by index |
| `ValueDemand::EncodeImmediate` | can the value be returned as immediate? | int/bool `array.get` / runtime-data reads |
| `ValueDemand::EncodeAlias` | can a borrowed alias handle be reused or cached? | array/map borrowed string read outcomes |
| `ValueDemand::OwnedPayload` | does the sink need unpublished owned bytes/value? | `KernelTextSlot` materialization |
| `PublishDemand::StableObject` | does the caller require object identity/public handle? | generic object consumer, host ABI |
| `StorageDemand::CellResidence` | can a container store the value without publication? | `kernel_slot_store_hi`, future lane cell |
| `StorageDemand::DegradeGeneric` | must typed residence fall back to generic object storage? | heterogeneous array/map paths |
| `MutationDemand::Invalidate` | must existing borrowed/cached aliases expire? | array/map mutation, drop-epoch change |

Current names stay valid until replaced, but their meaning must be documented as demand, not as helper ownership.

## Current Code Inventory Targets

These are inventory anchors for phase-289x docs. They are not implementation targets by themselves.

| Area | Current anchors | What to document |
| --- | --- | --- |
| codec profiles | `crates/nyash_kernel/src/plugin/value_codec/decode.rs` | map `Generic`, `ArrayFastBorrowString`, `ArrayBorrowStringOnly`, `MapKeyBorrowString`, `MapValueBorrowString` to demand |
| borrowed alias encode | `crates/nyash_kernel/src/plugin/value_codec/borrowed_handle.rs` | live-source, cached-handle, cold-fallback read outcomes |
| array encoded read | `crates/nyash_kernel/src/plugin/array_handle_cache.rs` | scalar immediate first, then borrowed alias encode |
| array residence write | `crates/nyash_kernel/src/plugin/array_slot_store.rs` | i64/bool/f64 raw slots, boxed fallback, string handle/slot store |
| array text observers | `crates/nyash_kernel/src/plugin/array_string_slot.rs` | direct by-index string len/indexof as read-only lane precedent |
| map key/value codec | `crates/nyash_kernel/src/plugin/map_key_codec.rs`, `map_slot_store.rs`, `map_slot_load.rs` | key decode, value residence, read publication split |
| runtime-data facade | `crates/nyash_kernel/src/plugin/runtime_data.rs`, `map_runtime_data.rs` | facade-only mixed data route; do not make it semantic truth |
| compat residue | `crates/nyash_kernel/src/plugin/map_compat.rs`, `array_compat.rs` | legacy export compatibility, not active lane authority |

## Rollout Shape

1. Finish docs vocabulary.
2. Inventory existing demand/profile/storage routes.
3. Define container lane-host contract without code.
4. Wait for phase-137x read-side keeper/reject.
5. If unlocked, run exactly one runtime-private storage pilot.
6. Only after storage contracts are proven, lift legality into MIR/verifier facts.
7. Only after objectization frequency falls, consider allocator/arena work.

## No-Go List

- Do not start runtime-wide implementation from this phase alone.
- Do not make `TextLane` a semantic truth.
- Do not turn Array/Map into immutable values.
- Do not widen public ABI for internal residence.
- Do not create a second string birth sink beside `freeze.str`.
- Do not let runtime helpers infer publication legality.
- Do not introduce allocator/arena work before perf evidence says allocation is the remaining owner.
- Do not retry rejected store-side owned string/text keeps as a runtime-wide design proof.
