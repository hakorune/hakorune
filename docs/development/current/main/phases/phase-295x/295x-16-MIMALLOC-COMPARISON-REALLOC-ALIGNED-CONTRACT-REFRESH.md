---
Status: Landed
Date: 2026-05-24
Scope: refresh the representative realloc/aligned workload contract.
Blocker: MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-15-MIMALLOC-COMPARISON-NEXT-WORKLOAD-SEAM-SELECTION.md
  - tools/allocator/c_mimalloc_explicit_runner.c
  - tools/allocator/c_mimalloc_explicit_runner.sh
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/allocator/mimalloc_comparison_memory_report.py
  - tools/checks/k2_wide_phase295x_realloc_aligned_contract_refresh_guard.sh
---

# 295x-16 Mimalloc Comparison Realloc/Aligned Contract Refresh

## Decision

Close:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-295X-REFRESH-001
```

The explicit C mimalloc runner now accepts:

```text
--workload representative-realloc-aligned-v0
```

and keeps the same base output contract:

```text
allocator-comparison-c-mimalloc-explicit-runner-v0
```

The new workload publishes the contract fields selected in `295x-15`:

- `operation_family=realloc-aligned`;
- `operation_sequence_id=representative-realloc-aligned-v0-seq`;
- `free_order_id=ascending-release-v0`;
- realloc/aligned count fields;
- moved/copy evidence fields;
- closed provider/replacement/hook fields.

The `.hako` realloc/aligned proof app also publishes the same operation family,
sequence id, free-order id, and model-side realloc/aligned evidence fields. Its
VM/MIR guard remains green.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001
```

Reason: the hako EXE memory runner can already execute the small-block proof
app, but the realloc/aligned proof app currently trips an exact-EXE backend
shape around PHI generation. The next row should fix or narrow that acceptance
before the full `.hako` vs C realloc/aligned evidence run.

## Stop Line

This row does not:

- run the full hako-vs-C realloc/aligned evidence comparison;
- make moved/copy/RSS parity claims;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- enable provider package / DLL generation, provider activation, provider API
  execution, process allocator replacement, hooks, backend matchers, or
  `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_realloc_aligned_contract_refresh_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
