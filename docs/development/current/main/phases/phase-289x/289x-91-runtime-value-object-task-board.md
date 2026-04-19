---
Status: Active Planning
Date: 2026-04-19
Scope: runtime-wide `value world / object world` rollout を、実装前に phase/card 粒度へ分割する taskboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/lifecycle-typed-value-language-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/phases/phase-137x/phase137x-text-lane-rollout-checklist.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md
  - docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md
  - docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md
  - docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md
---

# Phase 289x Runtime Value/Object Task Board

## North Star

```text
Internal execution:
  value world

Boundary:
  publish / promote effect

Public/host surface:
  object world / handle world
```

Rule:

- internal hot paths should carry values, aliases, cells, or immediates
- object / handle is a boundary representation
- array / map keep identity semantics, but only their internal element/key/value residence may become lane-hosted
- runtime-private lane work must not change public ABI
- `publish` / `promote` stay boundary effects; `freeze.str` remains the only string birth sink

## Authority Stack

1. Language / `.hako`
   - semantic value meaning
   - identity-sensitive container meaning
   - escape/public boundary meaning
2. Canonical MIR / lowering contract
   - demand facts
   - publication boundary
   - sink capability
3. Runtime microkernel
   - handle table
   - objectization
   - lane storage
   - cache / epoch / fallback mechanics
4. LLVM/native
   - scalarize / inline / specialize after the above contract is stable

## Type Matrix

| Family | Semantic reading | Internal target | Stable/object boundary | First action |
| --- | --- | --- | --- | --- |
| `String` | immutable value | `VerifiedTextSource / TextPlan / OwnedBytes / KernelTextSlot / alias lane` | `publish` + `freeze.str` | continue phase-137x |
| `Bytes` | value | future `BytesRef / OwnedBytes / BytesCell` | `publish.bytes` or host boundary | docs only |
| `Int` | scalar value | immediate | box only on object demand | audit first |
| `Bool` | scalar value | immediate | box only on object demand | audit first |
| `Array` | identity container | lane host for elements | array handle remains public identity | text lane design after string read-side keeper/reject |
| `Map` | identity container | key/value boundary lanes | map handle remains public identity | key/value boundary map |
| `View/Slice` | borrowed read view | `Ref` / read session | stable object only on escape | docs after text proof |
| tuple/optional small aggregate | value | `agg_local` | box only on escape | out of this phase unless needed |

## Boundary Vocabulary Lock

Use one term for one responsibility.

| Term | Owner | Meaning | Not allowed to mean |
| --- | --- | --- | --- |
| `borrow/project` | runtime under MIR/lowering demand | enter value-world read/session from an existing object/handle | publish or allocate stable identity |
| `materialize` | runtime executor | produce concrete unpublished bytes/value payload for a sink | public object birth by itself |
| `publish` | MIR/lowering boundary effect | value crosses to public/object world | string birth sink or helper-local guess |
| `promote` | MIR/lowering boundary effect executed by runtime | turn lane/cell/immediate into object-capable representation under named demand | semantic legality decision or string birth sink |
| `freeze.str` | string birth sink | retained string birth / reuse mechanics | publication policy owner |
| `handle issue` | object/host substrate | allocate/register public handle | proof that publication was legal |

Stop-line:

- if a code path needs a stable object, the reason must be named as demand/boundary
- do not allow runtime helper names to become the legality source
- do not use `publish` and `freeze.str` interchangeably

## Vocabulary Gaps Closed By This Phase

Before any runtime-wide implementation can start, these definitions must be explicit:

| Gap | Planning term | Resolution |
| --- | --- | --- |
| internal value world vs object world | `value world` / `object world` | lifecycle SSOT owns the world split |
| common carrier lifecycle | `Ref / Owned / Cell / Immediate / Stable` | phase-289x design brief gives phase-local mapping |
| helper-name drift | `get / set / call` as demand verbs | MIR/lowering demand facts, not helper names, select behavior |
| boundary effect drift | `publish / promote` | boundary effects only; not legality owners or string birth sinks |
| container semantic drift | lane host | Array/Map identity remains public semantic truth |

Task state:

- `289x-0a`: done in docs
- `289x-0b`: done in docs
- `289x-0c`: done in restart/current pointers
- `289x-0d`: done in docs, runtime vocabulary lock
- `289x-1f`: active post-keeper inventory sync after phase-137x keeper `49c356339`
- `289x-1g`: done in `289x-93-demand-vocabulary-ledger.md`
- `289x-2d`: done in `289x-94-container-demand-table.md`
- `289x-3a`: active pilot proposal in `289x-95-array-text-residence-pilot.md`
- `289x-3b`: active cutover inventory gate in `289x-96-demand-backed-cutover-inventory.md`
- `289x-3c`: done in code, `CodecProfile -> DemandSet`, behavior unchanged
- `289x-3d`: done in code, `BorrowedAliasEncodeCaller -> DemandSet`, behavior unchanged
- `289x-3e`: done in code, `PublishReason -> PublishDemand`, behavior unchanged
- `289x-3f`: done in code, array generic load/encode demand tags, behavior unchanged
- `289x-3g`: done in code, array store/append demand tags, behavior unchanged
- `289x-3h`: done in code, `KernelTextSlotState` demand bridge, high-risk, behavior unchanged
- `289x-7a`: done in code, C shim generic method set-route demand metadata, emitted lowering unchanged
- `289x-7b`: done in code, MIR demand/placement parallel facts, inspection-only, behavior unchanged
- `289x-6d`: done in code, Map key/value codec demand bridge, no typed map lane
- `289x-6e`: done in code, Map load encoding split, no public ABI change
- Rust runtime clusters in `289x-96`: closed
- `289x-7c`: next C shim cut, `get/len/has/push` policy split over demand metadata
- `289x-7e` / `289x-7f`: high-risk C shim emission/window work is planned later, not skipped
- optimization return: blocked until all `289x-96` clusters are done or explicitly rejected

## Phase 0. Authority / Vocabulary Lock

- Goal:
  - make runtime-wide value/object boundary a parent architecture reading, not a string-only optimization excuse
- Scope:
  - docs only
  - parent SSOT alignment
  - phase-137x remains active proving ground
- Non-goals:
  - no runtime storage rewrite
  - no public ABI changes
- Tasks:
  - `289x-0a`: link this phase from `lifecycle-typed-value-language-ssot.md`
  - `289x-0b`: lock container rule:
    - array/map are identity containers
    - only their element/key/value residence may be lane-hosted later
    - `publish` / `promote` stay boundary effects
    - `freeze.str` stays the only string birth sink
  - `289x-0c`: add restart/current pointers as parked successor only
- Acceptance:
  - docs can answer:
    - what is a value?
    - what is an identity container?
    - where does objectization happen?
    - why string is first proving ground?

## Phase 1. Demand Vocabulary Inventory

- Goal:
  - separate decode demand, storage demand, and publication demand before inventing new code APIs
- Scope:
  - `CodecProfile`
  - `BorrowedAliasEncodeCaller`
  - existing manifest classes
- Non-goals:
  - do not rename code broadly
  - do not introduce a new public ABI class
- Tasks:
  - `289x-1a`: inventory existing profiles:
    - `Generic`
    - `ArrayFastBorrowString`
    - `ArrayBorrowStringOnly`
    - `MapKeyBorrowString`
    - `MapValueBorrowString`
  - `289x-1b`: propose internal vocabulary:
    - `ValueDemand`
    - `StorageDemand`
    - `PublishDemand`
  - `289x-1c`: map existing boundary terms:
    - `borrow/project`
    - `materialize`
    - `publish`
    - `promote`
    - `freeze.str`
    - `handle issue`
  - `289x-1d`: identify which existing tests lock each demand
  - `289x-1e`: current code inventory anchors
    - record exact files/functions that hold current demand/profile/storage/publication vocabulary
    - keep this as inventory; do not rename or widen APIs in this card
  - `289x-1f`: post-keeper sync
    - record `49c356339` as the current string proof
    - mark pre-keeper whole-owner numbers and “next alias split” wording as historical where they are still useful
  - `289x-1g`: exact demand ledger
    - map current profile/helper/caller names to `ValueDemand`, `StorageDemand`, `PublishDemand`, and `MutationDemand`
    - identify which rows are compat residue and which rows are active boundary seams
    - status: done in `289x-93-demand-vocabulary-ledger.md`
- Acceptance:
  - no caller needs to infer stable/object demand from helper names
  - each profile has a documented owner and removal/evolution path

### Phase 1 Inventory Anchors

These anchors make the inventory concrete without authorizing code changes.

| Slice | Current vocabulary | Code anchor | phase-289x reading |
| --- | --- | --- | --- |
| profile policy | `CodecProfile::{Generic, ArrayFastBorrowString, ArrayBorrowStringOnly, MapKeyBorrowString, MapValueBorrowString}` | `crates/nyash_kernel/src/plugin/value_codec/decode.rs` | decode/storage/publication demands are still mixed |
| scalar immediate | `ArrayFastDecodedValue::{ImmediateI64, ImmediateBool, ImmediateF64, Boxed}` | `value_codec/decode.rs`, `value_codec/encode.rs` | current immediate-lane vocabulary |
| borrowed alias encode | `BorrowedAliasEncodeCaller`, `BorrowedAliasEncodePlan` | `value_codec/borrowed_handle.rs` | read outcomes are live-source, cached-handle, cold-fallback |
| string publication | `PublishReason`, `StringPublishSite`, `KernelTextSlotState` | `value_codec/string_materialize.rs` | current publish/storage vocabulary for the first proving ground |
| read-source classification | `StringHandleSourceKind`, `StringLikeProof`, `ArrayStoreStrSource` | `value_codec/string_classify.rs`, `array_string_slot.rs` | source-shape vocabulary for borrowed vs fallback reads |
| array facade/residence | `array_runtime_*`, `array_slot_store_*`, `array_string_*_by_index` | `array_runtime_any.rs`, `array_runtime_facade.rs`, `array_slot_store.rs`, `array_string_slot.rs` | facade, indexed residence, and string corridor are separate layers |
| map key/value boundary | `map_key_string_*`, `map_slot_load_*`, `map_slot_store_*`, `map_probe_*` | `map_key_codec.rs`, `map_slot_load.rs`, `map_slot_store.rs`, `map_probe.rs` | key decode, value storage, and read publication stay split |
| runtime-data bridge | `nyash.runtime_data.{get_hh,set_hhh,has_hh,push_hh}` | `runtime_data.rs`, `map_runtime_data.rs` | facade-only bridge, not semantic owner |
| compat residue | legacy `nyash.array.*` / `nyash.map.*` symbols | `array_compat.rs`, `map_compat.rs` | shrink-only compatibility surface |

No-go:

- do not turn phase-1 inventory into implementation
- do not add a public ABI class or row field to clean up `CodecProfile`
- do not erase caller-scoped borrowed-alias outcomes
- do not treat facade or compat exports as the new semantic owner

## Phase 2. Container Lane-Host Contract

- Goal:
  - define array/map as lane hosts without changing their public identity semantics
- Scope:
  - docs and tests first
  - array text lane as the first possible storage pilot
- Non-goals:
  - no `ArrayStorage::*` implementation before phase-137x keeper/reject
  - no map typed-lane implementation in this phase
  - no MIR legality / verifier lift in this phase
  - no allocator / arena work in this phase
- Tasks:
  - `289x-2a`: array lane-host contract
    - homogeneous residence
    - explicit degrade
    - stable object demand
  - `289x-2b`: map lane-host contract
    - key decode boundary
    - value residence boundary
    - compat export boundary
  - `289x-2c`: read/write demand table:
    - read-only
    - encoded alias
    - stable object
    - mutation / invalidation
  - `289x-2d`: Array/Map demand table:
    - Array text read-ref / encoded alias / stable-object rows
    - Array set cell-residence / generic-degrade / invalidation rows
    - Map key decode / value residence / read publication rows
    - RuntimeData facade rows marked as bridge, not semantic owner
    - status: done in `289x-94-container-demand-table.md`
- Acceptance:
  - future storage work can be judged as BoxShape, not by local helper names
  - array/map public semantics stay unchanged
  - container lane-host planning still reads as a generalization boundary, not implementation authorization

## Phase 3. First Storage Pilot After String Read-Side Keeper/Reject

- Gate:
  - phase-137x has a keeper/reject decision on the active read-side lane
  - post-keeper inventory `289x-1f` is complete
  - demand ledger `289x-1g` is complete
  - container demand table `289x-2d` is complete
- Goal:
  - start with one runtime-private storage pilot only
- Preferred pilot:
  - `Array` as lane host for text residence
  - selected:
    - `Array text residence through KernelTextSlot store`
  - first code cut:
    - runtime-private demand vocabulary module only; behavior unchanged
- Non-goals:
  - no generic typed array family yet
  - no bytes/scalar/map storage rewrite in the same series
- Acceptance:
  - exact stays closed
  - meso does not contradict
  - whole owner visibly moves or the card is reverted
  - runtime executes a named demand; it does not infer publication legality from helper names

### Phase 3 Cutover Inventory

- `289x-3b`: remaining cluster inventory
  - SSOT: `289x-96-demand-backed-cutover-inventory.md`
  - scope:
    - Rust runtime clusters: 8
    - C shim / MIR clusters: 8
  - return-to-optimization gate:
    - every cluster must be `done` or `rejected`
    - high-risk deferrals must have scheduled cards
- `289x-3c`: Rust `CodecProfile -> DemandSet`
  - status: done in code
  - behavior unchanged
  - `CodecProfile::demand()` is the runtime-private bridge from profile names to `DemandSet`
  - old branches remain the executor until downstream demand rows are wired
- `289x-3d`: Rust `BorrowedAliasEncodeCaller -> DemandSet`
  - status: done in code
  - behavior unchanged
  - `BorrowedAliasEncodeCaller::demand()` is the runtime-private bridge from caller names to `DemandSet`
  - fallback encode plans bind publish demand before the existing objectization branch
- `289x-3e`: Rust `PublishReason -> PublishDemand`
  - status: done in code
  - behavior unchanged
  - `PublishReason::demand()` is the runtime-private bridge from publish reason names to `PublishDemand`
  - old observation/objectization branches remain the executor
- `289x-3f`: Rust array generic load/encode demand tags
  - status: done in code
  - behavior unchanged
  - array encoded get/load sites bind `ARRAY_GENERIC_GET_ENCODED`
  - old scalar/alias/stable fallback branches remain the executor
- `289x-3g`: Rust array store/append demand tags
  - status: done in code
  - behavior unchanged
  - array `store_any` binds `ARRAY_GENERIC_STORE_ANY`
  - array `append_any` binds `ARRAY_GENERIC_APPEND_ANY`
- `289x-3h`: `KernelTextSlotState` demand bridge
  - status: done in code
  - behavior unchanged
  - no ABI change
  - state demand and boundary publish demand stay separated

## Phase 4. Scalar Immediate Widening

- Gate:
  - scalar boxed-object hot path is proven by perf/asm or contract audit
- Goal:
  - keep int/bool in immediate world longer
- Non-goals:
  - no broad numeric optimizer work here
  - no SIMD/vector work here
- Tasks:
  - `289x-4a`: audit boxed int/bool transitions
  - `289x-4b`: identify public/object boundaries that force boxing
  - `289x-4c`: choose one leaf cut with tests
- Acceptance:
  - reduction in objectization events, not just faster wrappers

## Phase 5. Bytes / View First-Class Planning

- Goal:
  - prevent text-only corridor patterns from being copied as ad-hoc bytes/view helpers later
- Scope:
  - docs-first vocabulary
  - no implementation unless a bytes/view benchmark or correctness card demands it
- Tasks:
  - define `Ref / Owned / Cell / Stable` applicability per family
  - reject state names that do not map to a real family need
- Acceptance:
  - `TextRef` lessons can be reused without making text semantics the universal truth

## Phase 6. Map Key/Value Boundary Planning

- Goal:
  - keep map key coercion, value residence, and read publication as separate seams
- Starting facts:
  - `MapKeyBorrowString` is now a map-key named profile
  - map value storage has `MapValueBorrowString`
  - map read outcomes are observed as live/cached/fallback
- Tasks:
  - `289x-6a`: map key/value boundary diagram
  - `289x-6b`: compat export retirement criteria
  - `289x-6c`: typed map lane only if evidence makes map the owner
  - `289x-6d`: Map key/value codec demand bridge
    - done; no typed map lane
  - `289x-6e`: Map load encoding split
    - done; no public ABI change
- Acceptance:
  - map does not regain generic object publication as an implicit read/write side effect

## Phase 7. MIR Legality / Verifier Lift

- Gate:
  - runtime-private contracts are proven by earlier phases
- Goal:
  - move boundary legality into MIR/lowering facts instead of runtime helper inference
- Non-goals:
  - no broad public MIR dialect expansion before the runtime contract is stable
- Tasks:
  - define demand facts as recipe metadata first
  - verifier-visible publication boundary
  - reject helper-name allowlists
  - `289x-7a`: C shim set-route demand metadata
    - done; metadata-only; emitted lowering identical
  - `289x-7b`: MIR parallel demand/placement facts
    - done; inspection-only; behavior unchanged
  - `289x-7c`: C shim `get/len/has/push` policy split over demand metadata
    - next
  - `289x-7d`: main `bname/mname` route classifier cutover
    - high-risk
  - `289x-7e`: concrete `slot_load_hi` / `slot_store` helper emission cutover
    - high-risk
  - `289x-7f`: `runtime_array_string` observer/window matcher cutover
    - high-risk
  - `289x-7g`: MIR string helper-name compat/recovery cutover
  - `289x-7h`: prepass/declaration need classifier cutover
    - high-risk
- Acceptance:
  - runtime can execute boundary decisions without re-deciding legality
  - no helper/class name remains the only source of publication legality
  - full `ArrayStorage::Text` / full `TextLane` stays out until `289x-7h` closes

## Phase 8. Allocator / Arena

- Gate:
  - perf evidence points at allocation after objectization frequency is already reduced
- Goal:
  - lane-local allocation only where it is proven to be the next owner
- Non-goals:
  - no generic allocator swap as first response
- Acceptance:
  - win is tied to a specific lane and benchmark front

## Commit Discipline

- docs/vocabulary cards commit separately from behavior cards
- one storage pilot per series
- no BoxCount + BoxShape mixing
- if a card only moves cost from write to read, revert or park it with reject evidence
