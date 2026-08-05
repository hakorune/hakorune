# Frozen Loop Route Policy Rows

This module is the neutral M3-C/M3-E boundary for one owned snapshot of the
legacy Loop route schedule and explicit policy evidence. Its legacy schedule
facade and LoopTrue fixture remain caller-zero; resolved DirectAccum and
NestedPredicate handoffs are separate live lanes, while the future Generic
family selector remains caller-zero.

Authority is deliberately narrow:

```text
owned route IDs + owned typed observations/evidence
  -> canonical schedule validation
  -> FrozenLoopRouteScheduleV1 + LoopRoutePolicyEvidenceV1
```

`FrozenLoopRouteScheduleV1` owns exactly the 19 canonical route IDs in raw
cursor order. Each `FrozenLoopRouteRowV1` owns its cursor, an opaque
parity/provenance route ID, typed suppression evidence, one mode/release
snapshot, one global-entry disposition, and one source disposition. The
schedule and its rows are non-`Clone`; consumers receive read-only views only.
A fresh schedule can be issued only from canonical row zero. There is no suffix
or resume constructor.

This module does not own or perform route predicates, suppression evaluation,
winner selection, retry, recipe construction, AST observation, Builder
mutation, composition, lowering, or physical ID allocation. `evaluate.rs`
performs structural validation and row sealing; `policy.rs` performs the pure
M3-E audit and emits only Qualified, Blocked, or Exhausted.

The migration fixture adapter is test-only. Its M3-F parity submodule may invoke
the legacy execution witness as an oracle, but it has no production caller; the
production facade `freeze_loop_route_schedule_v1` remains caller-zero.

The DirectAccum pilot adds one narrower production-shaped handoff:

```text
VerifiedDirectAccumSingletonObservationV1
  -> policy-owned 19-row matrix
  -> freeze/evaluate
  -> VerifiedDirectAccumPolicyHandoffV1
```

Only the policy module constructs that matrix. Non-Accum rows use the typed
singleton-exclusion evidence issued by the source certificate; Generic debt is
never silently projected to `None`. The handoff retains the source observation
for the next Recipe stage while hiding route IDs, raw cursors, and the frozen
schedule from physical lowerers. The handoff is profile-specific and must not
be mistaken for the future family selector; its resolved DirectAccum production
lane is a separate live owner.

The LoopTrue branch cohort has a separate policy-demand box:

```text
VerifiedLoopTrueBreakContinueSourceProjectionV1
  + owned FrozenLoopRouteScheduleV1 (consumed once)
  -> VerifiedLoopTrueBreakContinuePolicyDemandV1
```

Its private brand accepts only the canonical `LoopTrueBreakContinue` winner
and the matching source frame. The demand retains only a policy receipt and
the source projection; it does not retain a raw cursor, schedule, route ID,
Recipe, JoinSig, retry, or physical capability. The implementation is
caller-zero and exists solely as the S1 handoff to the later Recipe cohort.

At M12, migration-only schedule adapters and opaque route receipts retire after
M10/M11 cut over and remove the old physical route edges. Any retained
source-policy rows must remain data-only inputs to the common recursive recipe.

## D4-S3 family-selection boundary

D4-S3 closes the future authority without changing this module's existing
19-route schedule/evidence APIs. `CANONICAL_LOOP_ROUTE_ORDER_V1`, raw cursors,
and `evaluate_frozen_loop_route_schedule_v1` remain legacy migration
provenance; they are not the canonical NestedPredicate/DirectAccum/Generic
family selector. The resolved DirectAccum and NestedPredicate lanes already
have live family-specific handoffs, so this module is not globally
caller-zero; the Generic resolved-carrier selector remains caller-zero.

The next product is a separate resolver-branded, non-`Clone`
`VerifiedLoopFamilyObservationSetV1` containing one source receipt/window, an
exact mode snapshot, a coverage seal, and family-tagged typed dispositions.
A separate `CanonicalLoopFamilySelectionV1` entrypoint will consume it once
and return `Selected`, `NoCandidate`, `Rejected`, or `Unresolved`. It must not
inspect AST/LoopRouteContext/route IDs, reuse raw cursors, invoke Builder or
Recipe production, or retry/fallback. `NoCandidate` requires a sealed proof of
no Loop-family envelope; missing/foreign/incomplete/mismatched observations
remain typed rejection/unresolved. D4-S3-S0 is closed as a private
observation-set witness; selector implementation and Generic production
cutover remain closed.

D4-S3-S0 is now closed as a private witness outside this policy module. The
test-only set owns one resolver receipt, a mode snapshot, a loop-window
coverage seal, and unresolved family rows; it does not call or implement the
future selector. D4-S3-S1 is the next private source-backed matrix row, and
the existing legacy schedule/evidence APIs remain unchanged.
