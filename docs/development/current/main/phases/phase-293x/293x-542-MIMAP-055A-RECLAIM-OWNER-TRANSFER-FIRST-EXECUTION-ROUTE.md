# 293x-542 MIMAP-055A Reclaim Owner-Transfer First Execution Route

Status: landed
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

## Implementation Result

`MIMAP-055A` adds:

```text
SSOT:
  docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-execution-ssot.md

owner:
  lang/src/hako_alloc/memory/reclaim_owner_transfer_execution_box.hako

proof app:
  apps/hako-alloc-reclaim-owner-transfer-execution-proof/

guard:
  tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh
```

The owner composes the MIMAP-051A readiness contract and the MIMAP-054A
atomic-claim contract. When both succeed, it changes only:

```text
HakoAllocReclaimOwnerTransferExecution.current_owner
```

Blocked requests preserve the observed owner in the report and publish whether
the readiness contract or atomic-claim contract blocked the transfer.

All production surfaces remain inactive:

```text
would_change_production_page_owner = 0
would_execute_full_reclaim = 0
would_drain_remote_free = 0
would_schedule_thread = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
would_activate_provider = 0
```

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_execution_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-055A` selects `MIMAP-056A`.

```text
row:
  MIMAP-056A reclaim remote-free drain contract inventory

classification:
  allocator prerequisite / no-execution contract row

why now:
  modeled owner transfer can now execute locally, but full reclaim must still
  not proceed across pending remote-free work. The next narrow row names the
  remote-free drain readiness contract before any drain execution opens.

stop lines:
  no remote-free drain execution
  no thread scheduling
  no page-source call
  no OSVM unreserve / release
  no provider activation
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-056A.
```
