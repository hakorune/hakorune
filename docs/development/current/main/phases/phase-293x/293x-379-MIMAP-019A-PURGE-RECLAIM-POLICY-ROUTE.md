# 293x-379 MIMAP-019A Purge/Reclaim Policy Route

Status: landed
Date: 2026-05-15

## Decision

`MIMAP-019A` is the next allocator row after `MIMAP-018A`. It should integrate a
small purge/reclaim/decommit policy route only through existing lifecycle and
stats observers.

Selected existing owners:

```text
purge/decommit candidate policy:
  lang/src/hako_alloc/memory/purge_candidate_policy_box.hako
  box HakoAllocPurgeCandidatePolicyInventory

abandoned/reclaim inventory:
  lang/src/hako_alloc/memory/abandoned_reclaim_inventory_box.hako
  box HakoAllocAbandonedReclaimInventory
```

New route owner:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_purge_policy_box.hako
```

The new owner is an adapter. It may consume:

- `HakoAllocObjectLifecycleFacadeStatsSnapshot`
- one scalar view of facade-known `HakoAllocPageModel` lifecycle state
- explicit scalar caller facts for backing bytes and owner/observer thread
  state

It must produce a read-only combined decision by delegating to the selected
M211/M213 inventories. It must not execute decommit, recommit, reclaim, page
source, OSVM, provider, hook, remote-free, or backend behavior.

## Scope

- Select the smallest existing hako_alloc purge/reclaim policy owner that can be
  composed from stable lifecycle/stats observers.
- Add one proof app and one guard for the selected route.
- Keep the row read-only or policy-only unless the selected existing owner
  already has a bounded execution contract.

## Route Contract

```text
stats snapshot + known page scalar lifecycle view + backing/owner facts
  -> facade lifecycle report
  -> M211 purge candidate decision
  -> M213 abandoned reclaim decision
  -> combined read-only route decision
```

The route decision may expose:

- whether the known page is purge/decommit-policy eligible
- whether the known page is abandoned/reclaim-policy eligible
- whether reclaim can forward to the purge candidate policy
- counters from the adapter and selected existing inventories

All `would_*` execution fields remain zero for this row. A positive candidate is
policy evidence only, not a decommit/reclaim execution request.

## Stop Lines

- No OSVM/page-source activation unless this card is split into the explicit
  capability row.
- No provider hooks, host allocator replacement, or `#[global_allocator]`.
- No backend matcher shortcut.
- No page-map lookup unless the selected existing owner already owns that seam
  and the proof is explicitly scoped to it.
- No M212 scheduler or M199 state-aware decommit guard call.
- No page-source adapter, OSVM page-source policy, unreserve, or release route.
- No stats mutation while classifying a route decision.

## Required Evidence

```text
bash tools/checks/k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
```

## Closeout

MIMAP-019A landed a read-only `object_lifecycle_facade_purge_policy_box.hako`
route. The route consumes a facade stats snapshot and a scalar lifecycle view,
then delegates to the existing M211 purge candidate inventory and M213
abandoned reclaim inventory. The proof app keeps execution closed: no scheduler,
state-aware decommit guard, page-source, OSVM, provider, hook, page-map, or
backend shortcut is introduced.

Implementation:

```text
lang/src/hako_alloc/memory/object_lifecycle_facade_purge_policy_box.hako
apps/mimalloc-facade-purge-policy-route-proof/main.hako
tools/checks/k2_wide_mimalloc_facade_purge_policy_route_exe_guard.sh
```

Next:

```text
MIMAP-020A OSVM/page-source capability pilot
```
