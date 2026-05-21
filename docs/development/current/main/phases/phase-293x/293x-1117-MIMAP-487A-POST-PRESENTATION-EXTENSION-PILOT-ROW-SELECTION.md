# 293x-1117 MIMAP-487A Post Presentation Extension Pilot Row Selection

Status: landed
Date: 2026-05-22

## Decision

Select the next narrow allocator row after the presentation extension pilot. The
lane now has an extension-ready seam over the stabilized broader presentation
pack, while benchmark reruns and allocator/provider ladders remain closed.

## Candidate Next Rows

- presentation extension closeout
- presentation extension follow-on plan
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
MIMAP-488A Allocator Comparison C Mimalloc Result Presentation Extension Closeout
```

The extension pilot already opened the next behavior seam and fixed the first
extension-ready report shape. The next useful boundary is to close out that
pilot pack before any extension follow-on plan or broader row builds on it.
