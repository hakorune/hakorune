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
- `289x-5a`: bytes/view planning
  - prevent text-only patterns from being copied into bytes later
- `289x-6a`: map key/value boundary map
  - key decode, value storage, read publication, and compat exports stay separated

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
