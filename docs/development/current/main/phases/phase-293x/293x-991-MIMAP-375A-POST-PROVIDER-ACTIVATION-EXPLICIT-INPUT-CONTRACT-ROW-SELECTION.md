# 293x-991 MIMAP-375A Post Provider Activation Explicit-Input Contract Row Selection

Status: selected current
Date: 2026-05-21

## Decision

Select the next narrow allocator row after provider activation explicit-input
contract. The next row may plan activation first-pattern evidence, but provider
activation still remains closed until an explicit first-pattern behavior row
opens it.

## Candidate Next Rows

- provider activation first-pattern evidence plan
- provider activation input bundle inventory
- host allocator replacement / hooks optional-ladder planning

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
