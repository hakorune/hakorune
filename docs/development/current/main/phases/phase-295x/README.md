---
Status: Active
Date: 2026-05-24
Scope: phase-295x mimalloc comparison execution lane.
Related:
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
  - docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
  - docs/development/current/main/phases/phase-294x/294x-270-PHASE-294X-USIZE-COMPARISON-CLOSEOUT.md
---

# Phase 295x - Mimalloc Comparison Execution

Phase-295x resumes mimalloc-facing development after phase-294x closed the
exact `usize` comparison-quality slice.

## Goal

Build the next narrow comparison execution path without opening process-wide
allocator replacement or provider/DLL packaging.

The phase should advance by small rows:

1. lock the lane and choose the first comparison execution blocker;
2. refresh or select the explicit C mimalloc runner evidence contract;
3. compare that evidence against the `.hako` / `hako_alloc` vertical slice;
4. only then choose the next `.hako` port seam that directly improves the
   comparison workload.

## Stop Line

Keep closed unless a future phase explicitly reopens them:

- provider package / DLL generation;
- provider activation and provider API execution;
- process allocator replacement, hooks, backend matchers, and
  `#[global_allocator]`;
- worker/TLS, true threads, atomics, remote-free stress, abandoned heap stress,
  and native allocator replacement claims;
- broad exact `usize` field migration outside the comparison workload.

## Current Work

Read the taskboard:

```text
docs/development/current/main/phases/phase-295x/295x-90-mimalloc-comparison-execution-taskboard.md
```
