# Phase 29ai P3: Typed Freeze + CandidateSet implementation (code) — Instructions

Status: Ready for execution  
Scope: planner の型/契約をコードに落とす（仕様不変）

## Goal

`Result<Option<Plan>, Freeze>` の境界（NotApplicable vs Fail-Fast）を “型” と “候補集合→一意化” で固定し、入口の if 地獄を増やさない。

このP3は、Facts がまだ `Ok(None)` を返す限り、実行経路は変化しない（仕様不変）。

## Non-goals

- Facts の実装（P4以降）
- 既存 pattern/normalizer の置換
- 新しい env 変数 / トグルの追加
- 永続ログの追加

## Target Files

- `src/mir/builder/control_flow/plan/planner/mod.rs`
- `src/mir/builder/control_flow/plan/planner/build.rs`
- （必要なら追加）`src/mir/builder/control_flow/plan/planner/freeze.rs`
- （必要なら追加）`src/mir/builder/control_flow/plan/planner/candidates.rs`
- `src/mir/builder/control_flow/plan/facts/loop_facts.rs`（Freeze 型参照の調整のみ）

## Implementation Steps

1) `Freeze` を `type Freeze = String` から構造体へ
   - `Freeze { tag: &'static str, message: String, hint: Option<String> }`
   - `Display` は SSOT として安定化する:
     - 例: `"[plan/freeze:{tag}] {message}"`
   - 生成ヘルパーを用意:
     - `Freeze::contract(msg)`
     - `Freeze::ambiguous(msg)`
     - `Freeze::unsupported(msg)`
     - `Freeze::bug(msg)`

2) CandidateSet（候補集合→一意化）を追加
   - `PlanCandidate { kind: PlanKind, rule: &'static str }`
   - `CandidateSet::finalize()`:
     - 0件 → `Ok(None)`
     - 1件 → `Ok(Some(plan))`
     - 2件以上 → `Err(Freeze::ambiguous(...))`
   - `rule` は診断用の内部名のみ（公開APIに pattern 名を漏らさない）

3) `build_plan_from_facts` を candidate-set 方式に変更
   - `rules::*` は未実装でよいが、`unreachable!/todo!` は避ける
   - 未対応は `Freeze::unsupported("...")` で SSOT の“失敗形”を明確化
   - Facts が `Ok(None)` の現状では到達しないため、挙動は不変

4) docs の参照整合を確認
   - Tag/分類は `docs/development/current/main/design/planfrag-freeze-taxonomy.md` と一致させる

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- 仕様不変（quick/回帰パックが緑）
- Planner の入口が “候補集合→一意化” で固定され、入口分岐が増えない
- `Ok(None)` / `Err(Freeze)` の境界が “型” で表現され、将来の Facts 実装で揺れない

