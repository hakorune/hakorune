---
Status: Landed
Date: 2026-05-26
Scope: select the first backend-split comparison seam after the long-OR compiler slices landed.
Blocker: MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-250-MIMALLOC-COMPARISON-REMOTE-FREE-LONG-OR-BRANCH-CONTEXT-AND-NOT-IMPLEMENTATION.md
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/main.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_collect_only.hako
  - apps/mimalloc-remote-free-minimum-benchmark-run-proof/remote_free_publish_collect_cycle.hako
---

# 295x-251 Remote-Free Backend-Split Selection

## Decision

Close:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-SELECTION-295X-002
```

Select the first backend-split seam as a contract refresh row that keeps the
existing remote-free minimum benchmark workloads and introduces no new workload
family.

The selected split boundary is:

```text
same workload ids
same operation_repeat/warmup/sample policy
exact-exe-first output contract remains the baseline
add backend-split contract fields only (no native-C winner claims)
```

## Stop Line

This row does not open provider activation, DLL packaging, replacement/hooks,
`#[global_allocator]`, new thread/TLS/atomic seams, or native C/mimalloc winner
claims.

## Selected Next

Select:

```text
MIMALLOC-COMPARISON-REMOTE-FREE-BACKEND-SPLIT-CONTRACT-REFRESH-295X-002
```

The next row should refresh the backend-split contract over the existing
remote-free minimum benchmark proof apps without widening the workload set.
