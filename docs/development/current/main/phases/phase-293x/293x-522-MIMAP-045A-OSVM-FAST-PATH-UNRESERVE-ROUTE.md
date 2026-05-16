# 293x-522 MIMAP-045A OSVM-Backed Fast-Path Unreserve Route

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-045A` opens the fast-path unreserve seam that `MIMAP-044A` kept
inactive.

It must compose existing owners:

```text
MIMAP-043A:
  HakoAllocOsVmFastPathReuseRoute

MIMAP-033A:
  HakoAllocPageSourceUnreserveAdapter
```

The row proves that one OSVM-backed fast-path page can be allocated, released,
bounded-purged/decommitted, and then unreserved through the page-source
unreserve adapter. It must not directly call page-source or OSVM functions from
the new route owner or proof app.

## Scope

- Add a narrow route owner:
  `lang/src/hako_alloc/memory/osvm_fast_path_unreserve_route_box.hako`.
- Add a proof app:
  `apps/hako-alloc-osvm-fast-path-unreserve-route-proof/`.
- Add a focused guard:
  `tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_route_guard.sh`.
- Export the route owner from `lang/src/hako_alloc/hako_module.toml`.
- Update `lang/src/hako_alloc/memory/README.md`,
  `lang/src/hako_alloc/README.md`, `docs/tools/check-scripts-index.md`, and
  `tools/checks/proof_apps.toml`.

## Stop Lines

- No provider activation, hook installation, process allocator replacement, or
  `#[global_allocator]`.
- No direct `HakoAllocPageSourcePolicy` / `OsVmCoreBox` call in the new route
  owner or proof app.
- No OS release route beyond the existing MIMAP-033A unreserve adapter seam.
- No remote-free execution, TLS/atomic execution changes, thread scheduling,
  reclaim execution, page ownership migration, or user-facing concurrency work.
- No backend `.inc` app/name matcher shortcut.
- No fast-path reuse after unreserve in this row.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `045A.1` | Add the route owner by composing `HakoAllocOsVmFastPathReuseRoute` and `HakoAllocPageSourceUnreserveAdapter`. | route exposes scalar report and adapter counters. | no direct OSVM/page-source calls |
| `045A.2` | Add proof app and manifest row. | output proves allocate/release/purge/unreserve sequence. | no provider/host replacement |
| `045A.3` | Add guard and docs/index updates. | guard passes and rejects direct seam leaks. | no broad allocator gate expansion |
| `045A.4` | Update current pointers. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_route_guard.sh
[k2-wide-hako-alloc-osvm-fast-path-unreserve-route] ok
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

`MIMAP-045A` lands the route owner, proof app, proof manifest row, and focused
guard. The route composes the MIMAP-043A fast-path route with the MIMAP-033A
page-source unreserve adapter; it does not add direct page-source/OSVM calls in
the new owner/proof, post-unreserve reuse, provider activation, hooks, host
allocator replacement, remote-free/TLS/atomic execution changes, reclaim
execution, or user-facing concurrency work.

Next row:

```text
MIMAP-045B post-fast-path-unreserve row selection
```
