# 293x-540 MIMAP-053A Reclaim Execution Support Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-053A` is the planning row selected by `MIMAP-052B`.

Reclaim execution intent is now MIR-visible as `hako.alloc.reclaim`, and the
pure-first preflight can reject unsupported reclaim execution before backend
emission. The next step must choose exactly one implementation row before any
owner-transfer execution opens.

## Scope

- Read `MIMAP-051A` owner-transfer contract evidence.
- Read `MIMAP-052B` reclaim execution marker/preflight evidence.
- Decide whether the next implementation row should be:
  - a first guarded reclaim execution slice;
  - an atomic-claim contract sidecar;
  - a remote-free drain fail-fast row;
  - or another no-execution allocator row.
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
| `053A.1` | Read marker/preflight and owner-transfer evidence. | missing execution prerequisite is classified. | no implementation |
| `053A.2` | Select exactly one next row. | one token is named. | no bundle |
| `053A.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Evidence Review

`MIMAP-051A` names the owner-transfer preconditions and proves that execution
surfaces are still inactive.

`MIMAP-052B` adds a dedicated execution-intent marker:

```text
uses alloc_reclaim -> hako.alloc.reclaim
```

and an explicit unsupported preflight reason:

```text
reclaim_execution_route_unsupported
```

The remaining blocker before first owner-transfer execution is not a generic
capability marker. It is the atomic claim contract: a reclaim executor must
prove the owner token changes only when the expected abandoned owner still
matches, and failure must leave the modeled owner unchanged.

## Selection Result

`MIMAP-053A` selects `MIMAP-054A`.

```text
row:
  MIMAP-054A reclaim atomic-claim contract

classification:
  allocator prerequisite / no-execution contract row

decision:
  add a small `.hako` owner and proof app that model the owner-token atomic
  claim contract before any reclaim execution or page ownership mutation row

why before execution:
  owner-transfer correctness depends on compare-and-claim semantics. Opening
  reclaim execution with only read-only contract facts would make owner mutation
  look like an ordinary scalar assignment.

stop lines:
  no reclaim execution
  no page owner mutation in the production facade
  no remote-free drain
  no thread scheduling
  no page-source call
  no provider activation
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-054A.
```
