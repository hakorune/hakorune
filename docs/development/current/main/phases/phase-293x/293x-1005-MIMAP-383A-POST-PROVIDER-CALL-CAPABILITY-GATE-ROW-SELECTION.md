# 293x-1005 MIMAP-383A Post Provider Call Capability Gate Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after the provider-call capability gate
inventory. The provider-call capability gate is now modeled, but provider API
calls, host replacement, hooks, backend matcher additions, and global allocator
install remain closed.

## Candidate Next Rows

- provider-call capability gate diagnostics
- provider-call capability gate closeout
- provider-call dry-run unsupported behavior

## Stop Lines

- No provider API calls until an explicit provider-call behavior row.
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
