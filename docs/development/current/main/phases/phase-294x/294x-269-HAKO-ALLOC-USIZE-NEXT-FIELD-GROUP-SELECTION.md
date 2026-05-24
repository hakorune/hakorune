---
Status: Landed
Date: 2026-05-24
Scope: select the next row after the page-heap usize comparison refresh.
Blocker: HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-011
Related:
  - docs/development/current/main/phases/phase-294x/294x-268-MIMALLOC-COMPARISON-VSLICE-PAGE-HEAP-USIZE-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-269 Hako Alloc Usize Next Field Group Selection

## Decision

Close:

```text
HAKO-ALLOC-USIZE-FIELD-GROUP-NEXT-SELECTION-011
```

Do not select another exact `usize` production field group in phase-294x.

Select phase closeout readiness instead:

```text
PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001
```

## Why

The current comparison vertical slice has enough exact `usize` storage:

- facade-local stats;
- page-map release and result-ledger counters;
- page-queue stats and direct-page indexes;
- object lifecycle facade counters;
- C mimalloc comparison result-ledger counters;
- page-heap non-id size/count/capacity/requested-byte fields.

The remaining visible hako_alloc numeric fields are either:

- signed identity/index seams;
- negative sentinel-bearing fields;
- status/reason vocabulary fields;
- bool-like stop-line evidence;
- report mirror payloads;
- pointer-like payloads or provider/worker/atomic seams.

Those categories need their own semantic contracts. They should not be drained
as part of phase-294x's exact `usize` foundation.

## Stop Line

The closeout row must not:

- migrate any additional production field;
- migrate page/handle ids or sentinel-bearing indexes;
- open provider package / DLL generation, provider activation, host allocator
  replacement, hooks, backend matchers, worker/TLS, atomics, remote-free stress,
  abandoned heap stress, or `#[global_allocator]`;
- claim native allocator replacement or performance parity.

## Next Row

Implement:

```text
PHASE-294X-USIZE-COMPARISON-CLOSEOUT-001
```

Expected validation should run the refreshed comparison guard and prove the
taskboard no longer points at another exact `usize` field-group migration.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
