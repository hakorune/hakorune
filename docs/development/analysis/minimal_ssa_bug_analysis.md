# Minimal SSA Bug Analysis — loop(1==1) + break exit PHI

## 何が起きていたか
- Stage‑B / FuncScanner の `skip_whitespace` 系で、`loop(1 == 1) { ... break }` という形のループが exit PHI を壊し、`ValueId` 未定義エラー（`use of undefined value` / Dominator violation）が発生していた。
- 原因: exit PHI 入力に「存在しない predecessor（header）」由来の値を含めていた。
  - `loop(1 == 1)` の場合、`header → exit` の CFG エッジは存在しない（exit は break 経路のみ）。
  - それにもかかわらず、`merge_exit_with_classification` が header 値を無条件で PHI 入力に足していたため、非支配ブロックからの値参照になり破綻。

## 最小再現
- ファイル: `apps/tests/minimal_ssa_skip_ws.hako`
- 構造: `loop(1 == 1)` の中で `if i >= n { break }` を先頭に置く形。break 以外に exit pred が無い。
- Rustテスト: `src/tests/mir_loopform_conditional_reassign.rs::loop_constant_true_exit_phi_dominates`（今回新設）

## 修正内容
- `src/mir/phi_core/loop_snapshot_merge.rs` の `merge_exit_with_classification` で、header が exit predecessor に含まれている場合にのみ header 値を PHI 入力へ追加するようガード。
  - CFG に存在しない predecessor 由来の値を PHI に入れないことで Dominator violation を解消。

## 追加したもの
- 最小再現ハコ: `apps/tests/minimal_ssa_skip_ws.hako`
- Rustテスト: `src/tests/mir_loopform_conditional_reassign.rs`
  - `loop_constant_true_exit_phi_dominates`（今回のバグ再現→修正確認用）
  - 将来拡張用に 2 本を `#[ignore]` で占位（条件付き再代入 / body-local 変数の exit PHI 観測）

## 次のステップ
- `#[ignore]` テストを埋める: 条件付き再代入（`loop(i < n)`）や body-local 混在ケースの SSA 安定性を追加検証。
- Stage‑B / UsingResolver の既存スモークにも `apps/tests/minimal_ssa_skip_ws.hako` を流し込んで回帰を防ぐ。
