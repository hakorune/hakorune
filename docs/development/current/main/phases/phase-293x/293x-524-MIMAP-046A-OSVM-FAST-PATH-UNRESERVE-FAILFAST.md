# 293x-524 MIMAP-046A OSVM-Backed Fast-Path Unreserve Fail-Fast

Status: landed
Date: 2026-05-17

## Decision

`MIMAP-046A` is the fail-fast companion to `MIMAP-045A`.

It should reject duplicate, stale, or unknown fast-path unreserve requests
through scalar diagnostics while keeping the MIMAP-045A success path routed
through the existing page-source unreserve adapter.

## Scope

- Add a narrow diagnostics owner, expected path:
  `lang/src/hako_alloc/memory/osvm_fast_path_unreserve_failfast_box.hako`.
- Add a proof app, expected path:
  `apps/hako-alloc-osvm-fast-path-unreserve-failfast-proof/`.
- Add a focused guard, expected path:
  `tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_failfast_guard.sh`.
- Keep `HakoAllocOsVmFastPathUnreserveRoute` as the success route owner.
- Report scalar status/reason fields for:
  - first unreserve success;
  - duplicate unreserve rejection;
  - unknown page rejection;
  - stale/not-decommitted page rejection.

## Stop Lines

- No direct page-source / OSVM calls from the new diagnostics owner or proof app.
- No provider activation, hook installation, process allocator replacement, or
  `#[global_allocator]`.
- No post-unreserve reuse behavior.
- No OS release beyond the existing MIMAP-033A unreserve adapter seam.
- No remote-free execution, TLS/atomic execution changes, thread scheduling,
  reclaim execution, page ownership migration, or user-facing concurrency work.
- No backend `.inc` app/name matcher shortcut.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `046A.1` | Add the diagnostics owner. | duplicate/stale/unknown cases produce scalar reject reasons. | no direct OSVM/page-source calls |
| `046A.2` | Add proof app and manifest row. | proof shows first success and rejected repeats. | no provider/host replacement |
| `046A.3` | Add guard and docs/index updates. | guard passes and rejects seam leaks. | no broad allocator gate expansion |
| `046A.4` | Update current pointers. | current pointer guard passes. | no landed history copy expansion |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_osvm_fast_path_unreserve_failfast_guard.sh
[k2-wide-hako-alloc-osvm-fast-path-unreserve-failfast] ok
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Closeout

`MIMAP-046A` lands the diagnostics owner, proof app, proof manifest row, and
focused guard. The owner rejects duplicate, unknown, and not-decommitted
fast-path unreserve requests before adapter execution while keeping the
MIMAP-045A success route unchanged.

Next row:

```text
MIMAP-046B post-fast-path-unreserve-failfast row selection
```
