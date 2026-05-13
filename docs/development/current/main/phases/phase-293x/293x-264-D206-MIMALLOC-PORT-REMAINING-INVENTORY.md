# 293x-264 D206 Mimalloc Port Remaining Inventory

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

D206 maps completed `.hako` mimalloc / `hako_alloc` rows against the current
mimalloc port purpose and names the remaining surfaces before implementation
continues.

This is a docs/inventory row. It does not add allocator behavior.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/mimalloc-port-remaining-inventory-ssot.md` | Post-M213 mimalloc remaining inventory, next-row recommendation, inactive surfaces, and D207 contract. |

## Fixed decisions

The recommended next selection input is:

```text
M214 allocator options/defaults inventory
```

The row should be read-only and must keep mutable options, env toggles,
provider activation, hooks, allocator replacement, reclaim execution, unreserve,
and OS release inactive.

## Remaining inventory classes

```text
options/defaults/init vocabulary
thread heap ownership facts
reclaim execution ladder
unreserve / OS release proposal
secure randomness / entropy inventory
visible record materialization and source PackedArray semantics
language minimal-surface rows
selfhost migration
```

## Next blocker

```text
D207 mimalloc next-row selection
```

D207 must choose exactly one implementation row and name its owner files, proof
surface, stop lines, and guard.
