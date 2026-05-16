# 293x-448 MIMAP-029A Facade Huge Decommit Route

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-029A` is the selected post-backed-huge allocator behavior row.

It adds one narrow facade-facing success route that proves:

```text
MIMAP-028A backed huge allocation
same huge model -> M181 page-map unregister
same MIMAP-028A backing range -> M196 page-source decommit adapter
scalar report -> huge/page-map live state is zero and decommit succeeds
```

The row is not an unreserve, recommit, provider-activation, hook, or host
allocator replacement row. Duplicate decommit / stale decommit diagnostics are
deferred to MIMAP-030A.

## Scope

- Add:
  `lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako`.
- Reuse MIMAP-028A:
  `HakoAllocObjectLifecycleFacadeHugePageSourceRoute`.
- Bind M181 `HakoAllocHugeReleaseSeam` to the same huge model owned by the
  MIMAP-028A route:
  `page_source_route.huge_route.huge_model`.
- Reuse M196 `HakoAllocPageSourceDecommitAdapter` for the actual page-source
  decommit call.
- Publish scalar report fields that prove:
  - backing range identity
  - huge handle identity
  - unregister success through the same huge model/page map
  - decommit success on the same `source_base` / `source_bytes`
  - no unreserve/recommit/provider/hook activity
- Add an EXE/VM proof app:
  `apps/mimalloc-facade-huge-decommit-proof/main.hako`.
- Add a guard:
  `tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh`.

## Acceptance Preflight

Before cutting a compiler/language sidecar, MIMAP-029A must try the scalar owner
split below:

```text
allocate:
  page_source_route.allocateHugeWithPageSource(facade, size)

unregister:
  release_seam = new HakoAllocHugeReleaseSeam(page_source_route.huge_route.huge_model)
  release_seam.releaseHugePtr(result.huge_ptr)

decommit:
  decommit_adapter.decommitPage(result.source_base, result.source_bytes)
```

Do not call `MIMAP-026A allocateThenUnregisterHuge(...)` directly. That owner
allocates through a different huge route/model and cannot prove that the
decommit uses the MIMAP-028A backing range.

If this minimal owner fails pure-first route acceptance with
`target_body_supported=false`, `user_box_method_contract_missing`,
`structured_call_no_route`, or `mir_call_no_route`, stop MIMAP-029A and cut
`USERBOX-METHOD-COMPOSITE-001` with an allocator-neutral fixture for:

```text
typed report object-return method
  + field_set
  + generic_i64 global call
  + cross-owner same-module user-box method call
```

Implementation note:

The minimal owner exposed a compiler work-explosion in the MIR PHI base query
before it exposed a missing user-box route contract. The fix was kept as a
narrow compiler sidecar inside this closeout:

```text
src/mir/phi_query.rs
  BTreeSet clone per PHI branch -> memoized backtracking query
```

This does not add a new allocator shape or backend capability. It only keeps
the existing PHI base-relation query bounded for shared binary PHI DAGs.

The same proof also requires huge metadata scalar accessors to publish explicit
`.hako` return contracts. `HakoAllocHugePageMetaStore` now annotates its scalar
append/read/release methods with `: i64` so same-module user-box routes do not
depend on dynamic `ArrayBox.get` return inference. The scalar column reads also
materialize typed locals before return so the backend MIR route sees the same
contract as the source surface.

## Stop Lines

- Do not add OSVM unreserve, OS release, recommit, purge, or reclaim behavior.
- Do not implement duplicate/stale decommit diagnostics in this row.
- Do not add small release/free, realloc, alignment, remote-free, TLS, atomic,
  provider hook, host allocator replacement, or `#[global_allocator]`
  behavior.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.
- Do not call MIMAP-026A as a shortcut for unregistering the MIMAP-028A-backed
  allocation.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `029A.1` | Add the facade huge decommit owner. | One owner composes MIMAP-028A allocation, M181 unregister, and M196 decommit. | no unreserve/recommit |
| `029A.2` | Add the proof app and guard. | Guard proves same-backed unregister/decommit scalar report. | no provider activation |
| `029A.3` | Close docs and current pointers. | Current state points to the next row selected after 029A. | no duplicate decommit diagnostics |

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

Owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_huge_decommit_box.hako
```

Proof app:

```text
apps/mimalloc-facade-huge-decommit-proof/main.hako
```

Guard:

```text
tools/checks/k2_wide_mimalloc_facade_huge_decommit_exe_guard.sh
```

Compiler sidecar:

```text
src/mir/phi_query.rs
```

Scalar return contract cleanup:

```text
lang/src/hako_alloc/memory/huge_page_meta_store_box.hako
```

The implementation keeps the behavior to one success path: allocate a
page-source-backed huge handle through MIMAP-028A, unregister that same pointer
through M181 bound to the same huge model, and decommit the exact backing range
through the M196 page-source decommit adapter. Duplicate/stale decommit
diagnostics, unreserve/recommit, provider activation, host allocator
replacement, and backend matcher shortcuts remain outside MIMAP-029A.

## Closeout

MIMAP-029A is closed. The active blocker moves to MIMAP-029B post-huge-decommit
row selection.
