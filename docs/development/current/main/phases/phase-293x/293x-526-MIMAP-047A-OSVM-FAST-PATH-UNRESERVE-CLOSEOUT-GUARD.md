# 293x-526 MIMAP-047A OSVM-Backed Fast-Path Unreserve Closeout Guard

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-047A` closes the OSVM-backed fast-path unreserve surface by inventorying
the success and fail-fast rows:

```text
MIMAP-045A:
  HakoAllocOsVmFastPathUnreserveRoute

MIMAP-046A:
  HakoAllocOsVmFastPathUnreserveFailFastRoute
```

The row must add a closeout SSOT and guard only. It must not implement allocator
behavior or compiler acceptance.

## Scope

- Add `docs/development/current/main/design/mimalloc-osvm-fast-path-unreserve-closeout-ssot.md`.
- Add `tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_closeout_guard.sh`.
- Update `docs/tools/check-scripts-index.md`.
- Freeze ownership, proof app, guard, module export, README, and no-leak
  contracts for `MIMAP-045A` and `MIMAP-046A`.

## Stop Lines

- No `.hako` allocator behavior changes.
- No Rust compiler changes.
- No provider activation, hook installation, process allocator replacement, or
  `#[global_allocator]`.
- No post-unreserve reuse behavior.
- No OS release beyond the MIMAP-033A unreserve adapter seam.
- No remote-free/TLS/atomic execution changes, thread scheduling, reclaim
  execution, page ownership migration, or user-facing concurrency work.
- No backend `.inc` app/name matcher shortcut.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `047A.1` | Add closeout SSOT. | names 045A/046A completed surfaces and inactive surfaces. | no behavior |
| `047A.2` | Add closeout guard. | guard checks owner/proof/guard/doc/export/no-leak contracts. | no broad gate expansion |
| `047A.3` | Update check index and current pointers. | guard and pointer pass. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_closeout_guard.sh
[k2-wide-hako-alloc-osvm-fast-path-unreserve-closeout] ok
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

`MIMAP-047A` lands the closeout SSOT and guard for the OSVM-backed fast-path
unreserve success and fail-fast rows. It does not implement allocator behavior,
compiler acceptance, post-unreserve reuse, OS release, provider activation,
hooks, host allocator replacement, remote-free/TLS/atomic execution changes,
reclaim execution, or user-facing concurrency work.

Next row:

```text
MIMAP-047B post-fast-path-unreserve-closeout row selection
```
