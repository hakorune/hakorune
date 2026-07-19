---
Status: Active task
Date: 2026-07-19
Parent: callable-result-act0-arg0-source-gate-task-2026-07-19.md
Supersedes:
  - callable-result-act0-arg0-actual-selection-design-stop-2026-07-19.md
Scope: ACT0-ARG0 actual-profile rebase and bounded LOOP0 resumption
---

# Callable-result ACT0-ARG0 actual-profile rebase

## Decision — Direction 2 selected

Candidate 1-prime's source gate remains selected. The actual Parser fixture is
rebased to its observed source truth:

```text
actual method-call rows: 15
static target candidates: 2
actual selected activation rows: 0
actual Unselected rows: 15
```

Both target candidates are `skip_ws` calls whose required argument is a nested
current-owner `static_const_eval_pos(...)` result. That result is outside the
selected static source-proof profile, so no exact `(caller, site)` call-result
row exists. Both outer calls are therefore planned `Unselected` before Builder
effects.

Selected-terminal coverage moves to one explicit generic source fixture with a
direct exact required argument. The actual Parser fixture proves source
coverage, source-gate classification, source-order claims, and raw-primary
emission only. It does not claim a selected call.

`CALLABLE-RESULT-NESTED-REP0` remains parked. It is a future semantic widening,
not an L0 repair.

## Why direction 2

`Unselected` is already a first-class planned emission disposition:

```text
claimed Unselected row
  -> existing raw CorePlan effect emission

claimed SelectedExactI64 row
  -> canonical selected terminal
  -> final required-ValueId Integer gate
```

The first branch does not attempt the selected terminal and does not catch a
selected failure. It is therefore not a fallback.

Direction 1 would require all of the following together:

```text
current-owner instance source-target authority
instance-call result solver and source evidence
nested source-proof integration
final-remap MethodCall destination/receiver/site witness
successful MethodCall result-type publication
new type-publication conflict and no-retry laws
```

Those authorities cross the selected source gate, final CorePlan emission, and
function-local fact publication boundaries. They remain one separate
`CALLABLE-RESULT-NESTED-REP0` program, after the bounded Loop closeout and the
first `MIRBUILDER-CLEAN0-FSESSION0-CENSUS0` architecture row.

## Authority split

| Concern | Owner |
| --- | --- |
| actual source call identity | existing `SourceExprSiteV1` inventory |
| exact static target candidate | existing target catalog |
| target exact-i64 condition | existing result disposition |
| source-required argument fact | existing same-site call-result row |
| activation disposition | one activation-row classifier consumer |
| actual Unselected execution | existing raw CorePlan effect emission |
| selected terminal source coverage | separate generic fixture |
| final selected argument type | existing selected terminal / Builder type context |

No activation row stores a `ValueId`, `MirType`, Builder, AST, nested-result
fact, or retry capability.

## Fixed order

```text
CALLABLE-RESULT-ACT0-ARG0-D1
  this direction-2 decision lock; closed by this card

CALLABLE-RESULT-ACT0-ARG0-P0-REBASE0
  disconnected actual/generic source-proof matrix

CALLABLE-RESULT-ACT0-ARG0-I0
  one activation-row construction consumer

CALLABLE-RESULT-ACT0-ARG0-G0
  guard and generic selected-terminal fixture migration

LOOP0-L0-R0-S0
  one non-Clone claimed CorePlan execution session; it alone owns
  Active / Poisoned / Completed state

LOOP0-L0-R0-P0
  actual all-Unselected claim -> raw-effect -> finish proof

LOOP0-L0-R0-F0
  synthetic selected failure -> Poisoned / no retry / no finish proof

LOOP0-L0
  bounded located GenericLoop acceptance

EXPR0-C0
  one production root with exact coverage

MIRBUILDER-CLEAN0-FSESSION0-CENSUS0
  before any broad callable/control-flow widening

CALLABLE-RESULT-NESTED-REP0
  parked; open only if an explicit post-CLEAN0 profile needs it
```

`LOOP0-L0-R0-S0` is intentionally after I0/G0: an all-Unselected activation
plan does not exist until I0 connects the source gate. It introduces neither a
new route, ledger, site, or type authority; it only gives the already-claimed
CorePlan execution bundle one stack-scoped failure owner. P0 is the bounded
lowering and claim-completion proof, not a full Parser runtime claim.

## `P0-REBASE0` — disconnected source matrix

```text
production activation consumer: 0
Builder / CorePlan / ledger delta: 0
```

Build the existing declarations, target catalog, result catalog, and all 15
observed sites once inside a test-only scoped fixture view. Do not call the old
activation-row constructor.

Required proof:

```text
actual rows = 15
target candidates = 2
selected = 0
Unselected = 15

both target candidates:
  RequiredArgumentSourceProofUnavailable

nested static_const_eval_pos sites:
  NoStaticSourceTarget

claim schedule and source-site order:
  unchanged
```

Rename any helper that means target candidate to
`static_target_candidate_sites`; no test may call it `selected_*`.

Add one independent generic fixture:

```text
direct static source call
required argument = exact formal or literal
source gate = Selected
```

This fixture proves Gate S success only. It does not borrow actual Parser
source, name a Parser method, or widen nested instance results.

### P0-REBASE0 closeout (2026-07-19)

Closed with a disconnected scoped source fixture. It builds the actual
declarations, target/result catalogs, and observed method-call inventory, then
returns without constructing activation rows or touching Builder, CorePlan, or
the caller ledger.

```text
actual observed rows: 15
static target candidates: 2
Selected decisions: 0
Unselected decisions: 15

both static candidates:
  RequiredArgumentSourceProofUnavailable

nested static_const_eval_pos site:
  NoStaticSourceTarget
```

The target-candidate helper is named `static_target_candidate_sites`; no proof
labels a candidate as selected. Gate-S success is retained only in independent
generic fixtures: one literal required argument and one direct formal required
argument. This row has no production activation consumer and leaves Builder,
CorePlan, ledger, source-site order, and claim schedule unchanged.

## `I0` — one activation-row consumer

Replace the target-disposition-only selection branch in
`VerifiedCallableResultActivationRowsV1::verify` with exactly one call to the
S0 classifier per static candidate site.

Postconditions:

```text
actual activation plan = 15 Unselected rows
generic activation plan = one SelectedExactI64 row
direct target-disposition-only selection = 0
activation rows retain no lowered representation state
```

Catalog disagreement remains a typed seal failure. Missing call-result proof
remains ordinary `Unselected`.

### I0 closeout (2026-07-19)

Closed with one production consumer in
`VerifiedCallableResultActivationRowsV1::verify`. Every observed method-call
site calls `classify_activation_source_site_v1` once, then copies only the
owned selected target and required-ordinal disposition or the ordinary
`Unselected` disposition.

```text
activation-side target-disposition selection: 0
activation-side call-result lookup bypass: 0
source-gate production consumers: 1
actual activation plan: 15 Unselected rows
generic literal activation plan: 1 SelectedExactI64 row
```

The actual Loop claim batch remains all-Unselected. The selected-terminal
fixture now borrows the generic activation plan rather than the Parser fixture;
the remaining G0 fail-fast fixture owns the selected-terminal failure law.

## `G0` — regression and guard closeout

Reuse the existing callable-result activation guard and its expression-spine
importer. Do not add a manifest or shell-guard family.

Move the selected-terminal success and fail-fast fixtures from the actual
Parser plan to the generic fixture.

```text
generic selected terminal:
  uses claimed canonical target, not raw spelling
  exact required final ValueIds -> Call plus Integer result publication

generic selected terminal failure:
  Unknown required final ValueId -> no Call/result publication
  terminal returns typed fail-fast only
  execution-session poison is not claimed here

actual fixture:
  selected-terminal attempts = 0
  raw retry = 0
```

Guards:

```text
source-gate decision owners = 1
source-gate production consumers = 1
call-result lookups = one per static candidate
actual rows / candidates / selected / Unselected = 15 / 2 / 0 / 15
generic selected rows = 1
activation rows with ValueId/MirType/Builder/AST/retry state = 0
```

### G0 closeout (2026-07-19)

Closed with generic selected-terminal success and fail-fast coverage plus the
existing activation guard.

```text
generic selected success:
  claimed canonical target is used; raw GlobalCall spelling is not authority

generic selected failure:
  required final ValueId = Unknown
  -> typed fail-fast
  -> Call publication = 0
  -> result type publication = 0

actual Parser activation:
  15 Unselected
  -> selected-terminal attempts = 0
  -> existing raw-primary branch is selected before emission
```

This row deliberately does **not** claim poison, retry prohibition, or finish
prohibition for an executed located-loop session. The selected terminal is
stateless; `ClaimedLocatedCoreLoopExecutionV1` explicitly leaves failure-state
ownership to its caller. That owner is introduced by `LOOP0-L0-R0-S0`.

## `LOOP0-L0-R0-S0` — claimed execution-session owner

```text
production located-loop callers: 0
new authority: exactly one stack-scoped, non-Clone execution session
new route / ledger / site / type authority: 0
```

Introduce one `LocatedCoreLoopExecutionSessionV1`-style owner around the
existing claimed loop execution bundle. It owns the only state transition:

```text
Active claimed execution
  -> successful lowering and claim completion -> Completed
  -> selected or raw lowering failure -> Poisoned
```

The session starts only from the existing verified located plan plus its atomic
claim batch. It neither classifies source calls nor creates claims. It may lower
once; `finish` is permitted only after successful lowering, while retry and
finish after `Poisoned` fail explicitly. A fresh Builder/compiler/session stays
reusable. Do not reuse the legacy AST lowering session: this is a CorePlan
execution owner with a different authority boundary.

### S0 closeout (2026-07-19)

Closed with one private claimed bundle and one non-Clone
`LocatedCoreLoopExecutionSessionV1` owner.

```text
sole state-entry:
  VerifiedLocatedCoreLoopPlanV1::start_execution

states:
  Active(claimed bundle) -> Completed | Poisoned

session finish:
  Completed -> success
  Active -> Unexecuted
  Poisoned -> Poisoned

production session callers: 0
legacy AST session reuse: 0
claimed-bundle bypass: 0
```

The session does not create a second ledger or claim map. Its `lower_once`
consumes the existing bundle before calling the existing port, making a retry
structurally unavailable after either result. P0 owns first actual raw-primary
execution; F0 owns the first injected selected-terminal failure transition.

## `LOOP0-L0-R0-P0` — all-Unselected bounded lowering proof

Use the exact actual source carrier and its all-Unselected activation plan to
prove only:

```text
final located plan seal
source-order claim batch
one raw effect emission per claimed actual call
all claims consumed and finish succeeds
selected-terminal attempts = 0
retry / alternate route = 0
```

The source fixture is a reduced Parser method plus controlled dependencies.
This row does not claim full Parser-module VM execution. A full-runtime harness
is a separate decision.

If raw plan composition, lowering, or claim completion fails, stop at:

```text
RAW-PLAN-EMISSION-PARITY0
```

Do not open nested-result inference, type backfill, or `NESTED-REP0` there.

### P0 investigation stop (2026-07-19)

The first exact actual execution probe reached the claimed raw-primary port but
stopped at the existing lowering-time lifecycle verifier:

```text
[freeze:contract][value_lifecycle/typed_without_def]
fn=whole_loop_p0/0
tag=loop_lowerer:after_finalize_loop_variables
missing0=%1, type=String, varmap hit=[text]
```

The probe reused the existing `seeded_builder` helper. That helper creates an
empty `enter_function_for_test` function, then publishes typed values directly
into `variable_map`; those values are neither emitted definitions nor entries
in `MirFunction.params`. The verifier correctly rejects them when the actual
lowering first reaches its function-boundary contract.

This is not evidence for nested callable-result representation, source-gate
widening, selected-terminal type backfill, or a raw-route retry. The failing
test WIP is evidence-only and must not be committed while its focused gate is
red.

### `RAW-PLAN-EMISSION-PARITY0-D0` — canonical pre-Loop harness selected

```text
selected shape:
  one test-only bounded instance-method entry/prefix harness
  adjacent to the existing calls/lowering owner

entry:
  existing method skeleton + declared signature + setup_method_params

prefix:
  existing raw lowering of exact actual Body(0)..Body(3)

located execution begins:
  exact actual Loop Body(4)
```

The harness has no public generic parameter-seeding API. It must delegate to
the existing method entry owner through a cfg(test) closure or adjacent
test-support seam, not copy `setup_method_params` into GenericLoop tests.

```text
canonical entry owns:
  me / text / pos parameter identity and any existing representation facts

raw prefix owns:
  ret / value / updated pos and ordinary static-call resolution

forbidden synthetic seed facts:
  text:String
  pos:Integer
  value:Integer
  me:Box(ParserBox)
  ParserStringUtilsBox local binding
```

`static_const_parse_add(text, pos)` has unannotated explicit formals. The D0
proof records the canonical pre-Loop snapshot rather than preserving the old
synthetic `String`/`Integer` types. Any post-prefix mismatch is a raw
plan/emission parity fact, not permission to seed a replacement type.

Fixed order:

```text
RAW-PLAN-EMISSION-PARITY0-D0
  this decision lock; closed

RAW-PLAN-EMISSION-PARITY0-H0
  disconnected canonical method-entry + Body(0..3) prefix harness

RAW-PLAN-EMISSION-PARITY0-R0
  retry actual Body(4) all-Unselected session execution from that harness

LOOP0-L0-R0-P0
  closes only if R0 proves raw-primary claim/effect/finish parity
```

If canonical entry/prefix cannot produce the needed live function state, or
R0 still fails after it, stop again with the exact producer/route/CFG/lifecycle
fact. Do not add type backfill, fallback, source/ledger changes, nested-result
widening, or another test-only type authority.

## `LOOP0-L0-R0-F0` — selected execution failure proof

Use one synthetic generic selected row whose final required argument ValueId is
`Unknown`. The execution session must become `Poisoned` after the existing
selected terminal fails.

```text
Call instruction publication = 0
result type publication = 0
retry on the same session = rejected
finish on the same session = rejected
raw / alternate route retry = 0
fresh session = reusable
```

This is the only row that claims the session poison law. It does not widen
source selection, infer a nested result, or write a type fact.

## Explicit non-claims

```text
static_const_eval_pos is exact i64
the actual Parser caller contains a selected exact-i64 call
all exact-result static targets are selected
source proof publishes MIR types
activation proves final ValueId representation
instance-call result catalog or source-to-ValueId mapping
selected failure fallback
full Parser runtime execution from LOOP0-L0-R0
```

## Stop conditions

Stop the selected row if it needs:

1. AST rewalk, source/method/owner-name inference, or runtime tags;
2. a source call-result row absent from Gate S to reach selected emission;
3. ValueId, MirType, Builder, AST, or retry state in activation rows;
4. type backfill from a selected claim;
5. selected failure followed by raw/alternate retry;
6. an instance result catalog, current-owner result solver, or MethodCall
   selected terminal inside ARG0/L0;
7. claim schedule, caller ledger, PATH0, CoreCallSource, or site identity
   changes to make the actual fixture pass;
8. a full Parser runtime harness disguised as the bounded raw lowering proof;
9. stash restoration as implementation authority; or
10. a touched source/check file at or above 800 lines.

## Final lock

> Direction 2 is selected. The existing source-proof gate remains the sole
> activation selection law. Actual `static_const_parse_add` truth is 15 source
> rows, two static target candidates, and zero selected rows because both
> required argument expressions are unsupported nested instance results.
> Every actual row is therefore planned Unselected and may use only the
> existing raw primary effect route. One generic direct-argument fixture, not
> the actual Parser caller, owns successful selected-terminal and terminal
> fail-fast coverage. ARG0 closes through `P0-REBASE0 -> I0 -> G0`; then
> `LOOP0-L0-R0-S0 -> P0 -> F0` gives claimed CorePlan execution one exact
> poison owner, proves bounded all-Unselected lowering, and fixes the
> selected-failure no-retry law before L0 and EXPR0-C0 resume.
> Nested instance result representation remains parked until after the first
> selected MirBuilder clean-architecture census.
