# Loop Route Policy

> **Read first (2026-11-17, maintainer decision):** the design of source → MIR lives in the [MirBuilder blueprint](../../../docs/development/current/main/design/mirbuilder-blueprint.md). This README describes the code as it is today and is largely a change record; do not use it as design authority. Where it says loop families, routes or recipes decide acceptance, or names a stage order other than `DeclarationTable -> Resolve -> Lower -> Seal -> Verify -> Publish`, the blueprint wins.

This module owns typed Loop profile observations and policy admissions.
Production Generic G0 passes a single checked observation and its resolver
lease to the Recipe demand. The family admission module retains the mode and
coverage types carried by that demand.

`CANONICAL_LOOP_ROUTE_ORDER_V1` is the all-route coverage and provenance
vocabulary used by `all_route_observation.rs`. Lowering takes its order from
the source. Recipe construction and MIR emission have separate owners.

M8 S6A does not add code or a selector here. Its
`VariableAccumRecurrenceV1` producer remains a caller-zero provenance path
owned by the compiler projection and neutral Recipe contract; production
selection and legacy schedule retirement stay closed. Its duplicate-role and
source-site failures are Facts/observation rejections, not route-policy
selection signals.

## Generic G0 S1 observation

`generic_g0.rs` is a separate AST-free policy issuer for the bounded Generic
G0 profile. It consumes one sealed
`VerifiedGenericG0PolicyHandoffV1` plus a sealed owner/profile/mode/coverage
context and emits one move-only `Candidate`, typed `Unresolved`, or typed
`Rejected` disposition. The handoff retains the S0C bundle, a resolver/source
brand borrowed from the canonical selector window, exact role relations,
target, and post-loop return relation; it does not retain a second window
lease. Policy does not downgrade it to a bare bundle or reread source. Only `Less`
conditions and positive `Add` steps are admitted. It does not repeat S0A
BindingRef checks, select a family winner, issue Recipe keys, call Builder/MIR,
retry, or fallback. Its production caller is intentionally zero until the
later demand/Recipe and cutover rows. The S0C bundle remains owned by
`loop_structural_facts::generic_g0`; this policy module consumes the neutral
handoff capability rather than a compiler projection aggregate.

`generic_g0_observation.rs` is the caller-zero S1 row-normalization observer.
It consumes one neutral source attempt plus one owner/origin/source-kind/site/
frame, mode, and coverage context, rechecks candidate identity, and emits only
`Candidate`, `Declined`, `Unresolved`, or `Rejected` before calling the existing
Generic policy issuer. The compiler adapter is `cfg(test)`-only and requires an
explicit numeric target. Twelve adapter tests and seven policy tests are green;
ambiguous `ForestShape`/`BindingLookup` evidence remains conservative
`Unresolved` until a resolver-side split. There is no admission assembler,
selector, Recipe/JoinSig, Builder/MIR, retry/fallback, or production caller.

## Generic G0 demand

`issue_generic_g0_recipe_demand_from_observation_v1` consumes one
`VerifiedGenericFamilyObservationG0` and its resolver lease. The demand keeps
the source brand, typed bundle, post-loop read, role proof, mode and coverage.
The Recipe producer in `loop_recipe_contract::generic_g0` consumes the demand
and uses the shared Recipe, JoinSig and source-binding owners.

## DirectAccum S1 family observation

`direct_accum_observation.rs` is the caller-zero observation boundary for
`LOOP-FAMILY-DIRECT-OBSERVATION-S1`. It consumes one AST-free
`VerifiedDirectAccumSourceAttemptV1` and one owner/source/frame/mode/coverage
context, then emits exactly one of `Candidate`, `Declined`, `Unresolved`, or
`Rejected`. The three sealed modes share the same exact DirectAccum candidate;
known non-Direct shapes decline, incomplete or unsealed windows remain
unresolved, and foreign identity/binding/frame conflicts reject.

`nested_predicate_observation.rs` is now the landed caller-zero NestedPredicate
observation boundary. It consumes one neutral source attempt plus one sealed
identity/mode/coverage context and emits only `Candidate`, `Declined`,
`Unresolved`, or `Rejected`. It has seven focused policy tests and no selector,
Recipe/JoinSig, Builder/MIR, retry/fallback, or production caller. The next
design boundary is LoopTrue.

This observer is intentionally separate from `policy.rs` and the family
selector. It does not read AST or `LoopRouteId`, issue a winner,
Recipe/JoinSig/BindingKey, call Builder/MIR, retry/fallback, or open a
production caller. The seven focused tests and shared recursive guard fix this
boundary. The NestedPredicate S1 implementation is landed in its dedicated
observer; selection remains closed until the common admission-window row.

## LoopTrue S1 implementation receipt

LoopTrue is now a separate caller-zero observer, not a continuation of the
legacy schedule demand below. Its test-only compiler adapter consumes the sole
source projection and maps typed outcomes into a neutral AST-free attempt. The
policy layer consumes only that attempt plus a sealed owner/origin/kind/site/
frame, mode, and coverage context; it issues `Candidate`, `Declined`,
`Unresolved`, or `Rejected` and nothing else. Nine policy tests and eight
projection tests are green. The next boundary is common five-family
selection/admission design.

The current LoopTrue branch cohort below is a separate legacy policy-demand
owner and remains migration-only until common selection and physical cutover.

## Family row context-retention R0 receipt

The five caller-zero observer enums now use typed evidence-bearing variants for
`Candidate`, `Declined`, `Unresolved`, and `Rejected`. Each observer consumes
its source attempt exactly once before validation; no clone, relookup, AST, or
legacy schedule authority is introduced. The 89-test observation suite and
shared row-context guard are green, and all observer files remain below 800
lines. The common admission assembler is the next owner; this module still
has no selector, Recipe, Builder/MIR, production, retry, or deletion caller.

## LoopCond S1 implementation receipt

`loop_cond_break_continue_observation.rs` consumes only the AST-free source
attempt and sealed owner/origin/kind/site/frame, mode, and coverage context. It
emits exactly `Candidate`, `Declined`, `Unresolved`, or `Rejected`; nine policy
tests and five projection tests are green. The legacy LoopCond schedule,
Recipe/JoinSig, Builder/MIR, retry/fallback, and production caller remain
outside this observer.

The LoopTrue branch cohort has a separate policy-demand box:

```text
VerifiedLoopTrueBreakContinueSourceProjectionV1
  -> issue_loop_true_break_continue_policy_demand_v1
  -> VerifiedLoopTrueBreakContinuePolicyDemandV1
```

The demand seals directly from the verified source projection and retains only
a policy receipt and that projection; no raw cursor, schedule, route ID,
Recipe, JoinSig, retry, or physical capability crosses this boundary. The
LoopCond cohort follows the same shape from its typed source map.

At M12, migration-only schedule adapters and opaque route receipts retired:
the synthetic 19-row schedule and winner cursor were removed, and the profile
demands/admissions now seal directly from their upstream-selected inputs.

## M8 all19 closeout S6G receipt (2026-09-23)

`all_route_observation.rs` seals the caller-zero whole-unit coverage products
the S2 selector algebra anticipated. `issue_all_route_observation_set_v1`
accepts exactly 19 rows in `CANONICAL_LOOP_ROUTE_ORDER_V1` order; each row is
`RecipeBacked{backing}` only where the closed `ATTESTED_RECIPE_BACKED_V1`
inventory attests a landed cohort (seven routes after the M12-R2B
GenericResidual retirement: S6A/S6B/S6C-V2/DirectAccum/
NestedPredicate/LoopTrue/LoopCond), otherwise
`PreEffectDeclined{reason}`. At most one row may be `RecipeBacked` per unit —
a second backed row, an unattested backing, a missing/duplicate/out-of-order
route, or a wrong row count is a typed reject, never a silent skip.
`issue_whole_unit_loop_coverage_proof_v1` co-seals a complete set with the
five lease identity fields (owner, origin, source kind, site, frame) so a
foreign or mismatched proof fails `matches_lease`.
