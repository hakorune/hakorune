# 293x-1002 MIMAP-381A Post Provider Activation Modeled Open Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after the provider activation modeled-open
pilot. Activation is now open only in model space; provider calls, host
replacement, hooks, backend matcher additions, and global allocator install
remain closed.

## Candidate Next Rows

- provider activation modeled-open diagnostics
- provider activation modeled-open closeout
- provider call capability gate inventory

## Stop Lines

- No provider API calls until an explicit provider-call row.
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
