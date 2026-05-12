---
Status: Complete
Date: 2026-05-12
Scope: exact numeric typed-object field get/set ABI.
Related:
  - docs/development/current/main/phases/phase-294x/294x-19a-NATIVE-EXACT-NUMERIC-TYPED-SLOTS.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - crates/nyash_kernel/src/exports/typed_object.rs
---

# 294x-19b Exact Numeric Field Get/Set ABI

## Decision

Exact numeric typed-object slots need an explicit ABI before any non-VM backend
lowering can write or read them.

This row adds runtime helper entry points for exact signed and exact unsigned
slot access. Backend lowering remains a later step, but the helper ABI must
already reject wrong slot kinds and out-of-range writes.

## ABI Shape

Unsigned exact slots:

```text
nyash.object.field_get_u64_hii(handle, slot) -> u64
nyash.object.field_set_u64_hiu(handle, slot, value) -> i64
```

Signed exact slots:

```text
nyash.object.field_get_i64_hii(handle, slot) -> i64
nyash.object.field_set_i64_hii(handle, slot, value) -> i64
```

The runtime helper names intentionally describe the transport lane, not the
stored slot width. The slot layout still owns the actual storage kind and range.

## Contract

- Unsigned exact slots use an unsigned ABI and range-check by slot storage kind.
- Signed exact slots use a signed ABI and range-check by slot storage kind.
- `usize` uses the current Rust target `usize::MAX` range for the helper
  contract.
- Narrow integer slots reject writes outside their exact range and keep the
  previous value unchanged.
- Legacy `field_get_hii` / `field_set_hii` remain legacy `i64` helpers and do
  not become exact numeric fallback routes.
- Invalid/missing/wrong-kind gets return zero. Lowering must consult the slot
  storage contract before using the exact helper; the getter is not a dynamic
  type-discovery API.
- Failed set operations return `0`; successful set operations return `1`.

## Non-Goals

- No Python/LLVM lowerer consumption yet.
- No backend capability gate opening yet.
- No production `hako_alloc` field migration.
- No wrapping arithmetic vocabulary.

## Landed

- `nyash_kernel` now exports exact unsigned and signed field helper lanes for
  typed-object slots.
- Slot storage kind owns range checks: `usize`, fixed-width unsigned, `isize`,
  fixed-width signed, and legacy `i64` are checked separately.
- Wrong-kind and out-of-range writes fail with `0` and leave the previous value
  unchanged.
- Legacy `field_get_hii` / `field_set_hii` stay on the legacy `i64` ABI and do
  not mutate exact numeric slots.

Still closed after this row:

- Python/LLVM lowerer consumption;
- backend capability-gate opening;
- production `hako_alloc` field migration.

## Acceptance

```bash
cargo test -q -p nyash_kernel typed_object
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
```
