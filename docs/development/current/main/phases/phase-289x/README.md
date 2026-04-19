# Phase 289x: runtime-wide value/object boundary rollout

- Status: Active Planning
- Date: 2026-04-19
- Purpose: string で証明中の `value world -> publish/promote -> object world` 思想を、runtime 全体へ安全に広げるための phase/taskboard を切る。
- Parent SSOT:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- First proving ground:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
- Taskboard:
  - `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
  - `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
  - `docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md`
  - `docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md`
  - `docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md`
  - `docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md`
  - `docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md`

## Decision

やる価値はある。理由は、string の current owner が helper 個別問題ではなく
「どこで value を object/handle world に戻すか」という runtime-wide boundary 問題として読めるから。

ただし実装は一気に広げない。

新しい親SSOTは作らない。既存の
`lifecycle-typed-value-language-ssot.md` を親にして、この phase は taskboard に徹する。

- string は first proving ground
- array / map は semantic value ではなく identity container
- array / map の内部 residence だけを lane host として段階化する
- public handle ABI は維持する
- `publish` / `promote` は boundary effect として扱う
- `freeze.str` は string の唯一の birth sink に固定する
- container lane-host generalization は Array/Map semantics の再定義ではなく、内部 residence の stop-lined planning に限定する
- runtime は semantic owner ではなく executor / boundary microkernel として読む

## Non-Goals

- runtime 全体の即時 lane rewrite
- public ABI widening
- `text.ref` / `bytes.ref` / `array.text` などの public MIR dialect 先行導入
- `publish.text` を `freeze.str` と競合する第二 birth sink にすること
- array / map を immutable value として読み替えること
- evidence なしの allocator lane / arena 導入
- phase-137x keeper 後の demand/container inventory を閉じる前に container runtime work を開くこと

## Phase Order

1. `Phase 0`: authority / vocabulary lock
2. `Phase 1`: demand vocabulary inventory
3. `Phase 2`: container lane-host contract
4. `Phase 3`: first storage pilot after string read-side keeper/reject
5. `Phase 4`: scalar immediate widening
6. `Phase 5`: bytes / view first-class planning
7. `Phase 6`: map key/value boundary planning
8. `Phase 7`: MIR legality / verifier lift
9. `Phase 8`: allocator / arena only if evidence demands it

## Readable Design Brief

Read this phase in this order:

1. `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
2. `docs/development/current/main/phases/phase-289x/289x-90-runtime-value-object-design-brief.md`
3. `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`
4. `docs/development/current/main/phases/phase-289x/289x-92-value-boundary-inventory-ledger.md`
5. `docs/development/current/main/phases/phase-289x/289x-93-demand-vocabulary-ledger.md`
6. `docs/development/current/main/phases/phase-289x/289x-94-container-demand-table.md`
7. `docs/development/current/main/phases/phase-289x/289x-95-array-text-residence-pilot.md`
8. `docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md`

The brief is phase-local planning material.
It does not create a new parent SSOT and does not authorize implementation.

## Runtime Vocabulary Lock

phase-289x uses this shared lifecycle vocabulary:

| Term | Meaning | Scope |
| --- | --- | --- |
| `Ref` | borrowed/read-only view or read session | internal value world |
| `Owned` | unpublished owned payload | internal value world |
| `Cell` | container/lane residence | internal storage |
| `Immediate` | unboxed scalar payload | internal value world |
| `Stable` | object-capable public representation | object/handle world |

Demand verbs:

- `get` asks for read ref, immediate encode, borrowed alias encode, or stable object publication
- `set` asks for immediate store, owned payload consume, cell residence, generic degrade, or invalidation
- `call` asks for thin internal value entry or public object/handle entry

`publish` / `promote` are effects selected by demand facts.
They do not decide language legality and they do not create a second string birth sink.

## Relationship To Phase 137x

Phase 137x remains the active string optimization lane.
Phase 137x produced the current string proof in keeper `49c356339`
(`array.get -> indexOf -> branch -> same array.set` suffix store without
`slot_load_hi` on that exact path).

That keeper unlocks post-keeper inventory, not runtime-wide rewrite.
Phase 289x must finish demand/container boundary inventory before opening any
`TextLane`, container storage, MIR legality, or allocator implementation card.

Reading:

- Phase 137x proves the pattern on `String`
- Phase 289x organizes how to generalize the pattern
- Phase 289x does not bypass phase-137x stop-lines
- Optimization work stays paused while `289x-1f` / `289x-1g` / `289x-2d`
  inventory cards define the next implementation cut

## First Concrete Cards

- `289x-0a`: parent SSOT alignment
  - update lifecycle/value-repr/string docs so the authority order is explicit
- `289x-0b`: lane-host rule lock
  - array/map identity stays public semantic truth; only internal residence may specialize later
  - docs-only stop-line; no runtime/storage/MIR legality work opens here
- `289x-0c`: restart/current pointers
  - keep phase-289x visible as parked successor only
- `289x-0d`: runtime vocabulary lock
  - define `Ref / Owned / Cell / Immediate / Stable`
  - define `get / set / call` as demand verbs
  - keep this as planning vocabulary, not public ABI
- `289x-1a`: `CodecProfile` inventory
  - document which profiles are decode demand, storage demand, or compat residue
- `289x-1b`: `ValueDemand` vocabulary proposal
  - docs only; no code until callers and acceptance tests are known
- `289x-1c`: boundary vocabulary lock
  - `publish`, `promote`, `freeze`, `materialize`, `handle issue`, `borrow/project`
  - one term, one responsibility
- `289x-1f`: post-keeper value-boundary inventory sync
  - record the `49c356339` string keeper as proof, not as runtime-wide permission
  - mark pre-keeper owner numbers as historical where needed
- `289x-1g`: exact demand ledger
  - map profile/helper/caller names to `ValueDemand`, `StorageDemand`,
    `PublishDemand`, and `MutationDemand`
  - status: done in `289x-93-demand-vocabulary-ledger.md`
- `289x-2a`: array lane-host design
  - text lane first, generic degrade explicit, public array semantics unchanged
- `289x-2d`: array/map demand table
  - read-ref, encoded alias, stable object, cell residence, degrade, invalidation
  - status: done in `289x-94-container-demand-table.md`
- `289x-3a`: scalar immediate audit
  - identify boxed int/bool hot paths before any implementation cut
- `289x-3b`: first storage pilot selection
  - one runtime-private storage pilot only, after `289x-1g` and `289x-2d`
  - selected pilot: `Array text residence through KernelTextSlot store`
- `289x-3c`: Rust `CodecProfile -> DemandSet` mapping
  - status: done in code; behavior unchanged
  - `CodecProfile::demand()` maps profile names to runtime-private `DemandSet`
  - `any_arg_to_box_with_profile` and `decode_array_fast_value` bind demand metadata before old branches
- `289x-3d`: Rust `BorrowedAliasEncodeCaller -> DemandSet` mapping
  - status: done in code; behavior unchanged
  - `BorrowedAliasEncodeCaller::demand()` maps caller names to runtime-private `DemandSet`
  - borrowed-alias encode plans bind fallback publish demand without changing execution branches
- `289x-3e`: Rust `PublishReason -> PublishDemand` mapping
  - status: done in code; behavior unchanged
  - `PublishReason::demand()` maps publish reason names to runtime-private `PublishDemand`
  - publish helpers bind boundary-effect demand before old observation/objectization branches
- `289x-3f`: Rust array generic load/encode demand tags
  - status: done in code; behavior unchanged
  - array encoded get/load sites bind `ARRAY_GENERIC_GET_ENCODED`
  - demand names immediate encode, borrowed alias encode, and stable object fallback
- `289x-3g`: Rust array store/append demand tags
  - status: done in code; behavior unchanged
  - `array_slot_store_any` binds `ARRAY_GENERIC_STORE_ANY`
  - `array_slot_append_any` binds `ARRAY_GENERIC_APPEND_ANY`
- `289x-3h`: `KernelTextSlotState` demand bridge
  - status: done in code; high-risk; behavior unchanged; no ABI change
  - state demand and boundary publish demand stay separated
- `289x-7a`: C shim generic method set-route demand metadata
  - status: done in code; emitted lowering unchanged
  - `ArrayStoreString` carries source-preserve plus publish-handle demand metadata
  - stable-object demand remains off
  - direct array-store-string smoke still stops before lowering on the existing pure-shape recipe gate
- `289x-7b`: MIR demand/placement parallel facts
  - status: done in code; inspection-only; behavior unchanged
  - `ThinEntryCandidate` / `ThinEntrySelection` carry demand facts beside value-class/carrier facts
  - folded `PlacementEffectRoute` carries demand beside decision/source/state
  - MIR JSON emits the demand fields for downstream inspection
- `289x-5a`: bytes/view planning
  - prevent text-only patterns from being copied into bytes later
- `289x-6a`: map key/value boundary map
  - key decode, value storage, read publication, and compat exports stay separated
- `289x-6d`: Map key/value codec demand bridge
  - status: done in code; behavior unchanged; no typed map lane
  - Map key decode binds explicit i64/any/runtime-data demand metadata
  - Map value store binds value-residence + alias-invalidation demand metadata
- `289x-6e`: Map load encoding split
  - status: done in code; behavior unchanged; no public ABI change
  - materializing map load and caller-scoped alias encode now carry separate demand metadata
  - Rust runtime clusters in `289x-96` are now closed
- `289x-7c`: C shim `get/len/has/push` policy split
  - status: done in code; behavior and emitted lowering unchanged
  - `get` stable-object/publish demand, `len`/`has` read-ref demand, and `push` encode/residence/mutation demand are now explicit route metadata
- `289x-7d`: main `bname/mname` route classifier cutover
  - status: done in code; behavior unchanged
  - `bname/mname` strings are normalized into receiver/method surface enums before route-bit selection
  - verified by RuntimeData array/map get/has/size/length/push, array-string indexOf, and StringBox length/indexOf route smokes
- `289x-7e`: concrete `slot_load_hi` / `slot_store` helper emission cutover
  - status: done in code; behavior and helper symbols unchanged
  - `hako_llvmc_ffi_array_slot_emit.inc` is now the single C-shim emission entry for array slot load/store, array string len, and array string indexOf concrete calls
  - verified by exact kernel slot-store, live-after-get, array set/get, and array-string len/indexOf smokes
- `289x-7f`: `runtime_array_string` observer/window matcher cutover
  - status: done in code; behavior unchanged
  - `hako_llvmc_ffi_array_string_window_policy.inc` now owns array text-read/read eligibility for C-shim window matchers
  - verified by branch/select/cross-block/interleaved/live-after-get/len-live exact window smokes
- `289x-7g`: MIR string helper-name compat/recovery cutover
  - status: done in code; behavior unchanged
  - `src/mir/string_corridor_names.rs` now owns helper/runtime-export name vocabulary for compat recovery and recognizers
  - verified by `string_corridor`, `string_corridor_names`, and release build
- `289x-7h`: prepass/declaration need classifier cutover
  - status: done in code; behavior unchanged
  - need/prepass classifiers now consume normalized receiver/method surfaces
  - declarations and prepass needs remain exact; no silent fallback or helper declaration widening
  - verified by RuntimeData array/map get/has/size/length/push, array-string indexOf, and array set/get canary smokes
- `289x-96`: demand-backed cutover inventory
  - status: closed
  - all Rust/C-shim/MIR clusters are done

## Return To Optimization Gate

Optimization work was paused until
`docs/development/current/main/phases/phase-289x/289x-96-demand-backed-cutover-inventory.md`
was fully closed.

Current state: closed by `289x-7h`.
Next optimization work may resume only through the owner-first perf entry:
`docs/development/current/main/design/perf-owner-first-optimization-ssot.md`.

High-risk work is planned, not skipped:

- full `ArrayStorage::Text` / full `TextLane`: separate phase after `289x-96`
- Map typed lane: separate phase after `289x-96`
- allocator / arena: only after value-boundary cutover and perf evidence

## Stop-Line

Stop immediately if a proposed card:

- mixes docs/vocabulary with storage rewrite
- changes public ABI before runtime-private proof
- treats container identity as a value-lane detail
- turns container lane-host planning into Array/Map semantic rewrite or new birth-sink design
- adds a helper-name allowlist instead of a boundary contract
- starts allocator work before perf evidence points there
- opens MIR legality / verifier lift or allocator / arena work before their scheduled rollout phase
- makes runtime infer publish legality that MIR/lowering did not request
- returns to optimization without using the owner-first perf entry after `289x-96` closure
