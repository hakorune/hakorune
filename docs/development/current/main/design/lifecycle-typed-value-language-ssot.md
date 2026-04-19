---
Status: SSOT
Decision: provisional
Date: 2026-04-09
Scope: Hakorune を "box language" ではなく "lifecycle-typed value language" として読むための architecture SSOT。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/phases/phase-163x/README.md
  - docs/development/current/main/design/semantic-optimization-authority-ssot.md
  - docs/development/current/main/design/birth-placement-ssot.md
  - docs/development/current/main/design/string-transient-lifecycle-ssot.md
  - docs/development/current/main/design/string-canonical-mir-corridor-and-placement-pass-ssot.md
  - docs/development/current/main/design/primitive-family-and-user-box-fast-path-ssot.md
  - docs/development/current/main/design/enum-sum-and-generic-surface-ssot.md
  - docs/development/current/main/design/value-repr-and-abi-manifest-ssot.md
  - docs/development/current/main/phases/phase-289x/README.md
  - docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md
  - docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md
  - src/mir/instruction.rs
  - src/mir/storage_class.rs
---

# Lifecycle-Typed Value Language SSOT

## One-Line Thesis

Hakorune は box を物理オブジェクトではなく lifecycle 契約として扱い、
hot path の値は value world で流し、identity / shared escape / host boundary のときだけ
handle world へ objectize する。

短く言うと:

- inside = value
- boundary = box/handle

## Goal

- `new Point(...)` を「即 heap object を作る命令」ではなく「Point という意味の値が生まれる」
  として読める architecture を固定する
- primitive fast path / string borrowed corridor / enum sum lane / typed user-box field access
  を 1 本の設計軸で説明できるようにする
- allocator / wrapper / helper 連鎖を性能の primary lever にせず、
  objectization の頻度そのものを下げる方向へ寄せる

## Relationship To Existing SSOTs

この文書は既存 SSOT を置き換えるものではなく、上位の parent reading を与える。

### Runtime-wide reading order

runtime-wide value/object boundary work は次の順で読む。

1. `semantic-optimization-authority-ssot.md`
   - authority order の正本
2. `lifecycle-typed-value-language-ssot.md` (this file)
   - runtime-wide generalization の parent architecture reading
3. `string-semantic-value-and-publication-boundary-ssot.md`
   - first proving ground の semantic/boundary lock
4. `phase-137x/README.md`
   - active implementation lane
5. `phase-289x/README.md` / `289x-91-runtime-value-object-task-board.md`
   - runtime-wide successor planning/taskboard only

Rule:

- phase-289x は authority を持たない
- phase-137x string lane has a keeper proof, but runtime-wide implementation
  still requires separate phase gates for each deferred carrier/storage rewrite

### Semantic authority order stays unchanged

authority order は引き続き次で固定する。

1. `.hako owner / policy`
2. `MIR canonical contract`
3. `Rust executor / accelerator`
4. `LLVM / native optimization`

This document does not weaken
`semantic-optimization-authority-ssot.md`.

### Birth / Placement vocabulary stays valid

`Birth / Placement` の outcome 語彙:

- `ReturnHandle`
- `BorrowView`
- `FreezeOwned`
- `FreshHandle`
- `MaterializeOwned`
- `StoreFromSource`

は current truth のまま維持する。

この文書はそれを一般化して、

- value world で流す
- object world へ入る条件

の architectural reading を与えるだけだよ。

### Current ABI/public value classes stay owned by the manifest SSOT

`value-repr-and-abi-manifest-ssot.md` の current public/runtime classes:

- `imm_i64`
- `imm_bool`
- `handle_owned`
- `handle_borrowed_string`
- `boxed_local`

は immediate ABI truth のまま維持する。

この文書は future generalization として:

- `imm`
- `borrow`
- `agg_local`
- `handle`

を architecture vocabulary に固定する。

必要なら `owned_buf` は backend-private / future child vocabulary として追加してよいが、
この文書の MVP では primary class に含めない。

### String docs remain the first proving ground

`string-transient-lifecycle-ssot.md` と
`string-canonical-mir-corridor-and-placement-pass-ssot.md`
は、value-world reading の first proving ground としてそのまま使う。

`string-semantic-value-and-publication-boundary-ssot.md` と
`phase-137x/README.md` は implementation 側の first proving ground でもある。
runtime-wide generalization の taskboard は `phase-289x` に置いてよいが、
active lane は引き続き phase-137x string lane とする。

## Fixed Decisions

### 1. `box` means lifecycle contract, not immediate allocation

`box` declaration is allowed to imply:

- object-capable semantic type
- methods / fields / identity-sensitive capabilities
- eventual objectization/publication rules

It must not imply:

- immediate heap allocation
- immediate handle issuance
- mandatory stable identity on every local step

`new BoxType(...)` should be read as:

- birth of a semantic value

and only later, if required:

- objectize into a stable object
- publish into a handle

### 2. Hot-path values live in a small value world

This document fixes the primary hot-path value classes as:

1. `imm`
   - scalar immediate
   - current examples: integer / bool / future float immediate
2. `borrow`
   - non-owning borrowed/read session value
   - current proving ground: string view / borrowed corridor
3. `agg_local`
   - non-escaping local aggregate
   - current intended home for tuple / record / enum payload / non-escaping user-box body
4. `handle`
   - published stable identity
   - current object / host / plugin / shared alias world

Rule:

- optimize toward `imm`, `borrow`, and `agg_local`
- treat `handle` as boundary-class, not default transport

### 2.5. Runtime-private carrier vocabulary

runtime-wide value/object work uses this shared carrier vocabulary.
It is an internal planning vocabulary, not a public ABI class list.

| Term | Meaning | Examples today |
| --- | --- | --- |
| `Ref` | borrowed/read-only view or read session | string read session, verified source, array/map read view |
| `Owned` | unpublished caller-owned payload | `OwnedBytes`, owned bytes prepared for a sink |
| `Cell` | mutable storage residence inside a container/lane | future `TextCell` / array/map lane cell; current `KernelTextSlot` is transport adapter |
| `Immediate` | unboxed scalar payload | current `imm_i64`, `imm_bool` lowering |
| `Stable` | object-capable public representation | `StringBox`/handle, scalar box, generic object |

Rule:

- `Ref`, `Owned`, `Cell`, and `Stable` are lifecycle states, not language types.
- A family only adopts a state when it has a real owner, tests, and boundary demand.
- `publish` / `promote` are boundary effects over these states; they are not legality owners.

Carrier lock:

- `TextRef`, `TextPlan`, `OwnedText`, and `TextCell` are the future semantic
  text carriers.
- `BorrowedHandleBox` is boundary/cache for borrowed-alias encode and cached
  stable-handle reuse. It is not `TextRef`.
- `KernelTextSlot` is runtime-private transport adapter / sink seed. It is not
  the `TextCell` abstraction.
- `StringViewBox` is object-world view with API/compat semantics. It is not
  the internal substring carrier.

### 2.6. Demand verbs drive boundary behavior

`get`, `set`, and `call` must be read as demand-bearing verbs, not as helper-name authority.

- `get` may demand read-only ref, immediate encoding, borrowed alias encoding, or stable object publication.
- `set` may demand cell residence, owned payload consumption, generic degrade, or mutation invalidation.
- `call` may demand thin internal value entry or public object/handle entry.

The demand owner is MIR/lowering facts and future recipe metadata.
The runtime executes the selected demand and must not infer publication legality from helper names.

### 3. Objectization is driven by explicit barriers

For the current architectural cut, objectization is allowed only when one of the following is true.

1. `ObserveIdentity`
   - pointer identity / `box_id` / weak-ref-sensitive semantics become visible
2. `EscapeShared`
   - value escapes the current local region and shared aliasing must survive
3. `HostBoundary`
   - value crosses plugin / FFI / host / public ABI boundary

Current simplification:

- `publish` reads through `HostBoundary`
- finalizer / weak-ref / observer-heavy types are handle-required capabilities,
  not separate top-level value classes

### 4. Canonical MIR stays semantic; compat lowering may box later

Canonical MIR should describe semantic intent first, not the fallback runtime object shape.

This means:

- enum/sum stays on `VariantMake` / `VariantTag` / `VariantProject`
- typed user-box access stays on `FieldGet` / `FieldSet`
- future tuple / record / enum payload should become aggregate-shaped MIR truth
- `Program(JSON v0)` and VM fallback may still materialize hidden payload boxes as compat carriers

The important rule is:

- hidden payload box is compat/backend fallback
- not semantic truth

### 5. User boxes are struct-first, handle-second

For known receiver / known declaration cases, read user boxes as local typed aggregates first.

Target reading:

```text
box Point { x: Int, y: Int }

local p = Point(1, 2)
p = p.move(3, 4)
```

should be able to lower as:

- local aggregate payload
- typed field access
- thin monomorphic method call

without forcing object/handle world on every step.

Dynamic reflection / unknown receiver / host-visible publication may still force `handle`.

### 6. One canonical `Call`, multiple physical entries

MIR should keep one canonical `Call` surface.
Do not introduce a second semantic call dialect for "fast" mode.

Physical entry split happens below canonical MIR:

- public entry
  - generic ABI / boundary-safe
- thin internal entry
  - monomorphic / value-class-specific / backend-private

The selection owner is:

- pass + manifest

not ad-hoc helper naming.

### 7. Rust is a microkernel for object world and boundaries

Rust should own:

- objectization mechanics
- publication / handle table
- TLS / epoch / lifetime substrate
- host/plugin/public ABI
- GC / weak ref / finalization hooks
- native leaf acceleration

Rust should not own:

- semantic route/policy
- user-visible shape meaning
- canonical lifecycle policy choice

## Architectural Reading

Read the end-state through the following stack:

```text
.hako
  meaning / policy / route / escape contract
    ->
MIR
  semantic ops + lifecycle facts
    ->
passes
  choose imm / borrow / agg_local / handle
  sink objectization and thin entry selection
    ->
Rust microkernel
  object world + host boundary + runtime substrate
    ->
LLVM/native
  scalarize / inline / monomorphize hot path
```

## Aggregate Rule

To keep `.hako -> MIR` clean, aggregate payloads should be first-class before fallback boxing.

Preferred future reading:

- tuple payload
- record payload
- enum payload
- non-escaping user-box body

all enter MIR as aggregate truth.

Then:

- AOT/native may scalarize them
- compat/VM fallback may box them

This rule is the architectural replacement for making hidden payload box the semantic truth.

## Thin-Fast Rule

The main performance target is not "a faster allocator first".
The main target is:

- fewer objectization events
- fewer per-call wrapper layers
- earlier monomorphic route choice

Therefore:

- do not start from allocator swaps
- do not start from helper proliferation
- first reduce the number of times hot paths enter handle world

## Current Acceptance Scope

This document does not require an immediate whole-repo rewrite.

Current acceptable proving grounds are:

- string borrowed corridor
- primitive handle-unbox fast paths
- typed user-box field access
- enum/sum payload design cleanup

This document also does not require:

- new `.hako` syntax
- immediate public ABI break
- full `Program(JSON v0)` retirement in the same cut

## Runtime-Wide Rollout Handoff

`phase-289x` is the planning child for generalizing this SSOT from the
string proving ground to runtime-wide value/object boundaries.

Authority stays here. The phase doc is a taskboard, not a second semantic
truth.

Successor relationship は次で固定する。

- parent architecture authority:
  - `lifecycle-typed-value-language-ssot.md`
- active proving/implementation lane:
  - `string-semantic-value-and-publication-boundary-ssot.md`
  - `phase-137x/README.md`
- runtime-wide successor planning:
  - `phase-289x/README.md`
  - `289x-91-runtime-value-object-task-board.md`

phase-289x may refine rollout vocabulary and sequencing only.
It must not:

- start runtime-wide implementation before phase-137x reaches keeper/reject
  on the active read-side lane
- replace the string lane as the first proving ground
- change the public handle ABI truth

### Container rule

Array / Map are not reclassified as ordinary immutable values.

They remain identity-capable containers:

- public semantics may require stable container identity
- public ABI may keep handle-based array/map surfaces
- internal element/key/value residence may still become lane-hosted

This prevents the value-world rule from erasing container identity.

### Boundary rule

`publish` / `promote` are boundary effects.

- runtime may execute objectization
- runtime must not silently infer semantic publication legality
- MIR/lowering must eventually carry demand / boundary facts
- runtime stays executor / boundary microkernel, not semantic owner

For string specifically:

- `publish` stays boundary effect
- `freeze.str` remains the only string birth sink
- `TextLane` remains future storage specialization, not semantic truth

### Rollout rule

Generalization must proceed in this order:

1. docs / authority / vocabulary
2. demand inventory
3. container lane-host contract
4. one storage pilot after the active string lane is judged
5. scalar / bytes / map only when evidence picks them
6. MIR legality after runtime-private contracts are proven

Do not start with allocator work or a broad lane rewrite.

## Immediate Task Pack

1. docs-first lock
   - keep this document as the architecture parent for value-world work
2. payload/aggregate inventory
   - inventory where AST / Stage1 / MIR metadata still encode payload as single-scalar truth
3. objectization sinking inventory
   - inventory where known user-box / enum payload still objectize too early
4. thin-entry design inventory
   - define manifest-driven physical thin entry selection without adding a second semantic call dialect

## Non-Goals

- do not redefine current ABI/public value classes here
- do not make hidden payload box the new semantic truth
- do not fold string-specific birth vocabulary back into generic box semantics
- do not widen `@rune` or add optimizer-only source syntax
