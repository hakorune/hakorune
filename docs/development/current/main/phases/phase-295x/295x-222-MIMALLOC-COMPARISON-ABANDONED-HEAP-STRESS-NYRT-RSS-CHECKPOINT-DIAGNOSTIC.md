---
Status: Landed
Date: 2026-05-25
Scope: add env-gated NyRT self-RSS checkpoints for the abandoned-heap stress path.
Blocker: MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002
Related:
  - docs/development/current/main/phases/phase-295x/295x-221-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-CLOSEOUT.md
  - docs/development/current/main/phases/phase-295x/295x-223-MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN.md
  - crates/nyash_kernel/src/rss_observe.rs
  - src/runtime/rss_observe.rs
  - crates/nyash_kernel/src/entry.rs
  - docs/reference/environment-variables.md
  - tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_diagnostic_guard.sh
---

# 295x-222 Abandoned Heap Stress NyRT RSS Checkpoint Diagnostic

## Decision

Close:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-002
```

Add the NyRT entry diagnostic controlled by:

```text
HAKO_NYRT_RSS_CHECKPOINTS=1
```

When enabled, exact-EXE stderr contains stable one-line checkpoints:

```text
[nyrt/rss] checkpoint=entry_start rss_bytes=...
[nyrt/rss] checkpoint=after_ring0 rss_bytes=...
[nyrt/rss] checkpoint=after_runtime_hooks rss_bytes=...
[nyrt/rss] checkpoint=after_plugin_host rss_bytes=...
[nyrt/rss] checkpoint=before_ny_main rss_bytes=...
[nyrt/rss] checkpoint=after_ny_main rss_bytes=...
```

This is a diagnostic-only self-observation hook. The default path emits no
extra output and does not change runtime behavior.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-ABANDONED-HEAP-STRESS-NYRT-RSS-CHECKPOINT-RUN-295X-002
```

The follow-on should run the empty no-output exact-EXE with
`HAKO_NYRT_RSS_CHECKPOINTS=1`, capture the checkpoint deltas, and classify
where the fixed RSS step appears.

## Stop Line

This row does not reduce baseline RSS, change compiler/linker/runtime behavior
when the env is unset, compute memory/performance winners, require RSS parity,
enable provider/DLL or host replacement seams, install hooks, or open
worker/TLS, atomics, remote-free stress, abandoned heap stress, OSVM page-source
parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_mimalloc_abandoned_heap_stress_nyrt_rss_checkpoint_diagnostic_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
