---
Status: S0 BoxShape complete; S0-D1 design_stop
Task: MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0-D1
Date: 2026-08-22
Priority: preserve one exact callable source-to-GenericLoop Recipe relation before ordinary effects
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-static-import-target-authority-d0-2026-08-22
NextCard: MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0-D1 (this rolling card)
---

# Callable physical-header eligibility D0

## Six-line brief

```text
Decision: keep the physical-header cohort sparse. A selected callable with an unannotated or explicit Void source result is ordinary-valid but not header-eligible; it must not fail the whole package as CompletionNotValue. An explicitly admitted scalar result still requires a value-returning Completion proof.
Source authority + canonical issuer: VerifiedFinalCallableProgramSourceV1 and VerifiedResolvedCallableSemanticBatchV1 own the source result contract and opaque declaration identity; verify_function_completion_v1 owns exact exits/value classification; issue_callable_completion_seed_cohort_v1 is the sole package-scoped co-seal before issue_callable_physical_header_from_seeds_v1.
Non-authority: batch_slot alone, catalog key/name, source ordinal, AST/body/MIR inference, ResultCatalog, Builder state, fixture comments, completion.returns_value() without its declared source contract, and empty/default header rows.
Fail-fast boundary: classify each selected source row at the package pre-Builder boundary, before install, effects, Dynamic/S6C physical work, or publication; invalid explicit scalar rows reject, while source-valid no-header rows become an explicit sparse absence.
Smallest next slice: `MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0`; adapt only the existing completion-seed disposition so explicit accepted scalar values remain header-eligible while valid unannotated/Void rows become ordinary no-header, then add focused mixed/negative evidence. Do not change Text ABI or Builder paths.
Non-claims: no result inference, Text handle/wire, S6C activation, Dynamic publication, Completion verifier redesign, Builder/MIR/CFG/SSA, fallback/retry, production switch, legacy retirement, backend, or performance work.
```

## Observed production boundary

After `MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0`, the unchanged merged
production source passes the former `IfRegion(ControlContractMismatch)` and
stops before Builder effects at:

```text
[mir/callable-semantic-package/issue]
PhysicalHeader(CompletionNotValue { batch_slot: 36 })
```

The merged source contains a source-valid void-returning operator method:

```text
static box CompareOperator {
  apply(op, a, b) {
    return
  }
}
```

The read-only worker audit tied the observed row to the source anchor for
`apps/lib/std/operators/compare.hako` through the same parser semantic loan.
The batch slot is only transport evidence, not an identity. No implementation
may make `batch_slot == 36` the join key.

The current package path is:

```text
VerifiedFinalCallableProgramSourceV1
  -> complete resolved callable batch
  -> selected source-backed map
  -> exact parameter-contract rows
  -> issue_callable_completion_seed_cohort_v1
  -> issue_callable_physical_header_from_seeds_v1
  -> package install
```

The current seed issuer verifies Completion for each selected cataloged row
with a complete parameter contract, then rejects any row whose Completion is
not value-returning. The existing sparse-header README and tests state a
narrower contract: unannotated siblings are valid ordinary rows and absent
from the physical-header cohort. The open question is how the same rule must
cover a source-valid Void row without weakening explicit scalar validation.

## Worker audit and accepted boundary

The read-only audit is accepted for this D0. It found that the current
`completion_seed.rs` already classifies `Unannotated | Void` as
`result = None`, while `physical_header.rs` already treats `result == None`
as a normal sparse-header omission. The later unconditional
`!completion.returns_value()` rejection is therefore the narrow defect.

The accepted one-way classification is:

```text
source result contract
+ verified Completion
  ├─ explicit accepted scalar + value
  │    -> physical header seed
  ├─ unannotated / Void + valid Completion
  │    -> ordinary no-header seed
  └─ invalid / unsupported
       -> existing typed reject
```

This does not open a Void ABI or make `CompareOperator.apply/3` a physical
candidate. It only prevents an ordinary source-valid row from poisoning the
package's sparse physical-header cohort.

## Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `VerifiedFinalCallableProgramSourceV1` | one parser invocation, callable membership, declaration identity, declared result spelling | physical header eligibility inferred from body/MIR |
| `VerifiedResolvedCallableSemanticBatchV1` | source-bound callable row and borrowed lowering input | batch-slot/name re-pairing or physical ABI |
| `verify_function_completion_v1` | exact exit sites, target, cleanup, and value/unit classification | deciding whether an unannotated row is selected for the sparse header |
| `issue_callable_completion_seed_cohort_v1` | one package-scoped co-seal of source row, parameter contract, identity, Completion, and optional scalar result | inventing a result or turning every selected row into a header |
| `issue_callable_physical_header_from_seeds_v1` | sparse physical-header rows for eligible source scalar results | rejecting ordinary no-header siblings or re-reading AST |
| package install/Port | move and callback-scoped loan of the already-issued cohort | a second Completion/header issuer |

The intended one-way relation is:

```text
source result contract + exact Completion
  -> HeaderEligible(scalar value)
  -> one physical-header row

source-valid Unannotated/Void
  -> OrdinaryNoHeader
  -> no physical-header row
```

An explicit scalar annotation is not equivalent to an unannotated value
return. The former can admit a header only when Completion proves a value;
the latter remains ordinary and has no physical-header result row.

## Candidate state table

| State | Source evidence | Package effect | Next |
| --- | --- | --- | --- |
| `HeaderEligible` | explicit supported scalar annotation + exact value Completion + complete formal contract | one seed and one sparse header row | install/loan |
| `OrdinaryNoHeader` | unannotated or explicit Void contract with source-valid Completion | package continues; no header row | ordinary selected route |
| `OrdinaryValueWithoutHeader` | unannotated source contract + exact value Completion | package continues; seed may retain Completion but result remains absent | ordinary route |
| `ExplicitResultCompletionInvalid` | explicit supported scalar annotation but Completion is missing/unit/foreign | typed package reject before effects | discard |
| `UnsupportedResultAnnotation` | explicit result annotation outside the accepted scalar cohort | typed package reject | discard |
| `FormalCoverageInvalid` | missing/duplicate/foreign parameter contract | typed package reject | discard |
| `SourceIdentityInvalid` | missing, foreign, duplicate, or ordinal-repaired identity | typed package reject | discard |

`OrdinaryNoHeader` is not a `default`, empty catalog, or fallback. It is an
explicit source-backed absence of physical-header admission. The cohort stays
always present; only its row lookup is optional, as the existing package
contract requires.

## Required D0 census

Before code, record the following using the existing source/batch owners:

```text
parser invocation witness for the observed row
opaque callable declaration identity
declared result contract: Unannotated / Void / Annotated(name)
Completion disposition and returns-value classification
parameter-contract identity and coverage
selected role: ordinary or AppMainStaticChild
whether a physical-header row is expected
whether S6C child admission observes the row
```

The census must include these source shapes:

```text
1. explicit : i64 + terminal value Return       -> HeaderEligible
2. unannotated + terminal value Return          -> OrdinaryValueWithoutHeader
3. unannotated + bare/void Return               -> OrdinaryNoHeader
4. explicit : void + unit completion            -> OrdinaryNoHeader
5. explicit unsupported result                  -> UnsupportedResultAnnotation
6. explicit : i64 + unit/missing value          -> ExplicitResultCompletionInvalid
7. foreign/duplicate identity                   -> SourceIdentityInvalid
```

The production merged probe must identify the observed Void row through the
same parser identity relation. A manually counted ordinal, method name, or
catalog key is only diagnostic evidence.

## Fail-fast boundary and no-effects law

All row classification happens before:

```text
CompilationContext install
Dynamic admission effects
S6C child/physical ingress
Builder function/session work
module collector or publication
```

The accepted sequence is:

```text
selected source row
  -> exact identity/owner/parameter coverage
  -> verify_function_completion_v1
  -> classify source result contract
  -> HeaderEligible or OrdinaryNoHeader
  -> co-seal package cohort
```

There is no transition:

```text
CompletionNotValue -> infer result from body/MIR
CompletionNotValue -> empty/default header
CompletionNotValue -> legacy retry/fallback
```

If a package-level invariant is invalid, the unpublished compilation session
remains the only discard authority. A source-valid ordinary Void row must not
poison unrelated eligible rows in the sparse cohort.

## Focused acceptance

Positive:

```text
explicit : i64 value row still lends one header
unannotated value row remains ordinary and has no header
unannotated Void row remains ordinary and has no header
explicit : void row remains ordinary and has no header
mixed package keeps eligible scalar rows when Void siblings are present
production merged probe passes the observed Void row and stops at the next
  already-owned source/semantic boundary
```

Negative:

```text
explicit unsupported result remains a typed PhysicalHeader error
explicit : i64 with unit/missing value remains a Completion error
foreign parser identity cannot be repaired by batch slot/name/order
duplicate parameter contract remains a typed coverage error
missing formal coverage remains a typed coverage error
no Builder/session/effect occurs on a rejected explicit row
```

Structural guards:

```text
physical-header issuer does not infer result from AST/body/MIR/ResultCatalog
CompletionNotValue is not the ordinary no-header terminal after D0
batch_slot is not exposed as a public join key
empty/default header construction = 0
fallback/retry = 0
new Text/ABI/Builder/physical receipt = 0
source files remain <760 lines; 800 is hard stop
```

## Ordered task queue

```text
MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-D0
  source/identity/role census; freeze the finite state and no-effects law

MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0
  only after D0 acceptance: adapt the existing seed disposition so
  source-valid Unannotated/Void rows are sparse absence, preserve explicit
  scalar fail-fast, add focused mixed/negative tests, and update one reusable
  guard; no Builder or ABI change

MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-R0
  rerun the merged production probe, prove the observed row identity and
  next blocker, then close the package-level evidence before any activation
```

## P0 execution brief — MIR-CALLABLE-PHYSICAL-HEADER-ELIGIBILITY-P0

```text
change:
  reuse issue_callable_completion_seed_cohort_v1 and preserve its existing
  source result/Completion/parameter identity co-seal

accept:
  explicit accepted scalar + value -> existing header seed
  valid Unannotated/Void -> ordinary seed with no physical-header result
  unsupported or invalid explicit rows -> existing typed reject

files:
  completion_seed.rs, focused physical-header/package tests, one reusable guard

forbidden:
  body/MIR/ResultCatalog inference, new receipt, Builder/ABI change,
  fallback/retry, source reparse, ordinal/name pairing
```

P0 closes only after the focused mixed/negative tests, cargo check, source-size
guard, and the merged production probe all pass or the next blocker is
recorded. The next probe result is not a license to widen this row.

## P0 closeout and merged production probe

The bounded P0 is implementation-complete and pushed as `9bdd557b6c`.
Pointer selection was recorded in `0e2ef08ca4`. The change is deliberately
small: valid `Unannotated | Void` completion rows remain ordinary seed rows
with no physical-header result, while explicit scalar rows and their typed
rejects keep the existing authority and error boundary.

Evidence:

```text
physical-header focused suite: 5 passed, 0 failed
normal-callable semantic-package suite: 39 passed, 0 failed
cargo check --profile quick --lib: passed
cargo build --bin hakorune: passed
rustfmt / pointer / source-size checks: passed
```

The reusable complete-batch guard reaches its pre-existing selected-mapping
identity-repair vocabulary failure after all new P0 checks pass. The parent
baseline reproduces the same failure; it is recorded as baseline debt, not
silently weakened or counted as P0 evidence.

The rebuilt merged production probe no longer stops at:

```text
PhysicalHeader(CompletionNotValue { batch_slot: 36 })
```

It now stops at:

```text
PhysicalHeader(Completion {
  batch_slot: 33,
  issue: TerminalSiteIsNotReturn(
    SourceStmtSiteV1(SourceNodeSiteV1([Body(6), LoopBody(1), IfElse(0)]))
  )
})
```

The read-only parser/source-loan audit identifies this exact source-bound row
as `ParserCommonUtilsBox.trim/1`. The path is a nested `loop -> if -> else {
break }`, not a function Return. `batch_slot` remains diagnostic transport
evidence only and is not a pairing key.

## Next design stop — MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-D0

The next blocker has a bounded source-semantic fix. The resolver already emits
one atomic `ResolvedExitRecordV1` containing source region, origin, and transfer;
the existing control verifier already rejects malformed origin/transfer pairs.
The completion verifier is the only owner that currently feeds all resolved
exits into the function-Return validator.

```text
Decision: project a transient Completion Return-candidate view that excludes
  only exact ExplicitBreak + Break and ExplicitContinue + Continue pairs;
  keep the resolver exit inventory unchanged and leave malformed pairs on the
  typed-reject path.
Source authority + canonical issuer: the existing parser-owned final callable
  source loan and ResolvedFunctionLoweringInputV1 supply the same invocation,
  source, and resolver product; verify_function_completion_v1 remains the sole
  Completion issuer. No second resolver or source scan.
Non-authority: batch_slot, callable name/arity/ordinal, SourceStmtSite alone,
  AST/body/MIR/Builder/CFG/SSA/physical-header inference, returns_value(),
  fallback, compatibility, and fixture shape.
Fail-fast boundary: immediately after resolved_exits() is observed and before
  exit cardinality, terminal, value, or Completion seed validation; control
  exits are excluded only from this temporary view and remain resolver facts.
Smallest next slice: MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-P0;
  add the typed projection at the existing Completion verifier seam, preserve
  existing Return checks, and add nested loop/if plus malformed-pair evidence.
Non-claims: no parser/resolver inventory change, Break/Continue lowering,
  body/MIR inference, Return-value inference, new semantic receipt, Builder,
  ABI, Dynamic/S6C, physical publication, fallback, retry, or performance.
```

Finite state for the next slice:

| State | Meaning | Next |
| --- | --- | --- |
| `FunctionReturnCandidate` | exact `ExplicitReturn + Return` record | existing Return validation |
| `LoopControlExcluded` | exact `ExplicitBreak + Break` or `ExplicitContinue + Continue` | not a Completion Return candidate |
| `ImplicitCompletion` | no Return candidates remain after projection | existing unannotated/Void seed path |
| `ExplicitReturnSet` | one or more Return candidates remain | existing terminal/value/contract validation |
| `CompletionIntegrityReject` | origin/transfer, target, region, or source relation is malformed | typed reject before effects |

Ordered tasks:

```text
MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-D0
  accepted source/control authority and no-effects boundary (this section)

MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-P0
  implement only the transient projection at verify_function_completion_v1;
  preserve the resolver inventory and existing Return validator

MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-R0
  rerun the merged production probe, then close exact nested-control and
  regression evidence before any physical/publication activation
```

Remain `NoSafeSlice` if the existing origin/transfer pair cannot distinguish
loop control without hiding malformed records, if a second resolver/source
walk is required, or if the fix expands into body/MIR inference, a new
Completion issuer, Builder/ABI changes, or fallback.

## P0 closeout — MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-P0

The bounded projection is implementation-complete and pushed as
`bc052cb60d`. `verify_function_completion_v1` now removes only exact paired
`ExplicitBreak + Break` and `ExplicitContinue + Continue` records from its
temporary Return-candidate view. The resolver inventory, source loan, and all
Builder/publication routes are unchanged.

Evidence:

```text
function-control focused suite: 17 passed, 0 failed
resolved-control-flow suite: 33 passed, 0 failed
normal-callable semantic-package suite: 39 passed, 0 failed
cargo check --profile quick --lib: passed
cargo build --profile quick --bin hakorune: passed
targeted rustfmt / current-state pointer guard / diff check: passed
source size: function_control.rs 626 lines; function_control_tests.rs 418 lines
```

The reusable complete-batch guard still reaches its parent-baseline
identity-repair vocabulary failure after the new checks pass. The same failure
is reproduced on the parent baseline and remains recorded as baseline debt;
the guard was not weakened.

The rebuilt merged production probe no longer stops at the previous
`TerminalSiteIsNotReturn` for the nested loop-control site. It now stops at the
next existing source/semantic boundary:

```text
[freeze:contract][callable-loop-handoff/incomplete-binding-coverage]
```

This is a deliberate P0 stop. No Builder effect, ABI change, physical
publication, fallback, or retry was added. The exact missing binding/role
coverage must be identified before changing the existing handoff contract.

## Next design stop — MIR-CALLABLE-LOOP-HANDOFF-BINDING-COVERAGE-D0

```text
Decision: keep VerifiedCallableSemanticLoopBindingScheduleV1 strict and
  classify the exact source-backed coverage rows that reject the merged
  ParserCommonUtilsBox.esc_json/1 loop. The static census finds `out` and
  `handled` rebinds in the loop body without a LoopCondition read; the current
  issuer treats every BodyRebind as a Carrier and therefore rejects them as
  incomplete. Do not turn them into a default/read-only row without a cohort
  decision.
Source authority + canonical issuer: the parser-owned callable source loan,
  resolver-issued BindingRef/source-site inventory, and the existing
  CallableLoopSourceProjectionV1 -> VerifiedCallableSemanticLoopBindingScheduleV1
  issuer; no AST/body/MIR/name reconstruction and no second resolver.
Non-authority: the freeze string alone, batch slot, callable name/ordinal,
  loop shape guessed from the fixture, Builder current state, ValueId/SSA,
  physical header, fallback, compatibility, and a missing row synthesized by
  default/empty/Option merging.
Fail-fast boundary: after the existing source projection has collected its
  parser/resolver rows and before any Dynamic loop ingress, Builder/session,
  or physical effect; retain typed incomplete/invalid distinction.
Smallest next slice: freeze the one-carrier first-cohort boundary for
  body-only rebinds, record `esc_json/1` rows (`out`, `handled`, and condition
  carrier `i`) with their source roles, and choose one explicit state:
  `OutsideFirstCohort` or a source-backed multi-carrier extension. No physical
  handoff is opened until that choice has a named consumer.
Non-claims: no handoff implementation, nested-loop generalization, AST/MIR
  inference, new receipt, Dynamic/Builder/ABI/physical/publication change,
  fallback, retry, performance work, or production activation.
```

Finite state for the next design stop:

| State | Meaning | Next |
| --- | --- | --- |
| `CoverageObserved` | exact parser/resolver rows and binding sites are recorded | classify source ownership |
| `CoverageComplete` | existing roles already prove the loop contract | select the smallest implementation cell |
| `CoverageMissing` | required source row is absent from the authority | repair the owning parser/resolver source contract only |
| `CoverageMisclassified` | row exists but projection role/site rule rejects it | bounded projection P0, with negative evidence |
| `OutsideFirstCohort` | source shape is complete but not admitted | explicit outside lane; no guessed acceptance |
| `CoverageInvalid` | foreign/duplicate/contradictory relation | typed reject before effects |

## D0 census result — body-only rebind is not a missing read

The previous completion error was the nested `trim/1` `break` at
`[Body(6), LoopBody(1), IfElse(0)]`; exact paired loop-control projection
removed that false Return candidate. The current handoff error is a different
contract boundary. `ParserCommonUtilsBox.esc_json/1` contains:

```text
loop(i < n) {
  local ch = ...
  local handled = 0
  out = out + ...; handled = 1   // repeated in body branches
  i = i + 1
}
```

The source/resolver inventory can therefore supply:

```text
i       = ConditionRead + BodyRead + BodyRebind   -> current carrier
out     = BodyRead + BodyRebind                   -> body-only accumulator
handled = BodyRead + BodyRebind                   -> body-only local state
ch      = BodyRead                                 -> iteration local
n       = ConditionRead                            -> read-only operand
```

`VerifiedCallableSemanticLoopBindingScheduleV1::seal` currently classifies
every row with `BodyRebind` as `Carrier` and requires ConditionRead + BodyRead
+ BodyRebind. Thus `out` or `handled` reaches the existing typed
`incomplete-binding-coverage` stop. This is not evidence that a read is absent
from the parser; it is evidence that the current one-carrier cohort has no
state for body-only accumulators.

Decision boundary:

```text
one condition carrier + read-only operands + iteration locals
  -> current cohort

body-only accumulator/rebind
  -> explicit OutsideFirstCohort until a source-backed multi-carrier owner
     and named physical consumer are designed
```

Do not reinterpret `out`/`handled` as `ReadOnlyOperand`, do not drop their
rebinds, and do not infer a second carrier from ValueId/AST/body state.

Source-size stop:

```text
normal_callable_loop_handoff.rs = 749 lines
760 = split-design trigger; 800 = hard stop
next P0 must split the source-role/cohort owner before adding semantic state
```

## Decision closed — MIR-CALLABLE-LOOP-HANDOFF-OUTSIDE-DISPOSITION-D0

The first callable Loop cohort stays exactly one condition carrier. A
source-backed binding with a body rebind but no `LoopCondition` read is not
silently made a carrier, read-only operand, or missing row. The projection
classifies the complete, validated row set as:

```text
Ready(schedule)
  -> consume the callable pre-effect receipt
  -> existing selected Dynamic/JoinIR consumer

Outside(BodyOnlyRebind { bindings, sites })
  -> explicit ordinary JoinIR lane, exactly once
  -> no callable pre-effect receipt and no canonical retry
```

`Outside` is selected by the same parser/resolver source projection before
Builder effects. It is not an error produced after `Ready`, and it is not a
fallback from a rejected canonical handoff. The ordinary lane remains the
existing named `lower_loop_or_freeze_v1` owner; this slice does not make that
lane a callable semantic authority or widen its facts.

The new disposition is transport/classification state, not a second semantic
`Verified*` or physical `Prepared*` product. `Incomplete` and
`IntegrityInvalid` remain terminal typed rejects. A body-only row must carry
its source `BindingRef`/site evidence into the Outside reason; default, empty,
name, ordinal, AST-pointer, ValueId, or MIR reconstruction is forbidden.

## Accepted task order

```text
MIR-CALLABLE-LOOP-HANDOFF-OUTSIDE-DISPOSITION-S0
  behavior-neutral split of the 749-line handoff owner and its tests;
  preserve all existing Ready/error behavior

MIR-CALLABLE-LOOP-HANDOFF-OUTSIDE-DISPOSITION-P0
  issue Ready/Outside from the existing source projection, transport the
  disposition through the raw child entry, and branch Outside to the named
  ordinary JoinIR lane before any callable receipt consumption

MIR-CALLABLE-LOOP-HANDOFF-OUTSIDE-DISPOSITION-R0
  prove esc_json/1 rows, one existing Ready row, typed invalid negatives,
  Outside-before-effect counts, no canonical retry, and the next production
  boundary; no multi-carrier expansion yet
```

P0 must stop if the ordinary lane cannot be entered without a second parser or
resolver observation, or if its existing retry/debt behavior becomes a new
escape from a canonical `Ready` failure. In that case the disposition remains
an explicit terminal Outside and multi-carrier design reopens separately.

## S0 evidence and P0 boundary

S0 is complete in `9074535308`. The 749-line handoff owner was split into a
419-line production owner and a separate test module without changing the
existing one-carrier `Ready`/error behavior. The focused handoff suite remains
green with four original tests.

P0 now uses the same source projection to issue a move-only
`Ready(schedule)` or `Outside(reason)` disposition. The new Outside test proves
that a body-only rebind carries its exact `BindingRef` and source sites. The
raw child entry consumes a Ready schedule before the existing route, while an
Outside reason enters the named ordinary JoinIR lane without consuming a
callable pre-effect receipt. No AST, ValueId, fallback, or second observation
is introduced.

## P0 closeout — next blocker is ordinary-lane consumption

P0 is complete in the working tree. The source projection now issues
`Ready(schedule)` or `Outside(reason)`, the raw child entry has one explicit
consumer for each disposition, and the reusable structural guard is
`tools/checks/rust_mirbuilder_callable_loop_outside_disposition_p0_guard.sh`.

Evidence:

```text
normal_callable_loop_handoff focused suite: 6 passed, 0 failed
raw_loop_child_entry focused suite: 7 passed, 0 failed
cargo check --profile quick --lib: passed
cargo build --profile quick --bin hakorune: passed
outside disposition guard / pointer guard / diff check: passed
source-size: normal_callable_loop_handoff.rs 531; recursive_child_lowering.rs 730
```

The rebuilt merged production probe now passes the old
`callable-loop-handoff/incomplete-binding-coverage` boundary and stops at the
existing ordinary-lane terminal:

```text
[freeze:contract][callable-semantic-lowering/incomplete-consumption]
owner=FunctionOwnerIdV1 { compilation: 1, slot: 11 }
entry=true locals=4/4 variables=5/21 assignments=0/3
```

This proves that `Outside` is selected before the callable pre-effect receipt
and reaches the named ordinary JoinIR owner. It does not prove that the
ordinary lane can consume this callable resolver cohort. The ordinary lane's
existing `CallableSemanticLoweringState` still reports missing source rows at
its finish boundary. No retry, second parser/resolver observation, or default
row is authorized to make that error disappear.

## Next design stop — MIR-CALLABLE-LOOP-OUTSIDE-ORDINARY-CONSUMPTION-D0

```text
Decision:
  keep body-only rebinds outside the one-carrier Ready cohort; decide whether
  Outside is a terminal typed outcome or whether one existing ordinary-lane
  source-consumption bridge can consume the same resolver rows.
Source authority + canonical issuer:
  the parser/resolver callable ledger and CallableLoopSourceProjectionV1 remain
  the sole source-role authority; no ordinary-lane reconstruction is allowed.
Non-authority:
  ordinary JoinIR routing, CallableSemanticLoweringState finish diagnostics,
  AST/name/ordinal, Builder maps, ValueId, and a retry outcome.
Fail-fast boundary:
  before callable pre-effect consumption; an incomplete ordinary consumption
  remains terminal until this D0 names one consumer and one ownership relation.
Smallest next slice:
  census the exact missing rows for esc_json/1 and the ordinary route, then
  choose terminal Outside or a single source-backed consumption handoff.
Non-claims:
  no multi-carrier Ready, no second resolver/source walk, no AST reconstruction,
  no physical/publication change, no fallback, and no performance work.
```

Required D0 evidence:

```text
exact failing loop owner and source site
all BindingRef rows with role and source-site relation
one explanation for incomplete-binding-coverage
one positive existing handoff case preserved
one negative missing/foreign/duplicate case preserved
no Builder/session/effect count on rejection
```

Remain `NoSafeSlice` if the failing row cannot be tied to the parser/resolver
source authority, if accepting it requires body/MIR inference, or if the only
way to pass is default/empty synthesis, a second source walk, fallback, or a
new competing handoff issuer.

## D0 decision closure — MIR-CALLABLE-LOOP-OUTSIDE-TERMINAL-P0

The ordinary JoinIR route is not a consumer of the callable source ledger.
The local audit found the exact ownership boundary:

```text
with_selected_source_scope
  -> one Rc<RefCell<CallableSemanticLoweringState>>
  -> RawInvocationChildPortV1::callable_ledger

RawLoopChildEntryPortV1::lower_loop
  -> lower_loop_or_freeze_v1(&mut MirBuilder, condition, body)
  -> no callable-ledger observation

outer CallableSemanticLoweringState::finish
  -> variables 5/21, assignments 0/3
```

Therefore `Outside -> ordinary JoinIR` is not a completed handoff. It can
write partial MIR and only fail at the outer semantic finish. It is removed as
the current Outside consumer. The accepted state is:

```text
Ready(schedule)
  -> consume callable pre-effect receipt
  -> existing selected consumer

Outside(body-only rebind)
  -> typed terminal before ordinary JoinIR or callable pre-effect effects

Incomplete / IntegrityInvalid
  -> typed reject before effects
```

This is a bounded fail-fast correction, not a claim that `esc_json/1` is now
supported. The source projection remains the only issuer of the Outside
evidence. `CallableSemanticLoweringState::finish` remains strict and is not
weakened, and no missing row is marked consumed merely to make the probe pass.

### Authority and rejected alternatives

| Owner | Owns | Does not own |
| --- | --- | --- |
| parser/resolver callable ledger + `CallableLoopSourceProjectionV1` | binding/site rows and `Ready`/`Outside` classification | physical values, JoinIR route, fallback |
| `RawLoopChildEntry` terminal | pre-effect Outside stop | source reclassification or ordinary lowering |
| `CallableSemanticLoweringState` | exact row consumption and finish completeness | deciding that an unconsumed row is harmless |
| ordinary JoinIR | ordinary AST/Builder loop lowering | callable source consumption until a separate bridge is accepted |

Rejected as standalone fixes:

```text
Dynamic admission -> Ordinary
  insufficient: the ordinary Loop child entry still bypasses the ledger.

ignore or subtract missing rows in finish
  forbidden: it creates a false source-consumption receipt.

AST/name/ValueId post-walk to mark rows consumed
  forbidden: second observation and a competing authority.

ordinary JoinIR retry after terminal
  forbidden: fallback and partial-effect escape.
```

### Finite terminal table

| State | Evidence | Effect | Next |
| --- | --- | --- | --- |
| `Ready` | one-carrier source schedule complete | existing consumer may proceed | consume/finish |
| `Outside` | complete body-only rebind evidence | no JoinIR/Builder effect from this handoff | typed terminal |
| `Incomplete` | required source row absent | no effect | typed reject |
| `IntegrityInvalid` | foreign/duplicate/contradictory row | no effect | typed reject |
| `ReachableBoxDeclaration` | nested child ownership boundary | no effect | existing terminal |

### P0 boundary

`MIR-CALLABLE-LOOP-OUTSIDE-TERMINAL-P0` changes only the current Outside
consumer:

```text
lower_outside_callable_loop_v1(reason)
  -> stable typed terminal carrying the source-backed Outside reason
  -> no call to lower_loop_or_freeze_v1
```

Acceptance:

```text
Outside returns before the first ordinary JoinIR call
Outside leaves instruction count and callable-ledger consumption unchanged
Ready behavior remains unchanged
Incomplete/foreign/duplicate cases remain typed rejects
merged esc_json/1 probe stops at outside-first-cohort before effects
no retry/fallback/default/second resolver observation
source files remain below 760 lines (800 hard stop)
```

### Future bridge design stop

The real ordinary-consumption work is parked as
`MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-D0`. It may open only with a named
ledger-aware JoinIR/port consumer that consumes the same resolver rows during
the existing lowering traversal. It must not be implemented by a post-walk,
name join, ValueId inference, or by weakening `finish`. Its design must state
the source authority, exact consumer entry, nested-loop ownership, negative
matrix, and no-effects boundary before any new receipt or production edge.

## Feedback audit and task boundary — 2026-08-22

The latest audit is accepted as a boundary correction, not as permission to
open a second production route.

### Decision

Keep the current `Outside(body-only rebind)` terminal. The ordinary JoinIR
route is not a callable-ledger consumer, so `Outside -> ordinary JoinIR` is
not a completed handoff. `CallableSemanticLoweringState::finish` remains
strict; the late incomplete-consumption error must not be hidden by subtracting
rows, adding defaults, or weakening the finish check.

The eventual ordinary consumer is named here so the next design does not
drift:

```text
CallableOrdinaryLoopJoinIrConsumerV1
  = one scoped source-backed consumer at RawLoopChildEntryPortV1
  -> exact callable source port during the existing Loop normalization walk
  -> existing sole physical JoinIR/PlanLowerer only after source consumption
```

This is a consumer, not a new source issuer. It is not implemented by passing
the current diagnostic `CallableLoopOutsideReasonV1` into
`lower_loop_or_freeze_v1`.

### Authority boundary

```text
CallableSemanticLoweringState
  + CallableLoopSourceProjectionV1
  -> grouped source-role coverage and Ready/Outside classification
  -> CallableOrdinaryLoopJoinIrConsumerV1
  -> existing JoinIR/PlanLowerer physical route
```

The source projection remains the sole issuer of binding/site roles. When the
ordinary bridge is eventually opened, its input must preserve each
`binding + class + (site, role)` relation as grouped coverage rows. The current
Outside diagnostic, which stores separate `bindings[]` and `sites[]`, is enough
for a terminal error but is not enough to authorize ordinary consumption. Do
not widen that diagnostic or issue a new receipt until the named consumer has
an exact one-shot owner.

The existing `RawLoopPlanExpressionPortV1` and
`LocatedLoopPlanExpressionPortV1` are structural source ports only; neither
consumes `CallableSemanticLoweringState`. A wrapper that merely forwards them
to the generic Builder route is therefore rejected. The bridge must use a
dedicated source-aware normalizer/port, while the existing PlanLowerer remains
the sole physical owner.

### No-effects and nested-loop boundary

The bridge must validate its complete source relation, exact parent/condition/
body ownership, and first-cohort shape before any of these actions:

```text
callable pre-effect receipt consumption
ordinary JoinIR routing
CorePlan physical lowering
Builder instruction/block/ledger mutation
```

The first implementation cohort is one non-nested executable Loop. An
executable nested Loop encountered while preparing that cohort is a typed
`NestedLoopUnsupported` terminal before effects; it is not delegated to the
generic route. A later child-context/reborrow design is a separate slice.

Any missing, foreign, duplicate, or unconsumable source row is terminal and
discards the unpublished outer session. There is no retry, fallback,
post-walk, AST/name/ordinal join, `ValueId` inference, or finish relaxation.

### Ordered tasks

```text
1. MIR-CALLABLE-LOOP-OUTSIDE-ORDINARY-CONSUMPTION-D0
   close the boundary with the terminal Outside decision above; no code edge.

2. MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0
   design the exact source-aware port seam and grouped coverage input. Census
   the current normalizer/JoinIR call graph and nested-loop ownership. This is
   a BoxShape/port design cell; it does not claim ordinary support or add a
   production edge.

3. MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-I0
   one named consumer, one non-nested body-only-rebind cohort, same traversal,
   pre-effect complete consumption, positive/negative/no-effect evidence.

4. MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0
   only after I0: one named production caller, old bypass caller-zero, and no
   fallback/retry.
```

Before S0/I0, `recursive_child_lowering.rs` is a behavior-neutral BoxShape
split candidate because it is already near the 760-line design threshold.
That split must not acquire ordinary-consumption semantics.

### Cross-card hardening classification

The selected Dynamic Compare feedback was independently verified and is
already owned by
`mirbuilder-loop-compare-hardening-d0-2026-08-22.md`; no duplicate task is
created here.

```text
MIR-DYNAMIC-CURSOR-EOF-FAILFAST-P0       immediate typed-reject fix
MIR-DYNAMIC-PHYSICAL-PRECLAIM-D0         claims before effects
MIR-LOOP-COMPARE-PREPARE-RESERVE-I0      writer prepare -> last reserve -> commit
MIR-LOOP-BODY-BRIDGE-RETURN-AFFINITY-P0  OuterReturn relation + move-only cleanup
MIR-LOOP-GENERIC-COMPARE-RETIRE-D0       caller-zero generic debt, parked
```

The EOF indexing risk is a real correctness bug. The post-effect claims,
reserve-before-writer-prepare, `assert_eq!` pairing, destination/Bool
affinity, and missing OuterReturn physical-value relation are real hardening
items. They remain downstream of the current callable Outside boundary and
must not be mixed into the ordinary bridge. `compare_i64_writer.rs` is not
test-only in build scope: selected Dynamic imports the production writer;
only its generic test adapter/name cleanup is parked.

The projection currently creates and drops a Ready schedule while classifying
an Outside row. This is a valid structural cleanup candidate for the ordinary
bridge S0 (`validate_first_cohort_rows` before Ready issuance), but it is not a
reason to issue a new semantic product during the current terminal slice.

### Current stop

The D0 decision is accepted and the behavior-neutral S0 BoxShape is complete.
The active mode is now `design_stop` for the I0 source-aware consumer below.
This does not open ordinary consumption.

### S0 bounded BoxShape cell — MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0

S0 moves the existing `RawLoopChildEntryPortV1` trait and its legacy and
invocation implementations out of `recursive_child_lowering.rs` into one
dedicated child-entry port module. The move must preserve every caller and
every existing route; it is a physical responsibility split, not a new
semantic layer.

```text
recursive_child_lowering.rs
  -> existing recursive body/statement/expression and source-aware port owner

raw_loop_child_port.rs
  -> RawLoopChildEntryPortV1
  -> RawLegacyChildLoweringPortV1::lower_loop
  -> RawInvocationChildPortV1::lower_loop
```

Acceptance:

```text
old import paths remain valid through a narrow re-export
caller graph is unchanged
ordinary JoinIR consumer count remains zero
callable receipt/ledger product count remains unchanged
no new source walk, AST reconstruction, ValueId/name join, or fallback
focused raw-loop/recursive-child tests and quick check stay green
recursive_child_lowering.rs < 760 lines; touched production source < 800
```

Non-claims:

```text
no CallableOrdinaryLoopJoinIrConsumerV1 implementation
no Outside diagnostic widening or new semantic receipt
no ordinary body-only-rebind support
no nested-loop support
no production switch, publication, Compare hardening, performance, or main integration
```

### S0 evidence and red classification

The split is behavior-neutral and preserves the old import path through the
`recursive_child_lowering` re-export. Evidence:

```text
raw_loop_child_entry focused tests: 7 passed
cargo check --profile quick --lib: passed
rustfmt --check --edition 2021 on changed Rust files: passed
current-state pointer guard: passed
callable-loop Outside P0 guard: passed
loop pre-cutover authority guard: passed
git diff --check: passed
recursive_child_lowering.rs: 679 lines
raw_loop_child_port.rs: 68 lines
```

The broader `recursive_child_lowering` filter has 25 passed and 8 failures,
all with the existing
`[freeze:contract][raw-invocation/missing-expression-source-receipt]` error.
The same 25/8 result reproduces on the parent commit `49bcde52a6`, so it is
classified as baseline debt and is not attributed to S0.

S0 adds no callable ordinary consumer, receipt, source walk, Builder effect,
fallback, or production edge. I0 may add a source-aware ordinary consumer
only after its exact grouped coverage relation and same-traversal consumption
boundary are fixed.

## P0 execution brief — MIR-CALLABLE-COMPLETION-LOOP-CONTROL-PROJECTION-P0

```text
change:
  add one private projection at the existing verify_function_completion_v1
  seam; keep ResolvedFunctionProduct.resolved_exits() unchanged

accept:
  exact ExplicitBreak + Break and ExplicitContinue + Continue are absent from
  the temporary Completion Return candidates
  a function containing only loop control reaches existing implicit unit
  mixed loop control plus a root Return reaches existing explicit Return logic
  malformed origin/transfer pairs are not filtered into success

files:
  function_control.rs and its focused function-control tests only

forbidden:
  resolver/source rewalk, AST/body/MIR inference, new semantic receipt,
  Completion issuer, Builder/ABI/physical change, fallback/retry, or
  batch-slot/name/ordinal pairing
```

## NoSafeSlice conditions

Remain in `design_stop` if any proposal requires:

```text
body/MIR/ResultCatalog inference for an absent result;
batch slot/name/ordinal as the source identity;
re-running the resolver or reparsing the source;
making every selected Cataloged row a physical-header candidate;
dropping Completion ownership or creating a second Completion verifier;
using empty/default/Option merging to hide partial header state;
changing Text handle/ABI, S6C, Dynamic physical, Builder, or publication
  ownership to make this row pass;
falling back or retrying after package/header rejection;
accepting the production probe without exact source-bound row evidence.
```

## P0 closeout — MIR-CALLABLE-LOOP-OUTSIDE-TERMINAL-P0

The accepted terminal correction is complete. `Outside` no longer enters
ordinary JoinIR. `CallableLoopOutsideReasonV1::into_terminal_error` is the
single stable terminal formatter, and `RawLoopChildEntry` returns it before
the ordinary route or the callable pre-effect receipt can run.

Evidence:

```text
normal_callable_loop_handoff focused tests: 6 passed
raw_loop_child_entry focused tests: 7 passed
cargo check --profile quick --lib: passed
cargo build --profile quick --bin hakorune: passed
rustfmt --check: passed
P0 guard: passed
current-state pointer guard: passed
git diff --check: passed
source sizes: handoff 540, raw entry 491, recursive child 730 lines
```

The rebuilt merged `esc_json/1` probe now stops at:

```text
[freeze:contract][callable-loop-handoff/outside-first-cohort]
loop_site=[Body(5)] bindings=1 sites=8
```

It does not report the former late
`callable-semantic-lowering/incomplete-consumption` failure. This proves the
pre-effect terminal boundary, not support for the body-only rebind cohort.
Ready behavior, strict incomplete/foreign/duplicate rejection, and the
strict `CallableSemanticLoweringState::finish` contract remain unchanged.

Next design stop:

```text
MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-D0
```

This future slice must name a ledger-aware JoinIR/port consumer before adding
any receipt or production edge. Performance work, Builder cleanup, fallback,
publication, and main integration remain outside this closeout.

## S0-D1 worker audit — exact source-to-Recipe bridge

Two independent read-only audits and the local call-graph census reject a
direct jump from the completed S0 split to the former I0. The entry seam is
now known, but the relation that the consumer would consume is not yet present.

### Six-line brief

```text
Decision: keep body-only-rebind Outside terminal. Before any ordinary consumer, co-seal one exact callable source-to-GenericLoop Facts/Recipe relation and one front-selected terminal route; do not feed the current diagnostic Outside arrays to generic lowering.
Source authority + canonical issuer: CallableSemanticLoweringState and CallableLoopSourceProjectionV1 own grouped binding/site/role rows; RawInvocationSourceContextV1 owns exact parent/condition/body location. A future private CallableOrdinaryLoopSourceRecipeIssuerV1 may co-seal those existing facts at PreparedLocatedRawLoopChildEntryV1, but it may not issue new binding roles or route policy.
Non-authority: CallableLoopOutsideReasonV1 bindings[]/sites[], cloned GenericLoopV1Facts alone, AST/name/ordinal/ValueId, Builder variable_map, LoopRouteContext, RawLoopPlanExpressionPortV1, LocatedLoopPlanExpressionPortV1, and the retry-capable legacy route continuation.
Fail-fast boundary: exact source ownership, grouped coverage, source-to-Facts correspondence, non-nested first cohort, and one terminal selected route must all close before CorePlan composition, callable row consumption, Builder mutation, or PlanLowerer; every later reject discards the unpublished outer session and never advances the route schedule.
Smallest next slice: MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0-D1; fix the source-located Facts/Recipe and terminal-route contract, and census whether the first source-aware cohort is RecipeOnly or ExitAllowed. This is design/census only.
Non-claims: no new semantic receipt, ordinary support, body-only-rebind admission, nested child support, GenericLoop production switch, finish relaxation, fallback/retry, publication, Compare hardening, performance work, or main integration.
```

### Verified missing relations

The exact source entry already exists:

```text
RawInvocationChildPortV1::lower_loop
  -> RawLoopChildEntryPortV1
  -> PreparedLocatedRawLoopChildEntryV1
```

`PreparedLocatedRawLoopChildEntryV1` co-seals the parent, condition, and body
source contexts before ordinary routing. This is the sole future consumer
entry. Moving the consumer below `lower_loop_or_freeze_v1` is too late because
that API transports only `MirBuilder + AST`.

Three relations are still missing:

1. `CallableLoopOutsideReasonV1` stores `bindings[]` and `sites[]` separately.
   It is a terminal diagnostic, not a pairing authority. The existing
   `CallableLoopBindingCoverageRowV1` is the reusable grouped source shape.
2. `try_extract_generic_loop_v1` clones condition/body syntax into
   `GenericLoopV1Facts`, but those Facts retain no `SourceNodeSiteV1` relation.
   Rejoining the cloned Recipe body to source rows by name, ordinal, or AST
   shape would create a second authority.
3. the selected GenericLoop registry path is explicitly classified as
   `PostEffectRetryDebt`; `RouteExecutionWitnessV1::execute_selected_in_order`
   advances after that outcome. A one-shot source consumer cannot enter that
   continuation because partial consumption could be followed by another
   route attempt.

The existing expression ports solve only structural child access. They do not
call `CallableSemanticLoweringState::read_variable` or `rebind`, so wrapping
one of them does not complete source consumption. D1 keeps structural syntax
transport and semantic binding consumption as separate ports.

### Authority map

| Owner | Owns | Must not own |
| --- | --- | --- |
| `CallableSemanticLoweringState` | exact callable local/read/assignment rows, current `BindingRefV1 -> ValueId`, strict finish | GenericLoop route selection or Recipe shape |
| `CallableLoopSourceProjectionV1` | grouped `binding + class + (site, role)` classification | AST/Recipe re-pairing or physical values |
| `RawInvocationSourceContextV1` | exact loop/condition/body source path under one callable root | binding meaning or route policy |
| GenericLoop Facts issuer | one raw Loop shape and existing GenericLoop facts | source identity reconstructed from cloned AST |
| registry selection/preflight | ordered candidate route and policy observations | callable binding roles or source consumption |
| future source-Recipe relation issuer | co-seal already-issued source rows, located Loop inputs, Facts, and terminal route | new source meaning, fallback, or physical emission |
| `CallableOrdinaryLoopJoinIrConsumerV1` | consume the private relation once during the same normalizer traversal | source issuance, route reselection, or Builder inference |
| `PlanVerifier` / `PlanLowerer` | verify the resulting CorePlan / remain the sole physical route | source-row repair or ledger completion |

### Required target shape

The D1 target is a private, non-`Clone` relation; this is a contract sketch,
not authorization to add the type yet.

```text
exact callable owner
+ exact Loop / condition / body source contexts
+ grouped binding/site/role rows
+ source-located GenericLoop input relation
+ exact GenericLoop Facts/Recipe family
+ front-selected terminal route with unreached legacy tail
  -> CallableOrdinaryLoopSourceRecipeRelationV1
  -> immediate CallableOrdinaryLoopJoinIrConsumerV1
```

The consumer must be moved only after the selected route is terminal. It then
drives the existing GenericLoop normalizer once. Variable reads and rebinds are
claimed at their exact source inputs during that traversal; the normalizer may
produce the existing CorePlan, which continues through the existing verifier
and sole physical lowerer.

The current `Ready` schedule can supply an infrastructure fixture because its
carrier law is already closed. Body-only rebind cannot be relabeled as Ready:
the present schedule seal requires a carrier to have
`ConditionRead + BodyRead + BodyRebind`. The production body-only cohort gets
its own later source disposition/consumer extension after D1; until then it
remains `OutsideTerminal`.

### Finite state table

| State | Required evidence | Allowed next step | Effects |
| --- | --- | --- | ---: |
| `Unbound` | no callable source relation | existing non-callable route only | none from this bridge |
| `SourcePrepared` | exact parent/condition/body + grouped rows | source-to-Facts validation | none |
| `NestedUnsupported` | executable nested Loop in selected source body | typed terminal | none |
| `OutsideTerminal` | complete source observation outside admitted cohort | existing stable terminal | none |
| `RouteDeferred` | selected route is retry-capable or nonterminal | design stop / typed terminal | none |
| `ReadyToConsume` | source-to-Facts relation + one terminal route | immediate named consumer | none |
| `Consuming` | one moved consumer in the existing normalizer walk | Recipe completion only | no fallback |
| `RecipeReady` | all required source rows consumed + existing CorePlan | `PlanVerifier` | plan preparation only |
| `PhysicallyLowered` | verified CorePlan | existing `PlanLowerer` | physical |
| `Rejected` | missing/foreign/duplicate/bypass/route drift | outer session discard | unpublished |

`RouteDeferred` must not be encoded as an empty route, `Option::None`, or a
permission to enter legacy scheduling. `Consuming` cannot transition back to
another route.

### Ordered bounded tasks

1. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-S0-D1` — current design stop.
   Census the production and focused fixtures through
   `try_extract_generic_loop_v1`, `RecipeFirstRouteSelectionV1`, and
   `observe_selected_preflight_v1`; choose `RecipeOnly` or `ExitAllowed` for
   the first source-aware cohort and fix the exact source-to-Facts relation.
2. `MIR-CALLABLE-LOOP-GENERIC-TERMINAL-PORT-P0` — caller-zero only after D1.
   Add one private non-Clone continuation for the exact front-selected
   GenericLoop family. Compose/lower rejection is terminal; legacy suffix,
   `PostEffectRetryDebt`, fallback, and retry are inaccessible.
3. `MIR-CALLABLE-LOOP-ORDINARY-READY-PORT-P0` — caller-zero infrastructure.
   Thread one separate callable semantic port through the existing normalizer
   traversal for one non-nested already-Ready fixture. Structural expression
   lookup remains owned by the existing expression port; exact reads/rebinds
   are owned by the callable state. No production Outside row is admitted.
4. `MIR-CALLABLE-LOOP-BODY-ONLY-REBIND-I0` — first BoxCount.
   Issue grouped body-only rows directly into the named consumer, run the same
   source-aware normalizer once, and prove complete callable finish. Other
   Outside reasons remain terminal.
5. `MIR-CALLABLE-LOOP-ORDINARY-BRIDGE-R0` — atomic cutover.
   Named production caller = 1, admitted old terminal/bypass caller = 0,
   route fallback/retry = 0, and the merged probe advances past the old
   Outside boundary without weakening finish.

Each implementation cell must keep production files below 800 lines and split
at 760. `expression_port.rs` is already 648 lines and
`generic_loop/facts/extract/v1.rs` is 593 lines, so the semantic port and its
tests belong in new sibling modules rather than those files.

### Acceptance and structural guards

```text
positive:
  exact same-owner Ready fixture -> one source-aware Recipe -> one PlanLowerer
  later body-only fixture -> each BodyRead/BodyRebind site consumed exactly once

negative before effects:
  foreign parent/condition/body
  missing/duplicate/foreign binding receipt
  source row paired to cloned Facts from another invocation
  executable nested Loop
  selected route not front-terminal
  RecipeOnly/ExitAllowed mode drift
  normalizer statement path without semantic-port coverage

guards:
  OutsideReason used as ordinary authority = 0
  source/Recipe pairing by name, ordinal, AST pointer, or ValueId = 0
  callable consumer through legacy execution continuation = 0
  callable PostEffectRetryDebt / fallback / retry = 0
  second AST/source walk = 0
  CallableSemanticLoweringState::finish weakening = 0
  PlanLowerer physical owner count unchanged
```

### NoSafeSlice

Remain in `design_stop` if any proposal requires:

```text
GenericLoop Facts without an exact source-located relation;
Outside diagnostic arrays as grouped consumer input;
name/ordinal/AST-shape/ValueId re-pairing;
one port that silently mixes structural source lookup and callable authority;
source consumption before the selected route is terminal;
GenericLoop PostEffectRetryDebt or any later route attempt after consumption;
an uninstrumented normalizer path that can read/rebind through Builder maps;
body-only rows relabeled as the existing Ready carrier shape;
nested Loop delegation without a child-context/reborrow owner;
post-walk repair, finish relaxation, fallback, or retry.
```

The immediate current task is D1 design/census. No Rust implementation or new
receipt is authorized by this section.
