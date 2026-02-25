# Phase 29ai P8: Wire planner into `single_planner` (Pattern6 subset) — Instructions

Status: Ready for execution  
Scope: 入口収束の前進（仕様不変）

## Goal

29ai の `facts/normalize/planner` を “実行経路” に一歩だけ接続し、Pattern6（scan_with_init）の **最小・安全な部分集合** を
single_planner で先に処理できるようにする。

- 対象: `loop(i < s.length()) { if s.substring(i, i + 1) == ch { return i } i = i + 1 }` 形のみ
- それ以外は従来どおり `legacy_rules::pattern6` へ落ちる（観測/挙動/エラー文字列は維持）
- Fail-Fast: planner 側はこのP8では `Err(Freeze)` を出さない（NotApplicable を基本にする）

## Non-goals

- Pattern6 全派生（reverse/dynamic needle 等）の吸収
- Pattern7/Pattern2 の接続（P9+）
- legacy extractor の削除（P10+）
- 新しい env 変数/トグルの追加

## Implementation Steps

1) `plan::single_planner::rules` の先頭に “planner 由来” の rule を 1 本追加
   - `plan::planner::build_plan(ctx.condition, ctx.body)` を呼ぶ
   - `Ok(Some(domain_plan))` のときは即採用（`DomainPlan::ScanWithInit` が来る想定）
   - `Ok(None)` のときは次へ（従来の legacy rules）
   - `Err(freeze)` のときは当面 `Err(freeze.to_string())` で落とす（ただし P8 の範囲では freeze を出さない）

2) ルール順序（観測不変）
   - planner 由来 rule は **Pattern6 legacy より前**
   - それ以外の順序は P5/P6 で固定した legacy と同一

3) 回帰固定
   - 既存 fixture（`apps/tests/phase29ab_pattern6_scan_with_init_ok_min.hako`）が引き続き PASS すること
   - contract fixture（`apps/tests/phase29ab_pattern6_scan_with_init_contract_min.hako`）は legacy 側で freeze 維持

## Verification (SSOT)

- `cargo build --release`
- `./tools/smokes/v2/run.sh --profile quick`
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh`

## Acceptance Criteria

- quick/回帰パックが緑（仕様不変）
- Pattern6 OK 最小ケースが planner 経由でも PASS（結果は同じ）
- Pattern6 contract ケースは legacy 経由で freeze のまま（エラー文字列も同一）

