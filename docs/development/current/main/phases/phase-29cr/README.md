---
Status: Active
Decision: provisional
Date: 2026-03-22
Scope: repo physical structure cleanup の docs-first planning lane。まず root hygiene / `CURRENT_TASK` slim / `src/` top-level / `src/mir` navigation-first cleanup の順序を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md
  - docs/development/current/main/DOCS_LAYOUT.md
---

# Phase 29cr: Repo Physical Structure Cleanup

## Goal

- repo の物理構造を、設計文書の責務分離と同じ方向へ寄せる。
- root と `src/mir` の restart cost を下げる。
- cleanup を BoxShape として進める。

## Non-Goals

- immediate `src/mir` crate split
- broad `nyash -> hako` rename
- runtime/compiler の active exact blocker fix を同時に抱えること

## Fixed Order

1. `P0`: root hygiene contract
2. `P1`: `CURRENT_TASK.md` slim + archive policy
3. `P2`: `src/` top-level cleanup
4. `P3`: `src/mir` navigation-first cleanup
5. `P4`: `src/mir` physical clustering
6. `P5`: crate split preparation
7. `P6`: naming cleanup

## Immediate Slice

Docs-only first slice:

- write the cleanup SSOT
- point `CURRENT_TASK.md` at this phase
- mirror the fixed order in `10-Now.md`

The first implementation slice, when this lane is explicitly reopened, is:

- root allowlist
- `*.err` ignore policy
- scratch/archive relocation
- `CURRENT_TASK` archive destination and cutoff rule

## Pressure Summary

Local snapshot on 2026-03-22:

- `src/**/*.rs`: `1789` files / `342813` lines
- `lang/**/*.hako`: `451` files / `54853` lines
- `src/mir/**/*.rs`: `1031` files / `210851` lines
- `src/mir/builder` subdirectories: `92`

Interpretation:

- philosophy is already ahead of the tree
- first wins are root hygiene and restart cost
- `src/mir` needs navigation cleanup before crate split

## Acceptance

- `docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md` exists
- `CURRENT_TASK.md` points at this phase
- `10-Now.md` mirrors the fixed order
- no code movement is required for this first docs slice

## Next

When this lane is reopened for implementation:

1. root allowlist + `.gitignore` cleanup
2. `CURRENT_TASK.md` slim/archive
3. `src/` top-level inventory
