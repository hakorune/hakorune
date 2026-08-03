# JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-P0

Status: execution task — caller-zero preseeded physical-input seam only.
Date: 2026-08-04
Design authority:
`joinir-loop-nested-predicate-d4-physical-emission-design-2026-08-04.md`

## Objective

Create the smallest test-only/caller-zero physical-input seam for the bounded
Nested Predicate shape. It must preserve the verified Recipe/JoinSig pair,
consume the sealed symbolic topology once, and prove a candidate-local mapping
to physical blocks without adding a production caller.

```text
VerifiedNestedPredicateRecipeProductV1
  -> VerifiedNestedPhysicalEmissionInputV1 (non-Clone)
       { Recipe, JoinSig, Topology, preseeded block/input projection }
  -> caller-zero canonical-session adapter harness
```

The preseeded fixture is an explicit test capability, not a production source
authority. Resolver-issued prefix/effect claims are P1. Production cutover is
D5-I0 and is not part of this task.

## Required implementation slice

1. Add a named non-`Clone` emission-input product that keeps Recipe, JoinSig,
   and `VerifiedNestedPhysicalTopologyV1` together. Its only constructor
   consumes the producer product/topology once; no projection or AST reread is
   permitted.
2. Add a candidate-local `VerifiedNestedPhysicalBlockProjectionV1` with
   owner/frame brand, current root preheader, fresh root Header/Body/Step/After,
   child Header/Body/Step/After, explicit `Child.Preheader = Root.Body`, and a
   distinct `ParentBodyResume` block. It represents eleven symbolic node refs
   and ten unique physical blocks.
3. Add a test-only preseeded input projection for the existing resolved
   binding/value fixtures. It must not derive names, ordinals, or `ValueId`s in
   the physicalizer. P0 may use explicit fixture evidence only.
4. Add focused positive tests for semantic-pair preservation, owner/frame
   equality, current-vs-fresh block roles, alias uniqueness, and
   `ParentBodyResume` forwarding.
5. Add focused rejection tests for foreign owner/frame, duplicate physical
   block, missing symbolic node, alias mismatch, `Root.After` used as resume,
   and child `j` marked parent-visible.

## Hard boundaries

- production caller count for the emission input and adapter remains zero;
- no `route_loop`, scheduler, Retry/Option, Generic, JoinIR fallback, or
  legacy composer/CorePlan/PlanLowerer edit;
- no `MirBuilder` mutation in the portable input or topology modules;
- no `BasicBlockId` in Recipe/JoinSig/topology; physical IDs exist only in the
  candidate-local projection;
- no direct PHI writer, `LoopPhiMaterializerV1`, route-local materializer, or
  second `PhiTxn`;
- no resolver prefix/effect-plan implementation yet (P1);
- no external candidate commit, old-edge deletion, or selfhost claim.

## Acceptance gates

```text
emission input production callers = 0
topology issuer production callers = 0
physical block map production callers = 0
Recipe/JoinSig/topology consumed exactly once = yes
symbolic node refs = 11
unique physical blocks = 10
Child.Preheader aliases Root.Body = exactly one
ParentBodyResume is distinct and forwards root i/sum = yes
Root.After is never a normal child resume = yes
all touched Rust/test files < 800 lines = yes
```

Focused tests must be green, together with the existing Nested, Recipe,
structural-facts, DirectAccum physicalizer, PHI lifecycle, pointer, and
candidate-scope guards. A full production `cargo test` is not required by this
caller-zero task; unrelated repository-wide warning/format drift is not part
of the acceptance claim.

## Next task after P0

`JOINIR-LOOP-NESTED-PREDICATE-PHYSICAL-ADAPTER0-D5-P1` will add the resolver-
issued `VerifiedNestedPrefixInputV1` and
`VerifiedNestedBindingEffectPlanV1`, including the uninitialized `j`
declaration/first-assignment contract and scope retirement. P1 must be a
separate task and must not widen P0 into a production route.
