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

P0 first implementation batch landed:

- root archive relocation
- `*.err` / `*.backup*` ignore policy
- root keepers explicitly documented (`basic_test.hako`, `test.hako`)

P1 landed:

- `CURRENT_TASK.md` slim + archive policy

P2 first slice landed:

- `box_arithmetic.rs` -> inline facade
- `box_operators.rs` -> `src/boxes/operators/`
- `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`

The next implementation slice, when this lane is explicitly reopened, is:

- `src/box_trait.rs`
- `src/method_box.rs`
- `src/type_box.rs`
- `src/value.rs`
- `src/environment.rs`
- `src/instance_v2.rs`

## Pressure Summary

Local snapshot on 2026-03-22:

- `src/**/*.rs`: `1789` files / `342813` lines
- `lang/**/*.hako`: `451` files / `54853` lines
- `src/mir/**/*.rs`: `1031` files / `210851` lines
- `src/mir/builder` subdirectories: `92`

Interpretation:

- philosophy is already ahead of the tree
- first wins are root hygiene and restart cost
- `src/` top-level cleanup now has a landed first slice
- `src/mir` needs navigation cleanup before crate split

## Acceptance

- `docs/development/current/main/design/repo-physical-structure-cleanup-ssot.md` exists
- `CURRENT_TASK.md` points at this phase
- `10-Now.md` mirrors the fixed order
- P0 first batch is landed: root archive relocation + `*.err` / `*.backup*` ignore policy
- P1 is landed: `CURRENT_TASK.md` slim + archive policy
- P2 first slice is landed: box arithmetic / box operators / runner plugin init relocation

## Next

When this lane is reopened for implementation:

1. `src/box_trait.rs`
2. `src/method_box.rs`
