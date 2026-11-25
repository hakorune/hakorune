# Loop Canonicalizer (`src/mir/loop_canonicalizer/`)

> **Read first (2026-11-17, maintainer decision):** the design of source → MIR lives in the [MirBuilder blueprint](../../../docs/development/current/main/design/mirbuilder-blueprint.md). This README describes the code as it is today and is largely a change record; do not use it as design authority. Where it says loop families, routes or recipes decide acceptance, or names a stage order other than `DeclarationTable -> Resolve -> Lower -> Seal -> Verify -> Publish`, the blueprint wins.

This subtree owns loop-shape normalization and route detection. It is analysis-
first: observe shapes, do not rewrite semantics silently.

## Read First

1. `src/mir/README.md`
2. `src/mir/builder/README.md`

## Boundaries

- Keep canonicalizer policy in one place.
- Do not split loop-shape rules across builder / router / pass code.
- Avoid AST rewrite shortcuts; prefer analysis-only views and explicit plans.

## Main Responsibilities

- loop/if/break/continue shape observation
- route detection for normalized lowering
- canonical decision helpers used by MIR builder and JoinIR entry points

## P5 Crate Split Prep

`loop_canonicalizer/` is a future `mir-passes` candidate if it remains analysis-
only. It is not being split in the current P5 step.

SSOT:

- `docs/development/current/main/design/mir-crate-split-prep-ssot.md`

Prep rule:

- keep loop-shape rules centralized and shared with `src/mir/policies/`
- do not split analysis helpers away from the canonical decision policy yet
