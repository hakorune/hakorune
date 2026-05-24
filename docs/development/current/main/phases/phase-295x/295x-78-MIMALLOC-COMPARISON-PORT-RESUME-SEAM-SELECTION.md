---
Status: Landed
Date: 2026-05-25
Scope: choose the next narrow .hako mimalloc port seam after timing surfaces were separated.
Related:
  - docs/development/current/main/phases/phase-295x/295x-77-MIMALLOC-COMPARISON-HAKO-BODY-TIMING-FEASIBILITY-SELECTION.md
  - docs/development/current/main/design/mimalloc-comparison-execution-ssot.md
---

# 295x-78 Port Resume Seam Selection

## Blocker

```text
MIMALLOC-COMPARISON-PORT-RESUME-SEAM-SELECTION-295X-001
```

## Decision

Resume `.hako` mimalloc porting through a reuse-cycle small-block workload:

```text
MIMALLOC-COMPARISON-REUSE-CYCLE-SMALL-WORKLOAD-CONTRACT-295X-001
```

The new workload should extend `representative-small-block-v0` without opening
threads, remote-free stress, atomics, abandoned heaps, provider seams, or host
replacement:

```text
workload=representative-reuse-cycle-small-v0
operation_family=reuse-cycle-small
operation_sequence_id=representative-reuse-cycle-small-v0-seq
free_order_id=even-odd-release-then-reacquire-v0
```

## Why This Seam

The current comparison pack already covers:

```text
small-block
realloc-aligned
mixed-small
huge-ish
```

The next allocator-facing behavior should test whether a page-local free-list
can be consumed and reused in the same workload. That is closer to mimalloc's
page/block behavior than another presentation row, but still stays inside the
single-threaded, page-local `.hako` model already used by the comparison apps.

## Expected Contract Shape

The follow-on row should add a C runner workload and a `.hako` exact-EXE app
with matching operation identity and stable count evidence:

```text
allocation_count:
  first allocation wave + reacquire wave

free_count:
  all allocated blocks released by the end

requested_bytes:
  sum of both allocation waves

reuse_cycle_count:
  evidence-only for the second wave

winner_claim:
  0
```

`reuse_cycle_count` is not a performance claim. It only states that the workload
contains a second allocation wave after free-list population.

## Stop Line

This row does not implement the workload, compute speed winners, compute RSS
winners, require timing parity, reopen `.hako` body timing, change runtime
behavior, broaden `usize` migration, or open provider/DLL/replacement/hook/
global allocator seams, worker/TLS, atomics, remote-free stress, or abandoned
heap stress.

## Verification

```bash
bash tools/checks/k2_wide_phase295x_port_resume_seam_selection_guard.sh
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```
