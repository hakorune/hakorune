---
Status: P0 complete; next D0 selected; design_stop
Task: MIR-CALLABLE-LOOP-HANDOFF-BINDING-COVERAGE-D0
Date: 2026-08-22
Priority: classify source-valid Void/unannotated callable rows without poisoning the sparse physical-header cohort
Parent: MIR-CALLABLE-PROGRAM-REGION-CONTAINMENT-P0
PreviousCard: mirbuilder-static-import-target-authority-d0-2026-08-22
NextCard: MIR-CALLABLE-LOOP-HANDOFF-BINDING-COVERAGE-D0 (this rolling card)
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
