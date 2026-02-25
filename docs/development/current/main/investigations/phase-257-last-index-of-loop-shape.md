Status: Active  
Scope: `json_lint_vm / StringUtils.last_index_of/2` の最初の FAIL を、最小の再現と論点で固定する。  
Related:
- Phase 257 SSOT: `docs/development/current/main/phases/phase-257/README.md`
- Design goal: `docs/development/current/main/design/join-explicit-cfg-construction.md`

# Phase 257 Investigation: `last_index_of/2` loop shape

## Symptom（SSOT）

`./tools/smokes/v2/run.sh --profile quick` の最初の FAIL:

- `json_lint_vm` が `StringUtils.last_index_of/2` で停止
- エラー: `[joinir/freeze] Loop lowering failed: JoinIR does not support this pattern, and LoopBuilder has been removed.`

更新（P0/P1後）:

- Pattern6 を reverse scan 対応した後、`phi pred mismatch` が露出したが、P1 で以下により根治した:
  - MIR verifier で PHI predecessor を fail-fast 検証
  - loop header PHI の entry edge を CFG から復元（self pred 防止）
  - smoke の false positive（exit=1衝突）を `--verify` + VM error 検出で抑止

## Minimal Fixture

- `apps/tests/phase257_p0_last_index_of_min.hako`

形（要旨）:

```nyash
local i = s.length() - 1
loop(i >= 0) {
  if s.substring(i, i + 1) == ch { return i }
  i = i - 1
}
return -1
```

## StepTree / Capabilities（ログ観測）

（実ログは `tools/smokes/v2/profiles/quick/apps/json_lint_vm.sh` の tail を参照）

- caps: `If,Loop,Return`（break/continue なし）
- loop cond: `i >= 0`
- step: `i = i - 1`（const step のはず）
- early exit: `return i`
- not-found: `return -1`

`loop_canonicalizer` は `Missing caps: [ConstStep]` で FAIL_FAST しているが、ここは “Pattern2 側の試行ログ” であり、
JoinIR パターンがこの形を受理できていないのが本体。

## Root Cause Hypothesis（現状の仮説）

- Pattern6（ScanWithInit）が forward scan 前提（`i = i + 1`, `i < bound`）で、reverse scan を検出できていない。
- Pattern7（SplitScan）は適用対象外。
- Pattern3/1 は `return` を含む loop を扱わない（or 目的が違う）。

更新（P0/P1後）:

- Pattern6 の検出/生成はできているが、merge 側が “CFG（successors/preds）と terminator” の同期を前提にしていた。
  - joinir merge 中で `terminator` 直書きがあり、`successors` が未同期になることがある（CFG 解析が欠ける）
  - finalize 時点では host entry jump が未設定なため、header preds だけでは entry pred を復元できない場合がある
  - これらは P1 で補正済み（fail-fast + 復元ロジック）

## Decision（Phase 257 の方針）

Phase 257 では、以下で進める:

- Pattern6（ScanWithInit）を “scan direction” 付きに一般化し、reverse scan + early return を受理する。
- Phase 256 で固めた Contract（`JumpArgsLayout`, pipeline contract checks）に従い、merge 側で推測しない。

追加（P1）:

- MIR verifier に `InvalidPhi` チェックを追加し、`phi pred mismatch` を `--verify` で fail-fast にする
- loop header PHI の entry edge source を正す（必要なら preheader を生成）

## Questions（将来に残す設計論点）

1. `LoopPatternKind` に Pattern6/7 を増やすべきか？（router 側での分類SSOTを揃える）
2. scan 系の “forward/reverse” を 1 パターンにまとめるか、専用 Pattern を増やすか？
3. `return` を loop 語彙として Pattern 側で扱い続けるか、Normalization で “early exit” に正規化すべきか？
