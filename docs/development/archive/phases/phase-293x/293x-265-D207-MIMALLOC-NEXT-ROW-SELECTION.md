# 293x-265 D207 Mimalloc Next Row Selection

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

D207 selects one next mimalloc implementation row from the D206 remaining
inventory.

This is a docs-only selection row. It does not add allocator behavior.

## Selected row

```text
M214 allocator options/defaults inventory
```

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/mimalloc-next-row-selection-ssot.md` | D207 selected-row SSOT, M214 owner/proof/guard expectations, and stop lines. |

## M214 ownership

```text
primary owner: lang/src/hako_alloc/memory/options_inventory_box.hako
proof app: apps/hako-alloc-options-inventory-proof/
guard: tools/checks/k2_wide_hako_alloc_options_inventory_guard.sh
```

## Stop lines

M214 must keep these inactive:

```text
mutable runtime options
environment option toggles
allocation behavior changes
thread scheduling
reclaim execution
unreserve
OS release
provider activation
hooks
process allocator replacement
backend app/name matchers
language syntax implementation
selfhost migration
```

## Next blocker

```text
M214 allocator options/defaults inventory
```
