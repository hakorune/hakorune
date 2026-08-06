# Generic Loop V0/V1 Stage Matrix

Status: inspection-only reference
Date: 2026-08-06

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
The former “next accepted design stop” pointer to D2-B4-S2 is historical and
superseded. Current Generic source-bridge work is tracked by
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-shared-source-bridge-d3-s2-d4-design-2026-08-05.md`;
this inspection-only matrix does not become its selector or Recipe authority.
The machine-readable test observer is
`src/mir/builder/control_flow/joinir/route_entry/registry/generic_stage_matrix_tests.rs`.

Production selection remains the ordered registry in
`src/mir/builder/control_flow/joinir/route_entry/registry/selection.rs` and
`predicates.rs`. The `loop_route_policy` subtree is test-only evidence and is
not a Generic winner oracle.

## S0A implementation receipt

`GENERIC-G0-STRUCTURE-S0A` is now landed as a disconnected, caller-zero
structural witness. `src/mir/compiler/generic_g0_projection/mod.rs` performs
the exact natural-source navigation; `src/mir/loop_structural_facts/generic_g0/mod.rs`
issues the move-only AST-free product. The row verifies nested body order,
resolver `BindingRefV1` relations, owner/source/frame identity, and complete
duplicate-free coverage. Focused positive/negative tests and the shared
MirBuilder replacement guard are green. No type/numeric policy, candidate,
selector, Recipe, Builder/MIR, retry/fallback, or production support claim is
made; `GENERIC-G0-SOURCE-TYPE-S0B` is the next row.

## S0B implementation receipt

`GENERIC-G0-SOURCE-TYPE-S0B` is now landed as a disconnected, caller-zero
source-type witness. The compiler projector derives one callable header view
from the natural function root and emits exact owner-branded parameter/result
sites plus the four S0A literal role/context rows. The sole AST-free issuer in
`src/mir/resolved_semantics/generic_g0/` validates parameter binding origin,
raw type spelling, annotation presence, literal cardinality, and owner
relations, then moves the result with S0A into
`VerifiedGenericSourceBundleG0`.

Focused natural-source tests cover explicit `i64` headers, missing parameter
and result annotations, and a known non-`i64` parameter. The shared replacement
guard covers the recursive semantic directory, source/test line cap, and
caller-zero issuer boundary. S0B does not infer types, retag literals, choose
numeric representation, issue policy/Recipe keys, or enter Builder/MIR/
production; `GENERIC-G0-NUMERIC-REPRESENTATION-S0C` is the next row.

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

## D4-S4 Generic Recipe handoff boundary

D4-S4-D0 is closed as a design-only authority decision. The current
`SelectedFamilyV1` is a marker without source/window/`BindingRef` provenance,
and the current Generic V0/V1 facts plus P2 snapshot are AST/Builder-derived;
none is a portable Recipe input. A future `Selected(Generic)` must retain one
resolver-issued source lease/window, exact mode/coverage, a sealed Generic
candidate envelope, and role-level `BindingRef` provenance. Window `V1Only`,
`Both`, `Neither`, `NoStandaloneRow`, or planner-unsealed evidence cannot issue
that selection.

The future Generic-specific demand must be distinct from the legacy
`VerifiedSelectedLoopRecipeDemandV1`. Only the dedicated Generic Recipe
producer may issue `LoopBindingKeyV1` and seal the internal
`BindingRef`/recipe-key/source-role effect relation. Binding SSA remains the
sole `BindingRef` -> `ValueId`/`PHI` owner. Recipe/effect failure is terminal;
legacy route reconstruction, retry, fallback, and Generic-as-DirectAccum or
NestedPredicate aliases are forbidden. The following paragraph is retained as
historical handoff context: D4-S4-D0 through D4-S4-S3-S0 subsequently closed
as design/test-only evidence, without a public semantic row or production
caller. D4-S4-S3-D1 and S1-S1 have since closed their authority boundaries.
The current blocker is the shallow `GENERIC-SELECTION-OPEN-D0` promotion gate
design in `CURRENT_STATE.toml`; deep D4 substrate/policy evidence is closed.
No `Selected(Generic)` or Recipe claim is implied here.

```text
resolver SourceLease -> AST-free Generic shape/candidate envelope
  -> policy mode/coverage observation -> selector -> Generic demand
  -> Recipe producer (sole key/effect owner) -> Binding SSA (sole PHI owner)
```

Any later implementation cell must update this reference, the active/current
mirrors, and affected support READMEs in the same commit. A cfg(test)-only
numeric/policy witness still does not create a public reference row; only a
public semantic contract or production consumer may do so.

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

The D3-S2 P1 source-projection packaging is also closed as inspection-only.
The existing non-`Clone` resolver provenance product and projector/source
bridge remain the sole evidence owners. The machine-readable witness is
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-source-projection-d3-s2-p1-matrix-2026-08-05.tsv`;
it records exact source paths, resolver owner brands, `BindingRefV1` role
relations, strict-ancestor results, and typed pre-effect mismatch reasons.
This does not promote Generic facts, select a route, issue a Recipe key, or
add a production caller. The selected next row is the neutral AST-free facts
snapshot design/test task
`JOINIR-GENERIC-RESOLVED-CARRIER-FACTS-SNAPSHOT0-D3-S2-P2`.

The D3-S2 P2 neutral facts snapshot is now closed as cfg(test)-only evidence.
It consumes exactly one sealed P1 resolver provenance product and adds only
the mode-neutral `NestedWriteWithPostLoopRead` disposition. It does not modify
`LoopFacts`, `LoopStructuralFactsPayloadV1`, Generic V0/V1 facts, selector,
Recipe, Builder, MIR, PHI, Home, debt, retry, fallback, or runtime ownership;
P1 typed rejects remain the sole source/owner/frame gate. No production caller
is authorized.

The P3 bounded independent-column family-overlap census is closed as
cfg(test)-only evidence:
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-OVERLAP-CENSUS0-D3-S2-P3`. It records
raw Generic mode/carrier/schedule rows separately from resolved
NestedPredicate/DirectAccum/A+ rows and an explicit canonical rejection.
Fixture labels are reporting-only because no common source/owner/frame brand
exists. Existing overlap remains precedence evidence, not an exact
disjointness proof. The only cross-authority result is
`UnresolvedStop(FamilyOverlap)`; no shared classifier, winner, selector,
Recipe, BindingKey, Builder, MIR, or production caller is added.

## D4 shared source-window witness and next route stop

`JOINIR-GENERIC-RESOLVED-CARRIER-SHARED-SOURCE-BRIDGE-WITNESS0-D3-S2-D4-S0`
is closed as a private `#[cfg(test)]` transport witness in
`src/mir/shared_loop_source_window_tests.rs`. One non-`Clone` resolver-owned receipt
lends paired raw/resolved views through a consuming `with_views` call. Four
focused tests cover the canonical nested-loop row plus foreign-owner,
non-loop, and equal-shape distinct-session rejects. This proves source
owner/site/frame/forest identity only; it does not prove family disjointness or
authorize a classifier, selector, Recipe, Builder/MIR, or production caller.

The D4-S1 DirectAccum route design is accepted: the resolver/source unit stays
the sole identity authority and the existing DirectAccum preflight probe is
the first test-only consumer. The raw Generic edge, NestedPredicate
precedence, A+ fallback, and retry/fallback boundaries remain unchanged.

### D4-S1 witness closeout and D4-S2 boundary stop

The D4-S1-S0 witness is closed as cfg(test)-only. It consumes the D4 paired
source views, confirms the exact existing DirectAccum source-unit probe admits
the canonical Local/Loop envelope, and records foreign/non-loop receipt
rejects plus a loop-body-shape terminal reject before Builder effects. It adds
no production caller or family selector.

The next active row is the docs-only
`JOINIR-GENERIC-RESOLVED-CARRIER-FAMILY-BOUNDARY-DESIGN0-D4-S2`. It must freeze
one owner map and one complete disposition matrix for raw Generic V0/V1 and
resolved NestedPredicate/DirectAccum/A+ observations across modes, carrier
completeness, shadowing, owner/frame mismatch, and the listed unsupported
shapes. Natural Both remains `UnresolvedStop(FamilyOverlap /
WinnerCorrectnessUnavailable)`; planner-required V0 suppression remains typed
unresolved. No selector, retry, fallback, or edge retirement is authorized
until that design is accepted.

D4-S2 owner map and boundary are now frozen as docs-only policy: resolver owns
source identity; neutral `loop_structural_facts` may own only AST-free
facts/eligibility; the Recipe producer alone issues `LoopBindingKeyV1`; one
non-`Clone` canonical plan co-seals route-affecting inputs; and
`registry/selection.rs` alone may consume that plan for policy. Its matrix is
`V0-only|V1-only|Both|Neither` × `Release|Strict|planner-required` ×
`CompleteRecursive|CompleteNoRecursive|Unavailable|Ambiguous` × source relation
(`exact|shadowing|foreign/mixed|missing`) × shape (`exact|nested-wrapper|
duplicate-write|Index|Program|CompoundAssignment`), with resolved
NestedPredicate/DirectAccum/A+/canonical-reject columns independent. No old
edge is retired here; later cutover requires one selector, duplicate caller
zero, same-commit old-edge deletion, and retry/fallback zero.

### D4-S2-S0 legacy same-source census (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-LEGACY-SAME-SOURCE-CENSUS0-D4-S2-S0` is a
private `#[cfg(test)]` retirement inventory, not canonical policy evidence. It
consumes one resolver-owned non-`Clone` source receipt for each of six rows:
`nested-predicate` and `direct-accum` × `Release`, `Strict`, and
`StrictPlannerRequired`. Each row retains resolver owner/site/frame plus
`legacy_*` raw facts status, V0/V1 presence, carrier, raw schedule, and the
existing resolved preflight family.

The measured rows are exact and mode-stable: nested-predicate is
`CompleteRecursive(["j", "sum"])` with legacy
`[NestedLoopMinimal, GenericLoopV1]` and resolved `NestedPredicate`; direct-
accum is `CompleteNoRecursive` with `[AccumConstLoop]` and resolved
`DirectAccum`. All six rows are `Available`, V0 absent, and V1 present. The
census does not issue a selector, winner, Recipe/key, Builder/MIR effect, or
retry/fallback, and does not retire an old edge. D4-S3-D0 is now closed as the
docs-only authority decision; D4-S3-S0 is the closed private observation-set
witness and D4-S3-S1 is the next matrix-only row.

### D4-S3-D0 canonical selection authority (closed design)

The future canonical product is a resolver-branded, non-`Clone`
`VerifiedLoopFamilyObservationSetV1`: one source receipt/window, one exact
mode snapshot, one coverage seal, and family-tagged rows with typed
`Candidate|Declined|Blocked|Unresolved` dispositions. Semantic family tags
are not route IDs. The set contains no AST, raw schedule/cursor, Recipe/key,
Builder/MIR/ValueId/PHI, retry, or fallback.

A new family-level `CanonicalLoopFamilySelectionV1` in
`mir::loop_route_policy` is the future sole selector and consumes the set once,
returning `Selected|NoCandidate|Rejected|Unresolved`. `NoCandidate` requires a
sealed whole-unit proof that no Loop family envelope exists; missing or
foreign identity, incomplete coverage, planner-unspecified suppression, and
BindingRef/frame mismatch remain typed rejection/unresolved. A+/Trivial stay
outside this Loop-family set. The existing 19-route evaluator and the live
DirectAccum/NestedPredicate resolved lanes are preserved as migration/live
owners; Generic selection remains caller-zero. D4-S3-S0 is a closed private
observation-set witness, not a selector or production cutover.

### D4-S3-S0 observation-set witness (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-OBSERVATION-SET0-D4-S3-S0` is
closed as a private `cfg(test)` witness in
`src/mir/shared_loop_source_window_tests.rs`. Each
`TestLoopFamilyObservationSetV1` owns one non-`Clone` resolver receipt, one
private Release/Strict/planner-required mode snapshot, one loop-window-only
coverage seal, and three semantic family rows. All rows are typed
`Unresolved`; this proves the transport shape without selecting a winner,
precedence, or `NoCandidate` result.

The focused test covers two existing fixtures across three modes (six sets)
and checks owner/origin/source-kind/site/frame correspondence through the
consuming paired-view seam. No route ID, raw schedule/cursor, AST, Recipe/key,
Builder/MIR/ValueId/PHI, retry/fallback, selector, or production caller is
introduced.

### D4-S3-S1 canonical matrix (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-MATRIX-CLOSE0-D4-S3-S1` is closed
as a private registry witness. It issues one resolver-branded non-`Clone`
source-window receipt for three parsed fixtures (`Both`, `V1Only`, and the
existing `NoStandaloneRow`) across `Release`, `Strict`, and
`StrictPlannerRequired`: nine sets. Each set consumes its receipt once and
records resolver identity, facts status, V0/V1 presence, carrier provenance,
and four explicit presence cells (`V0Only`, `V1Only`, `Both`, `Neither`).

`NoStandaloneRow` is never collapsed into a real `Neither` Generic presence;
`V0Only` and a parsed `Neither` source remain `NotYetObserved`. The natural
`Both` fixture is V0/V1 in Release/Strict and observes mode-local V1-only under
planner-required V0 suppression; this is unresolved evidence, not intrinsic
winner or suppression policy. A planner-required facts freeze leaves all
cells unobserved. Foreign-owner and non-Loop inputs remain typed rejects.
The witness calls the facts owner directly and introduces no legacy schedule
selection, selector/winner/precedence, Recipe/key, Builder/MIR, retry,
fallback, runtime, or production Generic caller.

The next row is the private pure selector consumer
(`...CANONICAL-SELECTOR-PURE0-D4-S3-S2`).

### D4-S3-S2 pure selector (closed)

`JOINIR-GENERIC-RESOLVED-CARRIER-CANONICAL-SELECTOR-PURE0-D4-S3-S2` is
closed as a private `#[cfg(test)]` neutral consumer in
`src/mir/loop_route_policy/family_selection.rs`, separate from the legacy
19-route evaluator. The registry adapter passes only a neutral window-complete
Generic evidence row. It does not pass AST, LoopRouteContext, fixture labels,
owner coordinates, route IDs, raw schedules/cursors, or legacy policy
evidence.

The outcome vocabulary is typed `Selected`, `NoCandidate`, `Rejected`, and
`Unresolved`, but the current S1 input cannot construct the first two: a
window-complete seal is not a whole-unit no-Loop proof. All nine source/mode
rows therefore remain `Unresolved`, preserving overlap, V1-only,
NoStandaloneRow, and planner-mode-unsealed reasons. Foreign/non-Loop source
window rejects remain before the selector. No Recipe/key, LoopBindingKeyV1,
Builder/MIR, retry/fallback, runtime, or production Generic caller is added.

The next row is the design-only Generic Recipe handoff
(`...GENERIC-RECIPE-HANDOFF0-D4-S4-D0`).

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

The accepted implementation child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-NORECURSIVE-DISPOSITION0-D2-S5-D0`.
It is a docs-only boundary for choosing one parsed flat Assignment shape and
its disposition. `CompleteNoRecursiveCarrier` is an observation label, not a
winner or eligibility proof; the provisional one-member result is typed
`UnresolvedStop(NonRecursiveOutOfTarget)`, while facts absence or empty raw
schedule is `NoStandaloneRow`. Simple-while, local/effect V1-only,
CompoundAssignment, selector, Legacy, Recipe, PHI, Builder, MIR, Retry,
fallback, and production handoff remain separate.

The implementation child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-BOTH-NORECURSIVE0-D2-S5-S1`.
It may add exactly one parsed flat Assignment witness. Exact
`CompleteNoRecursiveCarrier` plus measured `[V0,V1]` maps only to typed
`UnresolvedStop(NonRecursiveOutOfTarget)`; facts absence, empty raw schedule,
simple-route/V1-only schedules, shape drift, and identity drift return to the
D2-S5-D0 design stop.

The S1 witness is now closed as cfg(test)-only evidence. It observes exact
`CompleteNoRecursiveCarrier` with Release/Strict raw `[GenericLoopV0,
GenericLoopV1]` and maps only to typed `UnresolvedStop(NonRecursiveOutOfTarget)`
for the one-member out-of-target shape. It does not establish a winner,
eligibility, Legacy, selector, Recipe, PHI, Builder, MIR, Retry, fallback, or
production handoff.

The accepted docs-only design child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SELECTION-DISPOSITION-MATRIX0-D3-S1-D0`.
It must partition source-backed rows into
`ResolvedCandidate`, `LegacyPreserveExistingSchedule`, `UnresolvedStop`,
`NoStandaloneRow`, and `NotYetObserved`, then define the winner/disjointness
proof for natural recursive Both. The current recursive Both row remains
`UnresolvedStop(WinnerCorrectnessUnavailable)`; route labels, digests, and
legacy receipts are corroboration only. Its selected cfg(test)-only child is
`JOINIR-GENERIC-RESOLVED-CARRIER-SOURCE-MATRIX-V1ONLY-LOCAL0-D3-S1-S1`, now
closed with V0=false, V1=true, `CompleteNoRecursiveCarrier`,
`has_body_local=false`, actual frame flags, no recipe contract, and raw `[V1]`;
its typed result is `UnresolvedStop(V1OnlyNonRecursive)`. No selector or
neutral handoff is authorized by this reference entry.

The D3-S1-S2 candidate-stage source bridge is closed as cfg(test)-only
inspection evidence. It reuses the parsed natural-Both source for resolver
forest/BindingRef facts and fresh V0/V1 candidate plans. Release/Strict retain
raw `[V0,V1]`, direct `LowerSome`/`GenericComposer`, order-independent
snapshots, and distinct resolver owners. V0 lacks outer `j` while nested V0
retains it; V1 records outer `j` with `loop_carrier_j` and `loop_step_in_j`.
These are label-backed plan projections, not typed BindingRef provenance, and
the direct loop context does not lower the full post-loop return. Planner-
required remains `[V1]` and unresolved; the actual legacy trace is V0
terminal/no-debt. No winner, selector, issuer, Recipe, PHI, Builder, MIR,
retry, fallback, or runtime authority is added.

The next accepted design stop is
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-HANDOFF-DESIGN0-D3-S2-D0`,
recorded in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-typed-provenance-handoff-d3-s2-d0-design-2026-08-05.md`.
It is docs-only: resolver-owned `BindingRefV1` provenance, an AST-free neutral
facts snapshot, a logical loop-binding relation, and one non-Clone opaque
handoff must be specified before any issuer/selector implementation. Full
scalar Return projection, natural debt-to-different-winner evidence, and Home
semantics remain deferred; label/ValueId inference and synthetic debt remain
non-authoritative.

The first selected child is the cfg(test)-only
`JOINIR-GENERIC-RESOLVED-CARRIER-TYPED-PROVENANCE-OBSERVATION0-D3-S2-S0`,
recorded in
`docs/development/current/main/investigations/joinir-generic-resolved-carrier-provenance-observation-d3-s2-s0-task-2026-08-05.md`.
It observes resolver forest/frame and exact `BindingRefV1` role/ancestry only;
Generic snapshot/key issuer, seed/opaque input, selector, Builder/MIR, and
Return/Home/debt meaning remain unimplemented.

That observation child is closed as cfg(test)-only evidence: four focused
tests seal natural resolver forest/frame plus exact `BindingRefV1` role and
ancestry, and reject shadowing, foreign owner, forest-shape, and frame
mismatch. Production caller/import is zero and artifact is none. Generic
snapshot/key/seed ownership and winner/Return/PHI/Home/debt semantics remain
the D3-S2 design stop. A premise audit additionally found that the current
forest/frame coordinates omit a resolver owner/invocation brand, so equal
origin/site coordinates from two sessions can be mixed; this witness is not a
production capability until the cross-session brand audit is accepted.

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
