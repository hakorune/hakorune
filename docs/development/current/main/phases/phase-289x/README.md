# Phase 289x: runtime-wide value/object boundary rollout

- Status: Proposed Planning
- Date: 2026-04-19
- Purpose: string で証明中の `value world -> publish/promote -> object world` 思想を、runtime 全体へ安全に広げるための phase/taskboard を切る。
- Parent SSOT:
  - `docs/development/current/main/design/lifecycle-typed-value-language-ssot.md`
- First proving ground:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - `docs/development/current/main/design/string-semantic-value-and-publication-boundary-ssot.md`
- Taskboard:
  - `docs/development/current/main/phases/phase-289x/289x-91-runtime-value-object-task-board.md`

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
- phase-137x keeper/reject 前に container runtime work を開くこと

## Phase Order

1. `Phase 0`: authority / vocabulary lock
2. `Phase 1`: demand vocabulary inventory
3. `Phase 2`: container lane-host contract
4. `Phase 3`: first storage pilot after string keeper
5. `Phase 4`: scalar immediate widening
6. `Phase 5`: bytes / view first-class planning
7. `Phase 6`: map key/value boundary planning
8. `Phase 7`: MIR legality / verifier lift
9. `Phase 8`: allocator / arena only if evidence demands it

## Relationship To Phase 137x

Phase 137x remains the active string optimization lane.
Phase 289x is not allowed to open implementation work before the phase-137x
string corridor reaches a keeper/reject decision on the active read-side lane.

Reading:

- Phase 137x proves the pattern on `String`
- Phase 289x organizes how to generalize the pattern
- Phase 289x does not bypass phase-137x stop-lines

## First Concrete Cards

- `289x-0a`: parent SSOT alignment
  - update lifecycle/value-repr/string docs so the authority order is explicit
- `289x-0b`: lane-host rule lock
  - array/map identity stays public semantic truth; only internal residence may specialize later
  - docs-only stop-line; no runtime/storage/MIR legality work opens here
- `289x-1a`: `CodecProfile` inventory
  - document which profiles are decode demand, storage demand, or compat residue
- `289x-1b`: `ValueDemand` vocabulary proposal
  - docs only; no code until callers and acceptance tests are known
- `289x-1c`: boundary vocabulary lock
  - `publish`, `promote`, `freeze`, `materialize`, `handle issue`, `borrow/project`
  - one term, one responsibility
- `289x-2a`: array lane-host design
  - text lane first, generic degrade explicit, public array semantics unchanged
- `289x-3a`: scalar immediate audit
  - identify boxed int/bool hot paths before any implementation cut
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
