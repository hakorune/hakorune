# 293x-536 MIMAP-051B Post-Reclaim-Contract Row Selection

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-051B` is the planning-only row selected by `MIMAP-051A`.

It must choose exactly one next allocator/compiler/language row after reclaim
owner-transfer contract inventory lands.

## Scope

- Read `MIMAP-051A` evidence and current Hakorune/mimalloc task order.
- Decide whether the next row should be:
  - Hakorune capability checker expansion;
  - reclaim execution preflight / fail-fast row;
  - a small allocator contract row;
  - a language ergonomics row that removes current allocator complexity.
- Update current pointers and taskboard after selection.

## Stop Lines

- No reclaim execution.
- No owner mutation.
- No atomic claim.
- No remote-free drain.
- No thread scheduling.
- No capability checker implementation in this planning row.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `051B.1` | Read MIMAP-051A evidence and joint task order. | next blocker is classified. | no implementation |
| `051B.2` | Select exactly one next row. | one token is named with stop lines. | no bundle |
| `051B.3` | Update current pointers and taskboard. | current pointer guard passes. | no landed history expansion |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-051B` selects `USES-002A`.

Reason:

```text
MIMAP-051A:
  reclaim owner-transfer preconditions are now named.

next reclaim execution rows:
  will require executable atomic/TLS/OSVM semantics and must not infer those
  from helper names.

current compiler state:
  `uses random` is MIR-visible, but `uses osvm` / `uses atomic` /
  `uses rawbuf` remain Stage0 metadata without a MIR CapabilityPlan mapping.
```

Decision:

```text
open:
  USES-002A declared uses capability plan mapping

do not open yet:
  reclaim execution
  atomic ownership claim
  remote-free drain
  TLS source surface
```

Closeout:

```text
current blocker moves to USES-002A.
```
