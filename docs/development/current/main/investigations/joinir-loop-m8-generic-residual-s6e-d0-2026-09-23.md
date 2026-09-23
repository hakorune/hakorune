---
Status: open__2026-09-23__producer_cohort_design
Task: JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E
Date: 2026-09-23
Parent: JOINIR-LOOP-M8-LOOPCOND-EXITS-S6D (landed)
PreviousCard: joinir-loop-m8-loopcond-exits-s6d-d0-2026-09-23.md
NextCard: same-row__bounded_producer_cohort_implementation
Implementation permission: false; name the owners to extend and fix the
bounded implementation slice only. No code, no manifest write, no
route/caller change, no new semantic receipt from this card.
---

# JOINIR-LOOP-M8-GENERIC-RESIDUAL-S6E — D0 producer-cohort design

## Six-line brief

```text
Decision: one caller-zero Generic residual producer cohort for the bounded single-induction integer-progression `GenericLoopV1` source profile: a resolver-side source projection, a role-keyed typed source map, a policy demand over the frozen schedule, and one portable Recipe producer — no new selector/verifier/CFG/PHI/physicalizer.
Source authority + canonical issuer: `compiler::issue_generic_residual_source_projection_v1` projects the sealed resolver sites once; `compiler::issue_generic_residual_typed_source_map_v1` is the sole map sealer; `loop_route_policy::issue_generic_residual_policy_demand_v1` is the sole policy-demand issuer; `loop_recipe_contract::produce_generic_residual_recipe_v1` is the sole Recipe issuer for this profile.
Non-authority: AST/name lookup, `GenericLoopV1ShapeId` diagnostic hints, `GenericLoopV0`/`GenericG0` provenance, the five-row family-admission window (sealed; no sixth tag), physical IDs, route-ID semantic dispatch, retry/fallback, the legacy mutating Generic composer, production callers.
Fail-fast boundary: winner cursor must be `GenericLoopV1` (position 18); an overlap selection that wins at `GenericLoopV0` (17) is a typed Unresolved/decline, never re-minted as V1 or G0 provenance; unmatched map/projection/frame is a typed producer reject, not Option/skip.
Smallest next slice: `VerifiedGenericResidualSourceProjectionV1` + `VerifiedGenericResidualTypedSourceMapV1` + `VerifiedGenericResidualPolicyDemandV1`/`ReceiptV1` + `LoopRecipeProducerIdV1::GenericResidualV1` + `generic_residual_producer.rs` emitting `VerifiedLoopRecipeV1` + `VerifiedLoopJoinSigV1` + policy receipt, caller-zero.
Non-claims: no production selection, no `route_loop` connection, no GenericLoopV0 arm, no S6G/M9 work, no Row F unblock claim, no legacy deletion, no caller-zero-retirement claim, no corpus-coverage claim.
```

## Row contract

From `generic-loop-source-to-portable-recipe-ssot.md` (coverage table,
`JOINIR-LOOP-M8-*-S6A..S6E`):

> one producer cohort per stable row: LoopV0 recurrence; exits/joins; scans;
> LoopCond exits; Generic residual — each row emits its common Recipe/core
> product and typed source-policy observation; no new
> selector/verifier/CFG/PHI/physicalizer.

Ordered ladder (`joinir-loop-selfhost-recipe-pipeline-ssot.md`):

```text
S6A (closed) -> S6B (closed) -> S6C (closed) -> S6D (landed)
  -> S6E <- this row -> S6G -> S7A..S7G -> M8/M9 selection design
  -> M10b -> R1/M11/M12
```

The S6E corpus gate recorded in the SSOT is closed: the normalized
corpus is fully observed (P1: 398/398 rows; batches
`849daed591`..`f5514cf608`), disposition-classified (D0:
`f91e5ba82c` + `197ea80fa4`, all 398 rows carry a decision), the
return-signature nondeterminism is repaired (`e6178e7335`), and the
cross-family edge inventory is landed (S0: `798551e8de`, 270 edge
rows). None of that is a producer claim: zero corpus rows prove a
landed GenericLoopV1 portable producer, and `callable-loop` accepted
rows are the existing callable authority, not this cohort.

## Bounded source profile (this card's design decision)

The first Generic residual cohort is a **structural subset** —
single-induction integer-progression `GenericLoopV1` with optional
body-local integer declarations/rebinds — not a
`GenericLoopV1ShapeId` cohort and not a branch/exit profile:

```hako
loop(i <op> bound) {           // op in {Less, LessEqual, Equal};
                               // lhs = the carrier Variable;
                               // rhs = Variable | Integer literal
    local tmp = <i64 lit>      // zero or more body-local integer decls
    tmp = tmp <+|-> <i64 lit>  // zero or more body-local integer rebinds
    i = i <+|-> <i64 lit>      // exactly one terminal top-level
                               // carrier step on the same binding
}
```

This subset is exactly what the existing callable Generic source
authority already admits: `CallableGenericLoopSourceFactsIssuerV1`
reaches `Ready` only for raw selection `[GenericLoopV1]` or
`[GenericLoopV0, GenericLoopV1]`
(`normal_callable_loop_source_facts/generic/issuer.rs:54-117`), and
the carrier relation enforces exactly one Carrier binding equal to
the induction variable with `increment_index` naming a top-level
`Assignment` to `loop_var`
(`generic/carrier_relation.rs:168-200`). It is fully expressible in
the existing `LoopRecipeV1` vocabulary (`ReadBinding`, `ConstI64`,
`BinaryI64{Add,Sub}`, `CompareI64{Less,LessEqual,Equal}`,
`WriteBinding`; `LoopConditionV1::Predicate`; zero declared exits is
a valid Recipe). The landed `generic_loop()` test helper
(`normal_callable_loop_source_facts_tests.rs:53-81`,
`loop(i < limit) { local tmp = 0; i = i + 1 }`) already exercises
this shape and selects exactly `[GenericLoopV1]`.

Out of this cohort (typed rejects / later arms): multi-carrier
accumulation (`sum = sum + i`), non-integer or method-call bodies,
`while`-spelled sources, division/non-{Add,Sub} steps, compound or
derived compares, nested loops, block-expr preludes, and any
`if`/`break`/`continue`/`return` inside the body.
`GenericLoopV1ShapeId` remains diagnostic-only
(`generic-loop-v1-acceptance-by-recipe-ssot.md`,
`generic-loop-v1-shape-ssot.md`) and never defines acceptance.

Winner-cursor decision: the demand accepts only the **Exact**
`GenericLoopV1` winner (position 18 in
`CANONICAL_LOOP_ROUTE_ORDER_V1`). A `[V0, V1]` overlap admission
evaluates left-to-right to winner 17 (`GenericLoopV0`); this cohort
records that as a typed Unresolved/decline — the emitted product is
V1-shaped only, and no `GenericLoopV0` or `GenericG0` provenance is
ever minted (a canonical G0 product must never claim
`GenericLoopV0`/`GenericLoopV1` provenance, and vice versa).

## Census (what exists vs what is missing)

Existing, reusable authority — do not rebuild:

- `normal_callable_loop_source_facts/generic/issuer.rs:54-117`: the
  sole callable Generic source-facts issuer; `Ready` for `[V1]` /
  `[V0,V1]` selections only.
- `generic/source_admission.rs`: `PreparedCallableGenericLoopSourceEvidenceV1`
  (session key = owner + parent/condition/body sites) and
  `CallableGenericLoopSourceRouteAdmissionV1` (`Exact`/`SourceOverlap`).
- `generic/carrier_relation.rs`: single-induction carrier relation
  and typed rejects (`InductionMismatch`, `MissingIncrement`,
  `DeclarationCoverage`, `ForeignOwner`, ...).
- `generic.rs:222-344, 505-543`: semantic recipe owner plus existing
  first-cohort rejects (`NestedLoop*`, `BlockExprPrelude*`,
  `CarrierRelation`) — this row does not extend that authority.
- `loop_route_policy/schema.rs:12-32`: frozen 19-route schedule;
  `policy.rs` evaluator and `policy_evidence.rs` `GenericDebt`
  block — the demand reads the same evaluation, it does not
  re-observe.
- `loop_recipe_contract/{schema,verify,source_binding,join_sig}`:
  verifier, source-bound artifact, provenance, and JoinSig
  elaborator — the S6D producer pattern
  (`loop_cond_break_continue_producer.rs`) is the exact mirror.
- `compiler/callable_single_loop_source_shapes.rs`: neutral
  `SyntaxBinaryOperatorV1`/`SourceLiteralShapeV1` vocabulary.
- `CallableGenericLoopV1PhysicalAdapterV1` + the `raw_loop_child_entry`
  Ready branch: the existing physical chain — untouched by this
  caller-zero row.

Missing (the row's deliverable):

- `VerifiedGenericResidualSourceProjectionV1` in `compiler/` — the
  AST-free resolver-side source projection: source root, loop
  membership/frame, carrier declaration, condition compare site,
  ordered body statement sites (decl/rebind/terminal step). The
  existing callable evidence is builder-private and AST-bearing
  (`generic.rs:102-103` carries `ASTNode`); it cannot serve as the
  AST-free Facts projection this pipeline requires.
- `VerifiedGenericResidualTypedSourceMapV1` in `compiler/` — the
  role-keyed co-seal over the projection: carrier decl, condition
  compare triple (lhs read, operator, rhs shape), ordered body rows
  (decl/rebind/step), ledger-verified; residual variable references,
  exits, nested-loop or prelude rows are typed rejects.
- `VerifiedGenericResidualPolicyDemandV1` /
  `VerifiedGenericResidualPolicyReceiptV1` in `loop_route_policy` —
  the typed source-policy observation; winner cursor ==
  `GenericLoopV1` position (18).
- `LoopRecipeProducerIdV1::GenericResidualV1`.
- `loop_recipe_contract/generic_residual_producer.rs` —
  `produce_generic_residual_recipe_v1` →
  `VerifiedGenericResidualRecipeProductV1` (policy receipt +
  `VerifiedLoopRecipeV1` + `VerifiedLoopJoinSigV1`), caller-zero.

## Owners to extend (bounded implementation slice)

| File | Change | Owner boundary |
| --- | --- | --- |
| `compiler/generic_residual_projection.rs` (new) | `issue_generic_residual_source_projection_v1` + `VerifiedGenericResidualSourceProjectionV1` | resolver sites only; consumes `ResolvedFunctionLoweringInputV1`; no AST, route, Recipe, or physical IDs |
| `compiler/generic_residual_typed_map.rs` + `generic_residual_typed_map_issue.rs` (new) | role-keyed row schema + `issue_generic_residual_typed_source_map_v1` | sole sealer; verifies carrier decl, condition triple, ordered decl/rebind/step rows against the resolver ledger; typed rejects only |
| `loop_route_policy/generic_residual.rs` (new) | `issue_generic_residual_policy_demand_v1(typed_map, schedule)` + demand/receipt | frozen-schedule evaluation; winner must be `GenericLoopV1` (position 18); overlap winner is typed Unresolved; no re-observation, no sixth family tag |
| `loop_recipe_contract/producer_id.rs` | add `GenericResidualV1` | one new variant + id string; never `GenericG0`/`GenericLoopV0` reuse |
| `loop_recipe_contract/generic_residual_producer.rs` (new, <=800 lines) | `produce_generic_residual_recipe_v1` + product type | consumes demand once; frame-key match; predicate block (carrier read + bound read/const + CompareI64) + body block (ConstI64/WriteBinding per decl, ReadBinding+ConstI64+BinaryI64+WriteBinding per rebind/step) + one carrier, zero exits; verified Recipe + JoinSig + receipt; typed rejects only |
| `loop_recipe_contract/mod.rs`, `loop_route_policy/mod.rs`, `compiler` registry | module/export wiring | re-export only |
| focused tests beside each new file | positive product shape + typed reject paths | caller-zero evidence only |

## Fail-fast boundary

- Projection/typed-map issuance requires the exact bounded profile;
  any other operand, operator, declaration, step, exit, or body
  statement shape is a typed map reject — no widening in this row.
- Demand issuance requires winner cursor 18 (`GenericLoopV1`);
  cursor 17 (`GenericLoopV0` overlap) is `Unresolved`, never a
  downgrade or re-mint.
- Producer requires `policy_receipt.frame_key() ==
  projection.root_frame_key()`; mismatch is `PolicyFrameMismatch`.
- Recipe/JoinSig failures propagate as typed producer rejects; no
  `Option`, no retry, no silent skip, no fallback through the legacy
  mutating Generic path.

## Non-claims (stop conditions)

- No production caller, no `route_loop`/registry/selection wiring —
  this row's product stays caller-zero.
- No `GenericLoopV0` arm and no `GenericG0` provenance; the five-row
  family-admission window is not extended.
- No new selector, verifier, CFG, PHI, or physicalizer authority; no
  `LoopRecipeV1` schema widening (V2 stays a separate wire).
- No corpus-coverage claim: the TSV dispositions classify legacy
  fronts; zero accepted rows prove a landed GenericLoopV1 portable
  producer.
- No S6G closeout claim, no Row F
  (`LOOP-PRODUCTION-SELECTION-D0`) unblock claim — Row F remains
  gated on the M10 seal series and M8 all-route coverage.
- No legacy Generic deletion or cutover; `GENERIC-LEGACY-DEAD-CODE-R1`
  stays blocked on M10b.
- No production-switch or caller-zero-retirement claim from green
  tests.

## Exit

When this card is accepted, `work_mode` moves to `fast` for the
bounded implementation slice named above (one producer cohort, one
commit family).
