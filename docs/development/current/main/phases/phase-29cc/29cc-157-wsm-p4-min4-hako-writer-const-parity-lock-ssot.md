---
Status: Done
Decision: accepted
Date: 2026-02-26
Scope: WSM-P4-min4（.hako-only roadmap P4）として const-return fixture 1件の binary-writer parity を実装 lock する。
Related:
  - apps/tests/phase29cc_wsm_p4_min_const_return.hako
  - src/backend/wasm/mod.rs
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh
  - tools/checks/dev_gate.sh
  - docs/development/current/main/phases/phase-29cc/29cc-156-wsm-p4-min3-hako-writer-entry-parity-doc-lock-ssot.md
---

# 29cc-157 WSM-P4-min4 `.hako` Writer Const Parity Lock

## Purpose
P4-min3 で固定した entry/parity 契約に対して、最小 fixture 1件で実装ロックを作る。  
対象は const-return 形のみとし、binary writer 直書きの pilot を fail-fast 境界で固定する。

## Implemented
1. fixture 追加:
   - `apps/tests/phase29cc_wsm_p4_min_const_return.hako`（`return 7`）
2. backend pilot:
   - `WasmBackend::compile_module` に最小形抽出（`main` 単一block、`Const(Integer)` + `Return`）を追加。
   - 形一致時のみ `binary_writer::build_minimal_main_i32_const_module` へ直結。
   - 形不一致は既存 `compile_to_wat -> wat2wasm` 経路にフォールバック。
3. parity test:
   - `wasm_demo_min_const_return_binary_writer_parity_contract`
   - compile_module 出力 bytes と `build_minimal_i32_const_wasm(7)` を厳密一致。

## Gate
1. smoke:
   - `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_p4_min4_hako_writer_const_parity_vm.sh`
2. integration:
   - `tools/checks/dev_gate.sh wasm-boundary-lite` に P4-min4 step を追加。

## Invariants
1. 対象は const-return 最小形のみ（1 fixture / 1 shape）。
2. silent fallback は禁止。形一致しない場合は既存経路で処理し、挙動差は parity test で捕捉する。
3. P4-min4 は pilot lock。一般 shape への拡張は次段で個別固定する。

## Next
- `WSM-P4-min5`: binary writer pilot shape を 1形追加（例: const 0 / const negative）し、shape table + parity gate を更新する。
