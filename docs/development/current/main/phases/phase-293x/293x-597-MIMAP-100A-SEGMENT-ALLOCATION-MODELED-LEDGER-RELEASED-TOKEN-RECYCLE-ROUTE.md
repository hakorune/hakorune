# 293x-597 MIMAP-100A Segment Allocation Modeled Ledger Released-Token Recycle Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-100A` is the allocator row selected by `MIMAP-099A`.

It should prove the scalar ledger contract for recycling a released modeled
allocation token:

```text
record live token
  -> release token
  -> record the same scalar token again as the new live allocation
  -> reject a second simultaneous live duplicate
```

The row stays ledger-only. It must not become real segment allocation/free
execution.

## Scope

Allowed:

- add a released-token recycle SSOT;
- add a proof app and local-run guard;
- use the existing modeled consume / ledger owner route;
- add a small owner helper only if it is needed to make the recycle contract
  explicit and local.

Forbidden:

- real segment allocation/free execution;
- arena backing allocation;
- raw pointer residence;
- segment-map pointer membership or lookup;
- atomic bitmap claim/unclaim;
- page-source or OSVM calls;
- real thread scheduling or worker spawning;
- source-level concurrency feature changes;
- provider activation, hooks, host allocator replacement, or
  `#[global_allocator]`;
- backend `.inc` app/name matcher;
- cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `100A.1` | Add recycle SSOT and proof app. | proof shows release then same-token live recycle. | no real free |
| `100A.2` | Add local-run guard. | guard fixes output and inactive stop lines. | no backend matcher |
| `100A.3` | Wire manifests/docs. | proof manifest, check index, taskboard, current pointers updated. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_modeled_ledger_released_token_recycle_guard.sh
tools/checks/run_proof_app.sh --only MIMAP-100A
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
