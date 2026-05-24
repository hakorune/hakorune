---
Status: Landed
Date: 2026-05-24
Scope: refresh the C-vs-Hako comparison result ledger for phase-295x.
Blocker: MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-03-MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION.md
  - docs/development/current/main/phases/phase-295x/295x-02-MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-229-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh
---

# 295x-04 Mimalloc Comparison Result Ledger Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001
```

The existing C-vs-Hako comparison result ledger and diagnostics remain stable
against the phase-295x evidence chain:

- current `.hako` / `hako_alloc` vertical slice;
- explicit C mimalloc runner evidence contract;
- existing MIMAP-454A result ledger;
- existing MIMAP-455A result-ledger diagnostics.

## Stop Line

This row does not:

- add benchmark repetition or a new workload family;
- make performance or memory winner claims;
- change the C runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Next Row

Return to row selection from:

```text
MIMALLOC-COMPARISON-POST-RESULT-LEDGER-ROW-SELECTION-001
```

Expected direction: either close the comparison ledger pack or select the next
`.hako` port seam that directly improves the comparison workload.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_comparison_result_ledger_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
