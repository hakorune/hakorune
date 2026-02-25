---
Status: SSOT
Scope: CorePlan skeleton choice for `match` / multi-branch
Related:
- docs/development/current/main/design/coreplan-skeleton-feature-model.md
- docs/development/current/main/design/coreplan-flowbox-interface-ssot.md
- docs/development/current/main/design/coreplan-migration-roadmap-ssot.md
---

# `match` is `BranchN` (SSOT)

## Goal

Represent `match` / switch-like control flow as a **first-class CorePlan skeleton**
(`BranchN`), instead of permanently normalizing it into nested `If2`.

## Decision

- CorePlan includes (or reserves) a `BranchN` skeleton for multi-branch control flow.
- `match` lowering targets `BranchN` as the final shape.
- Nested `If2` lowering may be used as a temporary bridge, but it is not the
  long-term SSOT for multi-branch.

## FlowBox invariants (BranchN)

`BranchN` follows the FlowBox interface contract:

- Ports: `entry` and `normal`, plus `exits: ExitMap<ExitKind, Port>`.
- Join payload is represented only via `block_params` + EdgeArgs layout (no
  implicit phi, no re-parse).

## Observability note (match_return subset)

The minimal `match_return` composer currently emits a `Seq` that includes
const effects followed by a `BranchN`. As a result, FlowBox tags may report
`box_kind=Seq` for this subset. This is acceptable for Stage-1; if we want
`box_kind=BranchN` in observability, we must first remove the `Seq` wrapper
by moving const effects into branch-local plans (SSOT change required).

## Rationale

- Keeps the plan pipeline “one flow”: `Facts → Planner → CorePlan → emit`, without
  an `if`-nest explosion.
- Preserves stable ordering/ports for post-phi join (`block_params` / EdgeArgs),
  making local verification and observability easier.
- Prevents planner/normalizer complexity from growing with the number of arms.
- Makes future `unwind`/cleanup integration more uniform via `ExitMap`.

## Non-goals

- No behavior change in release by introducing the skeleton reservation.
- No new env vars or by-name routing.
