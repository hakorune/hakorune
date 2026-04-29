# JoinIR (`src/mir/join_ir/`)

JoinIR is the normalized control-flow layer between MIR construction and the
VM/LLVM backends.

## Status

This subtree is docs-first only for now. Do not package it as
`hakorune-mir-joinir` yet.

Current blockers:

- lowering still couples AST/ProgramJSON, runtime/env, and MIR surfaces
- `json.rs` / JoinIR serialization is part of the same review lane
- `join_ir_vm_bridge/` is not stable enough for a crate split
- ownership analysis here is still analysis over the same lowering surface

Landed substrate slice:

- `hakorune_mir_joinir` now owns `join_ir/ownership/types.rs`
- `join_ir/ownership/bridge/*` now groups lowering adapters and validators
- `join_ir/ownership/analyzer/*` now groups the ProgramJSON analysis core
- the rest of `src/mir/join_ir/` stays in the docs-first review lane

## Read First

1. [`lowering/README.md`](./lowering/README.md)
2. [`ownership/README.md`](./ownership/README.md)
3. `frontend/` and `lowering/` submodules for the concrete emission flow

## Boundaries

- Do not add new lowering heuristics here when `builder/` already owns the shape decision.
- Treat ownership analysis as analysis-only; it must not mutate JoinIR structures.
- Prefer explicit contracts over by-name dispatch or hidden fallback.
- Keep `join_ir_vm_bridge/` and `join_ir_vm_bridge_dispatch/` in the same review lane
  until the lowering surface is stable.

## Main Responsibilities

- normalized JoinIR module structure
- ownership analysis and relay/capture bookkeeping
- lowering helpers that feed VM/LLVM bridge layers

## Internal Box Map

Prefer cleaning this subtree by sub-box, not by moving the whole directory at once.

- `hakorune_mir_joinir::ownership_types`
  - pure ownership substrate already extracted
- `ownership/analyzer/*`
  - ProgramJSON ownership analysis core
- `ownership/ast_analyzer/*`
  - AST ownership analysis core; still tied to AST inputs
- `ownership/bridge/*`
  - analysis-to-lowering and validator glue; keep inside the JoinIR review lane
- `lowering` substrate helpers
  - `canonical_names.rs`
  - `error_tags.rs`
  - `debug_output_box.rs`
  - `value_id_ranges.rs`
  - `join_value_space.rs`
- `lowering` condition cluster
  - `condition_env.rs`
  - `condition_lowering_box.rs`
  - `condition_to_joinir.rs`
  - `condition_var_extractor.rs`
  - `scope_manager.rs`
  - `update_env.rs`
- `lowering` loop-route cluster
  - `loop_form_intake.rs`
  - `loop_route_validator.rs`
  - `loop_view_builder.rs`
  - `loop_update_analyzer.rs`
  - `loop_with_*`
  - `simple_while*`
  - `scan_*`
- `lowering` target-specific lowerers
  - `skip_ws.rs`
  - `funcscanner_trim.rs`
  - `stage1_using_resolver.rs`
  - `stageb_body.rs`
  - `stageb_funcscanner.rs`
- `lowering/generic_case_a/*`
  - active Case-A loop lowerers, including the append-defs effect-step shape
- bridge fence
  - `join_ir_vm_bridge/`
  - `join_ir_vm_bridge_dispatch/`
  - keep these together until the lowering surface stabilizes

## P5 Crate Split Prep

`join_ir/` is a future `hakorune-mir-joinir` candidate, but it is not being split yet.
The prep step is to keep the lowering surface explicit, document the rejected
boundaries first, and keep the package move parked until the bridge/lowering
seam is stable.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- do not split `join_ir/` away from `join_ir_vm_bridge/` until the lowering
  surface is stable
- this subtree is docs-first only until the AST/ProgramJSON + runtime/env + MIR
  coupling is reduced
- prefer extracting pure sub-boxes first, then clean intra-tree boundaries, and
  only then revisit whole-subtree packaging
