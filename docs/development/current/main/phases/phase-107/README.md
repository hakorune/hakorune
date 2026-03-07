# Phase 107 — json_cur find_balanced_array_end（Active）

目的: `apps/libs/json_cur.hako` の `find_balanced_array_end`（depth scan + nested if + return-in-loop）を loop_break policy で受理し、VM/LLVM EXE parity を integration smoke で固定する。
形状: `loop(i < n)` + `ch = s.substring(i, i+1)` + `depth += 1/-1` + `if depth == 0 { return i }` + `i = i + 1`。
受け入れ基準: fixture で `[]`→`1`, `[[]]`→`3` を VM と LLVM EXE の両方で出す（LLVM 前提不足時は SKIP のみ許容）。

DONE:
- fixture: `apps/tests/phase107_find_balanced_array_end_min.hako`
- smoke(VM): `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_array_end_vm.sh`
- smoke(LLVM EXE): `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_array_end_llvm_exe.sh`
- fixture: `apps/tests/phase107_find_balanced_object_end_min.hako`
- smoke(VM): `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_object_end_vm.sh`
- smoke(LLVM EXE): `tools/smokes/v2/profiles/integration/apps/phase107_find_balanced_object_end_llvm_exe.sh`

補足: post-loop early return の plan 一般化は Phase 108（`docs/development/current/main/phases/phase-108/README.md`）で実施する。
