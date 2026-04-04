# Phase 105: digit OR-chain LLVM parity regression

目的: `loop(true)` 内の long digit OR-chain

`ch == "0" || ch == "1" || ... || ch == "9"`

を、VM と LLVM EXE の両方で同じ意味に戻す。

Current front:
- long digit OR-chain は parser / MIR emit ではなく LLVM parity が怪しい
- `phase-96 next_non_ws` の whitespace OR-chain は既に green
- target は `substring + string compare + long OR chain + break-only loop`

Exact focus:
- `apps/tests/phase104_read_digits_loop_true_min.hako`
- `apps/tests/phase104_read_digits_json_cur_min.hako`
- `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_llvm_exe.sh`
- `tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_json_cur_llvm_exe.sh`

Success condition:
- sequential-flag workaround を戻しても LLVM EXE が `2`, `1` を返す
- VM / LLVM EXE parity を long OR-chain shape で固定する
- `phase-110x` の vocabulary cleanup へ戻す
