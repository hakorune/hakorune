# 293x-1127 MIMAP-497A Post Presentation Extension Follow-On Extension Plan Row Selection

Status: landed
Date: 2026-05-22

## Decision

Select the next narrow allocator row after the presentation extension
follow-on extension plan. The next behavior row may open an additional
follow-on extension seam over the stabilized follow-on-ready extension pack
without rerunning benchmarks or reopening inactive allocator/provider ladders.

## Candidate Next Rows

- presentation extension follow-on extension pilot
- presentation extension follow-on extension plan closeout
- presentation-only extension row

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
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
```

## Decision Result

Selected:

```text
MIMAP-498A Allocator Comparison C Mimalloc Result Presentation Extension Follow-On Extension Pilot
```

The follow-on extension plan already fixed the admissible follow-on-ready
extension inputs and closed stop-line contract. The next useful boundary is to
open the first follow-on extension behavior row over the stabilized
follow-on-ready extension pack instead of adding closeout indirection first.
