---
Status: Landed
Date: 2026-05-23
Scope: hako-side pure-first EXE memory-use evidence runner.
Blocker: MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-001
Related:
  - docs/development/current/main/phases/phase-294x/294x-60-MIMALLOC-COMPARISON-POST-CLOSEOUT-FOLLOW-ON-SELECTION.md
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/checks/k2_wide_hako_alloc_mimalloc_comparison_hako_memory_evidence_runner_guard.sh
---

# 294x-61 Mimalloc Comparison Hako Memory Evidence Runner

## Decision

Add a small hako-side memory-use evidence runner.

The runner builds a selected comparison `.hako` app through the exact-MIR EXE
route, runs the generated EXE as an external process, and records stable
evidence:

```text
hako_exe_runner
output_contract
workload
result_code
run_count
requested_bytes
committed_bytes
peak_rss_bytes
memory_usage_evidence
output_summary_ok
closed stop-line fields
summary
```

This fills the immediate asymmetry from `294x-59`: hako-side memory evidence can
now be captured without changing allocator semantics.

## Stop Line

This row does not open:

- process allocator replacement;
- provider package generation;
- hook installation;
- `#[global_allocator]`;
- worker/TLS behavior;
- remote-free stress;
- atomic bitmap execution;
- performance or memory-use winner claims.

## Follow-On

```text
MIMALLOC-COMPARISON-HAKO-MEMORY-EVIDENCE-002:
  consume the hako EXE memory evidence in the comparison schema / presentation
  lane, or explicitly return to the next `usize` field-group row if no further
  comparison evidence is needed in this phase.
```

## Verification

```bash
bash tools/checks/k2_wide_hako_alloc_mimalloc_comparison_hako_memory_evidence_runner_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
