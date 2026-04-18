---
Status: SSOT
Decision: current
Date: 2026-04-07
Scope: string/collection hot path で helper 名ではなく Birth / Placement outcome を正本にし、`.hako owner -> MIR canonical contract -> Rust birth backend` の責務を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/phases/phase-137x/README.md
  - docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md
  - docs/development/current/main/design/string-birth-sink-ssot.md
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/value_codec/string_store.rs
  - src/runtime/host_handles.rs
---

# Birth / Placement SSOT

## Goal

- helper 名ではなく Birth / Placement outcome を正本にする
- `string_handle_from_owned(...)` や `freeze_text_plan(...)` を semantic source of truth にしない
- `.hako owner / policy -> MIR canonical contract -> Rust birth backend` の読みを固定する
- string hot path の最適化を局所 hack ではなく placement seam 単位で扱う
- Rust/C++/C/LLVM の borrowed/materialize/storage discipline を層ごとに分けて使う

Parent/child note:

- this doc owns the generic lifecycle vocabulary only
- string-specific sink semantics stay in `string-birth-sink-ssot.md`
- string semantic/boundary wording stays in
  `string-semantic-value-and-publication-boundary-ssot.md`

## Why Now

Current exact front は `const_suffix` route ではなく generic string consumer である。

- `kilo_micro_concat_hh_len`
  - current AOT consumer: `nyash.string.concat_hh` + `nyash.string.len_h`
  - current executor detail:
    - `string_concat_hh_export_impl(...)`
    - `string_len_from_handle(...)`
- current microasm read:
  - `string_concat_hh_export_impl`: `54.04%`
  - `string_len_from_handle`: `21.37%`
  - `__memmove_avx512_unaligned_erms`: `15.40%`

The next generic seam is not a new route name.
It is the Birth / Placement outcome that decides whether a value:

- reuses an existing handle
- stays borrowed
- freezes into owned form
- creates a fresh handle
- materializes from borrowed data
- stores from source

## Vocabulary

Use these six outcome names as the SSOT vocabulary:

1. `ReturnHandle`
2. `BorrowView`
3. `FreezeOwned`
4. `FreshHandle`
5. `MaterializeOwned`
6. `StoreFromSource`

Interpretation:

- `ReturnHandle`
  - rewrite / elision outcome
  - not a standalone executor op
- `BorrowView`
  - borrowed/view retained form
  - lifetime-sensitive, non-owning
- `FreezeOwned`
  - sink outcome
  - borrowed / planned text becomes owned string output
- `FreshHandle`
  - new host handle issue
  - runtime backend event, not owner policy
- `MaterializeOwned`
  - owned string realization in native runtime
  - backend leaf below policy/contract
- `StoreFromSource`
  - collection sink that preserves source-based string storage when allowed

Do not add `box_id` to this top-level vocabulary.
`box_id` belongs below this layer as a Rust-side objectization contract.

## Carrier Split

Birth should not be treated as one fused runtime event.

Current fused chain is:

- byte birth = `MaterializeOwned`
- object birth = `Objectization::StableBoxNow`
- publication birth = `RegistryIssue::FreshRegistryHandle`

Target reading:

- `MaterializeOwned` creates owned bytes
- `StableBoxNow` creates a stable Nyash object only when object world demands it
- `FreshRegistryHandle` publishes only when handle world demands it

This means:

- `MaterializeOwned` does not imply `StableBoxNow`
- `StableBoxNow` does not imply `FreshRegistryHandle`
- `FreshRegistryHandle` must not be used as a synonym for objectization

## Private Backend Carriers

Rust backend may keep private carriers below the public seam.

Current next carriers are:

1. `OwnedBytes`
2. `TextReadSession`
3. `SourceLifetimeKeep`

Interpretation:

- `OwnedBytes`
  - owned text payload that is not yet a stable `NyashBox`
- `TextReadSession`
  - end-to-end read-only borrowed string session
  - keeps drop-epoch / handle lookup / span reuse together for pure string reads
- `SourceLifetimeKeep`
  - backend-private carrier for source-preserving alias survival
  - not a public Birth / Placement outcome
  - must not be treated as a synonym for `StableBoxNow`
  - target stop-line:
    - `TextKeep`
    - `AliasSourceMeta`
    - cold copy-out to owned text
  - do not treat it as a "small box"
  - do not mix `OwnedBytes` into the keep carrier

These names are Rust backend family names only.
Do not promote them into `.hako` route vocabulary or MIR top-level outcome
names.

## Read-Contract Rule

Do not mix stable-object read and live-source read.

These are different backend-private contracts:

1. stable-object read
   - returns `Option<&str>`
   - examples:
     - `NyashBox::as_str_fast()`
     - `BorrowedHandleBox::as_str_fast()`
   - may read only from stable object state already held by the box

2. live-source read
   - uses closure/session APIs only
   - examples:
     - `host_handles::with_str_handle(...)`
     - `host_handles::with_text_read_session(...)`
   - may borrow text behind registry/session guards
   - must not escape as a naked `Option<&str>`

Reading rule:

- do not add live-source direct read into `as_str_fast()`
- do not use stable-object read names for registry/session-backed borrowing
- if a caller must cross the session boundary, use a guard/session carrier
  instead of a naked borrowed string

## Backend Second Axis

Birth / Placement outcome is the first reading.
Rust birth backend sub-contract is the second reading.

Use this backend-only axis when the question is about object identity or
registry payload shape:

### `Objectization`

1. `None`
2. `StableBoxNow`
3. `DeferredStableBox`

Interpretation:

- `None`
  - no stable Nyash object is created at this point
  - no `box_id` contract is involved
- `StableBoxNow`
  - a real `NyashBox` object is created now
  - strict unique `box_id` belongs here
- `DeferredStableBox`
  - owned/transient payload exists now
  - stable `NyashBox` objectization is intentionally delayed

### `RegistryIssue`

1. `None`
2. `ReuseSourceHandle`
3. `FreshRegistryHandle`

Interpretation:

- `None`
  - no host handle is issued
- `ReuseSourceHandle`
  - an existing host handle survives the seam
- `FreshRegistryHandle`
  - a new host handle is issued by the Rust runtime backend

This second axis is backend-only.
It must not replace the top-level Birth / Placement vocabulary.

## Layer Responsibilities

## Lifecycle Flow

Target lifecycle should be read as the following flow, not as one fused
"birth" event.

```text
.hako owner / policy
  decides:
    - source_preserve
    - identity_demand
    - publication_demand
        |
        v
MIR canonical contract
  carries:
    - public route name
    - delayed-materialization reading
    - escalation visibility
        |
        v
Rust planner / executor
  planner:
    - SourceKindCheck
    - choose RetargetAlias / StoreFromSource / NeedStableObject
  executor:
    - SourceLifetimeKeep
    - AliasUpdate
    - drop_epoch / registry / Arc mechanics
        |
        v
object / handle world
  only when demanded:
    - StableBoxNow
    - FreshRegistryHandle
```

This should also be read as five lifecycle stages:

1. value/text world
   - `BorrowView`
   - `MaterializeOwned`
   - `StoreFromSource`
2. source-lifetime keep
   - `SourceLifetimeKeep`
   - still not a stable object by itself
3. object birth
   - `Objectization::StableBoxNow`
4. publication
   - `RegistryIssue::ReuseSourceHandle | FreshRegistryHandle`
5. stale/invalidation
   - `drop_epoch`
   - alias survival
   - registry invalidation

For hot paths, `MaterializeOwned` should not imply `StableBoxNow`, and
`StableBoxNow` should not imply `FreshRegistryHandle`.

### `.hako owner / policy`

Owns:

- birth trigger
- retained-form choice
- boundary choice
- visible collection/string route semantics

Examples:

- `concat_suffix_retention_class(...)`
- substring retained-form rules
- route vocabulary such as `const_suffix`

`.hako` decides **whether** a route should reuse, borrow, freeze, or materialize.
It borrows Rust-like ownership vocabulary as semantic meaning only.

`.hako` also owns lifecycle policy questions such as:

- whether source-preserve is semantically allowed
- whether stable identity is semantically required
- whether publication is semantically required

### MIR canonical contract

Owns:

- canonical naming of the chosen outcome
- rewrite / elision reading
- stable transport from owner choice to backend execution

Examples:

- `str.concat2`
- `str.concat3`
- `str.len`
- `freeze.str`
- `store.array.str`
- `store.map.value`

MIR carries **what outcome was chosen**, not the runtime mechanics of issuing a handle.
It is also the right place to keep delayed-materialization reading stable.

MIR is also the right layer to freeze visible lifecycle policy for hot paths:

- source-preserve eligibility
- identity demand
- publication demand
- escalation conditions that allow object-world entry

### Rust birth backend family

Owns:

- freeze/materialize execution
- fresh handle issue
- registry alloc/write
- borrowed lifetime substrate
- store-from-source backend

Current backend leaves include:

- `string_handle_from_owned(...)`
- `freeze_text_plan(...)`
- `materialize_owned_string(...)`
- `string_handle_from_span(...)`
- `store_string_box_from_source(...)`

Current next backend-private carriers are:

- `OwnedBytes`
- `TextReadSession`

These are backend leaves only.
They must not become public policy vocabulary.
Rust keeps C-like storage/lifetime mechanics here.

Rust must not decide lifecycle policy.
Rust may only decide runtime facts under the already-frozen contract, for example:

- source kind at runtime
- whether a slot is a borrowed alias
- how source-lifetime keep is executed
- how alias metadata is updated
- how drop-epoch / registry / `Arc` mechanics are carried out

`box_id` also belongs here.
Treat it as part of `Objectization::StableBoxNow`, not as a top-level outcome
visible to `.hako` owner or MIR naming.

## Source-Contract Rule

For `store.array.str`, keep the public canonical name unchanged and split the
backend-private contract below it.

Current intended split:

1. `SourceKindCheck`
2. `SourceLifetimeKeep`
3. `AliasUpdate`
4. `NeedStableObject`

Reading rule:

- `SourceKindCheck` is a runtime fact read
- `SourceLifetimeKeep` is lifetime mechanics
- `AliasUpdate` is metadata update
- `NeedStableObject` is the only branch that may justify generic object-world entry

`SourceLifetimeKeep` must follow the read-contract rule:

- it may carry text/lifetime survival semantics
- it must not become a disguised live-source direct-read API

This keeps lifecycle policy above Rust while keeping runtime mechanics below it.

## Current Source Mapping

| Outcome vocabulary | Current Rust detail | Scope |
| --- | --- | --- |
| `ReturnHandle` | reuse branch in string concat/substr helpers | rewrite outcome |
| `BorrowView` | `StringSpan` / `StringViewBox` | borrowed/lifetime substrate |
| `FreezeOwned` | `freeze_text_plan(...)` | string sink backend |
| `FreshHandle` | `string_handle_from_owned(...)` | fresh handle backend |
| `MaterializeOwned` | `materialize_owned_string(...)` | registry/alloc backend |
| `StoreFromSource` | `store_string_box_from_source(...)` | collection sink backend |

## Current Coupling

Current implementation still couples:

- `FreshHandle`
- `MaterializeOwned`
- `Objectization::StableBoxNow`
- `RegistryIssue::FreshRegistryHandle`

Reason:

- host registry payload is currently `u64 -> Arc<dyn NyashBox>`
- `materialize_owned_string(...)` therefore creates:
  - `StringBox`
  - `Arc<dyn NyashBox>`
  - fresh host handle
in one backend chain

This means current `FreshHandle` often implies:

- stable objectization now
- strict unique `box_id` issue now

That coupling is current implementation detail, not the desired top-level
semantic vocabulary.

## Current Direction Lock

The next design slice is not “make `StringBox` cheaper first”.
The next design slice is:

1. keep owned text in `OwnedBytes`
2. keep pure string reads in `TextReadSession`
3. enter `StableBoxNow` only at object/sink boundaries

This is the intended way to break the current fused pipeline without lifting
`box_id` into the public Birth / Placement vocabulary.

## First Vertical Slice

The first slice is **generic `concat_hh + len_h`**, not `const_suffix`.

Reason:

- current exact front is `kilo_micro_concat_hh_len`
- current AOT consumer is `nyash.string.concat_hh` + `nyash.string.len_h`
- recent read-side small seams regressed; the next generic question is how often this path lands in:
  - `ReturnHandle`
  - `BorrowView`
  - `FreezeOwned`
  - `FreshHandle`
  - `MaterializeOwned`

### Reading Lock for the First Slice

Read `concat_hh + len_h` as:

- owner choice above
- canonical contract in the middle
- Birth / Placement outcome at the seam
- backend leaves below

Do **not** read it as:

- “the optimization of `string_concat_hh_export_impl(...)`”
- “the optimization of `string_len_from_handle(...)`”

Those are implementation details, not the seam vocabulary.

## Smallest Useful Backend Slice

Before widening registry payloads or changing object identity rules, split the
current backend read into these three responsibilities:

1. `materialize_owned_bytes`
2. `objectize_stable_string_box`
3. `issue_fresh_handle`

Interpretation:

- `materialize_owned_bytes`
  - owned bytes exist in native runtime
  - may still be `DeferredStableBox`
- `objectize_stable_string_box`
  - transition into `StableBoxNow`
  - this is where `box_id` becomes relevant
- `issue_fresh_handle`
  - transition into `FreshRegistryHandle`

The current implementation may still execute them as one path.
The SSOT should nevertheless read them as separate responsibilities first.

## Optimization Rule

Before adding a new hot-path optimization:

1. identify the Birth / Placement outcome being exercised
2. decide whether the issue is
   - owner choice
   - MIR reading
   - backend leaf cost
3. optimize only at that layer

This prevents:

- helper-local policy growth
- route-specific hacks becoming accidental authorities
- mixing read-side and birth-side optimizations without a seam name

## Current Direction

For the current `concat_hh + len_h` front:

- read-side small seams have been tried and reverted
- the next likely generic seam is Birth / Placement backend cost
- specifically:
  - `FreshHandle`
  - `MaterializeOwned`

For backend interpretation, also read the same front as:

- current objectization: `StableBoxNow`
- current registry issue: `FreshRegistryHandle`

The next structural optimization target is not “make `next_box_id` faster at
any cost”.
It is “reduce the number of paths that must reach `StableBoxNow` before they
reach `FreshRegistryHandle`”.

That means the next optimization work should target birth backend leaves while
keeping this SSOT vocabulary fixed.

## First Exact Probe Read

Current direct AOT probe for `bench_kilo_micro_concat_hh_len.hako` shows:

- `birth.placement`
  - `fresh_handle=800000`
  - `return_handle=0`
  - `borrow_view=0`
  - `freeze_owned=0`
- `birth.backend`
  - `materialize_owned_total=800000`
  - `materialize_owned_bytes=14400000`
  - `gc_alloc_called=800000`
  - `gc_alloc_bytes=14400000`

So the current generic exact front is not a `FreezeOwned`-heavy path.
It is a `FreshHandle -> MaterializeOwned` path.
