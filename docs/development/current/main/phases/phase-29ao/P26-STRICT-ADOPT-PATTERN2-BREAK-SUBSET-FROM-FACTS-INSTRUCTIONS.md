---
Status: Ready
Scope: code+tests+docs（strict/dev のみ、仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/phases/phase-29ae/README.md
  - tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh
  - tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh
  - src/mir/builder/control_flow/joinir/patterns/router.rs
  - src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs
  - src/mir/builder/control_flow/plan/facts/pattern2_break_facts.rs
---

# Phase 29ao P26: strict/dev Pattern2(Break) “plan subset” を Facts→CorePlan で shadow adopt

Date: 2025-12-30  
Status: Ready for execution  
Goal: Pattern2（conditional break）のうち **Facts が完全に表現できる最小 subset** だけを strict/dev で Facts→CorePlan に寄せ、DomainPlan 経路との差分（facts/extractor/normalize のズレ）を早期検知できるようにする（release 既定挙動は不変）。

## 背景

- Pattern2 は real-world 形状（LoopBodyLocal promotion など）も含むため、いきなり “全 Pattern2 を adopt 強制” すると strict/dev での fail-fast が過剰になりやすい。
- 一方で、`Pattern2BreakFacts` の subset は既に SSOT として存在し、fixture/smoke もある。
- そこで P26 は **subset だけ**を “planner 由来の DomainPlan のときに限って” shadow adopt し、段階的に CorePlan 合成へ寄せる。

## 非目的

- Pattern2 全体（LoopBodyLocal promotion を含む）を Facts→CorePlan に強制する
- Facts subset 拡張（Phase 29ao の次段で扱う）
- 新しい env var/恒常ログ追加
- release 既定経路の変更

## 実装方針

### 1) Facts→CorePlan の入口を PlanNormalizer に追加（Pattern2 subset）

対象:
- `src/mir/builder/control_flow/plan/normalizer/mod.rs`
- `src/mir/builder/control_flow/plan/normalizer/pattern2_break.rs`

追加:
- `pub(in crate::mir::builder) fn normalize_pattern2_break_from_facts(...) -> Result<Option<CorePlan>, String>`

仕様:
- `CanonicalLoopFacts.facts.pattern2_break` が `Some` のときだけ `Some(CorePlan)` を返す
- それ以外は `Ok(None)`（fallback維持）
- 実装は “薄い変換” のみ:
  - `Pattern2BreakFacts -> Pattern2BreakPlan` を機械的に詰め替える
  - `promotion` は `facts.facts.pattern2_loopbodylocal` があれば同様に乗せる（ただし P26 の gate は subset fixture）
  - 既存の `normalize_pattern2_break(builder, Pattern2BreakPlan, ctx)` を呼ぶ
  - Pattern2 のロジックは再実装しない（SSOTを増やさない）

### 2) router の strict/dev shadow adopt を Pattern2 subset に追加（planner 由来のみ）

対象:
- `src/mir/builder/control_flow/joinir/patterns/router.rs`

方針:
- strict/dev でも **Pattern2 全体には強制しない**
- 条件:
  - 選ばれた `domain_plan` が `DomainPlan::Pattern2Break(_)`
  - かつ `outcome.plan` が `Some(DomainPlan::Pattern2Break(_))`（planner が作った subset であることの判定）
- 上記を満たすときだけ adopt:
  - `facts.facts.pattern2_break` が `Some` であることを assert（fail-fast）
  - `PlanNormalizer::normalize_pattern2_break_from_facts(...)` を呼び、`Some(CorePlan)` を要求（fail-fast）

これにより、
- LoopBodyLocal promotion 等の “facts未対応の Pattern2” は、従来どおり DomainPlan 経路で動く（strict/dev の過剰failを避ける）
- subset は strict/dev で Facts→CorePlan に寄る（段階移行）

### 3) 回帰ゲート（SSOT）に subset smoke を追加

P26 の adopt 経路は、既存の phase29ab_pattern2_*（LoopBodyLocal含む）では踏まれない可能性があるため、subset 固定の smoke を gate に追加して “必ず踏む” を SSOT 化する。

対象の既存 smoke:
- `tools/smokes/v2/profiles/integration/apps/archive/phase29ai_pattern2_break_plan_subset_ok_min_vm.sh`（strict、exit=15）

追加:
- `tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` に filter を 1 行追加:
  - `run_filter "pattern2_subset" "phase29ai_pattern2_break_plan_subset_ok_min_vm"`
- `docs/development/current/main/phases/phase-29ae/README.md` の Regression pack 項目へ追記

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## docs 更新

- `docs/development/current/main/phases/phase-29ao/README.md`（P26 追加、Next 更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next 更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p26): strict/dev adopt pattern2 break subset from facts"`
