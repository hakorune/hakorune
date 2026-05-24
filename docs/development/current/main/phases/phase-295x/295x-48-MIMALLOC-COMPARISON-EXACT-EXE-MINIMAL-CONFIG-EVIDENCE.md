---
Status: Landed
Date: 2026-05-25
Scope: phase-295x exact-EXE minimal runtime config evidence.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-47-MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-PILOT.md
---

# 295x-48 Exact-EXE Minimal Config Evidence

## Blocker

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-EVIDENCE-295X-001
```

## Decision

Run root versus generated-empty runtime config evidence across the selected
phase-295x `.hako` exact-EXE workloads:

```text
tools/allocator/hako_minimal_config_evidence.py
```

## Contract

The evidence contract is:

```text
hako-exact-exe-minimal-config-evidence-v0
```

Each row records:

```text
workload
operation_family
root_external_peak_rss_bytes
empty_external_peak_rss_bytes
rss_reduction_bytes
winner_claim=0
```

The runner verifies that root and empty runtime config preserve the workload
identity, operation family, operation sequence, free order, allocation/free
counts, requested bytes, and closed provider/replacement/hook/global allocator
seams.

## Follow-On

```text
MIMALLOC-COMPARISON-EXACT-EXE-MINIMAL-CONFIG-CLOSEOUT-295X-001
```

## Stop Line

This row does not change default NyRT behavior, disable plugins by default,
alter provider selection, compute memory winners, open provider/DLL/replacement
/hook/global allocator seams, or require RSS parity.
