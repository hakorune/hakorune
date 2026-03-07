# Phase 102: real-app read_quoted loop regression (VM + LLVM EXE)

- 対象: `apps/selfhost-vm/json_loader.hako` の `MiniJsonLoader.read_quoted_from` を最小抽出して fixture 化。
- 固定: accumulator（`out = out + ch`）＋ escape（`\\` → 次の1文字を取り込み）＋ quote 終端（`"` で break）。
- フィクスチャ: `apps/tests/phase102_realapp_read_quoted_min.hako`（期待: `out.length() == 4`）
- smoke:
  - VM: `tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_vm.sh`
  - LLVM EXE: `tools/smokes/v2/profiles/integration/apps/archive/phase102_realapp_read_quoted_llvm_exe.sh`
- 次候補: `parse_object` / `parse_array` の key/value ループ（loop_continue_only; historical label `4`; continue + return 混在）や read_digits 系。
