# Docs Layout (SSOT)

Status: SSOT  
Scope: `docs/development/current/` 以下の「置き場所ルール」と、SSOT/履歴メモの混在を防ぐための最小ガイド。

## 目的

- 入口（読む順序）と、詳細（設計図/調査/Phaseログ）を分離して迷子を防ぐ。
- “Phase 文書が増えても” SSOT が埋もれないようにする。
- 大規模移動はしない（リンク切れ回避）。以後の追加分から秩序を作る。

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
- `design/current-docs-update-policy-ssot.md`
  - current docs update policy and mirror-thinning contract
- `design/current-docs-archive-policy-ssot.md`
  - archive buckets, landed ledger, and current-doc slimming contract
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
- `design/hako-alloc-policy-state-contract-ssot.md`
  - allocator policy/state owner vs native metal keep stop-line owner
- `design/hako-thread-substrate-boundary-ssot.md`
  - `.hako` source-level concurrency, runtime ThreadApi substrate, and
    allocator pthread benchmark claim boundary owner
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
- if an active entry/workstream/taskboard/design doc grows past roughly 1000
  lines, add a docs-slim task or archive split before adding more historical
  prose. Archive and investigation docs may remain long; active restart docs
  should point to them instead of duplicating them.
- normal card closeout should touch:
  - the active card
  - `CURRENT_STATE.toml` latest-card fields
  - code/test docs only when their contract changes
- current execution work stays in three buckets:
  - mimalloc migration and optimization
  - Array / representation fast paths only when selected by mimalloc perf evidence
  - docs and shell hygiene
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
  - MIR Cleanup Policy:
    `docs/development/current/main/design/mir-cleanup-policy-ssot.md`
  - Compiler Pipeline Thinning:
    `docs/development/current/main/design/compiler-pipeline-thinning-ssot.md`
  - Pure-First MIR Artifact / Diagnostics（selfhost/pure-first artifact exactness + preflight SSOT）: `docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md`
  - Hakorune Provider Package ABI v1（future DLL/shared-library provider package SSOT）: `docs/development/current/main/design/hakorune-provider-package-abi-v1-future-ssot.md`

### `docs/development/current/main/design/archive/`（historical design）

歴史化した設計メモ・移行 ledger を置く。

- current owner ではない historical docs を移す。
- 旧パスには short stub を残す。
- curated top からは外すが、traceability は保持する。

### `docs/development/current/main/investigations/`（調査ログ）

不具合調査のログ、切り分け、暫定メモを置く。

- 原則: “結論” は `10-Now.md` / `20-Decisions.md` / 該当 design doc に反映し、調査ログ自体は参照用に残す。
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
  - `docs/development/current/main/phases/phase-131/`
  - `docs/development/current/main/phases/phase-131/131-03-llvm-lowering-inventory.md`
  - `docs/development/current/main/phases/phase-131/131-11-case-c-summary.md`

### `docs/development/current/main/phases/archive/`（historical phase fronts）

closeout / accepted monitor-only / parked / superseded の phase front を置く。

- current active phase front は `phase-*/README.md` に残す。
- archived phase front は `phases/archive/<phase>/README.md` に移す。
- phase 配下の child docs は必要な限り元の場所に残してよい。

### `docs/development/current/main/phases/phase-293x/archive/`（phase-local execution archive）

phase-293x numbered-card archive prep lives here while the active phase remains
in `phase-293x/README.md`.

- card archive manifest:
  `docs/development/current/main/phases/phase-293x/archive/cards/phase-293x-card-archive-manifest.md`
- physical card moves require either guard-reference decoupling or forwarding
  stubs at the old paths.

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

既存のファイルは大量移動しない。移動が必要な場合は必ず旧パスに“転送スタブ”を残す。

例（旧ファイルの内容を最小化）:

```
# Moved

Status: Historical
Moved to: docs/development/current/main/phases/phase-131/131-03-llvm-lowering-inventory.md
```

## 命名（推奨）

- Phase 文書: `phase-<N>/` + `<N>-<NN>-<topic>.md`（同一フェーズ内で並べ替えが自然）
- 調査ログ: `<topic>-investigation-YYYY-MM-DD.md` など（時系列が分かる形）
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
