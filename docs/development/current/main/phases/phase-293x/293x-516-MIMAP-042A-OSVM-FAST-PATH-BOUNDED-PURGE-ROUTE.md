# 293x-516 MIMAP-042A OSVM-Backed Fast-Path Bounded Purge Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-042A` is the next allocator behavior row selected by
`MIMAP-NEXT-BEHAVIOR-SELECTION-001`.

It adds one `.hako` route owner that composes existing allocator owners:

```text
HakoAllocOsVmBackedFastPathHeap
  allocation / release / page backing owner

HakoAllocPurgeStateAwareDecommitGuard
  duplicate-aware bounded decommit guard

HakoAllocBoundedPurgeDecommitScheduler
  bounded scan and one-candidate scheduler
```

The row must not add provider activation, process allocator replacement, or new
concurrency language semantics.

## Owner

```text
lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako
```

Expected shape:

```text
box HakoAllocOsVmFastPathPurgeRoute
  heap: HakoAllocOsVmBackedFastPathHeap
  decommit_guard: HakoAllocPurgeStateAwareDecommitGuard
  scheduler: HakoAllocBoundedPurgeDecommitScheduler

  allocate(size)
  release(handle)
  purgeOne(max_scan_pages)
```

## Proof / Guard

```text
apps/hako-alloc-osvm-fast-path-purge-route-proof/
tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
```

The proof should demonstrate:

- a page-backed allocation succeeds through the OSVM-backed fast-path heap;
- releasing the handle makes the page a bounded purge candidate;
- `purgeOne(1)` executes at most one scheduler candidate;
- a duplicate `purgeOne(1)` does not execute source decommit again;
- the new route owner does not call page-source / OSVM APIs directly.

## Stop Lines

- No direct `HakoAllocPageSourcePolicy`, `OsVmCoreBox`, `reservePage`,
  `commitPage`, `decommitPage`, `unreserve`, or `releasePage` call from the new
  route owner.
- No unreserve, recommit, OS release, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, worker scheduling, or user-facing concurrency
  surface expansion.
- No broad report cleanup or unrelated BoxShape work.
- No `.inc` app/name matcher.
- If pure-first route preflight exposes an unsupported compiler shape, stop and
  split a compiler acceptance sidecar instead of rewriting `.hako` around the
  compiler gap.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `042A.1` | Add the route owner and export it from `hako_module.toml`. | owner composes heap / guard / scheduler only. | no direct page-source calls |
| `042A.2` | Add focused proof app. | allocation, release, first purge, duplicate purge all observed. | no provider / replacement |
| `042A.3` | Add local guard and index/manifest entries if needed. | MIR/EXE route proves same-module calls and no `.inc` matcher. | no broad allocator gate growth unless explicit |
| `042A.4` | Update owner docs and close card. | current pointer guard and quick gate pass. | no next-row behavior |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
cargo test -q user_box_method_route_plan::tests::object_handles::refresh_module_user_box_method_routes_refines_void_placeholder_object_route_result
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```

## Closeout

Implemented:

- `lang/src/hako_alloc/memory/osvm_fast_path_purge_route_box.hako`
  defines `HakoAllocOsVmFastPathPurgeRoute`, composing M168 allocation/release
  with M199 duplicate-aware guard state and M212 bounded scheduling.
- `apps/hako-alloc-osvm-fast-path-purge-route-proof/` proves allocation,
  release, first purge, and duplicate purge behavior.
- `tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh`
  fixes the route owner, MIR/EXE proof shape, no direct page-source/OSVM calls,
  and no `.inc` matcher leak.
- The route exposed the parked `MIR-ROW-D` acceptance seam: a same-module
  user-box route can publish `return_shape=object_handle` while the call result
  still carries an unannotated `void` placeholder. The compiler sidecar refines
  that placeholder from the route contract before checking nested receiver
  method calls, guarded by
  `refresh_module_user_box_method_routes_refines_void_placeholder_object_route_result`.

Evidence run:

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_purge_route_guard.sh
cargo test -q user_box_method_route_plan::tests::object_handles::refresh_module_user_box_method_routes_refines_void_placeholder_object_route_result
bash tools/checks/current_state_pointer_guard.sh
git diff --check
tools/checks/dev_gate.sh quick
```

Next current row:

```text
MIMAP-042B post-fast-path-purge route row selection
```

## Next Split Triggers

| Trigger | Split row |
| --- | --- |
| dense queue field read fails before EXE | `MIR-ROW-D` |
| route preflight reports missing user-box contract | focused compiler acceptance sidecar |
| route wants unreserve / recommit / OS release | new allocator behavior row after `MIMAP-042A` |
| route wants TLS / atomic / remote-free execution | concurrency-substrate follow-up row, not `MIMAP-042A` |
