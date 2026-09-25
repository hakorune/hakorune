# Loop Route Policy

This module owns the family observation/assembler/selector boundary plus the
profile-specific policy admissions for Loop rows. Resolved DirectAccum,
NestedPredicate, LoopTrue, LoopCond, and GenericG0 handoffs are live lanes.

Authority is deliberately narrow:

```text
owned typed observations/evidence
  -> five-family admission window
  -> canonical family selection
  -> profile-specific policy demand/admission (no route cursor)
```

The M12-R2C part 2 retirement removed the synthetic 19-row
`FrozenLoopRouteScheduleV1` and the `VerifiedLoopPolicyWinnerV1` ceremony:
family selection happens upstream in `family_selector.rs`, so the profile
admissions seal directly from their source-side observations and carry no
route ID, raw cursor, or winner. `CANONICAL_LOOP_ROUTE_ORDER_V1` remains only
as all-route coverage/provenance vocabulary for `all_route_observation.rs`;
it is not a semantic order and must not drive lowering.

This module does not own or perform route predicates, winner selection, retry,
recipe construction, AST observation, Builder mutation, composition, lowering,
or physical ID allocation.

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

## Generic G0 demand S3 I0/R0

The test-only `loop_recipe_contract::generic_g0_demand` seam consumes the
canonical `Selected(Generic)` product once. It validates the selected lease
against candidate evidence and the borrowed handoff brand, then retains one
move-only demand with the typed source bundle, post-loop tail read,
profile/mode/coverage, and an opaque role proof. It does not select another
family, issue Recipe/JoinSig/Core/After keys, call Builder/MIR, retry, fallback,
or open a production caller. The worker-reviewed S4 design and caller-zero
Recipe producer `GENERIC-G0-RECIPE-S4-I0-R0` are now closed. The producer lives
under `loop_recipe_contract::generic_g0`; this policy module does not issue
Recipe keys or aggregate products. The next ordered boundary is the legacy
corpus universe P0, while production selection remains closed.

S4 consumes this demand once and is the sole Generic Recipe/effect/provenance
producer. It must call common Recipe/JoinSig/Core owners rather than re-create
their products. Its After envelope owns the logical tail and exact return ABI;
P0 owns executable completion/DraftSeal. This policy module does not gain a
Recipe or physical authority.

The DirectAccum pilot's live handoff is:

```text
VerifiedDirectAccumSingletonObservationV1
  -> issue_direct_accum_route_admission_v1
  -> VerifiedDirectAccumPolicyHandoffV1
```

The admission seals directly from the source-side singleton observation; there
is no intermediate schedule, raw cursor, or winner evaluation. The handoff
retains the source observation for the next Recipe stage and is
profile-specific — it must not be mistaken for the family selector, which
already ran upstream in the production spine.

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

## Common admission D0 design receipt

The canonical five-row admission window is now worker-reviewed but remains
caller-zero. Its required semantic tags are `DirectAccum`, `NestedPredicate`,
`LoopTrueBreakContinue`, `LoopCondBreakContinue`, and `GenericG0`. A resolver-
issued AST-free window identity brand is co-sealed with one typed
`Candidate|Declined|Unresolved|Rejected` row per tag; legacy `Blocked` belongs
only to the schedule evaluator. The assembler checks identity/mode/coverage
and does not select. LoopCond S1 and Generic normalization now supply bounded
C/D/U/R observers. FAMILY-ROW-CONTEXT-RETENTION-R0 is also landed: every
family disposition keeps expected/observed identity, mode, and coverage
evidence. The resolver-owned `LOOP-FAMILY-WINDOW-LEASE-ISSUER-S0` source-brand
prerequisite is now landed in `resolved_semantics`; the next ordered cell for
this module is the common assembler. Selector promotion remains separate.

## Common admission assembler S1 receipt

`LOOP-FAMILY-COMMON-ADMISSION-ASSEMBLER-S1` is landed in
`family_admission.rs` as the sole cross-family admission owner. It consumes
one resolver-issued, non-`Clone` `VerifiedLoopFamilyWindowLeaseV1` and an
arbitrary-order move-only row vector, then co-seals exactly five typed family
rows with one common mode and complete coverage. Success stores fixed
DirectAccum, NestedPredicate, LoopTrueBreakContinue, LoopCondBreakContinue,
and GenericG0 fields; failure stores the lease, every input row, and typed
issues.

The assembler validates identity/frame, duplicate/missing tags, mode, coverage,
and row C/D/U/R status only. Rejected evidence dominates unresolved evidence.
It does not reissue source, count candidates, reject semantic overlap, handle
`OutOfWindow`, select a winner, or call Recipe/Builder/MIR. Six focused tests
and the shared caller-zero/line guard are green. The next owner is the pure
selector; production and legacy cutover remain closed.

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

## D4-S3 family-selection boundary

D4-S3 closes the future authority. `CANONICAL_LOOP_ROUTE_ORDER_V1` remains
migration provenance vocabulary for the all-route coverage proof only; it is
not the canonical NestedPredicate/DirectAccum/Generic family selector and the
19-route schedule/evaluator APIs are retired. The resolved DirectAccum and
NestedPredicate lanes already have live family-specific handoffs, so this
module is not globally caller-zero; the Generic resolved-carrier selector
remains caller-zero.

The next product is the separate resolver-branded, non-`Clone`
`VerifiedLoopFamilyAdmissionWindowV1` containing one identity-only source
lease, five canonical family rows, and assembler-owned mode/coverage seals.
A separate `CanonicalLoopFamilySelectionV1` entrypoint will consume only an
assembler `Ready(window)` once and return `Selected`, `Rejected(Overlap)`, or
`Unresolved(OutOfWindow)`. It must not inspect AST/LoopRouteContext/route IDs,
reuse raw cursors, invoke Builder or Recipe production, or retry/fallback.
Missing/foreign/incomplete/mismatched observations are assembler failures and
never enter the selector. `NoCandidate` requires a separate sealed whole-unit
proof and is not an S2 outcome. D4-S3-S0 is closed as a private
observation-set witness; selector implementation and Generic production
cutover remain closed.

D4-S3-S0 is now closed as a private witness outside this policy module. The
test-only set owns one resolver receipt, a mode snapshot, a loop-window
coverage seal, and unresolved family rows. D4-S3-S1 is also closed outside
this module as nine private source-backed fixture/mode matrix sets with typed
NoStandaloneRow/planner-freeze/reject separation; neither row calls or
implements the future selector. The next private row is the pure selector
consumer.

The historical `#[cfg(test)]` `family_selection.rs` marker was removed in
M12-R2C part 2 together with the synthetic schedule machinery; it was never
the canonical selector.
The next implementation adds a separate `family_selector.rs` consuming only
the common assembler's Ready window. Production selector, Recipe/key,
Builder/MIR, retry/fallback, and Generic caller remain zero until that new
consumer is independently verified.
D4-S4-D0 records that a future `Selected(Generic)` must retain a real
resolver source lease, candidate proof, and `BindingRef` roles; this selector
must not feed a Recipe from its current marker-only outcome. D4-S4-S0 is now
closed as `NoSafeSlice`: the shallow cfg(test)-only Generic candidate envelope
exists, but no selected callsite or Generic demand exists. D4-S4-S0-D0 closes
the future move-only lease -> shape ->
observation -> selector -> demand chain; this policy module only moves opaque
capabilities and never issues keys. GENERIC-SEMANTIC-SHAPE-SCHEMA-D1 is now
closed as the typed shape contract, and the bounded cfg(test)-only
source-lease/CarrierProof witnesses are closed. The next row is design-only
shape role extension; selector and demand remain gated.

## Family selector S2 implementation receipt (2026-08-06)

`family_selector.rs` is now the caller-zero consumer of the assembler's
`Ready(window)` product. It consumes the window by value exactly once and
returns only the fixed algebra:

```text
1 Candidate + 4 Declined -> Selected
2+ Candidates            -> Rejected(Overlap)
5 Declined               -> Unresolved(OutOfWindow)
```

The selected product retains the resolver lease, common mode/coverage, family
tag, and typed candidate. Failure products retain the consumed lease and all
five rows. The selector contains no source lookup, AST/resolver issuer,
route/schedule access, Recipe/JoinSig, Builder/MIR, retry, fallback, or
production call.

Three focused selector tests cover all five candidate variants, retained
`OutOfWindow` evidence, and overlap retention. The shared selector guard checks
the caller-zero boundary and the <800-line source/test limit. This is a
caller-zero semantic product only. The bounded
`GENERIC-SELECTION-OPEN-D0-I0-R0` candidate-envelope witness is closed; the
`GENERIC-SELECTION-POLICY-HANDOFF-D0` design and its single I0/R0 caller-zero
implementation are closed. The current accepted boundary is
`GENERIC-G0-DEMAND-S3-D0` (see
`docs/development/current/main/investigations/generic-g0-demand-s3-design-task-2026-08-07.md`):
the selector owns one canonical window lease, the Generic handoff borrows only
an opaque brand projection, and Demand consumes the selected Generic product
once while retaining exact source roles. Physicalization, production cutover,
and legacy deletion remain separate rows.
The implementation commit updates this README, the loop SSOT, reference
matrix, workstream, and current mirrors together as the required
post-implementation reference receipt.

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
foreign or mismatched proof fails `matches_lease` inside the selector.

`select_canonical_loop_family_v1` now takes the proof as a second argument
and the outcome algebra is closed:

```text
1 Candidate + 4 Declined            -> Selected (retains the coverage proof)
2+ Candidates                       -> Rejected(Overlap)
0 Candidates + all-declined proof   -> NoCandidate(proof)
0 Candidates + backed proof         -> Rejected(CoverageBackedWithoutCandidate)
foreign coverage identity           -> Rejected(CoverageIdentityMismatch)
```

The stale `Unresolved(OutOfWindow)` arm is removed: window input without a
coverage proof can no longer reach the selector, so every call resolves
through the sealed proof. `NoCandidate` is opened only here and only under a
fully pre-effect-declined set; the selected-family/backed-route forward
correspondence is not re-derived because no authoritative family-to-route
map exists and inventing one would make route IDs semantic authority. The
selector still creates no route, Recipe, Builder, MIR, or physical product
and the production caller remains zero.
