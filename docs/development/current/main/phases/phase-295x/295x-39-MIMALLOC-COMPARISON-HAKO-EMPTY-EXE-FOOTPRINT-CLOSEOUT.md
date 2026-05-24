---
Status: Landed
Date: 2026-05-25
Scope: close empty exact-EXE footprint diagnostic and select runtime RSS checkpoints.
Blocker: MIMALLOC-COMPARISON-HAKO-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001
Related:
  - docs/development/current/main/phases/phase-295x/295x-38-MIMALLOC-COMPARISON-HAKO-EMPTY-EXE-FOOTPRINT-DIAGNOSTIC.md
  - tools/checks/k2_wide_phase295x_hako_empty_exe_footprint_closeout_guard.sh
---

# 295x-39 Mimalloc Comparison Hako Empty EXE Footprint Closeout

## Decision

Close:

```text
MIMALLOC-COMPARISON-HAKO-EMPTY-EXE-FOOTPRINT-CLOSEOUT-295X-001
```

The footprint diagnostic showed that the no-output control has essentially the
same RSS as the evidence-printing empty app, so evidence output is not the
primary fixed-cost source. The ELF PT_LOAD footprint is also much smaller than
the observed empty exact-EXE RSS, so the next seam should observe runtime entry
checkpoints rather than start broad shrinking.

## Selected Row

Select:

```text
MIMALLOC-COMPARISON-NYRT-RSS-CHECKPOINT-DIAGNOSTIC-295X-001
```

The next row should add an env-gated NyRT self-RSS checkpoint diagnostic:

```text
HAKO_NYRT_RSS_CHECKPOINTS=1
```

Expected checkpoint shape:

```text
[nyrt/rss] checkpoint=entry_start rss_bytes=...
[nyrt/rss] checkpoint=after_ring0 rss_bytes=...
[nyrt/rss] checkpoint=after_runtime_hooks rss_bytes=...
[nyrt/rss] checkpoint=after_plugin_host rss_bytes=...
[nyrt/rss] checkpoint=before_ny_main rss_bytes=...
[nyrt/rss] checkpoint=after_ny_main rss_bytes=...
```

This keeps observation inside the exact-EXE process and makes the fixed RSS
step location visible without changing runtime behavior when the env is unset.

## Stop Line

This row does not add the checkpoint implementation, reduce baseline RSS,
change compiler/linker/runtime behavior by default, compute memory/performance
winners, require RSS parity, enable provider/DLL or host replacement seams,
install hooks, or open worker/TLS, atomics, remote-free stress, abandoned heap
stress, OSVM page-source parity, or native allocator replacement claims.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_hako_empty_exe_footprint_closeout_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
