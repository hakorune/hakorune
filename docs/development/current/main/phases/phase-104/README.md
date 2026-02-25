# Phase 104: loop(true) + break-only digits（read_digits 系）

目的: `loop(true)` の break-only ループ（read_digits_from 形）を Pattern2 経路で VM/LLVM EXE parity 固定する。  
Fixture: `apps/tests/phase104_read_digits_loop_true_min.hako`（expected: `2`, `1`）  
Smokes: `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_vm.sh` / `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_llvm_exe.sh`

DONE:
- loop(true) counter 抽出（契約SSOT+Fail-Fast）: `LoopTrueCounterExtractorBox`
- break 条件（break when true）正規化 + digit set 固定: `ReadDigitsBreakConditionBox`

P2（実ループ由来の回帰面増強）:
- Fixture: `apps/tests/phase104_read_digits_json_cur_min.hako`（json_cur.hako 由来, expected: `2`, `1`）
- Smokes: `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_json_cur_vm.sh` / `tools/smokes/v2/profiles/integration/apps/phase104_read_digits_json_cur_llvm_exe.sh`
