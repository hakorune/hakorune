# MIR Passes (`src/mir/passes/`)

This subtree contains MIR transformation passes and their local helpers.

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

`src/mir/passes/` is a future `mir-passes` candidate. Keep the public seam
small so the eventual split is a packaging step, not a redesign.

Prep rule:

- one pass should transform or verify, not both
- helper extraction should keep the pass entry thin
- shared policy still belongs in `src/mir/policies/`
