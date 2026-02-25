# Phase 29ai P6: Move Pattern6/7 extractors to Plan layer (structure) — Instructions

Status: Ready for execution  
Scope: 構造整理（意味論不変・観測不変）

## Goal

Pattern6/7 の “抽出（extractor）” を JoinIR layer から Plan layer に移し、依存方向を一方向にする。

- JoinIR は **ルーティングと lowering 実行**に徹する
- Plan は **DomainPlan 抽出（pattern固有の知識）**の SSOT を持つ
- `single_planner` は当面 legacy 呼び出しを維持しつつ、実装の所在地を plan 側へ寄せる

## Non-goals

- 仕様変更（挙動変更）
- 既存のエラー文字列変更
- by-name / hardcode の追加
- Pattern2 以降の大移設（P6は Pattern6/7 のみ）

## Current State (P5)

- JoinIR router は `plan::single_planner` を呼ぶだけ（入口は収束済み）
- `plan::single_planner::legacy_rules::{pattern6,pattern7}` は joinir 側の extractor を薄く呼ぶだけ

## Target Architecture (P6)

```
src/mir/builder/control_flow/plan/extractors/
  mod.rs
  pattern6_scan_with_init.rs   # moved from joinir/patterns
  pattern7_split_scan.rs       # moved from joinir/patterns
```

JoinIR 側は “互換の薄い再export” だけにする（削除はP7+）:

```
src/mir/builder/control_flow/joinir/patterns/
  pattern6_scan_with_init.rs   # re-export wrapper (temporary)
  pattern7_split_scan.rs       # re-export wrapper (temporary)
```

## Implementation Steps

1) Plan layer に extractor の新しい置き場を作る
   - `src/mir/builder/control_flow/plan/extractors/mod.rs` を追加
   - Pattern6/7 を `plan/extractors/` に **移動**（またはコピーしてから元を薄くする）

2) 公開シグネチャを維持（観測不変）
   - Pattern6: `extract_scan_with_init_plan(condition, body, fn_body) -> Result<Option<DomainPlan>, String>`
   - Pattern7: `extract_split_scan_plan(condition, body, post_loop_code) -> Result<Option<DomainPlan>, String>`
   - エラー文字列は **そのまま**（ログ/テストが既存の文字列に依存しうる）

3) single_planner の legacy_rules を plan 側へ向ける
   - `plan/single_planner/legacy_rules/pattern6.rs` の import を `plan::extractors::pattern6_scan_with_init` に変更
   - `plan/single_planner/legacy_rules/pattern7.rs` も同様

4) JoinIR 側の旧 module は “再export wrapper” に縮退
   - `joinir/patterns/pattern6_scan_with_init.rs` は `pub use plan::extractors::pattern6_scan_with_init::*;` のようにする
   - `joinir/patterns/pattern7_split_scan.rs` も同様
   - 依存が残っていないなら削除しても良いが、P6では安全に “薄い互換” を推奨

5) docs 更新
   - Phase 29ai README に P6 を追記
   - Now/Backlog の Next を P6 に更新

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- quick/回帰パックが緑（仕様不変）
- Pattern6/7 の extractor 実装の所在地が plan layer へ移る
- JoinIR layer は plan へ依存するのみ（逆依存なし）

