# Phase 29ai P7: Planner returns DomainPlan (remove duplicate Plan types) — Instructions

Status: Ready for execution  
Scope: 構造整理（仕様不変）

## Goal

Phase 29ai の `facts/normalize/planner` が “別系統の Plan 型” を持っている状態を解消し、既存の `plan::DomainPlan` を唯一の
Plan語彙（SSOT）として使う。

これにより、single_planner が将来 `facts→planner` を直接利用できるようになり、吸収作業（Pattern2/6/7/…）を一本道で進められる。

## Non-goals

- 既存の lowering 経路の変更（`single_planner` の legacy_rules は維持）
- 仕様変更（挙動/エラー文字列/ログの変更）
- 新しいトグル/環境変数の追加

## Current Problem

`src/mir/builder/control_flow/plan/planner/` に `Plan { kind: PlanKind }` が存在し、既存の `DomainPlan` と二重化している。

- DomainPlan はすでに Normalizer/Verifier/Lowerer の SSOT 語彙
- 29ai planner の “候補集合→一意化” は本来 DomainPlan の上で行うべき

## Target

- `CandidateSet::finalize() -> Result<Option<DomainPlan>, Freeze>`
- `planner::build_plan(...) -> Result<Option<DomainPlan>, Freeze>`
- `PlanKind` / `Plan`（29ai専用）を撤去し、候補は `DomainPlan` を直接保持する

## Implementation Steps

1) 29ai planner の型を DomainPlan に統一
   - `src/mir/builder/control_flow/plan/planner/mod.rs`:
     - `Plan` / `PlanKind` を削除
     - `pub use freeze::Freeze` のみ残す
     - `pub use build::build_plan` は `Result<Option<DomainPlan>, Freeze>` へ

2) CandidateSet の candidate を DomainPlan へ
   - `PlanCandidate { plan: DomainPlan, rule: &'static str }`
   - `finalize()` の 0/1/2+ 境界（SSOT）は維持

3) build.rs の最小 rule を DomainPlan で表現
   - 現状の “ScanWithInit” placeholder は `DomainPlan::ScanWithInit(ScanWithInitPlan{...})` の形へ
   - ただし P7 では実行経路に接続しないため、**未到達**でもコンパイルできるようにする
     - 例: facts が `Ok(None)` の間は build_plan が `Ok(None)` を返す

4) docs の参照整合
   - Freeze tag は `docs/development/current/main/design/planfrag-freeze-taxonomy.md` と一致させる
   - SSOT registry の “Plan語彙は DomainPlan” を明文化する

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- quick/回帰パックが緑（仕様不変）
- 29ai planner の “Plan語彙” が `DomainPlan` に一本化され、二重Planが消える
- candidate-set の 0/1/2+ 境界（SSOT）が保持される

