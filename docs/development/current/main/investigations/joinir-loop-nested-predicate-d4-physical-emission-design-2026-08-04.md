# JOINIR-LOOP-NESTED-PREDICATE-D4-PHYSICAL-EMISSION-DESIGN

Status: design stop — implementation is not authorized until the three
physical-emission input seams below are sealed.
Date: 2026-08-04

## Decision boundary

D4 has a caller-zero symbolic topology, but it is not yet a physicalizer
input. The next slice must connect that topology to the existing canonical
function session without creating a second CFG, SSA, or PHI authority:

```text
one resolved function owner/frame
  -> VerifiedNestedPhysicalEmissionInputV1 (non-Clone)
       { VerifiedLoopRecipeV1,
         VerifiedLoopJoinSigV1,
         VerifiedNestedPhysicalTopologyV1,
         VerifiedNestedPhysicalBlockProjectionV1,
         VerifiedNestedBindingEffectPlanV1,
         VerifiedNestedPrefixInputV1 }
  -> one canonical session
       CanonicalCfgSessionV1
       + one function-owned BindingSsaBuilderV1
       + one caller-owned PhiTxn
  -> physical block/edge/value emission
  -> outer function completion + candidate commit
```

The adapter is a resolved-lowering consumer, not a route handler. It receives
one owner-issued plan from the canonical resolved ingress. `route_loop`, the
ordered registry, `LoopPhiMaterializerV1`, `phi_input_materializer`, the old
composer/CorePlan/PlanLowerer path, and Generic retry remain outside this card.

## Hard preconditions found by independent audits

### 1. Preserve the semantic pair through the physical boundary

`issue_nested_predicate_physical_topology_v1` currently consumes
`VerifiedNestedPredicateRecipeProductV1` and returns only topology. That
silently drops the verified Recipe and JoinSig before a physicalizer can
consume them. The next seam must return one non-`Clone`
`VerifiedNestedPhysicalEmissionInputV1` (or an equivalent named product) that
owns the Recipe, JoinSig, topology, and effect plan together.

Rules:

- no second projection, AST read, JoinSig elaboration, or Recipe rebuild;
- no `Clone` escape of the semantic pair;
- Recipe/JoinSig remain portable, source-free, and physical-ID-free;
- topology remains the only source of physical port/edge topology;
- the physicalizer consumes the product exactly once.

### 2. Add exact source-effect claims, including uninitialized `j`

`VerifiedNestedPhysicalTopologyV1` currently carries BindingRef/ScopeId and
role sites, but not the complete execution claim needed by the canonical
identity ledger. The physical input must contain a separate, non-`Clone`,
owner/frame-branded `VerifiedNestedBindingEffectPlanV1` with exact
`SourceBindingSiteV1`/`SourceExprSiteV1` claims for:

- root `i` and `sum` declarations and entry initializers;
- root predicate `i` read and root `i` update read/write;
- child `j` declaration in the outer root-body scope;
- child `j` first assignment (`j = 0`) before the child predicate;
- child predicate `j` read and child `j` update read/write;
- child ancestor `sum` read/write;
- scope retirement of `j` at the root-body boundary.

The source projection currently identifies `j` as an uninitialized local. The
canonical identity owner therefore needs a narrowly named declaration seam
that adopts and activates `j` without defining a reaching ValueId; its first
assignment must define the value through the same Binding SSA owner. It must
not fabricate `Const(0)`, publish by name/ordinal, or make an unverified
`ValueId` projection. A read before that first assignment is a typed reject.

The effect plan is an execution claim, not a second identity ledger. The
resolved adapter must recheck owner/frame, exact source site, binding identity,
active scope, duplicate claims, and complete coverage before the first
physical effect.

The declaration/entry payload is a separate resolver-issued
`VerifiedNestedPrefixInputV1`. It carries exact `SourceBindingSiteV1`,
`BindingKindV1`, diagnostic name, `BindingRefV1`, and initial-value evidence for
root `i`/`sum`, plus the exact uninitialized `j` declaration claim. The
caller-zero P0 may use a preseeded binding/input fixture, but production may
not reconstruct these fields from a name or ordinal. P1 must issue the prefix
from the resolved function owner before the production cutover.

### 3. Map symbolic topology to physical blocks only after validation

The adapter must allocate a physical map only after checking the topology
owner/frame and effect-plan owner/frame against the canonical function session.
There are eleven symbolic node references but ten unique physical blocks:

```text
root:  Preheader Header Body Step After
child: Header Body Step After
       Child.Preheader aliases Root.Body (no second block)
extra: ParentBodyResume (distinct block)
```

`VerifiedNestedPhysicalBlockProjectionV1` is a candidate-local, non-`Clone`
owner/frame-branded map. The topology issuer never allocates `BasicBlockId`.
The projection explicitly proves `Child.Preheader = Root.Body`, identifies
the current root preheader, and allocates only the remaining fresh blocks.

The adapter must:

1. validate the complete symbolic predecessor seals and carrier destinations;
2. allocate/map physical blocks without reinterpreting JoinSig;
3. create edges only through `CanonicalCfgSessionV1`;
4. map each symbolic seal to the exact physical predecessor witness;
5. use the function-owned Binding SSA adapter for all reads/definitions;
6. seal Binding SSA only after CFG predecessor witnesses match;
7. leave `ParentBodyResume` as the forwarding point for root `i`/`sum`;
8. keep child `j` local and never publish it into the parent tail;
9. keep `forward_resume = ParentBodyResume` separate from
   `final_after = Root.After` in the carrier projection.

No direct `MirFunction` terminator mutation, raw `ValueId` remapping,
`LoopPhiMaterializerV1`, route-local PHI writer, or second `PhiTxn` is allowed.

## Canonical-session ownership contract

The production-shaped adapter must be a thin lowerer profile over one
`CanonicalSsaFunctionSessionV2`:

```text
ResolvedFunctionLoweringInputV1
  -> VerifiedNestedPhysicalEmissionInputV1
  -> CanonicalSsaFunctionSessionV2::new
  -> publish root declarations / adopt uninitialized j
  -> emit symbolic topology through CanonicalCfgSessionV1
  -> claim exact source effects through the identity adapter
  -> finish child scope and function completion
  -> finish identity + CFG + shared PhiTxn
```

The physicalizer returns an open continuation receipt, not `Return(None)` and
not a scheduler `Option`. The outer function owner consumes the continuation,
claims all remaining source coverage, and owns `finish`/`commit`. Any error
after a Builder effect aborts the shared `PhiTxn` and drops the unpublished
compile candidate; it never retries another route.

## Required rejection cases before implementation promotion

The caller-zero design/test seam must reject before physical mutation for:

- Recipe/JoinSig/topology owner or root-frame mismatch;
- effect-plan owner/frame mismatch;
- missing `j` declaration or first-assignment claim;
- uninitialized `j` read before its first assignment;
- duplicate or foreign source-site/binding claim;
- `ParentBodyResume` omitted or mapped to `Root.After`;
- child `j` given a parent-visible resume destination;
- missing or changed symbolic predecessor seal;
- attempted direct physical-ID/PHI/SSA authority outside the canonical session.

Late injected failures must prove shared `PhiTxn::abort_on_err`, candidate
discard, live-builder non-mutation, and fresh-candidate reuse.

Failure injection must cover first block allocation, first root PHI, child
operation, child-resume wiring, root backedge, and late function finish.

## Explicit non-claims

This design stop does not authorize:

- production `route_loop` or ordered scheduler wiring;
- Generic V0/V1 classification or Retry removal;
- all-route family adapters;
- global retirement of legacy PHI materializers;
- Recipe schema, block-argument MIR, or immutable-graph redesign;
- `.hako` selfhost physicalization;
- external candidate commit or old-edge deletion.

Those require a later winner-equivalence and caller census gate.

## Acceptance gates for the next implementation card

1. One non-`Clone` emission product preserves Recipe, JoinSig, topology,
   block projection, prefix input, and effect plan with one consuming path.
2. The effect plan covers every source declaration/use/assignment/retirement
   claim, including uninitialized `j`, without names or fabricated values.
3. The candidate-local physical block map has eleven symbolic node references,
   ten unique destinations, and explicit
   `Child.Preheader = Root.Body` alias plus distinct `ParentBodyResume`.
4. All physical edges and predecessor seals use the canonical CFG session;
   Binding SSA and PHI use exactly one function session and one shared
   transaction.
5. Focused success, pre-effect rejection, late-failure abort, candidate-drop,
   and fresh-reuse tests are green; every touched Rust/test file remains below
   800 lines.
6. P0 remains caller-zero with a preseeded binding/input fixture; P1 adds the
   resolver-issued prefix; only D5-I0 may add one production caller after the
   resolved canonical ingress can issue the complete product and preserve
   whole-function source coverage.

The implementation card may be opened only after these gates are accepted.
