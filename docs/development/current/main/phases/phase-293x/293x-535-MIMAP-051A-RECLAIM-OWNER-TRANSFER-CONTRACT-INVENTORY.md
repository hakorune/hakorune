# 293x-535 MIMAP-051A Reclaim Owner-Transfer Contract Inventory

Status: landed
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

## Implementation Result

`MIMAP-051A` adds:

```text
SSOT:
  docs/development/current/main/design/hako-alloc-reclaim-owner-transfer-contract-ssot.md

owner:
  lang/src/hako_alloc/memory/reclaim_owner_transfer_contract_box.hako

proof app:
  apps/hako-alloc-reclaim-owner-transfer-contract-proof/

guard:
  tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh
```

The owner composes:

```text
HakoAllocThreadHeapOwnerInventory
HakoAllocAbandonedReclaimInventory
```

into a read-only contract report:

```text
contract_ready
reason
owner_reason
reclaim_reason
inactive execution flags
```

`contract_ready = 1` is an inventory fact for a future execution row. It does
not mutate ownership, execute reclaim, drain remote frees, call page-source
APIs, or perform atomic claims in this row.

## Evidence

```text
bash tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Selection Result

`MIMAP-051A` selects `MIMAP-051B`.

```text
row:
  MIMAP-051B post-reclaim-contract row selection

classification:
  planning-only row

why now:
  reclaim owner-transfer preconditions are now named. The lane needs a
  single-row selection step before deciding whether to open capability checking,
  reclaim execution preflight, language ergonomics, or another allocator row.

stop lines:
  no reclaim execution
  no capability checker implementation
  no cleanup bundle
```

Closeout:

```text
current blocker moves to MIMAP-051B.
```
