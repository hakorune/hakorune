# JOINIR Loop M8 LoopV0 recurrence S6A — accepted design decision

Status: `accepted design; implementation blocked on LOOP-INPUT-SOURCE-RELATION-SET-R0`
Date: 2026-08-08
Parent: `LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0`
Decision row: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`
Selected next row: `LOOP-INPUT-SOURCE-RELATION-SET-R0`

## Decision

This decision fixes one shallow source-to-Facts-to-Recipe mapping for the M8
LoopV0 recurrence cohort. Implementation opens only through the selected R0
prerequisite and the bounded S6A execution brief. Do not open M9 `.hako`
parity, production selection, M10b cutover, or legacy deletion from this card.

### Audit correction (2026-08-08)

The first source audit is complete. The natural fixture
`apps/tests/loop_simple_while_inline_explicit_step_min.hako` is **not** a
positive S6A witness under the current Facts owners:

```text
body = [acc = acc + i, i = i + 1]
condition = i < 4

LoopSimpleWhileFacts:
  Declined — body is not step-only

DirectAccumFacts / DirectAccumV1:
  Declined — accumulator RHS is the induction binding, not an integer literal

Generic G0:
  Declined — this is not the typed nested G0 source profile
```

The fixture name and the `LoopSimpleWhile` fast-gate label are classification
evidence only. The shape is a real single-loop recurrence, but its resolver
observer, neutral Facts contract, and Recipe producer mapping are not
implemented yet. The S6A development row remains `NoSafeSlice` until
implementation; the source has no S6A disposition before that observer exists.
No route relabel, DirectAccum reuse, or new Recipe kind is allowed.

The active authority is the portable Recipe pipeline:

```text
resolver/source view
  -> neutral structural Facts
  -> one policy disposition
  -> existing LoopRecipeV1 / JoinSig / Core products
```

The 19 `LoopRouteId` values are migration/compatibility ingress evidence, not
semantic Recipe variants. `AccumConstLoop` has a proven `DirectAccumV1`
producer receipt; `LoopTrueBreakContinue` and `NestedLoopMinimal` also have
portable producer receipts. `GenericLoopV0`/`GenericLoopV1` remain
`legacy_only`, and Generic G0 is a separate portable producer. No route-name
alias may be inferred during S6A.

## Owner and supporting authorities

The sole normative design owner is:

- `docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`

The Generic task-order SSOT, public reference receipt, and module README are
subordinate views; they must not redefine S6A:

- `docs/development/current/main/design/generic-loop-source-to-portable-recipe-ssot.md`
- `docs/reference/mir/loop-recipe-contract.md`
- `src/mir/loop_recipe_contract/README.md`

The resolver owns source owner/site/scope/frame and `BindingRef` identity.
The source observer owns only neutral structural Facts and typed provenance.
The producer adapter owns the conversion to the existing Recipe/Core product.
There is no new selector, Recipe family, Builder owner, or physicalizer.

## Design input

Use one natural resolver-backed source cohort and its exact source-policy
observation. The audited fixture
`apps/tests/loop_simple_while_inline_explicit_step_min.hako` is the positive
source target for the new S6A observer, while remaining a typed Decline for the
existing SimpleWhile/DirectAccum/G0 observers. Its planner/fast-gate
`LoopSimpleWhile` label is legacy provenance, not a canonical Facts result. No
existing DirectAccum whole-function capability may be widened to ingest this
`Main.main` five-statement frame.

The mapping must record, for one function/frame/site:

```text
source owner/site/frame
condition and explicit step facts
recurrence carrier and binding relations
operation/effect relations
LoopRecipeV1 item/placement mapping
JoinSig/Core obligations
Candidate | Declined | Unresolved | Rejected disposition
```

After the observer exists, missing source evidence is typed `Unresolved`. If no
safe observer contract exists at all, the development row remains
`NoSafeSlice`; that is not a source disposition and must not be placed inside
the C/D/U/R enum.

The audited fixture is specifically `Main.main` with five root statements
(`local i`, `local acc`, the loop, `print`, `return`). Existing DirectAccum
capability rejects that function/frame before Recipe production (it requires
its own two-statement prefix and does not admit `main`), in addition to
rejecting the variable accumulator update. This is a source-owner boundary,
not a reason to widen DirectAccum.

## Frozen design target (Decision: accepted)

The S6A observer contract is for exactly this one-loop source shape:

```text
function/frame: Main.main, resolver-owned root loop site Body(2)
condition:      i < 4
body:           acc = acc + i; i = i + 1
carriers:       i, acc (two distinct local BindingRef values)
outer tail:     print(acc); return 0 (outside the LoopRecipe cohort)

i declaration:   Local { statement: Body(0), ordinal: 0 }
acc declaration: Local { statement: Body(1), ordinal: 0 }
i initializer:   Body(0) / Initializer(0)
acc initializer: Body(1) / Initializer(0)
```

The selected fixture uses literal zero initializers, but the recurrence family
does not interpret those values. Admission proves each exact declaration,
initializer site, `BindingRef`, and I64 class; initializer evaluation belongs
to the outer callable Prelude. Missing or opaque input evidence is
`Unresolved`, while the initializer value is not a Loop operation or a
recurrence-family discriminator.

The neutral Recipe/Core golden is schema-level and must be issued only after
the new observer proves the source anchors:

```text
Recipe: one L0 Predicate(Less), no exits, two carrier entries
inputs: V0=i-init, V1=acc-init
items:  I0..I10 (11 operations), values V0..V10
  condition: const(4), read(i), compare-less
  accumulator: read(acc), read(i), add, write(acc)
  step: read(i), const(1), add, write(i)
JoinSig:
  Preheader -> Header Enter(B0,B1)
  Header -> Body PredicateTrue(B0,B1)
  Header -> After PredicateFalse(B0,B1)
  Body -> Header Backedge(updated B0,B1)
input-source relations: 2
binding relations: 2
Core effects: 2 carrier-entry anchors + 6 exact read/write anchors = 8
item-source relations: 11
```

Canonical item/source-role assignment is exact:

```text
I0  ConstI64(bound)        -> V3   ConditionBound
I1  ReadBinding(B0)        -> V2   ConditionInductionRead
I2  CompareI64(Less,V2,V3) -> V4   ConditionCompare
I3  ReadBinding(B1)        -> V5   AccumulatorRead
I4  ReadBinding(B0)        -> V6   AccumulatorInductionRead
I5  BinaryI64(Add,V5,V6)   -> V7   AccumulatorAdd
I6  WriteBinding(B1,V7)            AccumulatorWrite
I7  ReadBinding(B0)        -> V8   StepInductionRead
I8  ConstI64(1)            -> V9   StepDelta
I9  BinaryI64(Add,V8,V9)   -> V10  StepAdd
I10 WriteBinding(B0,V10)           StepInductionWrite
```

Source anchors are uniform across producers: reads use the variable-reference
expression, constants use the literal expression, Compare/Add use the whole
binary expression, writes use the assignment target, carrier entries use the
Loop statement plus carrier key, and inputs use the declaration plus
initializer expression. Statement/expression sites are never inferred from a
Recipe ordinal.

This is representable by the existing `LoopRecipeV1` algebra but is not yet
issued by any resolver-backed producer. `NoSafeSlice` is the current
development state, not a fifth source disposition. The implementation row may
open only after the common input-set R0 has landed.

Typed disposition matrix:

```text
Candidate:
  the new observer proves the frozen shape and all anchors
Declined:
  the new observer fully sees an exit-only, nested, call-bearing,
  unsupported-operator, or other non-family shape
Unresolved:
  missing/opaque source, frame, BindingRef, carrier, or effect coverage
Rejected:
  foreign owner/frame/site, duplicate evidence, or foreign binding/effect
```

Independently, the selected fixture remains Declined by the existing
SimpleWhile, DirectAccum, and Generic G0 observers. That fact is not a
disposition issued by the not-yet-implemented S6A observer.

## Accepted observer/Facts/Core boundary

Private source observation may use small DTOs for the two inputs, condition,
accumulator update, induction step, and exact coverage. Those partial DTOs are
not independent verified products. Exactly one move-only aggregate crosses the
neutral boundary:

```text
VerifiedVariableAccumRecurrenceFactsV1
```

It retains the resolver-issued non-Clone Loop source capability, semantic
context, two input observations, exact source roles, binding identities, and
total body coverage. The producer consumes Candidate once and deterministically
assigns Recipe keys. It does not re-read AST or reclassify the family.

The accepted producer provenance is diagnostic only:

```text
LoopRecipeProducerIdV1::VariableAccumRecurrenceV1
serialized = variable_accum_recurrence_v1
```

Core effect ordinals are per `(Recipe binding, access kind)`:

```text
B0/i reads: 0 condition, 1 accumulator RHS, 2 step RHS induction operand
B0/i writes: 0 step target
B1/acc reads: 0 accumulator LHS
B1/acc writes: 0 accumulator target
```

Semantic source roles and exact owned expression sites are the Facts-level
authority. Ordinals are canonical transport/diagnostic order only.

Initializers are not Loop operations. They are two external input-source
relations co-sealed beside Core. `print(acc)` and `return 0` are an outer
callable tail and remain outside S6A.

## Selected prerequisite

The existing callable-only `VerifiedLoopInputRelationV1` already owns the same
truth for one input. Creating an S6A-only two-input product would create a
second authority. Therefore the next executable row is the behavior-preserving
`LOOP-INPUT-SOURCE-RELATION-SET-R0` task, which moves the model into the common
Recipe contract and introduces one move-only exact-coverage set. Callable uses
cardinality one; S6A later uses cardinality two.

Task order:

```text
LOOP-INPUT-SOURCE-RELATION-SET-R0
  -> JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A
  -> S6B / S6C / S6D / S6E / S6G
```

## Required output

The implementation that follows R0 may issue only neutral products:

```text
LoopRecipeV1
VerifiedLoopCoreProductV1
VerifiedLoopInitializedLocalInputSourceSetV1
VerifiedLoopOperationEffectProductV1
typed source-policy observation/disposition
```

It must not add `LoopV0` as a Recipe kind, preserve an AST-bearing Recipe
transport, or create a route-specific physicalizer. Builder/MIR/ValueId/
BasicBlockId/PHI/Completion/DraftSeal/module publication effects are zero for
the design row and remain closed for the first implementation cohort.

## Positive and negative boundary

Positive design witness:

- one exact single-loop recurrence with `acc = acc + i` and `i = i + 1`;
- one normalized source → Facts → Recipe/Core golden with two bindings and
  eleven operations (condition read/constant/compare, accumulator read/read/
  add/write, and induction read/constant/add/write);
- one typed observer/producer contract distinct from
  `LoopSimpleWhileFacts`, `AccumConstLoopFacts`, and Generic G0.

The disposition matrix must reject or defer:

- non-recurrence or exit-only shapes;
- nested or opaque scope lineage not owned by this cohort;
- foreign frame/site/owner evidence;
- missing, duplicate, or foreign operation/effect relations;
- unsupported calls/effects or an unresolved carrier mapping.

The design must state whether each case is `Declined`, `Unresolved`, or
`Rejected`; do not collapse them into `None` or a retryable fallback.

## Implementation gate (after this D0)

The later S6A implementation is one bounded cohort and one commit after the
common input-set R0. It must have:

- Builder-free source/Facts/Recipe tests with normalized golden evidence;
- caller-zero, in-place-replacement, and R4 guards green;
- no route retry, scheduler fallback, AST rewrite, or by-name repair;
- every touched source/check file below 800 lines;
- the relevant module README and `docs/reference/**` updated in the same
  implementation commit.

After S6A implementation, update the reference receipt again with
the landed producer. The final reference update is required again at M10b
production cutover; this design card itself makes no production claim.

## Ordered follow-on work

```text
S6A  LoopV0 recurrence mapping and producer
S6B  LoopV0 exits/joins
S6C  LoopV0 scans
S6D  LoopCond exits
S6E  Generic residual (blocked until its observation/disposition corpus is real)
S6G  all-19 typed coverage closeout
S7A..S7G  .hako producer/parity cohorts after M8 closeout
M8/M9 production-selection design
M10b one-time production cutover
R1/M11/M12 legacy retirement and reference closeout
```

Do not add deeper `S6A-S1-S2` task suffixes. If this mapping reveals a
different bounded prerequisite, replace this single card with one explicit
design decision rather than implementing by guesswork.

## Explicit non-claims

This card does not claim:

```text
all 19 routes are portable
M8 or M9 is closed
.hako is semantic authority
Generic residual is observable
production selection or M10b is ready
retry/fallback/scheduler deletion
legacy removal
backend, VM, LLVM, or performance parity
```
