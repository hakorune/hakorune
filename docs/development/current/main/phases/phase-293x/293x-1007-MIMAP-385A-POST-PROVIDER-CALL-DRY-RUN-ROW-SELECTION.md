# 293x-1007 MIMAP-385A Post Provider Call Dry-Run Row Selection

Status: landed
Date: 2026-05-21

## Decision

Select the next narrow allocator row after provider-call dry-run unsupported
behavior. The provider-call capability gate and unsupported dry-run outcome are
now modeled, but provider API calls, host replacement, hooks, backend matcher
additions, and global allocator install remain closed.

## Candidate Next Rows

- provider-call dry-run unsupported closeout
- provider-call modeled call-open pilot
- provider-call execution capability preflight

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

## Decision Result

Selected:

```text
MIMAP-386A Provider Call Modeled Open Pilot
```

This row opens provider-call readiness in model space only. It records that an
accepted unsupported dry-run can advance to a modeled provider-call-open state,
but actual provider API calls, host replacement, hooks, backend matcher
additions, worker/thread execution, and global allocator install remain closed.
