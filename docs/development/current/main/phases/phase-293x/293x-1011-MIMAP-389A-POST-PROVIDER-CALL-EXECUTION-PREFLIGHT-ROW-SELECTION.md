# 293x-1011 MIMAP-389A Post Provider Call Execution Preflight Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after provider-call execution capability
preflight. Provider-call execution readiness is now modeled, but actual provider
API calls, host replacement, hooks, backend matcher additions, and global
allocator install remain closed.

## Candidate Next Rows

- provider-call execution capability closeout
- provider-call explicit execution first-pattern plan
- provider-call no-op execution seam pilot

## Stop Lines

- No provider API calls until an explicit provider-call execution row.
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
