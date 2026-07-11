# 293x-263 D205 Post-M213 Next-Lane Selection

Status: Complete
Decision: accepted
Date: 2026-05-14

## Purpose

Select the recommended task order after the M192-M213 purge/lifecycle ladder
closeout.

This is a docs-only selection card.

## Landed docs

| Path | Role |
| --- | --- |
| `docs/development/current/main/design/post-m213-next-lane-selection-ssot.md` | Recommended order for mimalloc remaining inventory, mimalloc migration rows, language minimal-surface rows, and selfhost migration. |

## Selected order

```text
1. D206 mimalloc port remaining inventory
2. selected mimalloc migration/parity rows
3. language minimal-surface implementation rows
4. selfhost migration
```

## Next blocker

```text
D206 mimalloc port remaining inventory
```

## Stop line

D205 does not implement allocator behavior, language syntax, or selfhost
migration.

