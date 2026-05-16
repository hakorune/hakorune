# 293x-542 MIMAP-055A Reclaim Owner-Transfer First Execution Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-055A` is the first guarded reclaim execution row selected by
`MIMAP-054A`.

The row may execute one modeled owner transfer only when:

```text
MIMAP-051A owner-transfer contract is ready
MIMAP-054A atomic-claim contract succeeds
remote_free_pending == 0
page is backed and not decommitted
```

The execution surface is still narrow. It may change only an executor-local
modeled owner token and report the result. It must not drain remote-free queues,
schedule threads, call page-source APIs, unreserve/release OSVM memory, or
activate allocator providers.

## Scope

- Add an SSOT for the first reclaim owner-transfer execution route.
- Add a `.hako` executor owner that composes `HakoAllocReclaimOwnerTransferContract`
  and `HakoAllocReclaimAtomicClaimContract`.
- Add a proof app and focused guard.
- Keep the route scalar and observable: selected page id, old owner, new owner,
  reason, and inactive surfaces.
- Select the next row after execution proof lands.

## Stop Lines

- No remote-free drain.
- No thread scheduling.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `055A.1` | Write first execution route SSOT. | owner, preconditions, reason vocabulary are fixed. | no broad reclaim |
| `055A.2` | Add `.hako` executor owner. | one ready transfer succeeds; blocked cases stay reasoned. | no remote drain |
| `055A.3` | Add proof app. | proof observes success and blocked cases. | no page-source |
| `055A.4` | Add focused guard and docs index row. | VM / MIR JSON / pure-first EXE proof passes. | no backend matcher |
| `055A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
