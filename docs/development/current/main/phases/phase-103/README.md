# Phase 103: if-only regression baseline（VM + LLVM EXE）

Status: Active  
Scope: loop を含まない `if` の lowering/merge を、VM と LLVM EXE で同一出力に固定する。  
Related:
- 入口: `docs/development/current/main/10-Now.md`
- 地図: `docs/development/current/main/design/joinir-design-map.md`
- 既存の LLVM EXE smoke / plugin gating: `docs/development/current/main/phases/phase-97/README.md`

## 目的

- 「loop が無い `if`」でも JoinIR 経路（if lowering + merge）が壊れないことを、**VM/LLVM EXE parity** で固定する。
- ループ系の変更（loop_break / loop_continue_only / derived/pinned/mutable accumulator）の回帰を、if-only で早期検知できるようにする。
- CI は最小のまま（`tools/smokes/v2` の `integration` にのみ追加）。

## P0: Fixture + integration smokes（最小）

### 1) fixture

- 追加: `apps/tests/phase103_if_only_merge_min.hako`
- 要件:
  - `if { ... } else { ... }` で同一変数へ代入し、merge（PHI 相当）が必要になる形にする。
  - nested if を 1 段だけ入れて「if の中の if」も踏む。
  - 出力は **数値 1 行**（例: `2`）に固定（exit code には依存しない）。

### 2) VM smoke

- 追加: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_vm.sh`
- 実行:
  - `HAKO_JOINIR_STRICT=1 ./target/release/hakorune --backend vm apps/tests/phase103_if_only_merge_min.hako`
  - 先頭 `[` のデバッグ行は除外して数値行のみを比較する（Phase 97 方式）。
- 受け入れ:
  - stdout の数値行が `2` と一致。

### 3) LLVM EXE smoke

- 追加: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_llvm_exe.sh`
- 方針:
  - Phase 97/98 と同様に、必要 plugin の `.so` を `ctypes.CDLL` で確認し、必要時のみ `tools/plugins/build-all.sh` を実行。
  - `tools/build_llvm.sh` で exe を生成して実行し、数値行のみを比較する。
- 受け入れ:
  - stdout の数値行が `2` と一致。

## 受け入れ基準（P0）

- `bash tools/smokes/v2/profiles/integration/apps/phase103_if_only_vm.sh` が PASS
- `bash tools/smokes/v2/profiles/integration/apps/phase103_if_only_llvm_exe.sh` が PASS（前提不足は SKIP）
- 回帰確認:
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase94_p5b_escape_e2e.sh` が PASS
  - `bash tools/smokes/v2/profiles/integration/apps/archive/phase97_next_non_ws_llvm_exe.sh` が PASS（前提不足は SKIP）
- 新しい環境変数は追加しない（既存の `HAKO_JOINIR_STRICT` などで制御）。

## DONE（P0）

- Fixture: `apps/tests/phase103_if_only_merge_min.hako`（expected: `2`）
- VM smoke: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_vm.sh`
- LLVM EXE smoke: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_llvm_exe.sh`

## P1（任意）: if-only early return

if 文の then/else 内で return する形（merge 不要 or merge 途中で return）を 1 本追加して、Boundary/ExitLine の退行を拾いやすくする。

## DONE（P1）

- Fixture: `apps/tests/phase103_if_only_early_return_min.hako`（expected: `7`, `2`）
- VM smoke: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_early_return_vm.sh`
- LLVM EXE smoke: `tools/smokes/v2/profiles/integration/apps/phase103_if_only_early_return_llvm_exe.sh`

## Next（別フェーズ候補）

### loop(true) + break-only（digit scan など）

現状の Pattern 群では、`loop(true)` で **continue なし / break 複数**の実ループが取りこぼされやすい。

- 実在例:
  - `apps/libs/json_cur.hako:29`（`read_digits_from`）
  - `apps/selfhost-vm/json_loader.hako:25`（`read_digits_from`）
- 進め方（箱理論）:
  - “新route増殖” ではなく、**loop_true_early_exit family（historical label `5`）**として扱う方針を先に決める。
  - まずは fixture + shape guard + Fail-Fast で段階投入し、VM/LLVM parity を固めてから lowering を広げる。
