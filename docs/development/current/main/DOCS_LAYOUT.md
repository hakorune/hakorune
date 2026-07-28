# Docs Layout (SSOT)

Status: SSOT  
Scope: `docs/development/current/` 以下の「置き場所ルール」と、SSOT/履歴メモの混在を防ぐための最小ガイド。

## 目的

- 入口（読む順序）と、詳細（設計図/調査/Phaseログ）を分離して迷子を防ぐ。
- “Phase 文書が増えても” SSOT が埋もれないようにする。
- 大規模移動はmanifestと参照closureなしには行わない。archive policyの
  bounded batch/resolver/stub手順を満たす場合だけ物理整理を進める。

## Current Ownership Contract

- `AGENTS.md`
  - root AI/developer instruction entry
  - current-first pointer only; not a current lane ledger
  - historical sections inside it are subordinate to `CURRENT_STATE.toml`
  - tracked contract:
    `docs/development/current/main/design/agent-current-entry-contract-ssot.md`
- `CURRENT_TASK.md`
  - root restart anchor
  - thin entry that points to `CURRENT_STATE.toml`
  - not a landed-history ledger
- `CURRENT_STATE.toml`
  - machine-readable current lane / blocker / latest-card pointer SSOT
- `10-Now.md`
  - docs-side thin mirror/dashboard
  - one-screen summary + links only
- `15-Workstream-Map.md`
  - one-screen lane order mirror
- `workstreams/*.md`
  - active multi-day work cards
  - inventory / selection / smoke / parking-lot notes live here instead of
    becoming one numbered row per observation
  - `workstreams/mirbuilder-inplace-replacement-current.md`
    - active Rust MirBuilder production-responsibility replacement order,
      finite pack counters, detached-asset disposition, and current cutover
  - `workstreams/language-v1-convergence-current.md`
    - parked Language v1 macro-row order, acceptance gates, and selfhost resume
      boundary
  - `workstreams/compiler-foundation-current.md`
    - active compiler foundation taskboard when exact-front optimization is
      paused for BoxCallable / TypeAbiCatalog / CorePlan foundation work
- `05-Restart-Quick-Resume.md`
  - fastest reboot path only
- `design/kernel-implementation-phase-plan-ssot.md`
  - canonical rough task-order SSOT
- `design/perf-owner-first-optimization-ssot.md`
  - optimization lane の `front split` / `owner transition` / `keeper-revert stop-line` owner
- `design/hako-optimization-toolbox-usability-ssot.md`
  - optimization toolbox entry for `hako_check`, MIR shape adapters, exact-EXE measurement, and row guard surfaces
- `design/hako-inspect-scope-dump-ssot.md`
  - source anchors vs `hako_check inspect` artifact dumps; keeps MIR / LLVM IR / assembly dump as tool queries, not `.hako` source commands
- `design/substring-concat-len-closed-form-lowering-ssot.md`
  - `kilo_micro_substring_concat` stable-length exact route residual owner; keeps the next slice in LLVM lowering/codegen, not `.hako` or MIRBuilder witness work
- `design/hotline-core-method-contract-ssot.md`
  - zero-cost hot-line keeper gate and CoreMethodContract migration owner
- `design/box-callable-registry-ssot.md`
  - final callable truth owner for builtin/plugin/user Box callables;
    Type ABI becomes projection and PluginLoader/type_registry become providers
- `design/type-abi-naming-and-box-descriptor-ssot.md`
  - naming boundary for TypeBox ABI v2 vs historical TypeAbi* descriptor
    projection surfaces and future BoxDescriptor naming
- `design/typed-object-exact-slot-abi-ssot.md`
  - typed-object exact slot ABI split owner; keeps compat `field_get_hii`
    separate from selected `typed_object.slot_load/store_*` exact routes
- `design/type-abi-view-and-plan-stamp-ssot.md`
  - Type ABI view boundary and PlanStamp owner; keeps Type ABI as read-only
    descriptor/snapshot surface over existing domain truth instead of a third
    canonical ABI or hot execution path
- `design/type-abi-catalog-planning-spine-ssot.md`
  - TypeAbiCatalog planning spine contract; keeps Catalog as a thin
    cross-domain query index and TypeAbiPack as a downstream artifact
- `design/type-abi-box-domain-ssot.md`
  - Box Domain ownership for TypeBox slots, PluginLoader route contracts,
    lifecycle routes, and NewBox/DropBox plan boundaries
- `design/coreplan-compat-normalizer-legoization-ssot.md`
  - COREPLAN-FOUND-000/001 owner for the first selected CorePlan foundation
    family; keeps remaining compatibility normalizer work as BoxShape-only
    lego-ization before any accepted-shape expansion
- `design/current-docs-update-policy-ssot.md`
  - current docs update policy and mirror-thinning contract
- `design/mirbuilder-inplace-replacement-policy-ssot.md`
  - active Rust MirBuilder migration law: one live production Builder,
    responsibility-by-responsibility caller switch, immediate selected old-path
    deletion, post-cutover parity, and no detached replacement pipeline
- `design/mirbuilder-final-pipeline-ssot.md`
  - MirBuilderの最終production authority:
    Resolve -> Observe -> Facts -> Recipe -> Verify -> Lower -> Seal ->
    Collect -> Atomic Publish。cell数やLOCではなく、この一方向graphへの
    収束をcompletionの北極星にする
- `design/derived-to-native-hako-artifact-model-ssot.md`
  - Rust-derived Hako artifact migration owner; keeps Rust source as editable
    reference during derived phases, generated Hako as execution artifact, and
    Hako-native adoption as the source-selfhost exit for compiler semantic
    families
- `design/mirbuilder-rust-to-hako-converter-task-order-ssot.md`
  - MirBuilder-only Rust-to-Hako converter task order; keeps day-to-day work in
    implementation commits and uses human design stops instead of route/card
    churn
- `design/mirbuilder-authority-based-hako-migration-ssot.md`
  - MirBuilder authority-based migration order; keeps migration units as
    Facts / Recipe / REGISTRY rule / symbolic command / allocation authority
    instead of Rust module or struct translation
- `design/hakorune-naming-and-rename-task-order-ssot.md`
  - Hakorune naming charter and rename task order; defines RHako / HHako,
    reserves naked `stage` for bootstrap vocabulary, separates run-pipeline /
    converter / adoption-plan, and stages the `nyash` -> `hakorune` migration
- `design/current-docs-archive-policy-ssot.md`
  - archive buckets, landed ledger, and current-doc slimming contract
  - repository artifact lifecycle baseline, bounded archive batches, design
    registry, and check-script retirement task order
- `design/design-registry-v1-sharded-manifest-ssot.md`
  - 7,000-line embedded registryを deterministic 16-shard manifestへ移す
    BoxShape-only authority。実装は必ず clean worktree を作る `CLEAN0` から始める
- `investigations/design-registry-v1-sharded-manifest-task-2026-07-14.md`
  - parked execution taskboard。V0 parity、atomic cutover、V0 retirement、prior
    laneへの `RETURN0` までを固定し、current blockerを暗黙に切り替えない
- `design/mir-cleanup-policy-ssot.md`
  - BoxShape-only MIR cleanup policy; keeps cleanup work separate from
    acceptance-shape, optimizer, and perf keeper changes
- `design/compiler-pipeline-thinning-ssot.md`
  - compiler pipeline thinning execution order; keeps semantic refresh,
    optimizer, verifier, and JoinIR thinning as facade/boundary cleanup before
    any behavior merge
- `design/joinir-target-lowerer-thinning-ssot.md`
  - JoinIR target-specific lowerer thinning order; keeps shared seams,
    LowerOnly observation, and route-specific behavior separated
- `design/loop-update-analyzer-thinning-ssot.md`
  - BoxShape-only cleanup order for JoinIR carrier-update observation; keeps
    accepted update shapes unchanged while tests/helpers are split
- `design/loop-body-local-init-thinning-ssot.md`
  - BoxShape-only cleanup order for JoinIR body-local init lowering; keeps
    expression support and receiver lookup order unchanged
- `design/inline-boundary-builder-thinning-ssot.md`
  - BoxShape-only cleanup order for JoinInlineBoundary construction; keeps
    builder defaults and ParamRole routing unchanged
- `design/generic-case-a-trim-thinning-ssot.md`
  - BoxShape-only cleanup order for FuncScanner trim JoinIR lowering; keeps
    trim shape, ValueId ranges, and whitespace semantics unchanged
- `design/user-method-policy-thinning-ssot.md`
  - BoxShape-only cleanup order for JoinIR user-defined static method policy;
    keeps allow-list truth and unknown-box fail-fast behavior unchanged
- `design/kernel-replacement-axis-ssot.md`
  - `K-axis` / artifact / task-placement vocabulary owner
- `design/substrate-capability-ladder-ssot.md`
  - allocator / collection / runtime substrate capability ladder owner
- `design/selfhost-lift-boundary-and-task-order-ssot.md`
  - parent decision for what moves upward into `.hako` or MIRBuilder versus
    what stays substrate; orders BoxCallable, collection visible semantics,
    concurrency semantics, and Arc retirement work
- `design/selfhost-program-json-boundary-vocabulary-ssot.md`
  - daily selfhost vocabulary owner; keeps Program(JSON v0) as the single
    current boundary, reads stage0/stage1 through that boundary, and quarantines
    stage2 / K-axis wording as roadmap or historical vocabulary for daily work
- `design/decoded-utf8-byte-length-contract-v0.md`
  - RHako/HHako SnapshotV0 decoded UTF-8 byte-count authority, internal
    capability boundary, independent parity task order, and retirement owner
- `design/hako-alloc-policy-state-contract-ssot.md`
  - allocator policy/state owner vs native metal keep stop-line owner
- `design/hako-thread-substrate-boundary-ssot.md`
  - `.hako` source-level concurrency, runtime ThreadApi substrate, and
    allocator pthread benchmark claim boundary owner
- `design/arc-retirement-and-ownership-substrate-ssot.md`
  - Arc retirement parent map; keeps RC insertion, ownership substrate,
    Box object model, TypeAbiCatalog, BoxCallableRegistry, and optional GC
    recipe responsibilities separated before any Arc replacement implementation
- `design/box-lifecycle-bprime-tombstone-adaptive-ownership-ssot.md`
  - B′ Shared/resource Box lifecycle constitution; separates explicit eager
    `fini`, Ownership SSA tokens, generation-tagged ObjectCell materialization,
    and later StaticUnique/LocalRc/SharedRc strategies; source owner/alias/share
    selection belongs to `docs/reference/language/ownership.md`
- `docs/reference/language/ownership.md`
  - normative source owner/ScopedAlias/AnchoredView/Shared contract; ordinary
    source stays sparse and only explicit `share` enters independent lifetime
- `investigations/hakorune-sparse-ownership-surface-task-2026-07-15.md`
  - parked bounded implementation board for evidence -> grammar -> Loan Flow
    -> first Box -> callable ABI/View -> explicit Shared; current D-prime lane
    remains unchanged and the next selected O2 row must be a generated artifact
- `investigations/hakorune-ownership-v2-root-anchored-alias-task-2026-07-14.md`
  - superseded detailed evidence/archive for root aliases, corpus censuses,
    fixtures, and historical alternatives; no longer an execution or source
    semantics authority
- `investigations/hakorune-ownership-v2-anchored-view-return-abi-task-2026-07-15.md`
  - parked P1 call-result-view subtask; keeps result type and ownership axes
    separate, seals explicit receiver/parameter anchors and conservative
    WholeObject invalidation first, and reserves field domains, temporary
    anchors, and ViewPhi as later rows
- `design/object-handle-box-identity-contract-ssot.md`
  - ARC-RETIRE-003 contract owner for `ObjectHandle`, `BoxIdentity`,
    generation, weak handles, root visibility, plugin instance mapping, and
    fini ownership
- `design/box-object-model-replacement-map-ssot.md`
  - ARC-RETIRE-005 contract owner for clone/share semantics, dyn dispatch and
    downcast surfaces, plugin lifecycle ownership, and `VMValue::BoxRef`
    carrier migration planning before any family Arc retirement gate
- `design/object-storage-plan-boundary-ssot.md`
  - exact-AOT object boundary owner; keeps MIRBuilder as source-semantics
    emitter only, RoutePlan as call execution truth, ObjectStoragePlan as
    representation truth, and backend lowering as the C-like consumer
- `design/compiler-object-final-shape-ssot.md`
  - selfhost-before-object-shape map; keeps MIRBuilder as meaning owner,
    SemanticRefresh as fact owner, RoutePlan as execution truth, ObjectPlan as
    representation/publication truth, backend as plan consumer, and runtime as
    generic fallback world
- `design/vm-active-lane-retirement-ssot.md`
  - VM active-lane retirement owner; keeps Rust VM as a small semantic
    reference subset, `.hako` VM as optional subset experiment, and moves
    product/app validation to EXE/AOT
- `design/build-crate-split-plan-ssot.md`
  - build-time crate split owner; stages `mir_core` growth before
    `hakorune-mir-plans`, backend, frontend, deep lowering, and runtime/boxes
    splits
- `design/selfhost-mir-object-metadata-ssot.md`
  - selfhost `.hako` MIRBuilder object metadata boundary; allows only
    source_span / receiver_origin / known_type_hint / field_key / call_site_id /
    newbox_origin and keeps representation/publication/backend route truth out
    of selfhost MIRBuilder
- `design/record-box-two-surface-one-substrate-ssot.md`
  - user-facing `record` / `box` boundary owner; keeps `record` as
    identity-free value aggregate, `box` as identity/behavior/lifecycle
    boundary, and allows both to share aggregate/object storage planning
    substrate internally
- `design/arc-retirement-family-gate-and-first-family-ssot.md`
  - ARC-RETIRE-006..018 contract owner for family retirement gates, first
    candidate selection, refcount storage prototype, atomic retain/release
    vocabulary, first host-handle text payload carrier cutover, and first text
    producer cutover
- `design/mimalloc-replacement-front-fidelity-ssot.md`
  - mimalloc fidelity guard for replacement-front execution shape; prevents
    accepting a fast but non-mimalloc-shaped allocator route as keeper
- `design/pure-first-mir-artifact-and-diagnostics-ssot.md`
  - pure-first/selfhost MIR artifact exactness, route preflight, and no-output diagnostics owner
- `design/hakorune-provider-package-abi-v1-future-ssot.md`
  - parked future provider package / DLL shared-library ABI owner; not part of current MIMAP-451A runner execution
- `lang/README.md`
  - source-root / logical-layer placement contract
- `lang/src/hako_alloc/README.md`
  - physical root contract for allocator policy/state modules
- `tools/smokes/v2/README.md`
  - smoke profile / suite placement contract
- `main/phases/**`
  - execution detail / blocker history / narrow ledgers

Rule:

- do not let `CURRENT_TASK.md` or `10-Now.md` regrow into landed-history ledgers.
- do not let `05-Restart-Quick-Resume.md` or `15-Workstream-Map.md` regrow into landed-history ledgers either.
- if a block already has a better owner, replace it with a short summary plus a link.
- keep the 800-line hard cap for source code files, not active docs.
- if an active entry/workstream/taskboard/design doc grows past roughly 1000
  lines, add a docs-slim task or archive split before adding more historical
  prose. Archive and investigation docs may remain long; active restart docs
  should point to them instead of duplicating them.
- normal card closeout should touch:
  - the active card
  - `CURRENT_STATE.toml` latest-card fields
  - code/test docs only when their contract changes
- current execution follows the single `active_lane` selected by
  `CURRENT_STATE.toml`; the current selection is MirBuilder in-place
  responsibility replacement
- Language v1, Ownership, selfhost, optimization, allocator, and
  representation work remain parked unless that pointer explicitly changes
- do not add a new phase row or one-off `.sh` for every inventory note; use the
  active working card and reusable lane guards first.
- update `AGENTS.md`, `CURRENT_TASK.md`, `10-Now.md`,
  `05-Restart-Quick-Resume.md`, phase README, taskboards, or ledgers only when
  lane / blocker / restart order / phase status / durable policy changes.
- `phases/README.md` is an index, not a full chronology.
- archive historical docs per area:
  - `docs/development/current/main/design/archive/`
  - `docs/development/current/main/phases/archive/`
- when physically moving a doc, keep a short stub at the old path with:
  - `Status: Historical`
  - `Moved to: ...`
  - optional pointer to the current owner

## ディレクトリの役割（推奨）

### `docs/development/current/main/`（入口・現状）

ここは「まず読む」入口を置く場所。SSOT を全部ここに置かない。

- 入口（例）: `00-Overview.md`, `01-JoinIR-Selfhost-INDEX.md`, `10-Now.md`, `20-Decisions.md`, `30-Backlog.md`
- `CURRENT_TASK.md` は root の machine-readable anchor、`10-Now.md` は docs 側の薄い mirror/dashboard。

### `docs/development/current/main/design/`（設計図・SSOT寄り）

設計の SSOT / 長期参照の設計図を置く。

- 原則: Phase 依存のログ/作業記録は置かない（それは phases へ）。
- 例: JoinIR の設計、Boundary/ExitLine の契約、Loop パターン空間、runtime/box 解決の地図。
- よく参照する設計SSOT:
  - Join-Explicit CFG Construction（north star）: `docs/development/current/main/design/join-explicit-cfg-construction.md`
  - EdgeCFG Flow Fragments（Structured→CFG lowering SSOT）: `docs/development/current/main/design/edgecfg-fragments.md`
  - Hotline CoreMethodContract（zero-cost hot-line / method contract migration SSOT）: `docs/development/current/main/design/hotline-core-method-contract-ssot.md`
  - Box Callable Registry:
    `docs/development/current/main/design/box-callable-registry-ssot.md`
  - Type ABI Naming / BoxDescriptor:
    `docs/development/current/main/design/type-abi-naming-and-box-descriptor-ssot.md`
  - Substrate Capability Ladder（allocator/collection/runtime substrate parent SSOT）: `docs/development/current/main/design/substrate-capability-ladder-ssot.md`
  - Selfhost Lift Boundary / Task Order（`.hako` / MIRBuilder / substrate 仕分け親SSOT）: `docs/development/current/main/design/selfhost-lift-boundary-and-task-order-ssot.md`
  - Hako Alloc Policy/State（allocator policy/state stop-line SSOT）: `docs/development/current/main/design/hako-alloc-policy-state-contract-ssot.md`
  - Hako Thread Substrate Boundary（source concurrency / ThreadApi substrate / pthread benchmark claim boundary）: `docs/development/current/main/design/hako-thread-substrate-boundary-ssot.md`
  - Hako Inspect Scope Dump（source anchors / MIR / LLVM IR / assembly inspect artifact boundary）: `docs/development/current/main/design/hako-inspect-scope-dump-ssot.md`
  - Substring Concat Closed-Form Lowering（StableLengthScalar exact route lowering residual）: `docs/development/current/main/design/substring-concat-len-closed-form-lowering-ssot.md`
  - Mimalloc Replacement-Front Fidelity（mimalloc-shaped execution keeper guard）: `docs/development/current/main/design/mimalloc-replacement-front-fidelity-ssot.md`
  - MIR FastMem MemOp Dialect（FastMemory MIR representation boundary）: `docs/development/current/main/design/mir-fastmem-memop-dialect-ssot.md`
  - Typed Object Exact Slot ABI（compat field route vs exact slot route boundary）: `docs/development/current/main/design/typed-object-exact-slot-abi-ssot.md`
  - Type ABI View / PlanStamp:
    `docs/development/current/main/design/type-abi-view-and-plan-stamp-ssot.md`
  - Type ABI Catalog Planning Spine:
    `docs/development/current/main/design/type-abi-catalog-planning-spine-ssot.md`
  - Type ABI Box Domain:
    `docs/development/current/main/design/type-abi-box-domain-ssot.md`
  - Arc Retirement / Ownership Substrate:
    `docs/development/current/main/design/arc-retirement-and-ownership-substrate-ssot.md`
  - MIR Cleanup Policy:
    `docs/development/current/main/design/mir-cleanup-policy-ssot.md`
  - Compiler Pipeline Thinning:
    `docs/development/current/main/design/compiler-pipeline-thinning-ssot.md`
  - Pure-First MIR Artifact / Diagnostics（selfhost/pure-first artifact exactness + preflight SSOT）: `docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md`
  - Hakorune Provider Package ABI v1（future DLL/shared-library provider package SSOT）: `docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md`

### `docs/development/current/main/design/archive/`（historical design）

歴史化した設計メモ・移行 ledger を置く。

- current owner ではない historical docs を移す。
- repository内参照を同時更新できる場合は旧パスを削除する。
- stable external entrypointを維持する必要がある場合だけshort stubを残す。
- curated top からは外すが、traceability は保持する。

### `docs/development/current/main/investigations/`（調査ログ）

再生成不能な不具合証拠、独立した再現、または current workstream card
に収まらない明示例外を置く。

- 原則: consultation / design stop / selection / execution / closeout は
  active workstream card の状態として更新し、状態遷移ごとに新しい
  investigation file を作らない。
- 原則: ordinary row/cell の investigation file delta は 0。
- 新規ファイルは `Exception:` と `ParentCurrentCard:` を明記する。
- 原則: “結論” は該当 design/reference SSOT または workstream card に
  反映し、調査ログ自体は authority にしない。
- 原則: 調査ログを SSOT にしない（参照元を明記して“歴史化”できる形にする）。
- よく参照する調査ログ:
  - Phase 259: block-parameterized CFG / ABI/contract 相談パケット: `docs/development/current/main/investigations/phase-259-block-parameterized-cfg-consult.md`
  - Mimalloc current workstream historical log:
    `docs/development/current/main/investigations/mimalloc-current-history-2026-06-02.md`

### `docs/development/current/main/workstreams/`（active work cards）

数日〜1週間の作業単位を置く。row/card を毎回増やさず、current work
の checklist / short evidence / decisions / parking lot を 1 枚に集約する。

- Workstream Card は current work の作業台であって、長期 SSOT ではない。
- 設計の正解が変わる場合は `design/*.md` を直接更新する。
- 小さい棚卸しや guard 文言修正は Ghost Task として commit message に残す。
- 新しい numbered row は lane 変更、実装境界変更、durable keeper/nonkeeper、
  新 contract / ABI / verifier / measurement policy の時だけ作る。

### `docs/development/current/main/phases/`（Phaseログ）

Phase ごとの記録・完了サマリ・実装チェックリストを置く。

- 推奨構造:
  - `docs/development/archive/phases/phase-131/`
  - `docs/development/archive/phases/phase-131/131-03-llvm-lowering-inventory.md`
  - `docs/development/archive/phases/phase-131/131-11-case-c-summary.md`

### `docs/development/current/main/phases/archive/`（transitional historical fronts）

既存のhistorical phase front互換置き場。新規archive先には使わない。

- current active phase front は `phase-*/README.md` に残す。
- 新しいhistorical phase移動先は
  `docs/development/archive/phases/<phase>/`。
- このtransitional rootはreference-closed batchで順次drainする。

### `docs/development/current/main/phases/phase-293x/archive/`（transitional phase-local archive）

phase-293x numbered-card archive prep lives here while the active phase remains
in `phase-293x/README.md`. 新規の一般archive authorityではない。

- card archive manifest:
  `docs/development/current/main/phases/phase-293x/archive/cards/phase-293x-card-archive-manifest.md`
- physical card moves require guard-reference decoupling or atomic tracked
  reference rewrites. Stubはstable external entrypointだけに限定する。

### `docs/development/current/main/phases/phase-294x/`（usize active phase）

- current taskboard:
  `docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md`
- landed field-group ledger index:
  `docs/development/current/main/phases/phase-294x/294x-usize-field-group-ledger.md`
- split detailed field-group ledgers:
  `294x-usize-field-group-ledger-084-139.md` and
  `294x-usize-field-group-ledger-140-196.md`

Rule:

- keep `294x-90-usize-semantics-taskboard.md` under the active queue/current
  blocker shape; do not append the full landed field-group chronology there.
- add long landed field-group summaries to the split ledgers only when they are
  useful for handoff/debugging.

### `docs/private/development/current/main/`（private canonical）

公開したくない計画本文・作業メモの正本を置くローカル領域。

- public 側には同名 path の stub を残し、`Private Canonical Path` を明記する。
- machine parse される anchor 文書（`CURRENT_TASK.md` など）は public 側に残す。
- 境界SSOT: `docs/development/current/main/design/private-doc-boundary-migration-ssot.md`

## ドキュメントの種別（ファイル先頭に明記）

追加/更新する文書の先頭に、最低限これを付ける。

```
Status: SSOT | Active | Historical
Scope: ...
Related:
- <入口/SSOT>
```

- `SSOT`: 現行の正本（同じテーマの“別名ファイル”を増やさない）。
- `Active`: 現行だが SSOT ではない（実装の手順書/チェックリスト等）。
- `Historical`: 参照用（当時の調査・ログ）。入口や Now から “歴史” としてリンクする。

## 移行ポリシー（リンク切れ防止）

既存ファイルは clean worktree から reference-closed なbounded batchで
移動する。repository内参照を同一commitで更新できる場合は旧パスへ
stubを残さない。stable external entrypointを移せない場合だけ、短い
転送stubを残す。current pointer targetは移動もstub化もしない。

例（旧ファイルの内容を最小化）:

```
# Moved

Status: Historical
Moved to: docs/development/archive/phases/phase-131/131-03-llvm-lowering-inventory.md
```

## 命名（推奨）

- Phase 文書: `phase-<N>/` + `<N>-<NN>-<topic>.md`（同一フェーズ内で並べ替えが自然）
- 調査ログの明示例外: `<topic>-investigation-YYYY-MM-DD.md` など
  （`Exception:` / `ParentCurrentCard:` 必須）
- 入口/SSOT: “Phase番号を入れない” ことを基本にする（寿命が長いので）

## 運用の最小ルール

- 新しい Phase 文書は `main/phases/` に入れる（`main/` 直下に増やさない）。
- 設計図（SSOT）は `main/design/` に寄せる（Phase の完了サマリと混ぜない）。
- `10-Now.md` は「現状の要約＋正本リンク」に徹し、詳細ログの本文は抱え込まない。
- `CURRENT_TASK.md` は root anchor なので、重要な blocker / current priority はまずそこへ置く。
- `05-Restart-Quick-Resume.md` は restart 手順と読む順だけに徹し、landed chronicle は抱え込まない。
- `15-Workstream-Map.md` は rough order の one-screen mirror に徹し、phase detail は抱え込まない。
- `phases/README.md` は current / guardrail / recent landed の index に徹し、repo-wide landed ledger を再掲しない。
- historical phase fronts are archived under `docs/development/current/main/phases/archive/`.
- current active phase fronts are linked from `CURRENT_TASK.md` and `15-Workstream-Map.md`.
