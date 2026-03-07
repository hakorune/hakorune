# Phase 29ai P6: Move Pattern6/7 extractors to Plan layer (structure) — Instructions

Status: Historical reference (implemented)
Scope: 構造整理（意味論不変・観測不変）

## Goal

scan_with_init / split_scan の “抽出（extractor）” を JoinIR layer から Plan layer に移し、依存方向を一方向にする。

- JoinIR は **ルーティングと lowering 実行**に徹する
- Plan は **route-specific extraction knowledge** の SSOT を持つ
- `single_planner` は当面 legacy 呼び出しを維持しつつ、実装の所在地を plan 側へ寄せる

Historical note:
- `joinir/patterns/*`, `plan/extractors/pattern6_scan_with_init.rs`,
  `plan/extractors/pattern7_split_scan.rs`, `legacy_rules/*` は P6 実行時の path token だよ。
- current runtime では facts / recipe / route-entry surface に寄せてあり、old path は historical lane として扱うよ。

## Non-goals

- 仕様変更（挙動変更）
- 既存のエラー文字列変更
- by-name / hardcode の追加
- loop_break 以降の大移設（P6は scan_with_init / split_scan のみ）

## Current State (P5)

- JoinIR router は `plan::single_planner` を呼ぶだけ（入口は収束済み）
- execution-time では historical compatibility lane が extractor を薄く呼ぶだけだった

## Target Architecture (P6)

```
src/mir/builder/control_flow/plan/facts/
  loop_scan_with_init.rs       # current scan_with_init facts lane
  loop_split_scan.rs           # current split_scan facts lane
  loop_builder.rs              # current LoopFacts collection entrypoint
```

JoinIR 側は “route_entry から facts/planner を呼ぶ thin lane” に寄せる:

```
src/mir/builder/control_flow/joinir/route_entry/
  router.rs                    # current route entrypoint
  registry/                    # current registry surface
```

## Implementation Steps

1) Plan layer に extractor の新しい置き場を作る
   - current semantic outcome は `plan/facts/{loop_scan_with_init.rs,loop_split_scan.rs}` だよ
   - historical `plan/extractors/*` path token は instruction note として残す

2) 公開シグネチャを維持（観測不変）
   - scan_with_init / split_scan の観測契約は維持
   - エラー文字列は **そのまま**（ログ/テストが既存の文字列に依存しうる）

3) single_planner / route_entry の参照先を plan 側へ向ける
   - current SSOT は `single_planner::{mod.rs,rules.rs,rule_order.rs}` と `plan/facts/*`

4) JoinIR 側の旧 module は historical lane に後退
   - current runtime では `joinir/route_entry/*` だけを live path とする

5) docs 更新
   - Phase 29ai README に P6 を追記
   - Now/Backlog の Next を P6 に更新

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- quick/回帰パックが緑（仕様不変）
- scan_with_init / split_scan の抽出知識の所在地が plan layer へ移る
- JoinIR layer は plan へ依存するのみ（逆依存なし）
