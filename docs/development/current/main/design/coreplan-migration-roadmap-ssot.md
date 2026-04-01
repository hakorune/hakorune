---
Status: SSOT
Scope: JoinIR → PlanFrag → CorePlan への移行道筋（仕様不変で完了と言える条件）
Related:
- docs/development/current/main/10-Now.md
- docs/development/current/main/30-Backlog.md
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-unknown-loop-strategy-ssot.md
- docs/development/current/main/design/match-branchn-skeleton-ssot.md
- docs/development/current/main/design/return-in-loop-minimal-ssot.md
- docs/development/current/main/design/post-phi-final-form-ssot.md
- docs/development/current/main/design/effect-classification-ssot.md
- docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md
- docs/development/current/main/phases/phase-29ae/README.md
---

# CorePlan Migration Roadmap (SSOT)

目的: “JoinIR を CorePlan で組み立てる” 状態へ、仕様不変（release既定）で段階的に収束するための道筋を 1 枚に固定する。

## 0. 前提（守ること）

- 既定挙動不変（release の意味論/エラー文字列/恒常ログを変えない）
- silent fallback 禁止（strict/dev では Freeze/Fail-Fast で検出可能にする）
- by-name ハードコード禁止（構造条件・SSOT境界で解く）
- JoinIR integration gate（SSOT）を常に緑維持:
  - `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## 1. “CorePlan で組み立てる” の定義

ここでの完了は「legacy numbered labels の列挙が消えた」ではなく、以下が成立していること。

- 構造SSOTは `CorePlan`（emit/merge は CorePlan/Frag 以外を再解析しない）
- `Facts → Recipe → Verifier → Parts/Lower → emit → merge` が主経路
- historical planner-payload wording は移行履歴としてのみ残し、現行 contract は Recipe/Verifier 境界で固定する

## 1.1 Current (active)

Active phase: Phase 29bq（selfhost canary / expressivity-first）
Next step (planned): TBD

Within Phase 29bq (expressivity-first):
- `StepPlacement/StepMode`（InlineInBody, strict/dev only）で “step を動かさない” ループを受理し、selfhost canary の freeze を compiler 側で unblock する（no rewrite）。
  - SSOT: `docs/development/current/main/design/coreloop-stepmode-inline-in-body-ssot.md`

Recent (done):
- Plan decomposition: `generic_loop` / `scan_with_init` / `split_scan` / `loop_true_early_exit` / `loop_true_break_continue` / `loop_cond` は skeleton+pipeline+feature helper へ移行済み（legacy numbered labels 依存を増やさず、release 既定不変、gate green 維持）。

Candidate next (after selfhost canary / Stage-1):
- Remaining compatibility-lane normalizers の lego-ization（pipeline/skeleton/feature 化）: `src/mir/builder/control_flow/plan/REGISTRY.md` の “Remaining legacy normalizers” table を SSOT として進める
- Cleanup foundation: `CleanupWrap` + cleanup region boundary / `Seq(Block)` の SSOT 固定（selfhost 移植時の負債化を防ぐ）
  - SSOT: `docs/development/current/main/design/cleanupwrap-cleanup-region-boundary-ssot.md`
- `StepPlacement/StepMode::InlineInBody` の一般化（strict/dev only, no rewrite）


## 2. すでに固めた SSOT（再発防止の土台）

- Skeleton/Feature: `docs/development/current/main/design/coreplan-skeleton-feature-model.md`
- post-phi final form: `docs/development/current/main/design/post-phi-final-form-ssot.md`
- effect classification: `docs/development/current/main/design/effect-classification-ssot.md`
- cleanup contract: `docs/development/current/main/design/exitkind-cleanup-effect-contract-ssot.md`
- composer v0/v1 boundary: `docs/development/current/main/design/coreloop-composer-v0-v1-boundary-ssot.md`
- done criteria: `docs/development/current/main/design/coreplan-migration-done-criteria-ssot.md`

## 3. 移行タスクの流れ（安全順）

### 3.1 Ordered checklist (Rust-layer migration)

進捗はこの順でチェックを付ける（SSOT）。  
※ 状態の更新は `docs/development/current/main/10-Now.md` と併用する。

- [x] A0. JoinIR regression pack green (`phase29ae_regression_pack_vm.sh`) — evidence: `10-Now.md` L68 — role: baseline establishment
- [x] A1. CorePlan 語彙の lower/verify 最小セットが green（Phase 29am 完了条件） — evidence: `phases/phase-29am/README.md` (Status: Complete)
- [ ] B1. Remaining compatibility-lane normalizers を skeleton+feature へ移行 — evidence: `src/mir/builder/control_flow/plan/REGISTRY.md` L106-127 ("Remaining legacy normalizers" table)
- [x] B2. Facts/Recipe→CorePlan 合成入口を 1 箇所に固定（Phase 29ao） — evidence: `docs/development/current/main/phases/archive/phase-29ao/README.md` (Status: Closeout)
- [ ] C1. Planner の骨格一意化が strict/dev で fail-fast する（曖昧形は Freeze） — evidence: `single_planner/` — done: `Ok(None)` silent fallback = 0 in strict mode
- [ ] D1. Normalizer が「合成だけ」に縮退（再解析なし） — evidence: `plan/lowerer/` — done: `rg "ASTNode::" plan/normalizer/` = 0
- [ ] E1. compatibility fallback を 0 にする（planner-first only） — evidence: `REGISTRY.md` L106-127 — done: "Remaining legacy normalizers" table が空
- [x] F1. JoinIR integration gate green（回帰SSOT維持） — evidence: `phase-29ae/README.md` L75 + `10-Now.md` L82 — role: continuous maintenance
- [x] G1. Stage0→Stage1→Stage2 identity tests green — evidence: `selfhost-bootstrap-route-ssot.md` L122-136 — gate: `tools/selfhost_identity_check.sh --mode full` (smoke は参考) — latest: `--skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` PASS（2026-02-11）
- [x] G2. selfhost canary / E2E 実行の基準リスト更新 + green — evidence: `planner_required_selfhost_subset.tsv` + `CURRENT_TASK.md`（2026-02-08 `182/182`, `total_secs=682`） — gate: `SMOKES_ENABLE_SELFHOST=1 phase29bq_selfhost_planner_required_dev_gate_vm.sh`

### Step A: CorePlan 語彙の “穴” を埋める（lowerer/verifier）

狙い:
- `CorePlan` の構造ノードが単独でも lower/verify できる（未対応分岐を減らす）

典型:
- `CorePlan::If` / `CorePlan::Exit` の lowerer 対応
- verifier の不変条件を “局所検証” で完結させる

現状:
- ✅ Phase 29am（P0–P3）で “最低限 lower/verify できる語彙” を前倒しで固定済み

### Step B: Facts / Recipe を Skeleton+Feature の SSOTへ寄せる

狙い:
- traceability-only legacy numbered label を current route semantics の主語に戻さない
- `LoopSkeleton + ExitMap + ValueJoin + ...` の合成で表現できる状態に寄せる

やらない:
- Facts から emit/merge を助けるための再解析を前提にした “不足した Facts” を作る

直近の入口（SSOT）:
- Phase 29an: `docs/development/current/main/phases/archive/phase-29an/README.md`

### Step B.5: 合成入口（Facts/Recipe→CorePlan）を 1 箇所に固定（未接続）

狙い:
- Facts/Recipe→CorePlan 合成の入口を “1ファイル” に集約し、以後の実装を合成側へ閉じ込める

入口:
- Phase 29ao: `docs/development/current/main/phases/archive/phase-29ao/README.md`

### Step C: Planner を「骨格の一意化→特徴付与→Freeze」へ

狙い:
- planner境界は骨格の一意化に集中（現状は single-plan: 0/1 → None/Some）
- feature は “別ルートラベル” ではなく合成の材料として付与

### Step D: Normalizer を “合成だけ” にする

狙い:
- `Recipe / VerifiedRecipe` → `CorePlan` の純変換に収束
- join 入力（post-phi）と effect/cleanup 契約を壊さない

### Step E: 入口から legacy fallback を段階的に 0 へ

狙い:
- `single_planner` の legacy extractor fallback を削減し、planner-first を主にする
- JoinIR 側 wrapper/router を薄くし、PlanFrag 側 SSOT に寄せる

注意:
- “落ちる” を `Ok(None)` で隠さない（対象っぽいのに一意化できない場合は Freeze）

## 4. 完了（Done）の判定（設計上）

最低限の Done:
- JoinIR 回帰 SSOT が緑: `phase29ae_regression_pack_vm.sh`
- 回帰対象（現状の gate: loop_break / scan_with_init / split_scan smoke 群 + phase1883 + phase263）が “CorePlan 合成” 経路で通る

強い Done（段階2）:
- Facts が route-specific complete variants を増やさず Skeleton+Feature に寄っている
- scan/split/predicate 等の algorithm intent は Recipe/feature として表現され、historical planner-payload wording は移行履歴に限定される
- Freeze taxonomy が運用でぶれず、strict/dev の診断が安定タグで追える

## 5. 次の実装順（SSOTの "安全順"）

このドキュメントは「移行の定義 / 不変条件 / Done 判定」を SSOT として固定する。

- 実装順・進捗の SSOT: `docs/development/current/main/phases/archive/phase-29ao/README.md`
- Gate（SSOT）: `docs/development/current/main/phases/phase-29ae/README.md`

## 5.1 Post-Migration: CorePlan Role Shrink / Rename

### 背景

RecipeBlock dispatch 統一が完了すると、`CorePlan` の役割は以下に収束する:

- **意味論 SSOT**: `RecipeBlock` / `Recipe` / `Parts`（extractors/builders/lowerers）
- **機械的 SSOT**: `CorePlan`（emit/verify/merge のワイヤリングのみ）

つまり `CorePlan` は「意味を持たない配線表現」となり、名前が示唆する "Plan" としての責務は `Recipe` 側へ移る。

### Rename 候補

- `LoweredRecipe`: Recipe → 機械表現への最終形を示す
- `MirFragment`: MIR builder が扱う断片を示す
- `EmitPlan`: emit のみに使われることを示す

### いつやるか

- **Trigger**: features/ の `build_join_payload` 直接呼び出しが 0 件維持（dispatch 統一完了）
- **Gate**: drift check `rg -n "build_join_payload\\(" src/mir/builder/control_flow/plan/features/` が 0 件
- **Scope**: rename は 1 commit で閉じる（grep + sed / IDE refactor）

### 注意

- Rename は意味論不変（内部 API 変更のみ、release 既定に影響しない）
- SSOT ドキュメントの用語も同時更新（grep で漏れ確認）
