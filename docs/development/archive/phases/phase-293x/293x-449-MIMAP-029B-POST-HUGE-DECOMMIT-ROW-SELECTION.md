# 293x-449 MIMAP-029B Post-Huge-Decommit Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-029B` is a planning-only row. It must select exactly one next allocator
behavior row after the facade huge path has:

```text
MIMAP-028A page-source-backed huge allocation identity
MIMAP-029A same-backed unregister + decommit success
```

The selection must keep provider activation, host allocator replacement, hooks,
and `#[global_allocator]` inactive unless a future narrow ladder explicitly
reopens them.

Current prerequisite:

```text
MIR-EMIT-SSOT-001 -> MIR-ROUTE-PREFLIGHT-001 -> SELFHOST-PROGRESS-001
-> MIR-EMIT-SSOT-002
```

MIMAP-029A exposed that pure-first guard preflight and selfhost EXE build can
use different MIR emissions. `MIMAP-029B` resumes only after the same-artifact
route, route preflight, basic progress diagnostics, and canonical source-to-MIR
entry are stable. The sidecar SSOT is:

```text
docs/development/current/main/design/pure-first-mir-artifact-and-diagnostics-ssot.md
```

## Scope

- Review the post-MIMAP-029A same-backed decommit state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Expected Output

This card should close with one selected next row and the following fields
filled in:

```text
row:
  MIMAP-030A facade huge decommit fail-fast diagnostics
owner:
  lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_failfast_box.hako
proof app:
  apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako
guard:
  tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh
reused owners:
  HakoAllocObjectLifecycleFacadeHugeDecommitRoute
  HakoAllocObjectLifecycleFacadeHugePageSourceRoute
  HakoAllocHugeReleaseSeam
  HakoAllocPageSourceDecommitAdapter
  HakoAllocObjectLifecycleFacadeReason
primary proof:
  after one MIMAP-029A success, a duplicate/stale decommit attempt is rejected
  by allocator-side state before a second page-source decommit adapter call
stop lines:
  no OSVM unreserve
  no recommit
  no provider activation
  no host allocator replacement / hook / #[global_allocator]
  no backend .inc matcher shortcut
```

It should not land code. If the chosen row needs a new capability or verifier
contract first, `MIMAP-029B` should select that contract row explicitly instead
of silently widening the allocator owner.

## Selection Rubric

Prefer the next row in this order:

1. Choose a scalar `.hako` owner if it can prove the next allocator invariant
   without a new backend capability.
2. Choose a state-marker / verifier contract row if duplicate decommit needs a
   no-fallback allocator-side state contract before diagnostics.
3. Do not choose OSVM unreserve until decommit success and duplicate-decommit
   diagnostics are both green.
4. Do not choose provider/host replacement from this row.

## Draft Forward Rows

These are planning candidates for the sequence after `MIMAP-029B`; only the row
selected by `MIMAP-029B` becomes current.

| Row | Candidate purpose | Likely owner | Proof / guard | Stop lines |
| --- | --- | --- | --- | --- |
| `MIMAP-030A` | facade huge decommit fail-fast diagnostics | `object_lifecycle_facade_huge_decommit_failfast_box.hako` | `apps/mimalloc-facade-huge-decommit-failfast-proof/main.hako` / `tools/checks/k2_wide_mimalloc_facade_huge_decommit_failfast_exe_guard.sh` | no unreserve/recommit/provider |
| `MIMAP-030B` | post-huge-decommit-failfast row selection | docs-only | pointer guard / quick | no implementation |
| `MIMAP-031A` | OSVM unreserve capability inventory / planning row | docs/design only unless explicitly selected | pointer guard / quick | do not add `hako_osvm_unreserve*` in this planning row |

`MIMAP-030A` is the conservative default candidate, but it must not rely on the
OSVM page-source policy itself to detect duplicate decommit. The duplicate
guard belongs to allocator-side state, analogous to the existing purge
decommit state-marker rows.

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM unreserve, OS release, recommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-029A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `029B.0` | Confirm pure-first sidecar closeout. | Same-artifact route, route preflight, and progress diagnostics are closed or explicitly deferred with owner approval. | no allocator behavior |
| `029B.1` | Review MIMAP-029A landed evidence and duplicate-decommit risk. | One next row is selected with owner/proof/guard names. | no implementation |
| `029B.2` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `029B.3` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Selection Result

`MIMAP-029B` selects `MIMAP-030A`.

Rationale:

- `MIMAP-029A` already proves same-backed unregister + decommit success.
- The next correctness gap is duplicate/stale decommit diagnostics.
- The diagnostic must be allocator-side stateful fail-fast. It must not rely on
  OSVM/page-source decommit to reject a duplicate call.
- Existing purge rows M198/M199 already show the pattern: record successful
  decommit state and reject duplicate decommit before another source call.
- Unreserve/recommit/provider work remains too broad for the next row.

Closeout:

```text
current blocker moves to MIMAP-030A.
```
