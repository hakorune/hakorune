---
Status: SSOT
Decision: accepted
Date: 2026-05-14
Scope: D208 closeout after M214 allocator options/defaults inventory.
Related:
  - docs/development/current/main/design/hako-alloc-options-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-next-row-selection-ssot.md
  - docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
---

# Mimalloc Migration Closeout Check SSOT

## Decision

M214 is closed as a read-only options/defaults inventory row.

D208 selects one more safe mimalloc inventory row before any lane switch:

```text
M215 thread heap owner-token inventory
```

This keeps the allocator lane moving while still avoiding reclaim execution,
thread scheduling, atomics expansion, unreserve, OS release, provider
activation, hooks, process allocator replacement, language syntax work, and
selfhost migration.

## M214 closeout

M214 added:

```text
lang/src/hako_alloc/memory/options_inventory_box.hako
apps/hako-alloc-options-inventory-proof/
tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```

M214 remains read-only. It does not open mutable options or process/runtime
configuration.

## Selected next row

```text
M215 thread heap owner-token inventory
```

M215 should name owner-token facts needed by future abandoned/reclaim work:

```text
known owner id
same observer/owner thread
owner active vs inactive
abandoned owner token
remote-free pending reject
reclaim adoption candidate
```

M215 must not mutate owner state or schedule work.

## Stop line

D208 and M215 must not add:

```text
thread scheduling
atomic claim / CAS based ownership transfer
remote-free drain during reclaim
page ownership migration
reclaim execution
page-source calls
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
language syntax implementation
selfhost parser/mirbuilder changes
```
