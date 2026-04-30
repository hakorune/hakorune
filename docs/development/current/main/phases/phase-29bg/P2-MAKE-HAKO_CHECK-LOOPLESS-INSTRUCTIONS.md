---
Status: Ready
Scope: tooling refactor (`tools/hako_check/*`)
---

# Phase 29bg P2: Make `hako_check` loopless (selfhost tooling subset)

## Goal

`./tools/hako_check/deadcode_smoke.sh` が JoinIR freeze（未対応ループ形）で止まらないように、`tools/hako_check/*` を **loopless subset** に収束させる。

対象は「解析ツールとしての実用導線」であり、ここで汎用ループ対応を CorePlan に要求しない。

## SSOT

- Tooling loopless subset policy: `docs/development/current/main/design/selfhost-tools-loopless-subset-ssot.md`
- Phase 29bg gate: `docs/development/current/main/phases/phase-29bg/README.md`

## Why this is the right boundary

`tools/hako_check/*` は selfhost の開発導線であり、ここが素朴な一般ループを持つと、CorePlan 側に “ツール都合の汎用CFG” を背負わせてしまう。

stdlib/アプリの表現力は CorePlan を強くして伸ばすが、**ツールは subset-only** を守る。

## Work items

### 1) Identify freeze-causing loops

まず `NYASH_DISABLE_PLUGINS=1` のまま `tools/hako_check/cli.hako` を通し、freeze した関数名を収集する。

（例）今回の first FAIL:
- `HakoAnalysisBuilderBox.build_from_source_flags/3`（`tools/hako_check/analysis_consumer.hako`）

### 2) Refactor to recursion (no `while` / `loop`)

最低限、以下の種類のループを再帰に置換する:

- `while i < arr.size() { ... i = i + 1 }` → `fn walk(i) { if i>=n return; ...; walk(i+1) }`
- ネストした二重ループ → 外側/内側をそれぞれ再帰関数に分割
- `loop(true)` + `break` パターン → “停止条件を明示した再帰” に置換

対象候補（例）:
- `tools/hako_check/analysis_consumer.hako`
  - uses/boxes/methods/calls 収集の `while` 連鎖
  - `_infer_call_arity` の走査
- `tools/hako_check/rules/*` のテキスト走査が残っていれば同様に置換

注意:
- 挙動は維持（dead code 検出の精度を落とさない）
- 既定のログ/exit code は不変

### 3) Add/Update gate

- `./tools/hako_check/deadcode_smoke.sh` が PASS する
- 追加で “loopless lint” を入れる場合は、strict/dev のみ FAIL-Fast にする（release導線は不変）

## Acceptance criteria

- `cargo build --release`
- `./tools/hako_check/deadcode_smoke.sh` PASS
- `./tools/hako_check_deadblocks_smoke.sh` PASS
- `./tools/hako_check/run_tests.sh` PASS

## P2 gate (SSOT, RC expectations)

Expected RC=0 (HC017 skip is expected in `run_tests`).
- `./tools/smokes/v2/run.sh --profile quick` (RC=0)
- `./tools/smokes/v2/profiles/integration/joinir/phase29ae_regression_pack_vm.sh` (RC=0)
- `./tools/hako_check/deadcode_smoke.sh` (RC=0)
- `bash tools/hako_check/run_tests.sh` (RC=0; HC017 skipped)

## Notes

- `NYASH_DISABLE_PLUGINS=1` は維持（解析導線の決定性）
