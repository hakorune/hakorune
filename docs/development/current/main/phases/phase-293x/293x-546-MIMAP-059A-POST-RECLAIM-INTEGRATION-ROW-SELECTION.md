# 293x-546 MIMAP-059A Post-Reclaim-Integration Row Selection

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-059A` is the planning row selected by `MIMAP-058A`.

The reclaim lane now has:

```text
owner-transfer contract
atomic-claim contract
owner-transfer first execution route
remote-free drain contract
remote-free drain first execution route
post-drain owner-transfer integration route
```

Before opening broader reclaim behavior, this row should select exactly one
next row.

## Candidate Rows

| Candidate | Shape | Notes |
| --- | --- | --- |
| `MIMAP-060A` | full reclaim success route | compose post-drain transfer with final reclaim marker; still no page-source/OSVM release unless selected explicitly |
| `MIMAP-060B` | reclaim closeout guard | lock completed scalar reclaim lane before broader behavior |
| `MIMAP-COMPILER-*` | compiler/language sidecar | only if current proof apps expose a real acceptance blocker |

## Stop Lines

- No full reclaim execution in this selection row.
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
| `059A.1` | Read current reclaim SSOT/proof evidence. | landed row set is accurate. | no code |
| `059A.2` | Decide one next row. | candidate is named with stop lines. | no bundle |
| `059A.3` | Update taskboard/current pointers. | current pointer guard passes. | no implementation |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
