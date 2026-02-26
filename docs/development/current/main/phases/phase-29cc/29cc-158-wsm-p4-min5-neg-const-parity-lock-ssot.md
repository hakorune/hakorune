---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min5（.hako-only roadmap P4）として negative const shape（`return -1`）の parity lock を固定する。
Related:
  - apps/tests/phase29cc_wsm_p4_min_const_return_neg1.hako
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min5_hako_writer_neg_const_parity_vm.sh
  - tools/checks/dev_gate.sh
  - docs/development/current/main/phases/phase-29cc/29cc-157-wsm-p4-min4-hako-writer-const-parity-lock-ssot.md
---

# 29cc-158 WSM-P4-min5 Negative Const Parity Lock

## Purpose
WSM-P4 pilot の signed LEB128 境界を 1 shape 追加で固定する。  
P4-min4 の `return 7` に加え、`return -1` parity を lock して binary writer pilot の形を厚くする。

## Implemented
1. fixture 追加:
   - `apps/tests/phase29cc_wsm_p4_min_const_return_neg1.hako`
2. parity test 追加:
   - `wasm_demo_min_const_return_neg1_binary_writer_parity_contract`
   - `compile_module` bytes と `build_minimal_i32_const_wasm(-1)` の一致を固定。
3. smoke/gate:
   - `phase29cc_wsm_p4_min5_hako_writer_neg_const_parity_vm.sh`
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に統合。

## Invariants
1. 1 shape = 1 lock（今回の追加は `return -1` のみ）。
2. parity は byte-level 厳密一致。
3. fail-fast 原則を維持（silent fallback なし）。

## Next
- `WSM-P4-min6`: pilot shape 判定を shape table（箱化）へ整理し、既存 parity を table 経由で固定する。
