---
Status: Ready
Scope: code+tests+docs（仕様不変）
Related:
  - docs/development/current/main/phases/phase-29ao/README.md
  - docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
  - docs/development/current/main/design/coreplan-skeleton-feature-model.md
  - src/mir/builder/control_flow/plan/composer/mod.rs
  - src/mir/builder/control_flow/joinir/route_entry/router.rs
---

# Phase 29ao P30: Shadow adopt の “Facts→CorePlan” 入口を composer に集約（SSOT）

Date: 2025-12-30  
Status: Ready for execution  
Goal: 現在 Normalizer に散っている `*_from_facts`（shadow adopt 用の Facts→CorePlan 入口）を `plan/composer` へ移し、**Facts→CorePlan の SSOT 入口を1箇所**に収束させる。挙動は不変（strict/dev も含む）。

## 背景

- 現状:
  - DomainPlan→CorePlan の SSOT は `PlanNormalizer::normalize(domain_plan, ...)`
  - strict/dev shadow adopt は `router.rs` から `PlanNormalizer::*_from_facts(...)` を直接呼んでいる（Pattern2/3/5/6/7 など）
- しかし “Facts→CorePlan” は本来 Normalizer の責務ではなく、`composer`（合成入口）に寄せた方が構造が綺麗になる。

P30 は **責務位置を正すリファクタ**で、意味論は一切変えない。

## 非目的

- DomainPlan の撤去
- Facts の拡張
- strict/dev の採用条件変更
- ログ/タグの追加変更（既存タグは維持）

## 実装方針（構造で解く）

### 1) composer に “shadow adopt 専用 API” を追加

対象:
- `src/mir/builder/control_flow/plan/composer/mod.rs`

追加（推奨）:
- `src/mir/builder/control_flow/plan/composer/shadow_adopt.rs`

API 方針:
- 文字列による by-name 分岐は禁止
- router 側は `DomainPlan` variant を既に持っているので、**route-family ごとの関数**で受ける（曖昧さ排除）

例:
```rust
pub(in crate::mir::builder) fn compose_coreplan_for_split_scan(
    builder: &mut MirBuilder,
    facts: &CanonicalLoopFacts,
    ctx: &LoopRouteContext,
) -> Result<Option<CorePlan>, String>
```

実装:
- “薄い変換” + `PlanNormalizer::normalize` の再利用だけにする（ロジック再実装禁止）
  - facts から対応する `DomainPlan::*` を組み立てる
  - `PlanNormalizer::normalize(builder, domain_plan, ctx)` を呼ぶ
  - facts 不在なら `Ok(None)`

対象パターン（現状の shadow adopt 対象）:
- LoopSimpleWhile: 既に `normalize_loop_skeleton_from_facts` があるが、router 側の入口を composer 側に寄せてもよい
- Pattern2 subset: `facts.facts.pattern2_break`
- IfPhiJoin: `facts.facts.if_phi_join`
- Pattern5: `facts.facts.pattern5_infinite_early_exit`
- Pattern6 subset: `facts.facts.scan_with_init`
- Pattern7: `facts.facts.split_scan`

### 2) router の shadow adopt は composer を呼ぶだけにする

対象:
- `src/mir/builder/control_flow/joinir/route_entry/router.rs`

変更:
- `PlanNormalizer::*_from_facts` の呼び出しを、composer の `compose_coreplan_for_*` 呼び出しに差し替える
- strict/dev の fail-fast 条件・タグ出力・planner由来ゲート条件はそのまま維持

### 3) Normalizer から `*_from_facts` を撤去（or 最小 shim）

対象:
- `src/mir/builder/control_flow/plan/normalizer/{loop_break.rs,simple_while_coreloop_builder.rs,...}`

方針（どちらか）:
- A: `*_from_facts` を削除し、router/composer に集約（推奨）
- B: 互換のため残すが、中身は composer を呼ぶだけ（shim）にする

この時点で「Facts→CorePlan の SSOT 入口」は composer として明確になる。

## テスト（必須）

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

（任意）composer の境界ユニットテスト:
- facts が `None` のとき `Ok(None)`
- facts が `Some` のとき `Ok(Some(_))` まで到達（builder の variable_map を最小で埋める）

## docs 更新（追跡）

- `docs/development/current/main/phases/phase-29ao/README.md`（P30追加、Next更新）
- `docs/development/current/main/10-Now.md` / `docs/development/current/main/30-Backlog.md` / `CURRENT_TASK.md`
- `docs/development/current/main/design/coreplan-migration-roadmap-ssot.md`（Current/Next更新）

## コミット

- `git add -A`
- `git commit -m "phase29ao(p30): move shadow adopt composer ssot"`
