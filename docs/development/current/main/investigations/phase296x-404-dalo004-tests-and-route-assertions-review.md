---
Status: Draft
Date: 2026-05-30
Scope: row404 DALO-004 tests and route assertions review
Related:
  - docs/development/current/main/phases/phase-296x/296x-404-COLLECTION-METHOD-DIRECT-ARRAY-LANE-OWNER-SELECTION.md
  - src/llvm_py/tests/test_runtime_data_dispatch_policy.py
  - src/llvm_py/tests/test_collection_method_call.py
---

# DALO-004 Tests And Route Assertions Review

## Input

- `src/llvm_py/tests/test_runtime_data_dispatch_policy.py`
- `src/llvm_py/tests/test_collection_method_call.py`

## Route Assertion Table

| file | assertion family | line_hint | note |
| --- | --- | --- | --- |
| `test_runtime_data_dispatch_policy.py` | policy selector assertions | `42-280` | Pins the array-mono policy default, the runtime-data-only switch, invalid-value fail-fast, and the array-field map ABI. |
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

- The tests are mostly surface assertions, so they pin route selection rather than the full semantic payload.
- The direct-array tests depend on the exact env gate, the selected method name, and the direct-array origin facts in the resolver.
- `RuntimeDataBox.delete` is explicitly pinned as unrouted, but the rest of the consumer surface still needs the route-order split.
- The policy tests verify fail-fast behavior for invalid policy values, but they do not own the implementation contract itself.

## Verdict

The tests pin the current route split well enough to keep the row404 direct-array lane selection honest, but they are still surface assertions rather than deep semantic proofs.
