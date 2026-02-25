---
Status: Ready
Scope: code（未接続・仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - docs/development/current/main/phases/phase-29ae/README.md
---

# Phase 29ao P2: Composer → Normalizer bridge（CanonicalLoopFacts→DomainPlan→CorePlan、未接続）

Date: 2025-12-30  
Status: Ready for execution  
Scope: “CoreLoop skeleton を CorePlan で得る” 導線を **未接続のまま** 固定（既定挙動/ログ/エラー文字列は不変）

## 目的

- composer が `CanonicalLoopFacts` から `DomainPlan` を作れるようになった（P1）ので、次は
  `PlanNormalizer::normalize()` を通して **`CorePlan` を得る** bridge を用意する。
- “allocation は Normalizer が持つ” を現行構造のまま維持しつつ、composer 側に **CorePlan 合成の受け口**を作る。

## 非目的

- router / single_planner / joinir 経路の変更（P2 は未接続）
- Skeleton+Feature から直接 CorePlan を組む本実装（P3+）
- 新 env var 追加

## 実装方針（P2で固定する境界）

- **Facts/Planner/Composer は再解析しない**（facts の Option をそのまま使う）
- **allocation は Normalizer**（`BasicBlockId/ValueId/Frag` を composer は生成しない）
- `Ok(None)` は “対象外/未対応” として維持し、`Freeze` は P2 では増やさない

## 手順

### Step 1: composer に CorePlan bridge API を追加

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

追加する関数（案）:

- `pub(in crate::mir::builder) fn try_compose_core_plan_via_normalizer(builder: &mut MirBuilder, facts: &CanonicalLoopFacts, ctx: &LoopPatternContext) -> Result<Option<CorePlan>, String>`

処理:
1) `try_compose_domain_plan_from_canonical_facts(facts)` を呼ぶ  
2) `None` なら `Ok(None)`  
3) `Some(domain)` なら `PlanNormalizer::normalize(builder, domain, ctx)` を呼び `Ok(Some(core))`

注意:
- `Freeze` を `String` に変換する必要がある場合、**P2 では bridge 未接続**なので簡単に `freeze.to_string()` で良い
  （観測/エラー文字列の互換性要求は “接続時” に扱う）

### Step 2: unit test（Some/None 境界 + 最小 successful path）

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`（既存 tests へ追加）

最小成功条件（Pattern1 のみでOK）:
- builder に `loop_var` の初期値が入っている状態を作る（`builder.variable_ctx.variable_map.insert("i", <ValueId>)`）
- `LoopPatternContext::new(...)` を最小 AST で作る（debug=false でOK）
- `try_compose_core_plan_via_normalizer(...)` が `Ok(Some(CorePlan::Loop(_)))` を返すことを固定

境界テスト:
- Pattern1 facts が無いと `Ok(None)`（bridge は勝手に推論しない）

### Step 3: docs 更新

更新:
- `docs/development/current/main/phases/phase-29ao/README.md`（P2 指示書リンク、Next を P3 へ）
- `CURRENT_TASK.md`（P2 指示書パスへ）

## 検証（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## コミット

- `git add -A`
- `git commit -m "phase29ao(p2): composer coreplan via normalizer (unconnected)"`

## 次（P3 の入口）

P3 で “Skeleton+Feature から直接 CorePlan を合成する” に踏み込む。P2 の bridge は
「allocation 境界が崩れていない」ことを保証する保険として残してよい。
