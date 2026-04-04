# Phase 104: loop(true) + break-only digits（read_digits 系）

目的: `loop(true)` の break-only ループ（read_digits_from 形）を loop_break route（historical numbered label `2`）で VM/LLVM EXE parity 固定する。
Fixture: `apps/tests/phase104_read_digits_loop_true_min.hako`（expected: `2`, `1`）  
Smokes: `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_llvm_exe.sh`

DONE:
- loop(true) counter 抽出（契約SSOT+Fail-Fast）: `LoopTrueCounterExtractorBox`
- break 条件（break when true）正規化 + digit set 固定: `ReadDigitsBreakConditionBox`

P2（実ループ由来の回帰面増強）:
- Fixture: `apps/tests/phase104_read_digits_json_cur_min.hako`（json_cur.hako 由来, expected: `2`, `1`）
- Smokes: `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_json_cur_vm.sh` / `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_json_cur_llvm_exe.sh`

DONE:
- VM helper-method stub を避けるため、fixture を helper call 依存から外した
- LLVM EXE pure recipe reject を避けるため、archive smoke に `HAKO_BACKEND_COMPAT_REPLAY=harness` を pin した
- digit 判定の OR chain は sequential flag 判定に置き換え、VM/LLVM EXE parity を固定した
