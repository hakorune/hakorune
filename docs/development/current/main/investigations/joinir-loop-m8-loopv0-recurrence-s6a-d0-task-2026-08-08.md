# JOINIR Loop M8 LoopV0 recurrence S6A — design-only task

Status: `accepted design-only row`
Date: 2026-08-08
Parent: `LOOP-CALLER-ZERO-PARITY-G0-POST-I1-AUDIT-D0`
Current execution row: `JOINIR-LOOP-M8-LOOPV0-RECURRENCE-S6A-D0`

## Decision

The next bounded prerequisite is one shallow source-to-Facts-to-Recipe mapping
for the M8 LoopV0 recurrence cohort. This is a design row only. Do not open
M8 implementation, M9 `.hako` parity, production selection, M10b cutover, or
legacy deletion from this card.

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
observation. The likely fixture is
`apps/tests/loop_simple_while_inline_explicit_step_min.hako`, but the design
census must freeze the observed source shape and migration evidence together;
the fixture filename and route label are not authority.

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

- one simplest natural recurrence fixture with an explicit condition/carrier
  update;
- one normalized source → Facts → Recipe/Core golden with exact ownership and
  operation/effect membership.

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

After S6A implementation, update the reference receipt again with the landed
producer. The final reference update is required again at M10b production
cutover; this design card itself makes no production claim.

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
