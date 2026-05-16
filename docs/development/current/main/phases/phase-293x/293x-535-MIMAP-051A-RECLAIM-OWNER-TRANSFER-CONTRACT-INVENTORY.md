# 293x-535 MIMAP-051A Reclaim Owner-Transfer Contract Inventory

Status: selected current
Date: 2026-05-17

## Decision

`MIMAP-051A` is the allocator contract row selected by `MIMAP-050A`.

Secure entropy execution remains parked. The allocator lane returns to the
remaining reclaim surface, but only as an inventory / contract row:

```text
reclaim execution:
  inactive

owner-transfer preconditions:
  named and observable
```

## Scope

- Add a small `.hako` owner that classifies reclaim owner-transfer
  preconditions.
- Compose existing scalar facts from the abandoned/reclaim inventory,
  thread-heap owner inventory, and remote-free policy vocabulary.
- Add a proof app that observes why reclaim execution is still blocked.
- Add a focused guard that prevents accidental reclaim execution, atomic claim,
  remote-free drain, page-source calls, provider activation, or backend
  matchers.

## Stop Lines

- No reclaim execution.
- No owner mutation or ownership transfer execution.
- No atomic CAS / ownership claim execution.
- No remote-free drain during reclaim.
- No thread scheduling or worker wakeup.
- No page-source, OSVM, unreserve, or release call.
- No provider activation, hook, host allocator replacement, or
  `#[global_allocator]`.
- No backend `.inc` app/box-name matcher.
- No cleanup bundle.

## Planned Tasks

| Step | Task | Accept | Stop line |
| --- | --- | --- | --- |
| `051A.1` | Write reclaim owner-transfer contract SSOT. | contract names required facts and inactive execution surfaces. | no behavior |
| `051A.2` | Add `.hako` inventory owner. | owner returns scalar report fields for ready/blocked reasons. | no mutation |
| `051A.3` | Add proof app. | proof observes blocked reclaim and no execution flags. | no page-source call |
| `051A.4` | Add focused guard and docs index row. | guard proves stop lines and proof output. | no backend matcher |
| `051A.5` | Close out current pointers and select follow-up. | current pointer guard passes. | no multi-row bundle |

## Required Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Return Condition

This row closes when reclaim has a named owner-transfer contract inventory and
all execution surfaces remain explicitly inactive.
