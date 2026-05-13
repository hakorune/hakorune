# 293x-269 D209 Mimalloc Post-M215 Closeout Check

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

D209 closes the M214/M215 internal inventory wave and prepares a lane switch to
language minimal-surface work.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/mimalloc-post-m215-closeout-ssot.md` | Post-M215 closeout, remaining inactive surfaces, and next lane decision. |
| `docs/reference/runtime/substrate-capabilities.md` | Short reference note for internal read-only `hako_alloc` inventory surfaces. |
| `docs/reference/language/low-level-capabilities.md` | Language-facing note that M214/M215 are internal inventory, not user syntax. |

## Decision

M214/M215 stay internal and read-only. They do not open mutable options, env
configuration, scheduling, atomics, reclaim execution, unreserve, OS release,
provider activation, hooks, or allocator replacement.

## Next blocker

```text
D210 language minimal surface lane switch
```
