# Loop Canonicalizer (`src/mir/loop_canonicalizer/`)

This subtree owns loop-shape normalization and route detection. It is analysis-
first: observe shapes, do not rewrite semantics silently.

## Read First

1. `src/mir/README.md`
2. `src/mir/builder/README.md`
3. `src/mir/control_tree/step_tree/`
4. `src/mir/control_tree/normalized_shadow/`

## Boundaries

- Keep canonicalizer policy in one place.
- Do not split loop-shape rules across builder / router / pass code.
- Avoid AST rewrite shortcuts; prefer analysis-only views and explicit plans.

## Main Responsibilities

- loop/if/break/continue shape observation
- route detection for normalized lowering
- canonical decision helpers used by MIR builder and JoinIR entry points

