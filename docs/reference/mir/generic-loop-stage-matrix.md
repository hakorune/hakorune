# Generic Loop V0/V1 Stage Matrix

Status: inspection-only reference
Date: 2026-08-05

This page documents the current test-only evidence boundary for Generic Loop
V0/V1 post-effect debt. It is not a production route policy, Recipe contract,
PHI owner, scheduler, or backend lowering specification.

## Authorities

The design authority is
`docs/development/current/main/design/joinir-generic-post-effect-debt-classification-ssot.md`.
The executable task and acceptance evidence are
`docs/development/current/main/investigations/joinir-generic-structural-grammar-census-d2-a3-s1-execution-task-2026-08-04.md`;
the closed ledger is
`docs/development/current/main/investigations/joinir-generic-post-effect-debt-classification-d0-s1-execution-task-2026-08-04.md`.
The closed overlap parity evidence is recorded in
`docs/development/current/main/investigations/joinir-generic-overlap-semantic-parity-d2-b2-execution-task-2026-08-04.md`.
The closed bounded continuation (implementation row S1) is
`docs/development/current/main/investigations/joinir-generic-nested-carrier-winner-d2-b4-d0-design-2026-08-05.md`.
The next accepted design stop is
`docs/development/current/main/investigations/joinir-generic-nested-carrier-bindingref-disjointness-d2-b4-s2-design-2026-08-05.md`.
The machine-readable test observer is
`src/mir/builder/control_flow/joinir/route_entry/registry/generic_stage_matrix_tests.rs`.

Production selection remains the ordered registry in
`src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs` and
`predicates.rs`. The `loop_route_policy` subtree is test-only evidence and is
not a Generic winner oracle.

## Current source-to-selection evidence

| fixture class | source witness | current generic schedule | status |
| --- | --- | --- | --- |
| V0-only | `v0-additive` | no proven V0-only result | `UnresolvedStop` |
| V1-only | `v1-only` | `GenericLoopV1` | observed |
| Both | `both` | release/strict: `GenericLoopV0, GenericLoopV1`; planner-required: `GenericLoopV1` | observed overlap; precedence unresolved |
| Neither | `neither` | empty | `PreEffectDeclined` before Builder effects |

The `Both` fixture's nested inner Loop is also observed through the actual
depth-1 handoff: release, strict, and strict+planner-required all reach
`NestedDepth1Fastpath = Succeeded` with a Builder delta. The subsequent
`NestedGenericFallback` is `NotYetObserved` because the fastpath succeeds;
fresh-candidate repeats are identical. The matrix's nested `GenericLoopV1`
route label is trace metadata only; it is not a V1 selector or winner claim.

`contract_present = false` is an ordinary current Generic input for release
and strict modes. It is recorded in the matrix; it is not silently converted
to a Generic pre-effect decline. The pure nested-carrier policy probe may still
return `UnresolvedStop` when that contract receipt is absent.

## Stage and disposition contract

The matrix records these stage arms separately:

```text
facts absent/non-match
composer precondition with no candidate delta
composer first allocation/body/pipeline delta
composer error after candidate delta
strict shadow Some/None/Err
release verifier Ok/Err
release lower Some/Ok(None)/Err
nested fastpath and nested Generic fallback
```

The closed debt vocabulary is:

```text
PreEffectDeclined   facts/policy miss with no Builder effect
PreEffectBlocked    source/policy precondition unavailable before mutation
TerminalFreezeTarget candidate was effected; retry would reuse dirty state
ImpossibleEdge      closed invariant proves the arm cannot occur
UnresolvedStop      evidence is insufficient to choose the above
```

An effectful composer/verifier/lowerer failure is never labelled
`PreEffectDeclined`. Unobserved natural arms are retained as
`NotYetObserved`/`UnresolvedStop` rows; no failure injection is used.

The accepted-body re-observation still finds no natural strict shadow `Err`,
release verifier `Err`, or release lower `Err`. Those rows remain explicit
`UnresolvedStop` evidence; strict shadow `None` and release lower `Ok(None)`
retain the valid-Generic completion `ImpossibleEdge` invariant.

The nested diagnostic calls the raw `lower_nested_loop_depth1_any` helper to
preserve an `Err` outcome; production wraps that helper with `.ok()` before its
fallback. This keeps the observer aligned with production order without
creating a second route authority.

D2-A3-S1 has now closed its bounded natural strict/release failure-arm and
nested-depth observation. It preserved the lower-`None` `ImpossibleEdge`
invariant and changed no grammar or IR semantics. This page was synchronized
as the required post-implementation closeout surface; deeper failure arms and
V0/V1 winner equivalence remain parent design-stop work.

## D2-B2 overlap parity evidence

The test-only parity matrix joins the shared production frame, fresh direct
V0/V1 stage rows, semantic digests, and the real witness trace. Release and
strict retain `[GenericLoopV0, GenericLoopV1]`; both direct plans reach
`LowerSome`, but their nested-carrier digests differ. The witness terminates
at V0 with no debt receipt and no V1 attempt. Planner-required suppresses V0
before effect and reaches V1 separately. The pure probe and final comparison
remain `UnresolvedStop`; no winner or retry policy follows from this evidence.
The matrix is closed as deterministic evidence;
`ParityDispositionV1::UnresolvedStop` is a classification, not a policy
evaluator or winner. The next bounded design stop is D2-B4: a test-only
certificate candidate for complete recursive-carrier observations with a
natural V1 stage; all other classes remain unresolved.

## Snapshot ownership

The matrix compares `before_compose`, `before_lower`, and `after_lower`
snapshots containing block count, next ValueId, typed-value count, and variable
map size. Variable-map restoration is not candidate rollback: the composer can
leave block/value/type counters changed. Therefore `GenericComposer` is the
first effect owner whenever the compose delta changes those counters, even if a
later verifier is pure.

## Non-claims

This reference does not claim:

* V0/V1 semantic precedence or winner equivalence;
* a debt-to-later-winner trace;
* a portable Generic Recipe producer or consumer;
* shared JoinSig/PHI/physicalizer ownership;
* retry/fallback removal or JoinIR deletion;
* any language grammar or source syntax change.

## D2-B4-S1 certificate snapshot

The test-only S1 matrix uses the existing `Both` fixture. In release and strict
mode the frozen raw schedule is `[GenericLoopV0, GenericLoopV1]`; the recursive
facts label is `["j"]`, the natural V1 stage is `LowerSome` with
`GenericComposer` as its first effect owner, and a fresh repeat is stable. The
V1 outer final-value list is `["i", "j"]`; the carrier-projected subset is
`["j"]`, selected by the required `loop_carrier_j` and `loop_step_in_j` tags.
The legacy witness still attempts/terminates at V0 with no debt receipt, so the
parent D2 disposition remains `UnresolvedStop`.

Planner-required is a separate row with raw schedule `[GenericLoopV1]`; it does
not issue an overlap certificate. The certificate DTO and all five focused
tests live under `cfg(test)`; production selection, Recipe/JoinSig/PHI,
physicalization, Retry, and scheduler authority remain unchanged.

Those claims remain blocked until the parent M4 design stop closes with a
complete matrix, precedence/disjointness proof, and witness equivalence.

## D2-B4-S2 BindingRef disjointness witness

The bounded S2 witness is green with:

```bash
env -u HAKO_JOINIR_STRICT -u HAKO_JOINIR_PLANNER_REQUIRED \
  RUSTFLAGS='-Awarnings' cargo test --lib generic_d2_b4_s2 -- --nocapture
```

Its source authority is one parsed outer-`j` function, the sealed resolved
loop forest, resolver-issued assignment/read `BindingRefV1`s, the shared
function/frame identity, and canonical `GenericLoopV1Facts` observation. The
positive Release/Strict row captures `[GenericLoopV0, GenericLoopV1]`; the
shadowing row resolves the inner `local j` to a different binding and remains
`UnresolvedStop`. The strict planner-required row records V0 as
`SuppressedByPlannerRequired`, captures `[GenericLoopV1]` under the same mode
scope, and remains unresolved. V0/V1 final-value and PHI tags from the older
S1 observer are corroborating only and are not used as BindingRef authority.

This is cfg(test)-only evidence (443-line sibling, no production caller). It
does not claim runtime-result parity, V0/V1 precedence, a winner, a Generic
Recipe/JoinSig/PHI/physicalizer consumer, Retry/fallback removal, or any
Builder/MIR/backend route change. The exact source and typed suppression
boundary are recorded in the closed S2/D3 checkpoints and the active handoff
design card.

S2 and scoped D3 are closed as bounded evidence. The projector coverage row is
also closed as test-only evidence in the co-sealed source-to-selection handoff card
`investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`.
Its five focused tests include one parsed S2A nested-`IfThen` source-view path,
resolver/source/frame/facts-only co-seal, and typed cross-invocation mismatch.
It does not authorize a production selector, Recipe/JoinSig/PHI/physicalizer
caller, or Retry/fallback change. The parent Generic D2 disposition remains
unresolved.
The cfg(test)-only source-backed handoff bridge
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-BRIDGE0-D1` is closed. It connects one
parsed S2A projector receipt to actual facts/raw schedule/frame flags for
Release/Strict natural Both, and rejects a cross-invocation pairing before
selection. It adds no neutral issuer or production selector. The proposed
V0-only/CompleteNoRecursive subrow was rejected by premise audit because the
existing additive matrix is synthetic and does not establish a parsed source
row; no natural V0-only witness is proven. The planner-suppression row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-PLANNER-SUPPRESSION0-D2-S1` is
closed as cfg(test)-only evidence: the existing parsed S2A source runs under
actual Strict+planner-required mode, co-seals resolver/facts/frame/mode
evidence, and proves raw `[V1]` with typed
`UnresolvedStop(PlannerRequiredV0Suppression)`. No Legacy, eligibility, winner,
or production selector is implied; the parent source-to-selection boundary
remains a design stop.

The bounded row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-INDEX-AMBIGUOUS0-D2-S2` is
closed as cfg(test)-only evidence. One parsed S2A-shaped nested IndexWrite
(`items[j] = i`) co-seals resolver `IndexWrite`, facts
`Ambiguous("assignment target")`, exact source/forest/frame identity, actual
Release/Strict mode, and raw `[GenericLoopV0, GenericLoopV1]`. The typed result
is pre-effect `UnresolvedStop(IndexWriteAmbiguousCarrier)`; no eligibility
issuer or selector arm is implied. The bounded
`JOINIR-GENERIC-RESOLVED-CARRIER-ELIGIBILITY-PROTOCOL0-D3-S0` row is also
closed as cfg(test)-only evidence: actual Release/Strict natural-Both
`CompleteRecursiveCarrier` is the only test-only eligible result, while
planner, shadowing, missing-capability, and cross-invocation mismatches remain
typed unresolved. It does not close the production handoff;
Compound/Unavailable D2-S3 is now closed as the adjacent source-matrix row and
the parent D3 design stop remains current.

The scoped D3 matrix is now also green as one cfg(test) test over four typed
rows: natural Release, natural Strict, shadowing negative, and planner-required
V0 suppression. Its evaluator separates pre-effect BindingRef eligibility from
post-effect V1 corroboration. The projector coverage row is still test-only;
the source-to-selection handoff card remains the design authority
`investigations/joinir-generic-resolved-carrier-selection-boundary-d3-design-2026-08-05.md`;
no production selector change is implied.

The bounded source row
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-COMPOUND-UNAVAILABLE0-D2-S3`
is closed as cfg(test)-only evidence. It uses a parsed nested
`CompoundAssignment` under scoped basic sugar, actual resolver/source/frame/
BindingRef evidence, and the facts-owned
`Unavailable("CompoundAssignment")` disposition. Release/Strict measured raw
schedule is `[V0,V1]`; the only result is typed pre-effect
`UnresolvedStop(CompoundUnavailableCarrier)`. Top-level compound behavior,
eligibility, Legacy, winner/precedence, and production handoff remain outside
this row; execution returns to the parent D3 design stop.

The selected premise was
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-TOPLEVEL-COMPOUND-PREMISE0-D2-S4`.
It is not a policy row: one parsed top-level `CompoundAssignment` must first
be observed through resolver/source/frame identity and the facts extractor.
The result space was open between exact `CompleteNoRecursiveCarrier`,
`Unavailable`, `Ambiguous`, and typed `NoStandaloneRow`. The implementation
observed typed `NoStandaloneRow`: the parsed resolver/BindingRef/source/frame
witness is present, but no facts product is emitted and Release/Strict both
measure raw schedule `[]`. This is cfg(test)-only evidence and does not
authorize collector widening, selection, eligibility, Legacy/winner policy,
Recipe, PHI, Builder, MIR, Retry, fallback, or production handoff. The linked
task, current mirrors, and this reference page were closed together.

The next design child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`.
It is a docs-only boundary for choosing one parsed flat Assignment shape and
its disposition. `CompleteNoRecursiveCarrier` is an observation label, not a
winner or eligibility proof; the provisional one-member result is typed
`UnresolvedStop(NonRecursiveOutOfTarget)`, while facts absence or empty raw
schedule is `NoStandaloneRow`. Simple-while, local/effect V1-only,
CompoundAssignment, selector, Legacy, Recipe, PHI, Builder, MIR, Retry,
fallback, and production handoff remain separate.

## D2-B4-S2A nested `IfThen` carrier evidence

The bounded S2A row is closed as one parsed, `cfg(test)`-only carrier witness.
The source has an outer loop, an inner loop, a nested `IfThen` write to `j`, a
separate canonical inner `j` step, and a post-loop `j` read. Resolver-issued
`BindingRefV1` identity, strict ancestry, source/frame identity, and the exact
two-member loop forest are asserted. Release/Strict raw schedules remain
`[GenericLoopV0, GenericLoopV1]`; fresh direct V0/V1 stages are `LowerSome`
with `GenericComposer` as first effect owner and stable distinct digests. The
V1 witness records `CompleteRecursiveCarrier(["j"])`; the legacy witness still
terminates at V0 without a debt attempt.

This is inspection-only evidence. It does not select a winner or add a Generic
Recipe, JoinSig, PHI, physicalizer, Builder, MIR, backend, Retry, fallback, or
runtime consumer. Parent Generic D2 and the co-sealed source-to-selection
handoff remain unresolved; the current facts-only selector is unchanged.
