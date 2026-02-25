# Phase 97: LLVM EXE parity for MiniJsonLoader fixtures

- Phase 95/96 の VM 専用 fixture を LLVM EXE でも固定し、JoinIR/Trim 退行を早期検出できるようにする。
- 新規 smoke: `phase97_next_non_ws_llvm_exe.sh`（apps/tests/phase96_json_loader_next_non_ws_min.hako）
  / `phase97_json_loader_escape_llvm_exe.sh`（apps/tests/phase95_json_loader_escape_min.hako）
- LLVM/llvmlite が無い環境では SKIP（integration プロファイルのみで運用）
- PHI wiring: duplicate incoming per pred の上書きバグを修正（0フォールバック回避）
