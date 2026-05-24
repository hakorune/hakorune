---
Status: Landed
Date: 2026-05-24
Scope: select the next row after the C runner evidence contract refresh.
Blocker: MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-02-MIMALLOC-COMPARISON-C-RUNNER-EVIDENCE-CONTRACT-REFRESH.md
  - docs/development/current/main/phases/phase-294x/294x-229-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_guard.sh
  - tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_ledger_diagnostics_guard.sh
---

# 295x-03 Mimalloc Comparison Post C Runner Evidence Row Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-POST-C-RUNNER-EVIDENCE-ROW-SELECTION-001
```

Select:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001
```

## Why

The explicit C mimalloc runner evidence contract is refreshed for phase-295x.
The next useful comparison step is to refresh the existing C-vs-Hako result
ledger and diagnostics against the current evidence surfaces.

This row should not create a new result owner. It should reuse the existing
MIMAP-454A / MIMAP-455A ledger path and keep performance/memory conclusions
closed.

## Stop Line

The refresh row must not:

- run repeated or heavy benchmark packs;
- make performance or memory winner claims;
- change the C runner output schema;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Next Row

Implement:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-295X-REFRESH-001
```

Expected validation should run the phase-295x C runner evidence refresh guard
and the existing MIMAP-454A / MIMAP-455A result ledger guards at L2.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
