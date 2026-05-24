---
Status: Landed
Date: 2026-05-24
Scope: accept a narrow `.hako` realloc/aligned app through the hako EXE memory runner.
Blocker: MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-16-MIMALLOC-COMPARISON-REALLOC-ALIGNED-CONTRACT-REFRESH.md
  - apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako
  - apps/hako-alloc-mimalloc-comparison-realloc-aligned-slice-proof/main.hako
  - tools/allocator/hako_exe_memory_runner.sh
  - tools/checks/k2_wide_phase295x_realloc_aligned_hako_exe_acceptance_guard.sh
---

# 295x-17 Mimalloc Comparison Realloc/Aligned Hako EXE Acceptance

## Decision

Close:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-HAKO-EXE-ACCEPTANCE-295X-001
```

The existing realloc/aligned proof app remains the VM/MIR model guard for the
full branch-heavy proof shape. This row adds a separate exact-EXE-friendly app:

```text
apps/hako-alloc-mimalloc-comparison-realloc-aligned-exe-proof/main.hako
```

The new app uses the same comparison-facing operation contract selected by
`295x-15` and refreshed by `295x-16`:

- `workload=representative-realloc-aligned-v0`;
- `operation_family=realloc-aligned`;
- `operation_sequence_id=representative-realloc-aligned-v0-seq`;
- `free_order_id=ascending-release-v0`;
- matching allocation/free/requested/realloc/aligned count fields.

The app intentionally avoids the legacy proof helper shape that trips the
current exact-EXE PHI lowering path. The model proof is still covered by the
existing VM/MIR guard.

## Follow-On

Select:

```text
MIMALLOC-COMPARISON-REALLOC-ALIGNED-EVIDENCE-295X-RUN-001
```

Reason: both sides now expose executable evidence for the same
realloc/aligned workload family. The next row should run C mimalloc and `.hako`
through the normalizer and require count/requested/realloc/aligned parity while
keeping moved/copy/RSS as side-by-side evidence only.

## Stop Line

This row does not:

- make moved/copy/RSS parity claims;
- add benchmark warmup or final summary statistics;
- make performance or memory winner claims;
- replace the process allocator or install hooks;
- enable provider packages, DLL generation, provider activation, provider API
  execution, backend matchers, or `#[global_allocator]`;
- open worker/TLS, true threads, atomics, remote-free stress, abandoned heap
  stress, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_realloc_aligned_hako_exe_acceptance_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
