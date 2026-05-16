# 293x-459 MIMAP-033A Page-Source Unreserve Adapter

Status: landed
Date: 2026-05-16

## Decision

`MIMAP-033A` is the behavior row selected by `MIMAP-032B`.

It adopts the landed OSVM unreserve substrate route behind the allocator
page-source owner. This row is intentionally below facade huge-unreserve
behavior:

```text
OsVmCoreBox.unreserve_bytes_i64
  -> HakoAllocPageSourcePolicy.unreservePage
  -> HakoAllocPageSourceUnreserveAdapter
```

The row must not call unreserve directly from facade/lifecycle owners.

## Scope

- Add `HakoAllocPageSourcePolicy.unreservePage(base, bytes)`.
- Add a narrow `HakoAllocPageSourceUnreserveAdapter` owner with call / success /
  reject counters, mirroring the decommit adapter shape.
- Add a proof app that reserves, commits, decommits, and unreserves through the
  page-source owner / adapter.
- Add a focused EXE guard.

## Stop Lines

- Do not add facade huge unreserve-after-decommit behavior in this row.
- Do not add duplicate/stale unreserve diagnostics beyond adapter rc counters.
- Do not add recommit, purge, remote-free, TLS cache, provider activation,
  hooks, host allocator replacement, or `#[global_allocator]`.
- Do not add backend `.inc` matcher shortcuts or app/box-name classifiers.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `033A.1` | Add page-source unreserve owner. | `HakoAllocPageSourcePolicy.unreservePage` delegates to `OsVmCoreBox.unreserve_bytes_i64`. | no facade call |
| `033A.2` | Add unreserve adapter. | Adapter records call/success/reject/last rc and delegates only to page-source policy. | no duplicate diagnostics |
| `033A.3` | Add proof app and guard. | EXE guard proves reserve/commit/decommit/unreserve through page-source owner / adapter. | no backend matcher |
| `033A.4` | Close current pointers. | Current state moves to the next selected row. | no provider activation |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Landed Implementation

```text
page-source owner:
  lang/src/hako_alloc/memory/page_source_policy_box.hako
adapter:
  lang/src/hako_alloc/memory/purge_page_source_unreserve_adapter_box.hako
proof app:
  apps/hako-alloc-page-source-unreserve-adapter-proof/main.hako
guard:
  tools/checks/k2_wide_hako_alloc_page_source_unreserve_adapter_guard.sh
```

The landed row adds `HakoAllocPageSourcePolicy.unreservePage(base, bytes)` and
`HakoAllocPageSourceUnreserveAdapter.unreservePage(base, bytes)`. The adapter
records call / success / reject / last-result scalar counters and delegates only
to the page-source policy. Facade huge-unreserve behavior remains closed.

Focused proof output includes:

```text
hako-alloc-page-source-unreserve-adapter-proof
page=4096 reserved=1
route=0,0,0
adapter=1,1,0,0,4096
summary=ok
```

## Return Condition

This row closes when page-source unreserve adoption is live and proven, while
facade huge unreserve and provider/host allocator replacement remain inactive.

Closeout:

```text
current blocker moves to MIMAP-033B post-page-source-unreserve row selection.
```
