---
Status: Landed
Date: 2026-05-24
Scope: close the refreshed C-vs-Hako comparison result ledger pack for phase-295x.
Blocker: MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-05-MIMALLOC-COMPARISON-METHOD-CONSOLIDATION.md
  - docs/development/current/main/phases/phase-295x/295x-04-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md
  - tools/checks/k2_wide_phase295x_result_ledger_closeout_guard.sh
---

# 295x-06 Mimalloc Comparison Result Ledger Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001
```

The phase-295x comparison result ledger pack remains valid after the comparison
method consolidation. This closeout keeps the row as a contract/evidence
boundary and does not turn the result ledger into a performance or memory
winner claim.

Select:

```text
MIMALLOC-COMPARISON-SAME-WORKLOAD-295X-REFRESH-001
```

The next row should move from the broad result ledger to the same-workload
memory report path and execute the existing `.hako` and C mimalloc runners for
`representative-small-block-v0`.

## Stop Line

This row does not:

- add benchmark repetition or summary statistics;
- make performance or memory winner claims;
- change the C runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_result_ledger_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
