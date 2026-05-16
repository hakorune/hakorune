# 293x-447 MIMAP-028B Post-Backed-Huge Row Selection

Status: ready
Date: 2026-05-16

## Decision

`MIMAP-028B` is a planning-only row. It must select exactly one next allocator
behavior row after the facade huge path has:

```text
MIMAP-026A M181 success unregister
MIMAP-027A post-unregister reject diagnostics
MIMAP-028A page-source-backed huge allocation identity
```

The selection must keep provider activation, host allocator replacement, hooks,
and `#[global_allocator]` inactive unless the chosen row explicitly documents a
future narrow ladder step.

## Scope

- Review the post-MIMAP-028A backed huge allocation state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Expected Output

This card should close with one selected next row and the following fields
filled in:

```text
row:
owner:
proof app:
guard:
reused owners:
primary proof:
stop lines:
```

It should not land code. If the chosen row needs a new capability or verifier
contract first, `MIMAP-028B` should select that contract row explicitly instead
of silently widening the allocator owner.

## Selection Rubric

Prefer the next row in this order:

1. Choose a scalar `.hako` owner if it can prove the next allocator invariant
   without a new backend capability.
2. Choose a CorePlan / verifier contract row only if silent fallback or
   backend capability gating becomes the smallest blocker.
3. Decommit may be selected only after MIMAP-028A backing identity is reused
   explicitly by the selected owner.
4. Do not choose OS unreserve or provider/host replacement from this row.

## Draft Forward Rows

These are planning candidates for the sequence after `MIMAP-028B`; only the row
selected by `MIMAP-028B` becomes current.

| Row | Candidate purpose | Likely owner | Proof / guard | Stop lines |
| --- | --- | --- | --- | --- |
| `MIMAP-029A` | facade huge decommit-after-unregister success route | `object_lifecycle_facade_huge_decommit_box.hako` | `apps/mimalloc-facade-huge-decommit-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh` | no unreserve/recommit/provider |
| `MIMAP-029B` | post-huge-decommit row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-030A` | facade huge decommit fail-fast diagnostics | `object_lifecycle_facade_huge_decommit_failfast_box.hako` | `apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh` | no unreserve/recommit/provider |
| `MIMAP-030B` | post-huge-decommit-failfast row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-031A` | OSVM unreserve capability inventory / planning row | docs/design only unless explicitly selected | pointer guard / quick | do not add `hako_osvm_unreserve*` in this planning row |

`MIMAP-029A` is the conservative default candidate: it composes the newly
backed huge allocation identity with the existing M181 unregister seam, then
decommits exactly the known backing range. This keeps decommit success separate
from fail-fast diagnostics, unreserve, provider activation, and host allocator
replacement.

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM unreserve, OS release, recommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-028A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `028B.1` | Review MIMAP-028A landed evidence and nearby allocator backlog. | One next row is selected with owner/proof/guard names. | no implementation |
| `028B.2` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `028B.3` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```
