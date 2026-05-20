# 293x-1000 MIMAP-379A Post Provider Activation Dry-Run Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after provider activation dry-run
unsupported behavior. Provider activation is still closed; the next row should
either close out the dry-run pack or plan the first explicit activation evidence
row.

## Candidate Next Rows

- provider activation dry-run unsupported closeout
- provider activation first-pattern evidence plan
- provider activation explicit capability gate inventory

## Stop Lines

- No provider activation or provider calls until an explicit first-pattern row.
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
