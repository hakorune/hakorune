---
Status: Complete
Date: 2026-05-12
Scope: first production `hako_alloc` field-group migration from `i64` to
  exact `usize`.
Related:
  - docs/development/current/main/phases/phase-294x/294x-19d-EXACT-NUMERIC-OP-BACKEND-SUBSET.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - lang/src/hako_alloc/memory/allocator_facade_box.hako
---

# 294x-19e Hako Alloc Production Usize Facade Stats

## Decision

Production `hako_alloc` field migration reopens one non-negative field group at
a time. The first group is the public facade's event counters:

- `HakoAllocProductionFacade.alloc_count`
- `HakoAllocProductionFacade.free_count`
- `HakoAllocProductionFacade.reject_count`

These counters are monotonic, facade-local, and do not participate in page
selection, free-list stack tops, byte accounting, or sentinel-bearing APIs. This
makes them the smallest production proof that exact `usize` stored fields can
cross VM/MIR/Python LLVM without changing allocator behavior.

## Contract

- The three facade counters are declared as `usize`.
- Existing facade proof apps keep the same observable accounting values.
- Page-local, heap, queue, handle, capacity, size, index, and byte-length
  production fields remain on `i64` until their own field-group row lands.
- No allocator-provider activation, hook install, `#[global_allocator]`, or
  process allocator replacement is introduced.
- Unsupported backends continue to fail fast through the exact numeric backend
  capability gate instead of silently falling back to legacy `i64` field
  lowering.

## Why Facade Stats First

`NUMERIC_FIELDS.md` previously listed page capacity as an early structural
candidate while the migration order preferred low-risk stats. This row resolves
that ordering: start with facade counters to prove the production boundary, then
move inward to queue/page stats, capacity, stack-top, byte-length, and index
groups only after each group has its own invariant and guard.

## Landed

- `HakoAllocProductionFacade` now stores `alloc_count`, `free_count`, and
  `reject_count` as exact `usize` fields.
- The pure-first C shim now consumes exact typed-object slot storage by
  registering MIR-owned layout storage tags before user code, then using exact
  field get/set helpers for unsigned/signed non-legacy slots.
- The M46 production facade guard now requires those fields to appear as
  `usize` exact-storage fields in MIR typed-object plans.
- The M50 production facade stress guard verifies the same exact field group
  while keeping behavior parity.
- The hako_alloc numeric inventory is updated so remaining fields stay
  intentionally `i64`.

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_exe_guard.sh
bash tools/checks/k2_wide_hako_alloc_production_facade_stress_exe_guard.sh
bash apps/hako-alloc-usize-field-probe/test.sh
cargo test -q -p nyash_kernel typed_object
python3 -m unittest src.llvm_py.tests.test_typed_user_box_field_access src.llvm_py.tests.test_exact_numeric_ops
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/dev_gate.sh quick
```
