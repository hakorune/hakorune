# 293x-543 MIMAP-056A Reclaim Remote-Free Drain Contract Inventory

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-056A` is the no-execution contract row selected by `MIMAP-055A`.

The previous row can execute one executor-local modeled owner-token transfer
when readiness and claim contracts both succeed. Before any broader reclaim
execution opens, the remote-free drain boundary must be named separately.

This row should add a scalar contract owner that classifies whether a reclaimed
page may proceed without remote-free drain work, and why it is blocked when
remote frees remain pending.

## Scope

- Add an SSOT for reclaim remote-free drain contract vocabulary.
- Add a `.hako` contract owner that reports remote-free drain readiness as
  scalar facts.
- Add a proof app and focused guard.
- Compose with existing remote-free policy only as read-only facts if needed.
- Select the next row after the contract lands.

## Stop Lines

- No remote-free drain execution.
- No thread scheduling.
- No page-source call.
- No OSVM unreserve / release.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `056A.1` | Write remote-free drain contract SSOT. | readiness and blocked reasons are fixed. | no execution |
| `056A.2` | Add `.hako` contract owner. | pending/clear cases are scalar and observable. | no thread scheduling |
| `056A.3` | Add proof app. | proof observes ready and blocked cases. | no page-source |
| `056A.4` | Add focused guard and docs index row. | VM / MIR JSON / pure-first EXE proof passes. | no backend matcher |
| `056A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_remote_free_drain_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
