# 293x-267 D208 Mimalloc Migration Closeout Check

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

D208 closes M214 and selects the next single mimalloc row.

This is a docs-only closeout/selection card.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/mimalloc-migration-closeout-check-ssot.md` | M214 closeout and M215 selection SSOT. |

## Selected next row

```text
M215 thread heap owner-token inventory
```

M215 is read-only inventory. It may name owner-token facts, but it must not add
thread scheduling, atomics, reclaim execution, page-source calls, unreserve,
OS release, provider activation, hooks, process allocator replacement, language
syntax implementation, or selfhost migration.
