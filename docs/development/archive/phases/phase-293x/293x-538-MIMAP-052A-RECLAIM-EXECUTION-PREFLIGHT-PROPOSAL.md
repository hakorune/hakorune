# 293x-538 MIMAP-052A Reclaim Execution Preflight Proposal

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-052A` is the allocator planning / preflight row selected by
`USES-002A`.

It must decide the exact fail-fast gate needed before any reclaim execution
row can be opened.

## Scope

- Read `MIMAP-051A` owner-transfer contract evidence and `USES-002A`
  capability-plan mapping evidence.
- Decide whether the next implementation should be:
  - a reclaim execution unsupported-preflight diagnostic;
  - a capability checker / backend gate row;
  - a narrower owner-transfer rollback contract;
  - or a small no-execution allocator row.
- Update current pointers and taskboard after selection.

## Stop Lines

- No reclaim execution.
- No owner mutation.
- No atomic claim.
- No remote-free drain.
- No thread scheduling.
- No page-source call.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `052A.1` | Read contract/capability evidence. | missing gate is classified. | no implementation |
| `052A.2` | Select exactly one next row. | one token is named. | no bundle |
| `052A.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence Review

`MIMAP-051A` provides a read-only owner-transfer contract inventory. It can
report `contract_ready = 1`, but every execution-facing flag remains inactive:

```text
would_schedule_thread = 0
would_atomic_claim = 0
would_drain_remote_free = 0
would_change_page_owner = 0
would_execute_reclaim = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
```

`USES-002A` maps generic low-level declarations into MIR `CapabilityPlan`
allow ids:

```text
uses osvm   -> hako.osvm
uses atomic -> hako.atomic
uses rawbuf -> hako.rawbuf
uses random -> hako.random
```

Those ids are useful substrate facts, but they are not specific enough to mean
"this function intends to execute allocator reclaim". Reusing `hako.atomic` or
`hako.osvm` as the reclaim execution gate would make ordinary substrate access
look like reclaim ownership transfer.

## Selection Result

`MIMAP-052A` selects `MIMAP-052B`.

```text
row:
  MIMAP-052B reclaim execution intent marker preflight

classification:
  fail-fast / metadata gate row

decision:
  add a MIR-visible reclaim execution intent marker and reject that marker
  before backend emission until a later row explicitly opens reclaim execution

marker:
  source uses spelling: uses alloc_reclaim
  MIR allow id: hako.alloc.reclaim
  source: source_uses
  verified: false

preflight:
  tools/checks/pure_first_route_preflight.py
    --reject-unsupported-reclaim-execution

reason:
  reclaim_execution_route_unsupported

why not generic atomic/osvm:
  hako.atomic and hako.osvm are substrate capabilities. They are necessary for
  many allocator rows and must not become an implicit reclaim execution signal.

stop lines:
  no reclaim execution
  no owner mutation
  no atomic claim
  no remote-free drain
  no thread scheduling
  no page-source call
  no provider activation
```

Closeout:

```text
current blocker moves to MIMAP-052B.
```
