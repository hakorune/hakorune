---
Status: queued parallel test-only lane
Date: 2026-08-03
Decision: provisional — `JOINIR-GENERIC-V0-NESTED-CARRIER-SCOPE0-D0`
Scope: legacy Generic V0/V1 overlap observation and policy-boundary correction
Related:
  - ../design/joinir-generic-post-effect-debt-classification-ssot.md
  - ../design/joinir-loop-selfhost-recipe-pipeline-ssot.md
  - ../design/phi-lifecycle-ssot.md
  - ../design/binding-ssa-first-control-lowering-ssot.md
---

# Generic V0 nested-carrier scope correction

## Why this is a separate lane

The observed defect is local to the legacy Generic route-selection boundary:
`GenericLoopV0` can be selected for a nested loop that writes an enclosing
binding, even though V0 does not carry that binding's final value through the
nested body. The `.hako` nearest-enclosing-assignment rule requires the outer
binding to retain the nested update after the loop.

This is not a PHI/SSA redesign. The existing owners remain authoritative:

- `phi-lifecycle-ssot.md` owns Reserve/Define/Expose/Populate/Finalize and
  `PhiTxn` for MIR PHI lifecycle.
- `binding-ssa-first-control-lowering-ssot.md` owns function-scoped Binding SSA,
  reaching definitions, and PHI merge decisions.
- This lane owns only pre-effect structural observation and legacy route policy
  evidence. It must not add a PHI writer, SSA builder, Recipe variant, or
  physicalizer.

## Current evidence

```text
V0 nested-loop arm       -> no recursive carrier/final-value propagation
V1 nested-carrier arm    -> recursive carrier collection and final-value apply
outer `j` updated inside  -> V0 may leave post-loop `j` stale; V1 is the target
```

The current V1 policy probe is not yet a production qualification proof: it can
depend on post-effect stage results and the legacy V0/V1 route prefix still has
an unresolved winner-equivalence question. Therefore the target disposition is
not promoted automatically.

## Ordered tasks

### D0 — recursive carrier observation (Builder-free)

Consume the existing structural facts recursively through nested `Loop` and
`If` bodies. Produce an owned observation with one of:

```text
CompleteNoRecursiveCarrier
RecursiveCarrier([binding ...])
Unavailable
Ambiguous
```

The observation must be computed before any composer, verifier, lowerer, PHI,
or candidate effect. No AST rewrite and no name-only dispatch are allowed; the
binding identity must follow the existing resolved/enclosing-binding contract.

Acceptance:

- the nested `j = j + 1` fixture records `RecursiveCarrier([j])`;
- a flat loop records `CompleteNoRecursiveCarrier`;
- unsupported grammar or incomplete provenance records `Unavailable`;
- conflicting observations record `Ambiguous`;
- production caller count remains zero.

### D1 — test-only policy disposition

Feed only the sealed D0 observation into `generic_loop_overlap_policy.rs` (or a
neutral policy owner). The intended target is:

```text
RecursiveCarrier([j]) -> V1ForNestedCarriers target
CompleteNoRecursiveCarrier -> V0 candidate remains possible
Unavailable / Ambiguous -> UnresolvedStop
```

This step is evidence only. It may not suppress V0 in production until D2
winner equivalence is closed. In particular, it must not read
`v1_stage_accepted`, composer receipts, verifier/lower results, or any
post-effect boolean as a pre-effect qualification.

### D2 — winner-equivalence gate

Compare the real legacy witness on fresh candidates with the D0/D1 disposition.
The gate must cover:

1. digest parity for the nested `j` fixture, including the post-loop outer value;
2. a `.hako` end-to-end scope fixture where `j` is the expected value after the
   outer loop;
3. V0-only, V1-only, Both, and Neither schedules;
4. release, strict, and planner-required modes;
5. V1 route/prefix compatibility and any source grammar that V1 cannot lower.

If the target winner cannot be selected before the first Builder effect, keep
`UnresolvedStop`, leave the legacy scheduler unchanged, and record the exact
counterexample. Do not turn a post-effect `None` into a silent V1 preference.

### D3 — narrowly scoped legacy policy correction (only after D2)

If D2 closes with a pre-effect winner proof, add the smallest policy-only
suppression of V0 for the proven recursive-carrier row. The handler, composer,
PlanLowerer, PHI lifecycle, Binding SSA, Retry semantics, and production
physicalization remain untouched.

Required tests:

- policy unit: `RecursiveCarrier([j])` selects the V1 target;
- policy unit: `Unavailable`/`Ambiguous` stays `UnresolvedStop`;
- legacy witness digest and `.hako` scope fixture remain green;
- shared guards prove no new PHI/SSA/Recipe/physicalizer caller.

## Relationship to the main Loop order

This lane can be investigated in parallel with caller-zero M6 work, but it does
not unblock or reorder the current topology stop:

```text
P1-D0 logical Body->Header vs legacy Body->Step->Header topology
  -> P1a/P1b candidate and fixed-CFG test evidence
  -> post-M6 Accum vertical pilot
  -> optional singleton M10a bridge
  -> Generic D2 winner/disjointness closure
  -> M10b atomic all-route cutover
```

Generic rows with overlap remain on the old scheduler until their own gate is
green. SimpleWhile and Nested are not broad non-Generic cutover candidates.

## Stop conditions

Stop this lane if it requires any of the following:

- changing `phi_lifecycle` or `BindingSsaBuilderV1` ownership;
- adding a second PHI/SSA writer or a route-local repair path;
- changing portable Recipe/JoinSig schemas to encode the legacy V0/V1 split;
- retrying a dirty Builder candidate;
- selecting a winner from post-effect success/failure;
- modifying `route_loop` before the named M10a/M10b gates.

