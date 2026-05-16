# 293x-518 MIMAP-043A OSVM-Backed Fast-Path Recommit/Reuse Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-043A` is the allocator behavior row selected by `MIMAP-042B`.

It adds one `.hako` route owner that composes the already-landed
OSVM-backed fast-path purge route with the existing M205 recommit heap
integration:

```text
HakoAllocOsVmFastPathPurgeRoute
  allocation / release / bounded purge owner from MIMAP-042A

HakoAllocRecommitHeapIntegration
  M200-M204 recommit + page reactivation owner
```

The row proves the first post-purge recommit/reuse path through the route:

```text
allocate -> release -> purgeOne -> recommitAndAllocate
```

## Owner

```text
lang/src/hako_alloc/memory/osvm_fast_path_reuse_route_box.hako
```

Expected shape:

```text
box HakoAllocOsVmFastPathReuseRoute
  purge_route: HakoAllocOsVmFastPathPurgeRoute
  recommit: HakoAllocRecommitHeapIntegration

  allocate(size)
  release(handle)
  purgeOne(max_scan_pages)
  recommitAndAllocate(page_id, size)
```

## Proof / Guard

```text
apps/hako-alloc-osvm-fast-path-reuse-route-proof/
tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh
```

The proof should demonstrate:

- one OSVM-backed allocation succeeds;
- releasing the handle makes the page a bounded purge candidate;
- `purgeOne(1)` decommits and marks the page through the existing 042A seam;
- allocation before recommit is rejected by the page-local lifecycle state;
- `recommitAndAllocate(page_id, size)` routes through M205 and then allocates
  from the same page again;
- the decommit marker is no longer marked after recommit.

## Stop Lines

- No direct `HakoAllocPageSourcePolicy`, `OsVmCoreBox`, `reservePage`,
  `commitPage`, `decommitPage`, `unreserve`, or `releasePage` call from the new
  route owner.
- No unreserve, OS release, provider activation, hooks, host allocator
  replacement, or `#[global_allocator]`.
- No remote-free, TLS, atomic, worker scheduling, or user-facing concurrency
  surface expansion.
- No scheduler policy changes, page queue policy changes, or fresh-page fallback
  changes.
- No broad report cleanup or unrelated BoxShape work.
- No `.inc` app/name matcher.
- If pure-first route preflight exposes an unsupported compiler shape, stop and
  split a compiler acceptance sidecar instead of rewriting `.hako` around the
  compiler gap.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `043A.1` | Add the route owner and export it from `hako_module.toml`. | owner composes 042A route + M205 recommit only. | no direct page-source calls |
| `043A.2` | Add focused proof app. | pre-recommit allocation rejects; post-recommit allocation succeeds on same page. | no provider / replacement |
| `043A.3` | Add local guard and index/manifest entries. | MIR/EXE proof fixes route shape and no `.inc` matcher. | no broad allocator gate growth unless explicit |
| `043A.4` | Update owner docs and close card. | current pointer guard and focused guard pass. | no next-row behavior |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

Implemented:

- `lang/src/hako_alloc/memory/osvm_fast_path_reuse_route_box.hako` defines
  `HakoAllocOsVmFastPathReuseRoute`, composing the MIMAP-042A purge route with
  M205 recommit heap integration.
- `apps/hako-alloc-osvm-fast-path-reuse-route-proof/` proves allocation,
  release, bounded purge, pre-recommit allocation rejection, M205 recommit, and
  post-recommit allocation on the same page.
- `tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh` fixes
  the owner shape, MIR/EXE proof shape, no direct page-source/OSVM calls, and
  no `.inc` matcher leak.

Evidence run:

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_reuse_route_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

Next current row:

```text
MIMAP-043B post-fast-path-reuse route row selection
```

## Next Split Triggers

| Trigger | Split row |
| --- | --- |
| post-reuse route wants unreserve / OS release | new allocator behavior row after `MIMAP-043A` |
| route wants thread/TLS/atomic remote-free execution | concurrency-substrate follow-up row, not `MIMAP-043A` |
| route preflight reports missing user-box contract | focused compiler acceptance sidecar |
| route wants PackedArray/record backend storage | `PACKED-BACKEND-001` |
