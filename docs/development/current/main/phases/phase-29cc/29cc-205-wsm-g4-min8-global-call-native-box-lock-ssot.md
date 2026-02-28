---
Status: Accepted
Phase: 29cc
Task: WSM-G4-min8
Title: WASM Global Call Native Box Lock
Depends:
  - src/backend/wasm/codegen/mod.rs
  - src/backend/wasm/codegen/instructions.rs
  - apps/tests/phase29cc_wsm_g4_min8_global_call_probe_min.hako
  - tests/wasm_demo_min_fixture.rs
  - tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min8_global_call_probe_vm.sh
---

# 29cc-205 WSM-G4-min8 Global Call Native Box Lock

## Goal

`Callee::Global` を wasm backend で受理し、user-defined Box method call 経路を
native wasm compile で fail-fast せず通す。

## Scope

1. WAT 関数宣言に `MirFunction.params` を `(param ...)` として出力する。
2. `MirInstruction::Call + Callee::Global` を codegen する。
3. 不足引数は `i32.const 0` で補完する。
4. 余剰引数は arity mismatch で fail-fast する。
5. probe fixture の compile-to-wat 契約を固定する。

## Acceptance

- `bash tools/smokes/v2/profiles/integration/apps/phase29cc_wsm_g4_min8_global_call_probe_vm.sh`

## Notes

- この lock は route policy (`default => native-shape-table / bridge`) を変更しない。
- shape table 外は引き続き bridge plan だが、bridge backend compile 時の wasm codegen で
  `Callee::Global` が受理されることを保証する。
