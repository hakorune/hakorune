# JOINIR Loop M8 LoopV0 recurrence S6A — design-only task

Status: `revised design-only row — source cohort not yet sealed`
Date: 2026-08-08
Parent: `LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0`
Current execution row: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`

## Decision

The next bounded prerequisite is one shallow source-to-Facts-to-Recipe mapping
for the M8 LoopV0 recurrence cohort. This is a design row only. Do not open
M8 implementation, M9 `.hako` parity, production selection, M10b cutover, or
legacy deletion from this card.

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
implemented yet. The S6A design remains open as `NoSafeSlice`; no route
relabel, DirectAccum reuse, or new Recipe kind is allowed.

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

## Sole owner and authority

The sole design owner is the M8 source-to-portable-Recipe pipeline:

- `docs/development/current/main/design/joinir-loop-selfhost-recipe-pipeline-ssot.md`
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
`apps/tests/loop_simple_while_inline_explicit_step_min.hako` is retained as a
negative boundary: its planner/fast-gate `LoopSimpleWhile` label is legacy
provenance, not a canonical Facts result. The positive cohort must be the
variable-update recurrence shape itself, with a new observer contract designed
here; no existing DirectAccum whole-function capability may be widened to
ingest this `Main.main` five-statement frame.

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

If the natural source cannot be proven to have one exact neutral mapping, the
result is typed `Unresolved`/`NoSafeSlice`, not a new route label or producer.

The audited fixture is specifically `Main.main` with five root statements
(`local i`, `local acc`, the loop, `print`, `return`). Existing DirectAccum
capability rejects that function/frame before Recipe production (it requires
its own two-statement prefix and does not admit `main`), in addition to
rejecting the variable accumulator update. This is a source-owner boundary,
not a reason to widen DirectAccum.

## Frozen design target (not yet an implementation claim)

The S6A observer contract is for exactly this one-loop source shape:

```text
function/frame: Main.main, resolver-owned root loop site Body(2)
condition:      i < 4
body:           acc = acc + i; i = i + 1
carriers:       i, acc (two distinct local BindingRef values)
outer tail:     print(acc); return 0 (outside the LoopRecipe cohort)
```

The neutral Recipe/Core golden is schema-level and must be issued only after
the new observer proves the source anchors:

```text
Recipe: one L0 Predicate(Less), no exits, two carrier entries
inputs: V0=i-init, V1=acc-init
items:  V0..V10 (11 operations)
  condition: read(i), const(4), compare-less
  accumulator: read(acc), read(i), add, write(acc)
  step: read(i), const(1), add, write(i)
JoinSig: Header/Body/After payloads for i and acc, plus the backedge
Core effects: 2 carrier-entry anchors + 6 exact read/write anchors
```

This is representable by the existing `LoopRecipeV1` algebra but is not yet
issued by any resolver-backed producer. Therefore the current design result is
`NoSafeSlice`, not `Candidate`; the implementation row may open only after the
observer/Facts/effect-anchor ownership is sealed.

Typed disposition matrix:

```text
Candidate:
  only after the new observer proves the frozen shape and all anchors
Declined:
  current SimpleWhile/DirectAccum/G0 owners, exit-only, nested, calls,
  unsupported operators, or a non-recurrence shape
Unresolved:
  missing/opaque source, frame, BindingRef, carrier, or effect coverage
Rejected:
  foreign owner/frame/site, duplicate evidence, or foreign binding/effect
```

## Required output

The implementation that follows this design may issue only existing neutral
products:

```text
LoopRecipeV1
VerifiedLoopCoreProductV1
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

The later S6A implementation is one bounded cohort and one commit. It may
begin only after this card records the exact source membership, normalized
golden, disposition matrix, and acceptance commands. That implementation must
have:

- Builder-free source/Facts/Recipe tests with normalized golden evidence;
- caller-zero, in-place-replacement, and R4 guards green;
- no route retry, scheduler fallback, AST rewrite, or by-name repair;
- every touched source/check file below 800 lines;
- the relevant module README and `docs/reference/**` updated in the same
  implementation commit.

After this design is closed, the implementation must be a separate bounded
S6A cohort. After S6A implementation, update the reference receipt again with
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
