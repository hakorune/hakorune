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
