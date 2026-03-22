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

P2 landed:

- `box_arithmetic.rs` -> inline facade
- `box_operators.rs` -> `src/boxes/operators/`
- `runner_plugin_init.rs` -> `src/runner/plugin_init.rs`
- `box_trait.rs` -> `src/boxes/box_trait.rs`
- `operator_traits.rs` -> `src/boxes/operator_traits.rs`
- `channel_box.rs` / `environment.rs` / `exception_box.rs` / `finalization.rs`
  / `instance_v2.rs` / `method_box.rs` / `scope_tracker.rs` / `type_box.rs`
  / `value.rs` / `ast.rs` / `benchmarks.rs` / `wasm_test.rs`
  -> directory modules

P3 first slice landed:

- `src/mir/README.md`
- `src/mir/builder/README.md`
- `src/mir/join_ir/README.md`
- `src/mir/loop_canonicalizer/README.md`
- `src/mir/passes/README.md`
- `src/mir/control_tree/README.md`
- `src/mir/control_tree/step_tree/README.md`
- `src/mir/control_tree/normalized_shadow/README.md`

P4 first slice landed:

- `src/mir/builder/control_flow/plan/normalizer/helpers_pure_value.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_layout.rs`
- `src/mir/builder/control_flow/plan/normalizer/helpers_value.rs`
- `src/mir/passes/rc_insertion.rs` facade
- `src/mir/passes/rc_insertion_helpers.rs` implementation split
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_common.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_break_if.rs`

The next implementation slice, when this lane is explicitly reopened, is:

- `src/mir/builder/control_flow/plan/facts/loop_break_helpers.rs`

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
- P2 is landed: box arithmetic / box operators / runner plugin init + core-ish root relocations
- P3 first slice is landed: MIR navigation root + builder README
- P4 first slice is landed: `helpers_pure_value.rs` / `helpers_layout.rs` / `helpers_value.rs` extracted from `helpers.rs`

## Next

When this lane is reopened for implementation:

1. `src/mir/passes/rc_insertion.rs`
