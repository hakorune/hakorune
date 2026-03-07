# Phase 29ai P4: First LoopFacts (scan_with_init route; historical Pattern6 label) — Instructions

Status: Historical reference (implemented)
Scope: Facts→Planner を “1ケースだけ” 前進（仕様不変・未接続）

## Goal

Facts SSOT を「実際に1ケース取れる」状態にし、Planner が `Ok(Some(plan))` まで到達できる最小の縦割りを作る。

- 対象: scan_with_init route の **最小・正規形**（既存で PASS している fixture の形に限る）
- 入口は single-planner のまま（numbered route label で分岐しない）
- 既存ルーティング/emit には接続しない（仕様不変）

Historical note:
- `Pattern6`, `facts/loop_facts.rs`, `planner/build.rs` は P4 実行時の token だよ。
- current runtime では `facts/{loop_builder.rs,loop_scan_with_init.rs}` と `planner/mod.rs` が live lane だよ。

## Non-goals

- scan_with_init の全派生（reverse/matchscan/splitscan 等）の吸収
- 既存 `plan/normalizer/*` の置換
- 新しいトグル/環境変数の追加
- 永続ログの追加

## Target Shape (canonical only)

Loop + step の最小形だけを認識する（例）:

- loop condition: `i < n`（`ASTNode::BinaryOp { operator: Less, left: Variable{i}, right: Variable{n} }`）
- loop body 末尾 step: `i = i + 1`（`Assignment(target=Variable{i}, value=BinaryOp(Add, Variable{i}, Literal(Int(1))) )`）

上記以外は **Ok(None)**（NotApplicable）で返す。  
「対象っぽいのに契約違反」は **Err(Freeze::contract(...))** で Fail-Fast（ただし P4 では Freeze を出す形を最小に絞る）。

## Implementation Steps

1) `facts/scan_shapes.rs` を最小だけ具体化
   - `StepShape::AssignAddConst { var: String, k: i64 }`
   - `ConditionShape::VarLessVar { left: String, right: String }`
   - Unknown は残す（拡張余地）

2) `facts/loop_builder.rs` / `facts/loop_scan_with_init.rs` で canonical 抽出（厳格）
   - `try_build_loop_facts(condition, body)` で上記 shape を抽出できたときだけ `Ok(Some(LoopFacts{...}))`
   - それ以外は `Ok(None)`（NotApplicable）
   - “loop だけど条件/step が壊れてる” と断定できる場合のみ `Err(Freeze::contract(...))`

3) `normalize/canonicalize.rs` は表現ゆれ吸収の座席だけ用意
   - P4 では実質 no-op でも良い（P5 で reverse 等を吸収するための前提を置く）

4) `planner/mod.rs` 経由の最小 rule を追加（候補集合方式）
   - `rule = "loop/scan_with_init"` のような内部名で `PlanCandidate` を1件だけ出す
   - `PlanKind` に `ScanWithInit` を追加（Plan はまだ placeholder でも良い）

5) テスト（軽量）
   - Rust unit test（`#[cfg(test)]`）で `try_build_loop_facts` の `Ok(Some)`/`Ok(None)` を固定
   - 既存の quick/回帰パックも緑維持（仕様不変）

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- 仕様不変（Plan/Frag 既存経路に未接続、quick/回帰パックが緑）
- Facts が “1ケースだけ” `Ok(Some)` を返せる
- Planner がその Facts で `Ok(Some(plan))` まで到達できる（候補集合→一意化）
- `Ok(None)` / `Err(Freeze)` の境界が docs の taxonomy と矛盾しない
