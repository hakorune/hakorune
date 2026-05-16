# 293x-545 MIMAP-058A Reclaim Post-Drain Owner-Transfer Integration Route

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-058A` is the narrow integration row selected by `MIMAP-057A`.

The row may compose the MIMAP-057A modeled drain route and the MIMAP-055A
modeled owner-transfer route. It should prove the ordering around remote-free
pending work without opening full reclaim, page-source calls, thread
scheduling, OSVM unreserve/release, or provider activation.

## Scope

- Add an SSOT for post-drain owner-transfer integration.
- Add a `.hako` integration owner that composes the existing drain and
  owner-transfer execution owners.
- Prove one route where drain reduces pending work before transfer is allowed.
- Prove blocked routes stay diagnostic and scalar.
- Select the next row after the proof lands.

## Stop Lines

- No full reclaim / page-source reclaim cascade.
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
| `058A.1` | Write post-drain transfer integration SSOT. | integration order and reasons are fixed. | no full reclaim |
| `058A.2` | Add `.hako` integration owner. | drain-before-transfer success is observable. | no page-source |
| `058A.3` | Add proof app. | proof observes success and blocked cases. | no scheduling |
| `058A.4` | Add focused guard and docs index row. | VM / MIR JSON / pure-first EXE proof passes. | no backend matcher |
| `058A.5` | Close current pointers and select follow-up. | current pointer guard passes. | no bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_post_drain_owner_transfer_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
