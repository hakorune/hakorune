---
Status: Landed
Date: 2026-05-12
Scope: production hako_alloc usize migration preflight and task order.
Related:
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-18-HAKO-ALLOC-USIZE-FIELD-PROBE.md
  - src/mir/exact_numeric_backend_capability.rs
---

# 294x-19 Production Usize Migration Preflight

## Decision

Do not migrate production `hako_alloc` stored fields to `usize` yet.

The isolated 294x-18 probe is green under VM reference execution, but
production facade proofs are direct EXE / non-VM routes. Those routes still
fail fast for exact numeric typed-object storage until native exact numeric
slots and field get/set ABI are implemented.

## Recommended Order

1. Keep production `hako_alloc` state on `i64`.
2. Resume mimalloc algorithm rows using the current-lane `i64` boundary.
3. Add native exact numeric typed-object slots.
4. Add exact numeric field get/set ABI.
5. Reopen production field migration by field group.

## Stop Line

`HakoAllocUsizeFieldProbe` remains probe-only. A green VM probe must not be
treated as production hako_alloc migration readiness.

## Verification

```bash
cargo test -q exact_numeric_backend_capability --lib
bash tools/checks/current_state_pointer_guard.sh
```
