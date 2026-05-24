---
Status: Landed
Date: 2026-05-24
Scope: close `.hako` allocation/free count evidence refresh for phase-295x.
Blocker: MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-13-MIMALLOC-COMPARISON-HAKO-COUNT-EVIDENCE-REFRESH.md
  - tools/checks/k2_wide_phase295x_count_evidence_closeout_guard.sh
  - tools/checks/k2_wide_phase295x_hako_count_evidence_refresh_guard.sh
---

# 295x-14 Mimalloc Comparison Count Evidence Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-COUNT-EVIDENCE-CLOSEOUT-295X-001
```

The same-workload comparison report now carries matching `.hako` and C mimalloc
allocation/free counts:

- `hako_allocation_count=64`;
- `c_allocation_count=64`;
- `allocation_count_delta=0`;
- `hako_free_count=64`;
- `c_free_count=64`;
- `free_count_delta=0`.

Select:

```text
MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION-295X-001
```

The next row should choose whether to improve the current small-block workload
or add one new same-workload family. It must keep provider/replacement and
winner claims closed.

## Stop Line

This row does not:

- change allocation behavior or workload shape;
- change the C runner output schema;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_count_evidence_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
