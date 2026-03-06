# CorePlan migration done criteria (SSOT)

This document defines when the CorePlan migration can be considered done.

## Stage-2 Done (release adopt)

- Release adopt gate patterns execute via CorePlan without strict/dev flags.
- Gate smokes cover the release paths and remain green.
- JoinIR legacy loop table is removed; routing is plan/composer only.
- Return-in-loop minimal (stdlib `is_integer`) is covered via release adopt (Phase 29ar).

## Stage-1 Done (purity)

- Stage-1 purity SSOT is defined and followed:
  - `docs/development/current/main/design/coreplan-purity-stage1-ssot.md`

## Optional: Purity Stage-2 Done (fallback → 0)

- Purity Stage-2 SSOT is defined and followed:
  - `docs/development/current/main/design/coreplan-purity-stage2-ssot.md`

## Stage-3 Done (composer v0/v1)

- Composition is driven by skeleton/feature facts, not pattern name branching.
- Router is thin: it delegates to composer and does not encode v0/v1 decisions.
- v0/v1 boundary is SSOT, keyed by `value_join_needed`, with clear gate rules.

## Still allowed to remain

- Historical planner-payload wording may remain in docs or compat shims only; the runtime lane stays recipe-first.
- Legacy extractors may remain in the plan layer as long as they do not reintroduce
  JoinIR-side routing tables.
