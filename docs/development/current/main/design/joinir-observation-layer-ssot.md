---
Status: SSOT
Scope: JoinIR as observation layer
Related:
- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/condition-observation-ssot.md`
---

# JoinIR Observation Layer SSOT

## Purpose

JoinIR is an **observation layer** for AST → Facts. It exists to observe structure,
not to own lowering or verification logic.

## Role

- StepTree / ControlForm / CondProfile are **observation views**.
- JoinIR is the source of observation artifacts used by Facts.

## Non-role

- JoinIR is **not** the SSOT for Recipe/Verifier/Lower.
- Do not place Recipe/Verifier/Lower rules in JoinIR.

## Principles

- No rewrite (analysis-only).
- Fail-fast on invalid observations.
- Verifier is the only acceptance gate.

## Dependencies (SSOT)

- `docs/development/current/main/design/recipe-first-entry-contract-ssot.md`
- `docs/development/current/main/design/recipe-tree-and-parts-ssot.md`
- `docs/development/current/main/design/condition-observation-ssot.md`
