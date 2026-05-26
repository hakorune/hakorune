---
Status: Active
Date: 2026-05-27
Scope: phase-296x mimalloc benchmark contract lane.
Related:
  - docs/development/current/main/design/mimalloc-benchmark-dll-roadmap-ssot.md
  - docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
  - docs/development/current/main/phases/phase-295x/README.md
---

# Phase 296x - Mimalloc Benchmark Contract

Phase-296x starts after the first `.hako` mimalloc port pass and the phase-295x
comparison rows landed.

## Goal

Turn the existing `.hako` mimalloc proof/comparison work and the external
`hakmem` benchmark corpus into a stable benchmark contract.

This phase prepares benchmark input and result adapters first. It does not
activate DLL/provider or process-wide allocator replacement.

## Stop Line

Keep closed unless a later row explicitly opens them:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- performance or memory winner claims.

## Current Work

Read the taskboard:

```text
docs/development/current/main/phases/phase-296x/296x-90-mimalloc-benchmark-taskboard.md
```

The external benchmark corpus root is:

```text
/home/tomoaki/git/hakmem_20260525_extracted/hakmem
```
