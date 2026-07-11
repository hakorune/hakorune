---
Status: Complete
Date: 2026-05-12
Scope: native typed-object slot representation for exact numeric fields.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - crates/nyash_kernel/src/exports/typed_object.rs
---

# 294x-19a Native Exact Numeric Typed Slots

## Decision

Native typed-object storage must be able to represent exact numeric slot kinds
before any backend is allowed to lower exact numeric field get/set operations.

This row adds the storage shape only. It does not open production
`hako_alloc` field migration and does not make exact field access lowerable.

## Contract

- Typed object runtime storage records each slot's storage kind.
- Exact numeric slot kinds, including `usize`, are distinct from legacy `i64`.
- Legacy `i64` field helpers must not mutate exact numeric slots as if they
  were `i64`.
- Existing untyped/default typed object helpers keep their old `i64` behavior.
- Backend capability gates may remain closed until `294x-19b` adds the exact
  field get/set ABI.

## Landed

- `crates/nyash_kernel/src/exports/typed_object.rs` now keeps a per-type slot
  layout registry.
- New typed objects materialize each slot with its declared storage kind.
- `usize` and other exact numeric storage tags are held separately from legacy
  `i64`.
- The legacy `field_get_hii` / `field_set_hii` helpers keep working for
  default `i64` slots, while exact numeric slots are not mutated through that
  legacy ABI.

## Non-Goals

- No production `hako_alloc` field migration.
- No exact numeric field get/set lowering.
- No exact numeric op lowering for non-VM backends.
- No allocator-provider activation or host allocator replacement.

## Acceptance

```bash
cargo test -q -p nyash_kernel typed_object
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
```
