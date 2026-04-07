---
Status: SSOT
Decision: current
Date: 2026-04-07
Scope: semantic optimization authority を `.hako owner -> MIR canonical contract -> Rust executor -> LLVM generic optimization/codegen` で固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/nyash-kernel-semantic-owner-ssot.md
  - docs/development/current/main/design/birth-placement-ssot.md
  - docs/development/current/main/design/canonical-lowering-visibility-ssot.md
  - lang/src/runtime/kernel/string/README.md
  - lang/src/runtime/kernel/string/chain_policy.hako
  - lang/src/runtime/kernel/string/search.hako
  - lang/src/runtime/collections/array_core_box.hako
  - lang/src/runtime/collections/map_core_box.hako
  - crates/nyash_kernel/src/exports/string_helpers.rs
  - crates/nyash_kernel/src/exports/string_plan.rs
  - crates/nyash_kernel/src/exports/string_view.rs
  - crates/nyash_kernel/src/plugin/array_string_slot.rs
  - src/runtime/host_handles.rs
---

# Semantic Optimization Authority SSOT

## Goal

- `.hako` に semantic ownership を保ったまま、optimization authority も `.hako` 側に寄せる
- Rust helper が policy owner に見えない構造を固定する
- shape-specific fast path を authority にせず、canonical contract の sugar に閉じ込める

## Final Stack

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust executor / accelerator`
4. `LLVM generic optimization / codegen`

This is the only intended authority order.

## Target Execution Flow

Read hot-path lifecycle through this stack:

```text
.hako
  decides policy:
    - source_preserve
    - identity_demand
    - publication_demand
        |
        v
MIR
  carries public canonical contract:
    - store.array.str
    - delayed-materialization reading
    - escalation visibility
        |
        v
Rust
  executes runtime facts and mechanics:
    - SourceKindCheck
    - SourceLifetimeKeep
    - AliasUpdate
    - NeedStableObject
        |
        v
LLVM / native code
  optimizes the chosen shape
```

The rule is:

- policy is fixed above Rust
- mechanics are executed below MIR
- helper-local branching must not invent new lifecycle policy

## Layer Borrowing Rule

Hakorune should borrow ideas per layer without mixing their responsibilities.

- `.hako owner / policy`
  - borrow Rust-like ownership vocabulary as meaning only
  - examples: `BorrowView`, `ReturnHandle`, `FreezeOwned`, `StoreFromSource`
- `MIR canonical contract`
  - borrow C++-like delayed materialization as canonical rewrite/placement reading
  - keep helper names out of the contract surface
- `Rust executor / accelerator`
  - borrow C-like storage/lifetime discipline for runtime mechanics
  - keep borrowed view/span, freeze/materialize, registry issue, slot/probe/store here
- `LLVM generic optimization / codegen`
  - borrow only generic optimization/codegen discipline
  - do not ask LLVM to rediscover owner-aware placement

This means:

- do not copy Rust's type system upward into `.hako`
- do not push C++-style materialization policy down into Rust helpers
- do not move storage/lifetime mechanics into `.hako`
- do not encode owner choice as an LLVM discovery problem

## `.hako owner / policy`

`.hako` が持つもの:

- route vocabulary
- retained-form / boundary choice
- visible collection contract
- adapter / route semantics

`.hako` は 「何を選ぶか」 を決める。
Rust helper に semantic branching を戻さない。

## MIR canonical contract

MIR が持つもの:

- owner choice を backend へ運ぶ canonical op 名
- `.hako` policy を Rust executor に渡す stable contract

Keep existing canonical ops:

- `thaw.str`
- `str.slice`
- `str.concat3`
- `str.len`
- `str.find_byte_from`
- `str.eq_at`
- `freeze.str`

Next candidate ops:

- `lit.str`
- `str.concat2`
- `store.array.str`
- `store.map.value`

Current source-backed mapping:

- `.hako` route `const_suffix`
  - current concrete path: `nyash.string.concat_hs`
  - intended canonical MIR reading: `thaw.str + lit.str + str.concat2 + freeze.str`
- `.hako` route `ArrayStoreString`
  - current concrete path: `nyash.array.set_his`
  - intended canonical MIR reading: `store.array.str`
- `.hako` route `MapStoreAny`
  - current concrete path: `nyash.map.slot_store_hhh`
  - intended canonical MIR reading: `store.map.value`

The candidate MIR names are current docs/SSOT truth.
They are not first-class MIR enum variants yet.

Do not encode `return_handle` as a standalone executor op.
It is a rewrite / elision outcome.

Birth / Placement vocabulary should also be read through canonical contract
names, not helper names. See `birth-placement-ssot.md`.

## Rust executor / accelerator

Rust が持つもの:

- borrowed view/span lifetime
- raw copy/search/compare
- freeze/materialize leaf
- direct slot/probe/store leaves
- drop-epoch invalidation
- single-lock borrowed access

`BorrowedText` と `TextSink` を使う場合でも、位置づけは Rust 内部 protocol のみ。
Public authority 名にしない。

The same rule applies to birth backend leaves.

- `string_handle_from_owned(...)`
- `freeze_text_plan(...)`
- `materialize_owned_string(...)`

These are executor/backend details only. Treat them as the Rust birth backend
family, not as semantic vocabulary.

The same rule applies to `box_id`.

- do not lift `box_id` into `.hako` route vocabulary
- do not encode `box_id` as a MIR top-level outcome name
- keep `box_id` inside Rust-side objectization contract

Read it through the Birth / Placement backend second axis:

- `Objectization = None | StableBoxNow | DeferredStableBox`
- `RegistryIssue = None | ReuseSourceHandle | FreshRegistryHandle`

`box_id` belongs only to `Objectization::StableBoxNow`.

## Lifecycle Policy Rule

Lifecycle policy must be decided above Rust.

### `.hako owner / policy` decides

- source-preserve eligibility
- retained-form / boundary choice
- identity demand
- publication demand

### `MIR canonical contract` carries

- the visible canonical route name
- delayed-materialization reading
- escalation conditions for object-world entry
- lifecycle visibility that must stay above Rust:
  - `source_preserve`
  - `identity_demand`
  - `publication_demand`

### `Rust executor / accelerator` executes

- source kind checks
- source-lifetime keep
- alias survival / alias update
- drop-epoch / registry / `Arc` mechanics

Rust may decide runtime facts.
Rust must not invent new lifecycle policy by helper-local branching.

`store.array.str` is the current concrete example.

- public canonical contract stays `store.array.str`
- backend-private split stays:
  - `SourceKindCheck`
  - `SourceLifetimeKeep`
  - `AliasUpdate`
  - `NeedStableObject`
- only `NeedStableObject` may justify generic object entry

## LLVM generic optimization / codegen

LLVM が持つもの:

- generic SSA optimization
- alloca / scalar / aggregate friendly lowering
- target codegen

LLVM に owner-aware placement を発見させない。
`borrow / freeze / store` の semantic choice は MIR までで確定する。

## Observer Plane

observer は authority stack の外に置く。

- observer identity は canonical contract 名に揃える
- observer backend は exact counter / trace / sink に分ける
- observer は route 選択や fallback 条件を決めない
- observer は default release から compile-out 可能でなければならない

Current rule:

- default build: observer compile-out
- `--features perf-observe`: observer compile-in
- `NYASH_PERF_COUNTERS=1`: feature-on build の runtime gate
- exact counter backend: TLS-first
- stderr summary: current-thread flush + exited-thread merge を current truth にする
- observer backend / sink remain Rust runtime mechanics keep; only observer
  identity is allowed to align with `.hako` / MIR canonical contract names
- `--features perf-trace`: heavy trace / sampled probe / scoped timing lane
- `NYASH_PERF_TRACE=1`: trace-only runtime gate

Observer reading lock:

- canonical contract identity belongs above, with `.hako` / MIR naming
- backend / sink / TLS exact counter mechanics remain below, in Rust runtime keep
- do not try to move observer backend itself into `.hako`; move only its public
  identity and policy vocabulary upward

## Capability-Seam Consumer Rule

Current hot executors should be read as capability-seam consumers before they
are read as standalone helpers.

| Canonical contract | Current Rust executor detail | Future capability seam reading |
| --- | --- | --- |
| `store.array.str` | `array_string_store_handle_at(...)` | `RawArray` consumer + lower `hako.mem` / `hako.ptr` / `hako.value_repr` family |
| `const_suffix` | `concat_const_suffix_fallback(...)` | string borrow/freeze consumer + lower `hako.mem` / `hako.ptr` family |
| observer exact counter | `observe/backend/tls.rs` | out-of-band runtime mechanics keep; only contract identity aligns upward |

This rule exists so perf work does not accidentally grow helper-local policy
again while capability-family planning is still docs-first.

## Birth / Placement Rule

Hot-path performance work must read Birth / Placement outcome before it reads a
helper name.

Current outcome vocabulary is:

1. `ReturnHandle`
2. `BorrowView`
3. `FreezeOwned`
4. `FreshHandle`
5. `MaterializeOwned`
6. `StoreFromSource`

Layer split:

- `.hako owner / policy`
  - birth trigger
  - retained-form / boundary choice
- `MIR canonical contract`
  - rewrite / elision reading
  - stable contract name
- `Rust executor / accelerator`
  - birth backend / registry issue / materialize / freeze / store execution

This keeps helper-local names below the public seam.

Backend-only follow-up:

- top-level Birth / Placement outcome stays:
  - `ReturnHandle`
  - `BorrowView`
  - `FreezeOwned`
  - `FreshHandle`
  - `MaterializeOwned`
  - `StoreFromSource`
- second-axis backend reading may additionally ask:
  - did this path require `StableBoxNow`?
  - did this path require `FreshRegistryHandle`?

That second axis is a Rust keep-line question, not a `.hako` owner question.

Current backend-private support seam:

- `OwnedBytes`
  - private owned-text carrier below top-level Birth / Placement naming
- `TextReadSession`
  - private read-only borrowed string session across
    `string_view.rs` / `string_helpers.rs` / `host_handles.rs`
- `SourceLifetimeKeep`
  - private source-preserving keep contract for alias survival
  - must remain backend-private until a higher-layer lifecycle rule explicitly widens it

Read-contract rule inside Rust:

- `as_str_fast() -> Option<&str>` is stable-object read only
- `with_str_handle(...)` / `with_text_read_session(...)` are live-source
  session reads only
- do not merge those two contracts by returning naked borrowed text from
  registry/session-backed reads

## `store.array.str` Contract Rule

Keep the visible canonical name `store.array.str`.
Do not promote `RetargetAlias` into a public MIR op.

Below that canonical name, freeze the backend-private split as:

1. `SourceKindCheck`
2. `SourceLifetimeKeep`
3. `AliasUpdate`
4. `NeedStableObject`

Interpretation:

- `SourceKindCheck`
  - runtime fact gathering
- `SourceLifetimeKeep`
  - source-preserving keep mechanics
- `AliasUpdate`
  - slot metadata update
- `NeedStableObject`
  - the only branch that may justify generic object-world entry

Read-side corollary:

- `SourceKindCheck` may use object/session-backed runtime facts
- `SourceLifetimeKeep` may preserve text/lifetime semantics
- neither of them should redefine `as_str_fast()` into a live-source API

This rule exists to stop Rust leaf helpers from re-owning lifecycle policy.

These names are allowed only as Rust backend family names.
Do not lift them into `.hako` route vocabulary or MIR top-level outcome names.

## Stop Lines

- `.hako` は owner / policy / route semantics まで
- MIR は canonical contract まで
- Rust は executor / accelerator に徹する
- LLVM は generic optimization と codegen に留める

Reopen gate:

- do not reopen `phase-137x` while canonical readings only exist as doc-level names
- first make the current concrete lowering visibly answer to:
  - `thaw.str + lit.str + str.concat2 + freeze.str`
  - `store.array.str`
  - `store.map.value`
- only then treat perf work as the next consumer

Do not:

- make Rust helper names the semantic source of truth
- grow shape-specific fast paths into independent policy systems
- let ABI facade absorb route semantics

## Implementation Order

1. freeze `.hako` route vocabulary as SSOT
2. freeze MIR canonical op set
3. adopt the first vertical slice on `concat const-suffix`
4. adopt the second vertical slice on `array string-store`
5. lock canonical-lowering visibility against current concrete symbols
6. only then generalize helper naming inside Rust
7. only then reopen perf consumers
