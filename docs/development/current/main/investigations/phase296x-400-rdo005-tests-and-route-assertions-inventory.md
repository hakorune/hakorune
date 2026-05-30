---
Status: Draft
Date: 2026-05-30
Scope: row400 RDO-005 tests and route assertions inventory
Related:
  - docs/development/current/main/phases/phase-296x/296x-400-RUNTIME-DATA-DISPATCH-ROUTE-POLICY-OWNER-REFRESH.md
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
  - src/llvm_py/instructions/mir_call/runtime_data_dispatch.py
  - src/llvm_py/instructions/mir_call/collection_method_call.py
---

# RDO-005 Tests And Route Assertions Inventory

## Input

- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

## Route Assertion Table

| file | assertion family | line_hint | note |
| --- | --- | --- | --- |
| `test_runtime_data_dispatch_policy.py` | policy selector assertions | `42-280` | Pins the array-mono policy default, the runtime-data-only switch, invalid-value fail-fast, and the shared RuntimeDataBox array-field map ABI. |
| `test_runtime_data_dispatch_policy.py` | field ABI assertions | `282-320` | Pins `RuntimeDataBox.getField/setField` to `nyash.map.slot_load_hh` / `nyash.map.slot_store_hhh`. |
| `test_collection_method_call.py` | ArrayBox collection assertions | `87-147` | Pins `nyash.array.slot_load_hi`, `nyash.array.slot_store_hii`, and the non-i64 RuntimeDataBox fallback. |
| `test_collection_method_call.py` | direct-array exact-lane assertions | `149-229` | Pins the direct-array exact-lane get/set lowering and the non-origin fallback to the helper path. |
| `test_collection_method_call.py` | map / runtime-data assertions | `231-329` | Pins the MapBox raw-kernel surface and the RuntimeDataBox delete non-route. |

## Exact Callsites

- `self.assertEqual(spec[0], "nyash.array.slot_store_hii")` at `59`
- `self.assertEqual(spec[0], "nyash.array.slot_load_hi")` at `77`
- `self.assertEqual(spec[0], "nyash.array.slot_append_hh")` at `95`
- `self.assertEqual(spec[0], "nyash.runtime_data.get_hh")` at `113`
- `self.assertEqual(spec[0], "nyash.runtime_data.set_hhh")` at `132`
- `self.assertEqual(spec[0], "nyash.runtime_data.has_hh")` at `150`
- `self.assertEqual(spec[0], "nyash.array.slot_load_hi")` at `165`
- `self.assertEqual(spec[0], "nyash.runtime_data.has_hh")` at `180`
- `self.assertEqual(spec[0], "nyash.runtime_data.set_hhh")` at `199` / `218`
- `self.assertIsNone(spec)` and `self.assertIsNone(lowered)` at `228` / `247` / `264`
- `self.assertIn("direct_array_i64_base", ir_text)` at `169` / `197`

## Likely Miss Points

- The tests are mostly string-surface assertions, so they pin route selection rather than the full semantic payload.
- The direct-array tests depend on the exact env gate, the selected method name, and the direct-array origin facts in the resolver.
- `RuntimeDataBox.delete` is explicitly pinned as unrouted, but the rest of the consumer surface still needs the row400 attribution sweep.
- The policy tests verify fail-fast behavior for invalid policy values, but they do not own the implementation contract itself.

## Verdict

The tests pin the current route behavior well enough to keep the row400 consumer attribution honest, but they are still surface assertions rather than deep semantic proofs.
