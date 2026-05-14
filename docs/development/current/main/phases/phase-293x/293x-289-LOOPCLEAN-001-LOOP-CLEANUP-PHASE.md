---
Status: active
Date: 2026-05-14
Scope: Docs-only phase cut for loop-surface cleanup before PACKED-001.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/loop-cleanup-before-packedarray-ssot.md
  - docs/development/current/main/design/language-minimal-surface-task-breakdown-ssot.md
---

# 293x-289 LOOPCLEAN-001 Loop Cleanup Phase

## Purpose

Pause `PACKED-001` long enough to clean loop-family source/AST drift.

This is a BoxShape cleanup lane. It must not add new loop semantics.

## Canonical Surface

```text
canonical:
  loop cond { ... }
  loop i in start..end { ... }
  loop { ... }

legacy accepted:
  while cond { ... }
  for i in start..end { ... }
```

## Task Rows

| Row | Task | Stop line |
| --- | --- | --- |
| `LOOPCLEAN-002` | normalize `while` parsing to `Loop` and keep JSON `While` compat decode | do not delete all compat arms in same row |
| `LOOPCLEAN-003` | quarantine/remove remaining safe `ASTNode::While` references | no behavior changes |
| `LOOPCLEAN-004` | commonize range-header parser for canonical `loop i in` and legacy `for i in` | do not desugar ranges to source-level loop |
| `LOOPCLEAN-005` | decide future `ForRange` -> `LoopRange` naming migration | docs-first only |

## Explicit Non-Goals

- no `ForRange` -> `Loop` merge
- no source-level range desugar
- no `while` / `for` canonical resurrection
- no `LOOP-003` Stage1 range lowering in this docs-only row
- no PackedArray implementation in this row

## Next

Start with `LOOPCLEAN-002 while parser normalization`.
