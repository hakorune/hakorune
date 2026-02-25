# Phase 29ai P1: Planner “candidate-set” + Freeze SSOT — Instructions

Status: Ready for execution  
Scope: Plan/Frag 導線の「判定/失敗」のSSOT化（docs-first, 仕様不変）

## Goal

P0 の骨格を “一本道のパイプライン” として完成させるために、Planner の契約を先に固定する。

- 外部APIは **常に1本**: `build_plan(...) -> Result<Option<Plan>, Freeze>`
- “pattern名で入口分岐” をしない（候補推論は planner 内部に閉じる）
- `Ok(None)` / `Err(Freeze)` の境界を SSOT 化して、後続の Facts 実装が迷わない土台を作る

## Non-goals

- 既存 JoinIR/Pattern 実装の置換（P1では未接続のまま）
- 仕様変更（挙動変更）/ 既存のエラー文言変更
- 新しい env 変数 / トグルの追加

## Contract (SSOT)

### `Ok(None)` の意味（NotApplicable）

「この region は plan 化の対象ではない」。既存経路へ委譲して良い。

- 例: 特殊なCFG構造がない / plan を挟む必要がない / 入口条件が揃っていない

### `Err(Freeze)` の意味（Fail-Fast）

「対象っぽいが、契約違反/曖昧/禁止形で plan を作れない」。silent fallback 禁止。

- 例: loop header が一意に定まらない、exit が曖昧、step/cond が複数解釈できる、shape契約を破っている

### `Ok(Some(plan))`

「一意に plan が確定し、emit に必要な情報が揃っている」。

## Target Structure (P1)

```
src/mir/builder/control_flow/plan/planner/
  mod.rs
  build.rs            # build_plan, build_plan_from_facts
  freeze.rs           # Freeze SSOT（tag + message + hint）
  candidates.rs       # PlanCandidate / CandidateSet（0/1/Many の一意化）
```

Note: ファイル分割は必須ではないが、Freeze と候補集合は “再利用される契約” なので build.rs から分離する。

## Implementation Steps (P1)

1) `Freeze` を `type Freeze = String` から構造体へ昇格
   - `Freeze { tag: &'static str, message: String, hint: Option<String> }`
   - 文字列表現（`Display`）は SSOT として安定化する（例: `"[plan/freeze:{tag}] {message}"`）。
   - `Freeze::ambiguous(...)` / `Freeze::contract(...)` など、生成ヘルパーを用意する。

2) “候補集合 → 一意化” の小さな仕組みを追加
   - `PlanCandidate { kind: PlanKind, rule: &'static str }` 程度で良い（公開APIに pattern 名を漏らさない）。
   - `CandidateSet::finalize()`:
     - 0件 → `Ok(None)`
     - 1件 → `Ok(Some(plan))`
     - 2件以上 → `Err(Freeze::ambiguous(...))`

3) `planner::build_plan_from_facts` を candidate-set 方式に置き換え
   - まだ Facts が `Ok(None)` なので、ここは **未到達でもコンパイルできる** 状態にする。
   - `todo!/unreachable!` で落とすのは避け、`Err(Freeze::not_implemented(...))` のように “将来のFail-Fast” をSSOT化しておく。

4) docs / 道筋の更新
   - `docs/development/current/main/phases/phase-29ai/README.md` に P1 を追加（指示書リンク＋目的）。
   - `docs/development/current/main/10-Now.md` の Next を P1 指示書へ更新。
   - `CURRENT_TASK.md` に Phase 29ai P0 完了 + P1 次の作業を追記（入口/コマンドも固定）。

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- 仕様不変（quick/回帰パックが緑）
- `Ok(None)` と `Err(Freeze)` の境界が SSOT として文章・型で固定される
- Planner が “候補集合→一意化” で一本道化され、入口に pattern 名分岐が増えない

