# Generic G0 Recipe S4 design task

Status: `accepted` on 2026-08-07 after independent source audit and worker
review. This document is the design gate for the next caller-zero implementation
cell, `GENERIC-G0-RECIPE-S4-I0-R0`.

## Decision

S4 has exactly one Generic producer. It consumes
`VerifiedGenericRecipeDemandG0` by value and emits one portable, route-agnostic
product. S4 does not open a production caller, physical MIR, retry/fallback, or
legacy retirement.

```text
Demand
  -> deterministic LoopRecipeV1 + source/effect relations
  -> Recipe verifier and source binding claim
  -> common JoinSig elaborator
  -> exact After capability
  -> common source-bound Core issuer
  -> Generic After/tail envelope
  -> VerifiedGenericRecipeProductG0
```

The sequence is fixed. No stage may recover source facts from names, AST,
`RecipeBody`/`RecipeBlock`, route IDs, legacy schedules, or test fixtures.

## Ownership boundaries

1. **Demand consumer** consumes the selector lease, source brand, typed source
   bundle, post-loop read, target, mode/profile/coverage, and private role lease
   once. S3 provenance is checked and then dropped; it is not copied into the
   portable Recipe product.
2. **Source binding** uses the sole
   `bind_resolved_loop_source_forest_v1` adapter. The forest binding remains
   live until the source-bound Recipe has been verified. Foreign, duplicate, or
   unsupported paths are typed rejects; source lookup is never reissued.
3. **Recipe producer** privately maps the exact G0 source roles to dense
   `LoopBindingKeyV1`, `LoopNodeKeyV1`, `LoopValueKeyV1`, and item keys, then
   invokes `LoopRecipeVerifierV1::verify`. `LoopRecipeProducerIdV1::GenericG0`
   is issued only here.
4. **JoinSig** remains common-owned. S4 calls
   `LoopJoinSigElaboratorV1::elaborate` once and then calls
   `require_after_binding(LoopNodeKeyV1(0), LoopBindingKeyV1(1), I64)` once.
   S4 does not construct JoinSig rows itself.
5. **Core** remains common-owned. S4 supplies the exact source binding and
   effect relations to `issue_source_bound_core_v1`; Core does not infer
   Generic roles, reselect a family, or inspect source names.
6. **After envelope** owns the logical `L0.After/b1` capability, the moved S3
   post-loop read, the exact `ExactTrivialReturnAbiV1`, and owner/frame/tail
   relation. It does not contain a function tail or executable Return writer.
   P0 owns completion/DraftSeal. The outer product retains `core`, `after`, and
   `NumericTarget`; the ABI is exposed through `after` rather than duplicated.

## Golden G0 source mapping

The deterministic source roles are:

```text
bindings: b0 = outer i, b1 = inner j
loops:    L0 = root, L1 = child of L0
inputs:   v0 = initial i, v1 = initial j
carriers: C0=(L0,b0,v0), C1=(L0,b1,v1), C2=(L1,b1,child-entry)
```

The recipe contains the root/child condition blocks, the child loop entry, the
inner `j + delta` write, and the outer `i + delta` write in canonical recursive
preorder. It contains no function tail or explicit Return exit. The exact
value/item numbering and block membership are fixed by the S4 golden fixture;
the producer must not derive them from source names or insertion order.

The three carrier rows are mandatory. Omitting `C1` loses the child result at
the outer loop; omitting `C2` loses the child header recurrence; using `v1`
directly for `C2` would reset the child on each outer iteration.

## Exact source/effect relation matrix

The Generic producer owns this deterministic relation mapping. Common
verification remains role-agnostic.

```text
b0: SourceRead  outer-condition-lhs
b0: SourceRead  outer-update-lhs
b0: SourceWrite outer-update-target

b1: SourceRead  inner-condition-lhs
b1: SourceRead  inner-update-lhs
b1: SourceRead  post-loop-tail-value
b1: SourceWrite inner-update-target

c0: DerivedCarrierEntry(root L0, carrier C0)
c1: DerivedCarrierEntry(root L0, carrier C1)
c2: DerivedCarrierEntry(child L1, carrier C2)
```

The relation set must reject duplicate, missing, foreign, wrong-anchor, and
uncovered rows. RHS literals are values, not BindingRef effects.

## Outcome and stop boundary

S4 emits `Ready` only after all of the following are sealed:

- demand invariants and source-forest claim;
- deterministic Recipe verification;
- producer provenance;
- JoinSig and exact After capability;
- source binding relation and the ten-row effect relation matrix;
- post-loop tail, owner/frame, and exact return ABI pairing.

Contradictions are typed `Rejected`. An already verified S3 demand does not
produce `Unresolved`; opaque capability handling, if ever admitted, is a
separate design row. S4 never emits `NoCandidate`, retries, fallback, `.ok()`,
or a legacy demand.

## Implementation shape

Keep the producer below the 800-line source boundary by splitting the future
module into small owners:

```text
loop_recipe_contract/generic_g0/
  mod.rs       product/error exports
  recipe.rs    private key map and Recipe draft
  relations.rs source/effect matrix
  after.rs     After/tail/ABI envelope
  producer.rs  one orchestration function
```

`generic_g0_demand.rs` remains test-only. No Builder, MIR, ValueId, PHI,
physical ID, route ID, legacy scheduler, AST rewrite, or Generic-specific
physicalizer is allowed in S4.

## Acceptance evidence for S4 I0/R0

- one natural G0 demand is consumed exactly once;
- deterministic Recipe golden round-trips through the common verifier;
- source forest binding is claimed once and cannot be reissued;
- JoinSig is elaborated once and After is requested once;
- exact ten-row relation matrix has positive and negative fixtures;
- foreign provenance, stale demand, duplicate/missing role, wrong anchor,
  wrong After binding, and wrong ABI reject before Builder effects;
- no production caller, physicalizer, completion writer, retry, fallback, or
  legacy route is opened;
- focused tests, shared guards, pointer guard, line limits, and
  `git diff --check` are green.

The S4 implementation commit must update this task receipt, the Generic source
to portable Recipe SSOT, `docs/reference/mir/generic-loop-stage-matrix.md`,
affected module READMEs, and all current mirrors in the same commit. The
reference closeout remains mandatory after later physical/cutover rows.
