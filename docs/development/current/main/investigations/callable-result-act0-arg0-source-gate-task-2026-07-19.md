---
Status: `CALLABLE-RESULT-ACT0-ARG0-S0` closed; P0 is next
Date: 2026-07-19
Parent: callable-result-i64-site0-r0-expression-spine-loop0-task-2026-07-18.md
Supersedes frontier: callable-result-loop0-l0-i64-argument-authority-design-stop-2026-07-19.md
Scope: source-proof-gated callable-result activation
---

# Callable-result ACT0-ARG0 source gate

## Decision

Candidate 1′ is selected. An activation row is selected only after two
independent gates:

```text
Gate S — source representation
  exact static source target
  + target ExactI64 disposition
  + exact (caller, SourceExprSiteV1) call-result row

Gate L — lowered representation
  final required argument ValueIds are exact Integer
```

Gate S failure is a pre-Builder `Unselected` disposition. Gate L failure keeps
the existing selected-terminal fail-fast: poison the selected session, publish
no Call/result, and never retry raw or an alternate route.

This is a generic `callable_result_representation` owner, not a Loop-local
type repair. The existing source target catalog still owns target candidates;
the result catalog still owns source call-result proof; the activation plan
owns only the final selected/unselected classification.

## Exact source gate

The sole construction owner receives the same branded declarations, targets,
and results catalogs already used by activation-row construction. For one
observed source MethodCall site it performs:

```text
no source target
  -> Unselected(NoStaticSourceTarget)

target disposition absent or Unavailable
  -> Unselected(TargetResultUnavailable)

target disposition ExactI64 but call_result(caller, site) absent
  -> Unselected(RequiredArgumentSourceProofUnavailable)

all three present
  -> require SameModuleStatic evidence to borrow the exact source-target row
  -> require its callee-required ordinals to equal the target disposition
  -> SelectedExactI64
```

`VerifiedCallableResultCallSiteV1::required_i64_arguments()` is the
*caller-propagated* requirement set. It is not the target's ordinal authority.
The classifier instead validates the `SameModuleStatic` evidence's
`callee_required_i64_arguments` against the target disposition.

The following are invariant/seal errors, never `Unselected`:

```text
catalog brand mismatch
non-static selected target
CoreStringMethod evidence at a static-target site
borrowed evidence source-target identity mismatch
callee-required ordinal mismatch or ordinal outside target arity
```

The classifier is construction-only and borrowed. Owned activation rows remain
unchanged:

```text
SelectedExactI64 { target, required_i64_arguments }
Unselected
```

They store no call-result row, `ValueId`, `MirType`, Builder state, AST,
source-to-ValueId map, nested-result fact, or retry authority.

## Actual fixture classification

Both `skip_ws` source-target candidate rows remain in the target catalog. They
are not synonymous with selected activation rows.

```text
actual MethodCall rows: 15
source target candidates: 2
selected: 1
unselected: 14

Body(3) pre-Loop skip_ws(text, pos):
  SelectedExactI64

Body(4) / LoopBody(5) cleanup skip_ws(text, me.static_const_eval_pos(rhs)):
  Unselected

nested static_const_eval_pos call:
  Unselected
```

Every row remains in source coverage, the caller ledger, and the Loop claim
schedule. The cleanup outer call remains claimed before its nested argument;
the unselected raw payload may still evaluate the nested argument first. This
is planned primary execution, never a caught selected-call fallback.

## Task order

```text
CALLABLE-RESULT-ACT0-ARG0-D0
  selected decision; closed by this card

CALLABLE-RESULT-ACT0-ARG0-S0
  private borrowed source-decision classifier
  production consumers = 0

CALLABLE-RESULT-ACT0-ARG0-P0
  source-proof matrix and actual 15-row reclassification
  Builder / ledger delta = 0

CALLABLE-RESULT-ACT0-ARG0-I0
  one activation-row construction consumer
  activation-classification delta only

CALLABLE-RESULT-ACT0-ARG0-G0
  existing guard consolidation and closeout

then
  clean LOOP0-L0 reimplementation
  -> EXPR0-C0
```

This card is the decision-to-code transition: another docs-only closeout is
forbidden. S0 is the sole next code-facing row.

### S0 — construction-only source decision

Add one private borrowed decision/seal vocabulary beside activation-row
construction. It may inspect `results.call_result(caller, site)` exactly once
per static candidate site and validate same-module-static evidence. It creates
no catalog and has no production consumer.

Closeout (2026-07-19): closed. One private sibling source-gate classifier
borrows the existing declarations, target, and result catalogs and returns
only a borrowed selected co-seal or one explicit unselected reason. It checks
catalog branding, pointer identity of same-module-static evidence, and the
callee-required ordinal set without reading the caller-propagated requirement
set as target truth. The owned activation schema, Builder, ledger, plan, and
production activation consumers are unchanged. Focused tests cover literal
source proof selection, missing target, and a nested instance required argument
whose source proof is absent.

### P0 — proof matrix

Prove:

```text
required literal / caller parameter / exact nested static result: Selected
dynamic non-required argument: Selected
required Unknown, non-i64, or nested instance result: Unselected
missing source target or target disposition: Unselected
evidence/target/ordinal inconsistency: typed error
declaration reorder: normalized decision unchanged
actual Parser rows: 15 / selected 1 / Unselected 14
```

Rename the actual-fixture helper that means “target candidate” if necessary;
it must not imply that both candidate sites are selected. Preserve both target
sites, all 15 source sites, and the existing claim schedule unchanged.

### I0 — one activation consumer

Replace only the target-disposition-only branch in
`VerifiedCallableResultActivationRowsV1::verify` with the S0 classifier.
There is one production decision consumer. CorePlan, source site identity,
caller ledger, Loop seals, PATH0, selected emission, and the Gate L terminal
are unchanged. Production located roots remain zero.

### G0 — existing guard family

Extend the existing callable-result activation helper and its public
expression-spine importer; do not add a new shell/manifest family. Guard:

```text
source-decision owners = 1
source-decision production consumers = 1
direct target-disposition-only selection = 0
activation call_result lookup = one per static candidate site
activation rows retain no ValueId/MirType/Builder/AST/retry state
actual candidates = 2; rows = 15; selected = 1; Unselected = 14
pre-Loop skip_ws selected = 1
Loop cleanup skip_ws selected = 0 and raw-planned = 1
nested instance call selected = 0
claim schedule / ledger / CoreCallSource / located-plan delta = 0
```

Keep all touched source and check files below 800 lines.

## Parked widening

`CALLABLE-RESULT-NESTED-REP0` is parked, not next. It may open only when an
explicitly selected profile requires a current-owner/instance nested result as
an exact required input. It would need a separate instance-result authority and
an emission-local non-Clone source-site-to-final-result witness. It must not
be introduced by ARG0 or used to repair LOOP0-L0.

## Stop conditions

Stop ARG0 if it needs any of the following:

```text
AST rewalk or method/target/owner-name inference
runtime tag or finalized metadata as lowering-time type authority
ValueId, MirType, Builder, or retry state in activation rows
call-result-row absence reaching the selected terminal
selected failure followed by raw retry
instance/nested result catalog or solver-scope widening
writing Integer to the observed nested result
claim schedule, ledger, PATH0, located-plan, or CoreCallSource change
stash apply/pop/wholesale-copy authority
```

If the Loop cleanup site is correctly `Unselected` but its planned raw path
still fails, stop for a raw plan/emission parity owner. Do not widen type
inference.

## Claims after ARG0 and resumed L0

```text
exact static calls are selected only with target and source-site proof
non-required dynamic arguments remain allowed
source-proof-incomplete calls are planned Unselected before Builder effects
selected terminal still independently requires exact final Integer values
actual 15-row traversal can retain one selected call and all exact coverage
```

No claim is made that `static_const_eval_pos` is exact i64, that all
exact-result static targets are selected, or that source proof publishes a MIR
type.
