---
Status: Landed
Date: 2026-05-25
Scope: phase-295x native multi-worker substrate stress evidence on the comparison lane
Blocker: MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-210-MIMALLOC-COMPARISON-PAR-STRESS-SELECTION.md
  - tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_evidence_guard.sh
  - tools/allocator/mimalloc_parallel_substrate_stress_runner.py
---

# 295x-211 PAR Stress Evidence

## Decision

Close:

```text
MIMALLOC-COMPARISON-PAR-STRESS-EVIDENCE-295X-002
```

Ran the native multi-worker substrate stress as a stable comparison evidence
row and recorded its deterministic worker/TLS/atomic/remote-free counts.

## Evidence

The native stress fixture reports the following stable shape:

```text
worker_count=4
iterations_per_worker=64
expected_remote_free_count=256
observed_remote_free_count=256
drained_nodes=256
payload_sum_nonzero=1
```

The runner normalizes the stress report into:

```text
output_contract=mimalloc-comparison-par-stress-evidence-v0
cargo_test_target=nyash_kernel
cargo_test_filter=mimalloc_parallel_substrate_stress
cargo_test_passed=1
```

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-PAR-STRESS-PRESENTATION-295X-002
```

## Stop Line

This row does not broaden provider/DLL or host replacement seams, install
hooks, change default runtime behavior, compute winner claims, or make RSS
parity claims unless this card explicitly says so.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_parallel_substrate_stress_evidence_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
