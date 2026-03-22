# MIR Passes (`src/mir/passes/`)

This subtree contains MIR transformation passes and their local helpers.

## Status

Docs-first only for now. Do not package this subtree as
`hakorune-mir-passes` yet.

Current blockers:

- `callsite_canonicalize.rs` still couples to `crate::ast::ASTNode` through
  closure-body metadata
- `cse.rs`, `dce.rs`, and `escape.rs` still assume the main `crate::mir::*`
  surface and module layout
- `rc_insertion.rs` / `rc_insertion_helpers.rs` still depend on AST, runtime,
  and config/env seams
- `concat3_canonicalize/` is the only plausible future extraction candidate,
  but it still depends on the same MIR surface for now

Next review target:

- `concat3_canonicalize/` as the first real substrate-style extraction candidate

## Read First

1. `src/mir/README.md`
2. `src/mir/contracts/README.md`
3. `src/mir/policies/`

## Boundaries

- New acceptance rules belong in `contracts/`, not hidden inside a pass.
- Shared policy belongs in `policies/` and should be reused by consumers.
- A pass should do one job: transform or verify, not both.

## Main Responsibilities

- MIR-wide transformations
- pass-local verification and fail-fast checks
- small helper wiring for optimizer / normalization stages

## P5 Crate Split Prep

`src/mir/passes/` is a future `hakorune-mir-passes` candidate. Keep the public seam
small so the eventual split is a packaging step, not a redesign.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- one pass should transform or verify, not both
- helper extraction should keep the pass entry thin
- shared policy still belongs in `src/mir/policies/`
- this subtree is docs-first only until the AST/runtime/config coupling is
  reduced enough to make packaging mechanical
