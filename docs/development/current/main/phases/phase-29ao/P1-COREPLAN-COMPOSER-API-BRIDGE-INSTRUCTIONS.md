---
Status: Ready
Scope: code（未接続の足場・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/design/post-phi-final-form-ssot.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P1: Composer API decision + bridge（Facts→CorePlan 合成の “allocation 境界” を固定）

Date: 2025-12-30  
Status: Ready for execution  
Scope: “未接続のまま” 合成の入口を育てる（既定挙動/ログ/エラー文字列は不変）

## 目的

- `CorePlan` が `BasicBlockId/ValueId/Frag` を要求するため、Facts→CorePlan 合成の設計で最大の論点である
  **「どこが allocation を持つか」** を、コード構造として SSOT に固定する。
- まずは bridge として composer が `CanonicalLoopFacts` から `DomainPlan`（既存語彙）を組み立て、
  `PlanNormalizer::normalize()` を呼べる形まで整える（P1 では未接続）。

## 非目的

- 既存ルーティング/観測/エラー文字列の変更
- composer の結果を本番経路で採用する（P1 は未接続）
- 新 env var 追加（strict/dev を含む既存導線は OK）
- by-name ハードコードの追加

## 設計（P1で固定する方針）

### 1) allocation は Normalizer に置く（現状維持）

- `BasicBlockId/ValueId/Frag` の生成は **Normalizer（Tier 1）** が担当する。
- composer は Facts から “合成に必要な決定” を行い、必要なら `DomainPlan` を組み立てて Normalizer に渡す。

理由:
- `CorePlan` は ValueId-only を維持する（String式禁止）。
- Facts/Planner が builder を直接触り始めると、層境界が崩れやすい（将来の verify/observability も難化）。

### 2) bridge は「DomainPlan を生成するだけ」（未接続）

- P1 では composer は `CorePlan` を直接構築しない。
- `CanonicalLoopFacts` から `DomainPlan` を “必要最小の subset” で組み立てる関数を用意する。

## 実装手順

### Step 1: composer に bridge API を追加

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

追加する関数（案）:

- `pub(in crate::mir::builder) fn try_compose_domain_plan_from_canonical_facts(facts: &CanonicalLoopFacts) -> Result<Option<DomainPlan>, Freeze>`

ルール（P1のsubset、誤マッチ防止を最優先）:
- `facts.skeleton_kind == SkeletonKind::Loop` のみ対象（それ以外は `Ok(None)`）
- `facts.facts.pattern*_...` の **既存 facts が Some の場合のみ** DomainPlan を構築する（再解析しない）
- `Ok(None)` と `Freeze` の境界は既存 planner と同じ運用（P1では Freeze を増やさない）

注意:
- P0 の `try_compose_core_plan_from_canonical_facts()` は未接続の足場として残して OK（P2以降で置換/削除検討）。

### Step 2: bridge で作る DomainPlan（P1は 1個だけ）

最初のターゲットは Pattern1（SimpleWhile）推奨:
- `facts.facts.pattern1_simplewhile.is_some()` のときだけ `DomainPlan::Pattern1SimpleWhile(...)` を構築
- Pattern1 の fields は facts から “そのまま” 移す（構造条件は facts 抽出側が SSOT）

理由:
- もっとも小さく、`PlanNormalizer` での割当/verify/emit が既に安定している。

### Step 3: unit test（bridge の境界を固定）

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

追加テスト（例）:
- Pattern1 facts があると `Ok(Some(DomainPlan::Pattern1SimpleWhile(_)))`
- pattern1 facts がないと `Ok(None)`（bridge が勝手に推論しない）

### Step 4: docs 更新（入口/Next を固定）

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P1 追加、Next を P2 へ）
- `docs/development/current/main/10-Now.md`（Next を P1 に）
- `docs/development/current/main/30-Backlog.md`
- `CURRENT_TASK.md`

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p1): composer domainplan bridge scaffold"`

## 次（P2の入口）

P2 では bridge で作った `DomainPlan` を Normalizer に通し、`CorePlan` を得る経路（未接続のまま）まで伸ばす。
その後、strict/dev のみで “CorePlan equivalence selfcheck” を追加し、徐々に採用へ移る。
