---
Status: Landed
Date: 2026-05-24
Scope: consolidate how phase-295x compares `.hako` / hako_alloc evidence with
  C mimalloc runner evidence.
Blocker: MIMALLOC-COMPARISON-POST-RESULT-LEDGER-ROW-SELECTION-001
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-04-MIMALLOC-COMPARISON-RESULT-LEDGER-REFRESH.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
---

# 295x-05 Mimalloc Comparison Method Consolidation

## Decision

Close:

```text
MIMALLOC-COMPARISON-POST-RESULT-LEDGER-ROW-SELECTION-001
```

Before adding more comparison rows, consolidate the comparison method in the
phase-295x SSOT.

Select:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001
```

## Why

The current evidence chain is useful, but it is not yet a final performance
claim:

- `.hako` / `hako_alloc` V5 proves the vertical-slice schema and allocator
  model evidence;
- the C mimalloc runner proves explicit runner output and memory-use evidence;
- the result ledger compares available scalar evidence;
- single-run RSS and non-identical workload shapes are not enough to declare a
  winner.

The comparison method must therefore distinguish contract/evidence comparison
from later apples-to-apples benchmark comparison.

## Stop Line

This row does not:

- change any workload or runner;
- add repeated benchmark execution;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Next Row

Implement:

```text
MIMALLOC-COMPARISON-RESULT-LEDGER-CLOSEOUT-295X-001
```

The closeout should verify the refreshed result ledger pack after the comparison
method is fixed.

## Verification

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
