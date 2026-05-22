# 293x-1191 MIMAP-561A Post Presentation-Only Extension Pilot Row Selection

Status: landed
Date: 2026-05-22

## Decision

Select the next narrow allocator row after the presentation-only extension
pilot. The lane now has an explicit presentation-only extension seam over the
landed comparison-ready pack while benchmark reruns, allocator/provider ladders,
and explicit runner execution remain closed.

## Candidate Next Rows

- presentation-only extension closeout
- deeper explicit C mimalloc runner planning row
- presentation-only extension follow-on row

## Stop Lines

- No repeated or heavy benchmark pack.
- No process allocator replacement.
- No hook installation.
- No backend matcher additions.
- No `#[global_allocator]`.
- No provider package / DLL generation.
- No hidden env, implicit discovery, or process-global activation config.
- No source-level worker-local or concurrency surface.
- No worker/thread execution.
- No explicit C mimalloc runner execution.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_allocator_comparison_c_mimalloc_result_presentation_only_extension_pilot_guard.sh --level L2
```

## Decision Result

Selected:

```text
MIMAP-562A Allocator Comparison C Mimalloc Result Presentation-Only Extension Closeout
```

The pilot already opened the presentation-only extension seam and fixed the
first narrow extension report shape. The next useful boundary is to close out
the pilot pack before any deeper runner planning or broader presentation row
builds on it.
