---
Status: selected implementation brief — cfg(test)-only
Date: 2026-08-05
Exception: durable source-backed candidate-stage bridge after the D3-S1 design stop
ParentCurrentCard: docs/development/current/main/investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md
Decision: accepted bounded test row; production selection remains stopped
Task: JOINIR-GENERIC-RESOLVED-CARRIER-CANDIDATE-STAGE-SOURCE-BRIDGE0-D3-S1-S2-D0
---

## Change

Add one private `cfg(test)` witness for the natural parsed `generic_both(i,j)`
source. Reuse the parsed source/projector, resolver-issued forest and
`BindingRefV1`, then compose fresh V0 and V1 candidate plans from that same
outer condition/body. Inspect the actual outer/nested `CoreLoopPlan` final and
PHI projections. This is an evidence bridge only; no production issuer,
selector, capability, Recipe, PHI, Builder, MIR, VM, retry, or fallback route
is authorized.

## Contract

- Release and Strict observe raw `[V0,V1]`, direct `LowerSome`, and first effect
  `GenericComposer` for both candidates; planner-required `[V1]` stays a typed
  unresolved case.
- The parsed source obligation is the post-loop `j` read. Resolver forest,
  frame, and binding identity are co-sealed with the plan projection, but a
  `diagnostic_name()` to final/PHI label match is corroboration only, never a
  typed `BindingRefV1 -> ValueId` relation. The direct loop context does not
  lower the full function return, so no runtime/Home/return parity is claimed.
- V1's actual outer projection contains `j`, `loop_carrier_j`, and
  `loop_step_in_j`; V0's outer projection lacks `j` while the nested projection
  retains it. Fresh builders, equal pre-snapshots, reverse-order repeat, and
  distinct repeat owner identities are required.
- Actual legacy remains V0 attempted/terminal with empty debts. A different
  winner after a synthetic DTO-only debt/V1-terminal mutation is evaluator-only
  negative evidence; no route/composer failure injection is allowed, and the
  disposition stays `Observed + UnresolvedStop(WinnerCorrectnessUnavailable)`.

## Done and stop

Add one test sibling and registration, run its focused test plus the
`generic_resolved_carrier_` suite, pointer/artifact guards, and keep every
touched source/test file below 800 lines. Update the parent design, Generic
post-effect/stage references, resolved-semantics README/indexes, current
mirrors, and active workstream in the same implementation closeout commit.

Stop and return to D3-S1 if parsed source and plan cannot be co-sealed, any
synthetic AST is used, the projection mismatches, a typed BindingRef-to-plan
provenance is required, full-function return lowering is attempted, natural
V0-only/Neither appears, raw/stage identity drifts, or any production caller
is introduced. Do not widen the API or claim winner correctness.
