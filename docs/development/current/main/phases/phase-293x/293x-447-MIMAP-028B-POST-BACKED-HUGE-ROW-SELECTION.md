# 293x-447 MIMAP-028B Post-Backed-Huge Row Selection

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-028B` is a planning-only row. It selects `MIMAP-029A` as the next
allocator behavior row after the facade huge path has:

```text
MIMAP-026A M181 success unregister
MIMAP-027A post-unregister reject diagnostics
MIMAP-028A page-source-backed huge allocation identity
```

Selected row:

```text
row: MIMAP-029A facade huge decommit-after-unregister success route
owner: lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako
proof app: apps/mimalloc-facade-huge-decommit-proof/main.hako
guard: tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
reused owners:
  - MIMAP-028A HakoAllocObjectLifecycleFacadeHugePageSourceRoute
  - M181 HakoAllocHugeReleaseSeam
  - M196 HakoAllocPageSourceDecommitAdapter
primary proof:
  allocate one page-source-backed huge handle, unregister that same handle
  through the release seam bound to the same huge model, then decommit the
  exact MIMAP-028A backing range.
stop lines:
  no unreserve/recommit/provider/host allocator replacement/hooks
```

The selection keeps provider activation, host allocator replacement, hooks, and
`#[global_allocator]` inactive.

## Scope

- Review the post-MIMAP-028A backed huge allocation state.
- Pick exactly one next allocator behavior row.
- Record the owner, proof app, guard, and stop lines for that row.
- Keep provider hooks, host allocator replacement, and `#[global_allocator]`
  inactive.

## Expected Output

This card closes with one selected next row and the following fields filled in:

```text
row: MIMAP-029A facade huge decommit-after-unregister success route
owner: lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako
proof app: apps/mimalloc-facade-huge-decommit-proof/main.hako
guard: tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
reused owners: MIMAP-028A, M181, M196
primary proof: same-backed huge allocation -> unregister -> decommit
stop lines: no unreserve/recommit/provider/host allocator replacement/hooks
```

It should not land code. If the chosen row needs a new capability or verifier
contract first, `MIMAP-028B` should select that contract row explicitly instead
of silently widening the allocator owner.

## Acceptance Preflight

MIMAP-029A must first try the scalar owner split below before adding any
compiler/language sidecar:

```text
HakoAllocObjectLifecycleFacadeHugePageSourceRoute
  -> allocateHugeWithPageSource(facade, size)
  -> backing identity remains on the returned report

HakoAllocHugeReleaseSeam
  -> constructed with page_source_route.huge_route.huge_model
  -> releaseHugePtr(result.huge_ptr)

HakoAllocPageSourceDecommitAdapter
  -> decommitPage(result.source_base, result.source_bytes)
```

Do not reuse `MIMAP-026A allocateThenUnregisterHuge(...)` directly for
MIMAP-029A. That owner creates its own huge route/model and would not prove
decommit of the same MIMAP-028A backing range.

Sidecar trigger:

```text
if the minimal MIMAP-029A owner fails with:
  target_body_supported=false
  user_box_method_contract_missing
  structured_call_no_route
  mir_call_no_route
then cut USERBOX-METHOD-COMPOSITE-001 before widening MIMAP-029A.
```

`USERBOX-METHOD-COMPOSITE-001` must be allocator-name neutral. Its fixture
should pin the generic pure-first acceptance shape:

```text
typed report object-return method
  + field_set
  + generic_i64 global call
  + cross-owner same-module user-box method call
```

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

`MIMAP-029A` is selected. It composes the newly backed huge allocation identity
with the existing M181 unregister seam, then decommits exactly the known backing
range through the existing M196 page-source decommit adapter. This keeps
decommit success separate from fail-fast diagnostics, unreserve, provider
activation, and host allocator replacement.

## Stop Lines

- Do not implement allocator behavior in this planning row.
- Do not add OSVM unreserve, OS release, recommit, small release/free, realloc,
  alignment, purge/reclaim, remote-free, TLS, atomic, provider hook, host
  allocator replacement, or backend `.inc` matcher shortcut.
- Do not widen MIMAP-028A while selecting the next row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `028B.1` | Review MIMAP-028A landed evidence and nearby allocator backlog. | `MIMAP-029A` is selected with owner/proof/guard names. | no implementation |
| `028B.2` | Add MIMAP-029A acceptance preflight and sidecar trigger. | Compiler widening is conditional and allocator-name neutral. | no speculative BoxCount |
| `028B.3` | Update taskboard and granularity SSOT. | Current pointers move to the selected behavior row. | no provider activation |
| `028B.4` | Run pointer/quick gates. | Current docs are internally consistent. | no behavior widening |

## Required Evidence

```text
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-028B is closed. The active blocker moves to MIMAP-029A facade huge
decommit-after-unregister success route.
