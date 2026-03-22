# Normalized Shadow (`src/mir/control_tree/normalized_shadow/`)

This subtree lowers selected StepTree shapes into the dev-only Normalized shadow
path.

## Read First

1. `src/mir/control_tree/README.md`
2. `mod.rs`
3. `contracts.rs`
4. `builder.rs`
5. `normalized_verifier.rs`

## Boundaries

- Dev-only lowering only; keep strict guard behavior explicit.
- Do not add hidden fallback or by-name heuristics.
- Shape acceptance must come from StepTree contracts, not a second local judgment.

## Responsibilities

- if-only and other selected normalized shadow routes
- parity and capability checks
- dev pipeline helpers for verification and debug

