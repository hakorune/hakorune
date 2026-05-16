---
Status: SSOT
Decision: accepted
Date: 2026-05-17
Scope: MIMAP-051A read-only reclaim owner-transfer contract inventory.
Related:
  - docs/development/current/main/design/purge-lifecycle-ladder-closeout-ssot.md
  - docs/development/current/main/design/hako-alloc-thread-heap-owner-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-hakorune-joint-task-order-ssot.md
  - docs/development/current/main/phases/phase-293x/293x-535-MIMAP-051A-RECLAIM-OWNER-TRANSFER-CONTRACT-INVENTORY.md
  - lang/src/hako_alloc/memory/reclaim_owner_transfer_contract_box.hako
  - apps/hako-alloc-reclaim-owner-transfer-contract-proof/
---

# Hako Alloc Reclaim Owner-Transfer Contract SSOT

## Decision

`MIMAP-051A` adds a read-only `.hako` owner that classifies reclaim
owner-transfer preconditions.

It does not reclaim pages. It names the contract a future execution row must
satisfy before ownership transfer can be opened:

```text
owner token known
foreign owner inactive
remote-free drain not pending
page has backing
page is not decommitted
abandoned/reclaim inventory says reclaim candidate
```

`contract_ready = 1` means the scalar facts are ready for a later guarded
execution row. It is not permission to mutate ownership in this row.

## Owner

```text
lang/src/hako_alloc/memory/reclaim_owner_transfer_contract_box.hako
```

Responsibilities:

```text
compose thread-heap owner inventory facts
compose abandoned/reclaim inventory facts
classify ready vs blocked owner-transfer preconditions
record read-only counts
report inactive execution flags
```

Non-responsibilities:

```text
thread scheduling
atomic ownership claim
remote-free drain
owner mutation
reclaim execution
page-source calls
OSVM unreserve / release
provider / hook / replacement
secure entropy execution
```

## Reason Vocabulary

| Reason | Meaning |
| --- | --- |
| `0` | owner-transfer preconditions are contract-ready |
| `1` | owner inventory rejected unknown owner |
| `2` | owner inventory rejected same-thread owner |
| `3` | owner inventory rejected active foreign owner |
| `4` | remote-free drain is still pending |
| `5` | page is decommitted |
| `6` | reclaim inventory rejected missing backing |
| `7` | reclaim inventory rejected non-abandoned owner facts |
| `8` | reclaim inventory rejected for another reason |

The owner preserves the original `owner_reason` and `reclaim_reason` beside the
contract reason so diagnostics do not have to infer from scalar fields.

## Proof Surface

```text
apps/hako-alloc-reclaim-owner-transfer-contract-proof/
tools/checks/k2_wide_hako_alloc_reclaim_owner_transfer_contract_guard.sh
```

Required inactive facts:

```text
would_schedule_thread = 0
would_atomic_claim = 0
would_drain_remote_free = 0
would_change_page_owner = 0
would_execute_reclaim = 0
would_call_page_source = 0
would_unreserve = 0
would_release_osvm = 0
```

## Stop Lines

No part of `MIMAP-051A` may add:

```text
reclaim execution
owner mutation
atomic CAS / ownership claim
remote-free drain
thread scheduling
page-source call
OSVM unreserve / release
secure entropy source
provider activation
hooks
host allocator replacement
backend app/name matcher
```
