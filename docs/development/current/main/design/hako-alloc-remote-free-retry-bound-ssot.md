---
Status: SSOT
Decision: accepted
Date: 2026-05-16
Scope: MIMAP-039A hako_alloc remote-free retry bound cleanup.
Related:
  - docs/development/current/main/phases/phase-293x/293x-471-MIMAP-039A-REMOTE-FREE-RETRY-BOUND.md
  - lang/src/hako_alloc/memory/remote_free_policy_box.hako
---

# hako_alloc Remote-Free Retry Bound SSOT

## Decision

`HakoAllocRemoteFreePolicy.pushRetry(...)` must not carry a raw retry-bound
literal in the loop condition.

The retry limit is owned by:

```text
HakoAllocRemoteFreePolicy.maxPushRetries()
```

Current value:

```text
5
```

## Contract

```text
max_retries = HakoAllocRemoteFreePolicy.maxPushRetries()
loop (done == 0 && retries < max_retries)
```

This is a naming / ownership cleanup only. It does not change the retry count,
pointer atomic route set, remote-free list semantics, abandoned-owner policy,
provider activation, or host allocator replacement.

## Guard

```text
tools/checks/k2_wide_hako_alloc_remote_free_retry_bound_guard.sh
```

The guard rejects `retries < 5` in `pushRetry`, requires the named owner method,
and reruns the existing hako_alloc remote-free policy EXE proof.
