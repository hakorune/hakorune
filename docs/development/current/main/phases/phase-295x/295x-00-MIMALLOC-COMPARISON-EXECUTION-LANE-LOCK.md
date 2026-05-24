---
Status: Landed
Date: 2026-05-24
Scope: lock phase-295x and select the first mimalloc comparison execution row.
Blocker: PHASE-294X-POST-CLOSEOUT-ROW-SELECTION-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-270-PHASE-294X-USIZE-COMPARISON-CLOSEOUT.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/README.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
---

# 295x-00 Mimalloc Comparison Execution Lane Lock

## Decision

Close:

```text
PHASE-294X-POST-CLOSEOUT-ROW-SELECTION-001
```

Activate:

```text
phase-295x mimalloc comparison execution
```

Select:

```text
MIMALLOC-COMPARISON-EXECUTION-ROW-SELECTION-001
```

## Why

Phase-294x closed the exact `usize` foundation and comparison-quality field
slice. Continuing to migrate unrelated fields would not improve the comparison
goal. The next useful work is to resume mimalloc-facing development from the
explicit C mimalloc evidence / `.hako` vertical-slice comparison boundary.

## Stop Line

Phase-295x does not open:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad exact `usize` field migration outside the comparison workload.

## Verification

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
