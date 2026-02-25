# Phase 29ai P5: Single-planner bridge (router → 1 entrypoint) — Instructions

Status: Ready for execution  
Scope: 構造整理（仕様不変）

## Goal

JoinIR の “pattern名ルーティング” を外部APIから剥がし、**Plan抽出の入口を 1 本に収束**させる。

- 現状: `src/mir/builder/control_flow/joinir/patterns/router.rs` が `PLAN_EXTRACTORS`（pattern名＋順序）で分岐
- 目標: router 側の分岐を撤去し、`plan::single_planner`（仮名）に “入口SSOT” を置く
- 重要: **既存挙動と既存エラーメッセージを変えない**（意味論不変・観測不変）

このP5は “吸収” の準備段階。内部の rule は当面、既存 extractor（Pattern2/6/7…）を呼ぶだけでよい。

## Non-goals

- Facts/Normalize/Planner/Emit の置換完了（P6+）
- extractor の削除（P6+）
- 既存のエラーメッセージ/RC/ログの変更
- 新しい env 変数/トグルの追加

## Target Architecture

```
src/mir/builder/control_flow/plan/
  single_planner/
    mod.rs            # SSOT entrypoint: try_build_domain_plan(...)
    rules.rs          # Rule list (順序SSOT) + guard (PatternKindなど)
    legacy_rules/     # 既存 extractor を薄く呼び出すだけのブリッジ
      pattern2.rs
      pattern6.rs
      pattern7.rs
      pattern1.rs ...
```

router 側は “単一呼び出し” のみになる:

- Before: `for entry in PLAN_EXTRACTORS { ... }`
- After: `if let Some(plan) = single_planner::try_build_domain_plan(ctx)? { ... }`

## Contract

- 戻り値型は **当面 String エラーを維持**する（router 既存の `Result<_, String>` を崩さない）
  - Typed `Freeze` は Phase 29ai の未接続導線（Facts→Planner）側で継続使用
- ルール順序は **既存の `PLAN_EXTRACTORS` と同一**（観測不変のため）
- Pattern1 guard（pattern_kind による抑制）など、router 側の既存ガードも single_planner に移す

## Implementation Steps

1) `src/mir/builder/control_flow/plan/single_planner/` を新設
2) `try_build_domain_plan(ctx: &LoopPatternContext) -> Result<Option<DomainPlan>, String>` を追加
3) 既存 router のループ（PLAN_EXTRACTORS）を撤去し、上記を呼ぶだけにする
4) 既存 extractor 呼び出しは “薄いラッパ” に留める（引数整形だけ、意味論不変）
5) docs 更新
   - Phase 29ai README に P5 を追記
   - `docs/development/current/main/10-Now.md` の Next を P5 に更新

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- quick/回帰パックが緑（仕様不変）
- router から pattern 名分岐（PLAN_EXTRACTORS）が除去され、入口が 1 本になる
- 既存の error string / log の観測が変わらない

