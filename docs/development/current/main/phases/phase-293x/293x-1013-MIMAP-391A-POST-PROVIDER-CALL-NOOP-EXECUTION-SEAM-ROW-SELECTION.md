# 293x-1013 MIMAP-391A Post Provider Call No-Op Execution Seam Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after provider-call no-op execution seam
pilot. The execution boundary can now be crossed in model space, but actual
provider API calls, host replacement, hooks, backend matcher additions, and
global allocator install remain closed.

## Candidate Next Rows

- provider-call no-op execution seam closeout
- provider-call real API execution preflight
- provider-call real API execution first-pattern plan

## Stop Lines

- No actual provider API calls until an explicit real provider-call execution row.
- No hidden env, implicit discovery, or process-global activation config.
- No host allocator replacement, hooks, or `#[global_allocator]`.
- No backend `.inc` matcher by app, box, owner, or row name.
- No source-level worker-local or concurrency surface.
- No cross-function `Result` direct ABI or runtime sum materialization.

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
