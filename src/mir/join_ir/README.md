# JoinIR (`src/mir/join_ir/`)

JoinIR is the normalized control-flow layer between MIR construction and the
VM/LLVM backends.

## Read First

1. [`lowering/README.md`](./lowering/README.md)
2. [`ownership/README.md`](./ownership/README.md)
3. `frontend/` and `lowering/` submodules for the concrete emission flow

## Boundaries

- Do not add new lowering heuristics here when `builder/` already owns the shape decision.
- Treat ownership analysis as analysis-only; it must not mutate JoinIR structures.
- Prefer explicit contracts over by-name dispatch or hidden fallback.

## Main Responsibilities

- normalized JoinIR module structure
- ownership analysis and relay/capture bookkeeping
- lowering helpers that feed VM/LLVM bridge layers

## P5 Crate Split Prep

`join_ir/` is a future `hakorune-mir-joinir` candidate, but it is not being split yet.
The prep step is to keep the lowering surface explicit and document the rejected
boundaries first.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- do not split `join_ir/` away from `join_ir_vm_bridge/` until the lowering
  surface is stable
