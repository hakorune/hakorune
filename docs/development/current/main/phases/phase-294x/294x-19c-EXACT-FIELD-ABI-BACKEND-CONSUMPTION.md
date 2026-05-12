---
Status: Complete
Date: 2026-05-12
Scope: Python LLVM backend consumption of exact typed-object field ABI.
Related:
  - docs/development/current/main/phases/phase-294x/294x-19b-EXACT-NUMERIC-FIELD-GET-SET-ABI.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - src/mir/exact_numeric_backend_capability.rs
  - src/llvm_py/instructions/field_access.py
  - src/llvm_py/instructions/newbox.py
---

# 294x-19c Exact Field ABI Backend Consumption

## Decision

After the kernel exact field ABI exists, the Python LLVM backend may consume
typed-object plans that contain exact numeric storage. This does not open exact
numeric arithmetic lowering and does not migrate production `hako_alloc` fields.

## Contract

- MIR JSON `typed_object_plans` are carried into the Python LLVM builder.
- Boxes whose typed-object plan contains exact storage use `nyash.object.*`
  typed-object handles instead of the legacy name-keyed `nyash.instance.*`
  helper family.
- `newbox` for an exact-storage typed object calls
  `nyash.object.new_typed_hi(type_id, field_count)`.
- `ny_main` registers exact typed-object slot layouts before calling user code.
- Exact unsigned fields lower to the `u64` transport helper lane.
- Exact signed fields lower to the `i64` transport helper lane.
- Exact field set status is checked; failure emits a backend trap instead of
  silently continuing.
- Unsupported backends keep the exact-storage capability gate closed.

## Non-Goals

- No WASM exact typed-object lowering.
- No exact arithmetic/compare/shift backend lowering.
- No production `hako_alloc` field migration.
- No wrapping arithmetic vocabulary.

## Landed

- MIR JSON `typed_object_plans` now reach the Python LLVM builder.
- `ny_main` registers exact typed-object layouts through `nyash.object.*`
  layout helpers before user code runs.
- `newbox` for exact-storage typed-object plans now creates
  `nyash.object.new_typed_hi` handles.
- Field get/set for exact-storage typed objects now lowers to slot-based
  `nyash.object.field_*` helpers.
- Exact field set status is checked with a trap on failure.
- The backend capability gate now allows exact typed-object storage only for
  the Python LLVM helper path; exact operation route facts and unsupported
  backends remain fail-fast.

Still closed after this row:

- exact add/sub/mul/compare/shift backend lowering;
- WASM exact typed-object lowering;
- production `hako_alloc` field migration.

## Acceptance

```bash
python3 -m unittest src.llvm_py.tests.test_typed_user_box_field_access
python3 -m unittest src.llvm_py.tests.test_mir_reader_builder_input
cargo test -q --lib exact_numeric_backend_capability
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
```
