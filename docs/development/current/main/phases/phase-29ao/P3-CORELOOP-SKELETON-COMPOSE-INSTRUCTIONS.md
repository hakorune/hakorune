---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P3: CoreLoop skeleton を CorePlan で直接生成（DomainPlan 非依存・未接続）

Date: 2025-12-30  
Status: Ready for execution  
Scope: 仕様不変（未接続のまま）、CorePlan 合成を “DomainPlan 経由” から 1段剥がす

## 目的

- composer が `CanonicalLoopFacts` から `CorePlan::Loop`（CoreLoop skeleton）を直接生成できる導線を作る。
- P2 までの `CanonicalLoopFacts → DomainPlan → PlanNormalizer → CorePlan` から、DomainPlan 依存を 1つ減らす。
- まだ Skeleton+Feature 合成の完成形へ飛ばず、まずは **Pattern1 subset のみ**で “direct CorePlan” を SSOT として固定する。

## 非目的

- 既存ルーティングの切替（P3 は未接続）
- Pattern2/3/4/5/6/7/8/9 の direct 化（P4+）
- 新 env var / 恒常ログ追加
- by-name 分岐の追加（Facts の Option を使うだけ）

## 重要な設計判断（P3の境界）

- `BasicBlockId/ValueId/Frag` の allocation は **Tier 1（Normalizer）側**に置いたままにする。
  - composer が builder を直接触って “独自の allocation 規約” を作らない（SSOTが割れるのを防ぐ）。
- “direct” の意味は **DomainPlan enum に依存しない**こと（Pattern固有 knowledge は最小化し、将来は Skeleton+Feature 合成へ寄せる）。

## 実装手順

### Step 1: Normalizer に skeleton builder を追加（Pattern1 subsetのみ）

追加（推奨）:
- `src/mir/builder/control_flow/plan/normalizer/skeleton_loop.rs`

配線:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs` に `mod skeleton_loop;` を追加

実装:
- `impl PlanNormalizer { pub(in crate::mir::builder) fn normalize_loop_skeleton_from_facts(builder: &mut MirBuilder, facts: &CanonicalLoopFacts, ctx: &LoopPatternContext) -> Result<Option<CorePlan>, String> }`

条件（subset、誤マッチ防止最優先）:
- `facts.skeleton_kind == SkeletonKind::Loop` のみ対象、それ以外は `Ok(None)`
- `facts.facts.pattern1_simplewhile.is_some()` のときだけ `Ok(Some(CorePlan::Loop(..)))`
- それ以外は `Ok(None)`

生成する CoreLoop skeleton（最小で正しい）:
- ブロック: Standard 5（preheader/header/body/step/after）
- PHI: loop_var の 1本（header）
- frag:
  - `frag.entry == header_bb`
  - header の Branch（then=body, else=after）
  - wires: body→step, step→header
  - exits: 空
- body: 空（effect-only 制約と整合）
- cond_loop/cond_match: ValueId を割り当て、Pattern1 では `cond_match = cond_loop` でOK

利用してよい既存 SSOT:
- `LoopBlocksStandard5::allocate(builder)`
- `create_phi_bindings(..)`
- `PlanNormalizer::lower_compare_ast(..)` / `lower_binop_ast(..)`（normalizer 内で完結させる）

### Step 2: composer から direct skeleton を呼ぶ API を追加

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

追加（案）:
- `pub(in crate::mir::builder) fn try_compose_core_plan_direct(builder: &mut MirBuilder, facts: &CanonicalLoopFacts, ctx: &LoopPatternContext) -> Result<Option<CorePlan>, String>`

処理:
- `PlanNormalizer::normalize_loop_skeleton_from_facts(builder, facts, ctx)` を呼ぶだけ

注意:
- 既存の `try_compose_core_plan_via_normalizer`（DomainPlan 経由）は残す（比較用・P4以降の移行用）。

### Step 3: unit test（direct path の Some/None 境界 + verify 固定）

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`（tests 追記）

追加テスト:
1) Pattern1 facts あり → `try_compose_core_plan_direct(..)` が `Some(CorePlan::Loop(_))`
2) Pattern1 facts なし → `None`
3) `PlanVerifier::verify(&core)` が通る（局所不変条件が満たされることを固定）

builder の事前条件:
- loop_var 初期値が `builder.variable_ctx.variable_map` に入っていること（Pattern1 normalizer と同条件）

### Step 4: docs 更新

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P3 指示書リンク追加）
- `CURRENT_TASK.md`（次の指示書を P3 に更新）

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p3): direct coreloop skeleton from facts (pattern1 subset)"`

## 次（P4の入口）

P4 では ExitMap presence（break/continue/return）を CorePlan/Frag の語彙へ寄せる。
Pattern2/4/5 を “別pattern” にせず feature 合成へ落とし始める（SSOT: skeleton/feature model）。
