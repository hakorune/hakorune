---
Status: Parked
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
   - actual rename batch is now tracked in `phase-29cs`

## Immediate Slice

Docs-only first slice:

- write the cleanup SSOT
- write the MIR crate split prep SSOT
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
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_realworld.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_local.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_condition.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_helpers_loop.rs`
- `src/mir/builder/control_flow/plan/facts/loop_break_trim_whitespace_helpers.rs`

P5 will start with crate boundary inventory and entry-map tightening before any split.

P5 docs-first seed:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

JoinIR boundary review remains docs-first only for now:

- `src/mir/join_ir/` still couples AST/ProgramJSON, runtime/env, and MIR
  surfaces
- `src/mir/join_ir/json.rs` keeps JoinIR serialization in the same review lane
- `join_ir_vm_bridge/` boundary is still unstable
- do not package `join_ir/` yet; tighten the README boundary map first
- landed substrate slice: `hakorune_mir_joinir` now owns `join_ir/ownership/types.rs`
- landed internal box tightening: `join_ir/ownership/bridge/` now owns
  `plan_to_lowering.rs` / `plan_validator.rs` under the ownership facade
- landed internal box tightening: `join_ir/ownership/analyzer/` now owns the
  ProgramJSON analyzer core (`mod.rs` / `core.rs` / `node_analysis.rs`)
- prefer next cleanup inside `join_ir/` as sub-box tightening:
  ownership analyzer core / ownership bridge glue / lowering substrate /
  condition cluster / loop-route cluster / target-specific lowerers /
  bridge fence

Passes boundary review also remains docs-first only for now:

- `src/mir/passes/` still couples AST/runtime/config/env and MIR surfaces
- `rc_insertion/` is especially blocked by those seams
- do not package `passes/` yet; tighten the README boundary map first
- landed internal box tightening: `concat3_canonicalize/analysis/` now owns
  `stringish.rs` / `def_use.rs` behind the pass facade

P5 first packaging slice landed:

- `crates/hakorune_mir_core/` with `types.rs` / `value_id.rs`
- `crates/hakorune_mir_defs/` with `definitions/call_unified.rs`

P5 second packaging slice landed:

- `crates/hakorune_mir_core/` gained `effect.rs`
- `src/mir/definitions/call_unified.rs` became a thin wrapper to `hakorune_mir_defs`

P5 substrate ID slice landed:

- `crates/hakorune_mir_core/` gained `basic_block_id.rs` / `binding_id.rs`
- `crates/hakorune_mir_core/` gained `value_kind.rs`
- `src/mir/basic_block.rs` now re-exports the substrate IDs
- builder / edgecfg / optimizer / tests now use public `crate::mir::{BasicBlockId, EdgeArgs}`
- backend/mir_interpreter now uses public `crate::mir::BasicBlock` / `BasicBlockId`
- `crates/hakorune_mir_builder/` gained `core_context.rs` / `context.rs`
- `crates/hakorune_mir_builder/` gained `binding_context.rs`
- `crates/hakorune_mir_builder/` gained `type_context.rs`
- `crates/hakorune_mir_builder/` gained `variable_context.rs`
- `crates/hakorune_mir_builder/` gained `metadata_context.rs`
- `crates/hakorune_mir_joinir/` gained `ownership_types.rs`
- `src/mir/builder/compilation_context.rs` is parked: mixed ownership (`ASTNode` / `FunctionSlotRegistry` / `TypeRegistry`)

P6 naming cleanup:

- `crates/hakorune_mir_core/` and `crates/hakorune_mir_defs/` use the `hakorune_*` crate naming now
- `src/mir/README.md` / MIR crate split prep SSOT now use `hakorune-mir-*` future names
- subtree READMEs for `builder/`, `join_ir/`, and `passes/` also use `hakorune-mir-*` future names
- `hakorune_mir_core` now also owns the basic block / binding ID substrate
- `hakorune_mir_core` now also owns the value kind substrate
- `hakorune_mir_builder` now also owns the first package slice (`core_context.rs` / `context.rs`)
- `hakorune_mir_builder` now also owns `binding_context.rs`
- `hakorune_mir_builder` now also owns `type_context.rs`
- `hakorune_mir_builder` now also owns `variable_context.rs`
- remaining README cleanup landed for:
  - `src/mir/contracts/README.md`
  - `src/mir/control_tree/README.md`
  - `src/mir/join_ir_vm_bridge/README.md`
  - `src/mir/join_ir_vm_bridge_dispatch/README.md`
  - `src/mir/policies/README.md`

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
- P4 landed: `helpers_pure_value.rs` / `helpers_layout.rs` / `helpers_value.rs` extracted from `helpers.rs`; `loop_break_helpers_common.rs` / `loop_break_helpers_break_if.rs` / `loop_break_helpers_realworld.rs` / `loop_break_helpers_local.rs` / `loop_break_helpers_condition.rs` / `loop_break_helpers_loop.rs` / `loop_break_trim_whitespace_helpers.rs` extracted from `loop_break_helpers.rs`

## Next

Stop-line judgment on 2026-03-22:

- the high-value structural wins are landed
- `scope_context.rs` is still blocked by the `MirFunction` / lexical-scope seam
- `compilation_context.rs` remains mixed-ownership and parked
- `join_ir/` and `passes/` whole-subtree packaging remain docs-first only
- remaining naming polish is optional and low-value
- it is technically sound to return to the `.hako` kernel migration lane next

When this lane is reopened for implementation:

1. `src/mir/builder/scope_context.rs` (blocked until the `MirFunction` / lexical-scope seam is split further)
2. `src/mir/builder/compilation_context.rs` (parked: mixed ownership / ASTNode + FunctionSlotRegistry + TypeRegistry)
3. remaining `hakorune-mir-*` naming surface polish
