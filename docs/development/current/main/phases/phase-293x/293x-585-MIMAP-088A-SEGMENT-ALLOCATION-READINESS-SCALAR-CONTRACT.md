# 293x-585 MIMAP-088A Segment Allocation Readiness Scalar Contract

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-088A` is the allocator behavior row selected by `MIMAP-087A`.

It adds a proof-only scalar contract that classifies whether a known
segment/page pair is ready for a small allocation request. The row composes the
landed segment lifecycle and page membership vocabulary without executing
segment allocation/free or opening lower substrate features.

It selects:

```text
MIMAP-089A segment allocation readiness closeout guard
```

## Scope

- Add one `.hako` owner for scalar segment allocation readiness.
- Accept tiny same-owner facts when:
  - segment state supports allocation readiness
  - page usage/capacity shape is valid
  - request block count fits available page capacity
  - unsupported substrate requirement is zero
- Return stable scalar reason codes and inactive substrate flags.
- Add one proof app and one local-run guard.

## Stop Lines

- No segment allocation/free execution.
- No arena backing allocation.
- No raw pointer residence.
- No segment-map pointer membership or lookup.
- No atomic bitmap claim/unclaim.
- No page-source call.
- No OSVM execution, unreserve, or release.
- No real thread scheduling.
- No worker spawning.
- No source-level concurrency feature change.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `088A.1` | Add scalar readiness SSOT and owner. | reason vocabulary and inactive flags are documented and implemented. | no execution |
| `088A.2` | Add proof app and manifest entry. | VM/MIR/EXE proof locks accepted/rejected rows. | no allocator-wide gate |
| `088A.3` | Add local-run guard and docs/index wiring. | guard checks owner/proof/stop lines. | no provider/backend matcher |
| `088A.4` | Select closeout row. | `MIMAP-089A` exists and is selected current. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_segment_allocation_readiness_scalar_guard.sh
[mimap088a-mir-json] ok
[k2-wide-hako-alloc-segment-allocation-readiness-scalar] ok

bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
