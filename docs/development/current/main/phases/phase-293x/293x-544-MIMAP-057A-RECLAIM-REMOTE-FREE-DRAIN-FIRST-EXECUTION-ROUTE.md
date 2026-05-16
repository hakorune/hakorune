# 293x-544 MIMAP-057A Reclaim Remote-Free Drain First Execution Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-057A` is the first narrow modeled remote-free drain execution row
selected by `MIMAP-056A`.

The row may execute one modeled remote-free drain through a dedicated owner
after the MIMAP-056A contract reports pending work. It must stay local to
already-owned page/inbox state and must not schedule workers, call page-source
or OSVM seams, activate providers, or perform full reclaim.

## Scope

- Add an SSOT for the first remote-free drain execution route.
- Add a `.hako` execution owner that composes the MIMAP-056A drain contract.
- Drain at most one modeled remote-free entry.
- Report success, blocked reasons, and inactive broader reclaim surfaces.
- Select the next row after the proof lands.

## Stop Lines

- No thread scheduling.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No full reclaim / page-source reclaim cascade.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `057A.1` | Write first drain execution SSOT. | one-entry modeled drain boundary is fixed. | no full reclaim |
| `057A.2` | Add `.hako` drain execution owner. | one pending entry drains; blocked cases stay reasoned. | no scheduling |
| `057A.3` | Add proof app. | proof observes success and blocked cases. | no page-source |
| `057A.4` | Add focused guard and docs index row. | VM / MIR JSON / pure-first EXE proof passes. | no backend matcher |
| `057A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_execution_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
