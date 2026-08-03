# JOINIR-LOOP-NESTED-PREDICATE-D4-PHYSICAL-TOPOLOGY

Status: source-handoff split and caller-zero symbolic topology issuer landed;
physical emission remains a separate design stop.
Date: 2026-08-04

## Decision

Before any Nested Predicate physicalizer or production caller is added, seal
one source-bound physical topology for the bounded two-loop shape:

```text
VerifiedNestedLoopSourceProjectionV1
  + VerifiedLoopRecipeV1 / VerifiedLoopJoinSigV1
  -> VerifiedNestedPhysicalTopologyV1
  -> later canonical-session physical adapter
```

`LoopJoinSig` is a logical dataflow contract. It must not be used to infer
physical blocks, predecessor identities, or PHI destinations.

## Linear source-handoff correction

`VerifiedNestedLoopSourceProjectionV1` is non-`Clone` and D2-D consumes it by
value while producing the Recipe and JoinSig. Therefore the topology issuer
must not accept a second projection, reread the source, or rebuild source
identity from the Recipe. D2-D must split one source projection into one
semantic product and one non-`Clone` physical handoff before either consumer
continues:

```text
VerifiedNestedLoopSourceProjectionV1 (consume once)
  -> VerifiedNestedPredicateRecipeProductV1
       { VerifiedLoopRecipeV1, VerifiedLoopJoinSigV1,
         VerifiedNestedPhysicalSourceHandoffV1 }
  -> D4 topology issuer consumes the product/handoff
```

`VerifiedNestedPhysicalSourceHandoffV1` is source-bound evidence only. Its
minimum fields are the resolver owner, root frame key, root/child statement
sites, the three resolver `BindingRefV1`/`ScopeId` pairs, recurrence-owner and
parent-visibility flags, plus the condition/update role sites needed to bind
topology roles. It contains no AST, source-name lookup, Recipe/JoinSig key
authority, `BasicBlockId`, `ValueId`, PHI/SSA state, or Builder reference.

This correction is a prerequisite design slice, not a production connection:
the D4 issuer remains caller-zero and consumes the handoff exactly once.

The handoff is split from the semantic view by a consuming method such as
`into_topology_input()`. The ordinary `recipe()`/`join_sig()` accessors remain
source-free. The handoff keeps the resolver `FunctionOwnerIdV1` from the
binding evidence and the existing root `LoopExecutionFrameKeyV1`; it does not
reconstruct either from portable source ordinals. A child frame is not
required by this bounded slice. Requiring one is a separate D2-C1b capability,
not a fabricated child-site token.

## Required topology evidence

The topology product must be non-Clone and carry resolver/source-bound
identity, not labels or ordinal guesses. It must seal:

- root preheader/header/body/step/after ports;
- child preheader/header/body/step/after ports;
- the child `After -> parent body resume` continuation edge;
- Standard5 edge roles and predecessor-seal witnesses for both predicates;
- carrier destinations for root `i`/`sum` and child `j`, including the lexical
  scope distinction that `j` is declared in the outer loop body but recurs in
  the child loop;
- owner/frame equality with the source projection and the canonical function
  session that will eventually consume it.

The topology issuer consumes the already split source handoff and existing
canonical role vocabulary, but it must not reread AST or legacy facts,
synthesize root input constants, or map by source name.

The issuer's physical-independent topology vocabulary is also explicit:

| scope | ports | required named continuation/expansion |
| --- | --- | --- |
| root | Preheader/Header/Body/Step/After | root Standard5 edges |
| child | Preheader/Header/Body/Step/After | child Preheader aliases the root Body port; alias is explicit, not inferred |
| nested resume | ParentBodyResume(root, child) | child After -> ParentBodyResume -> root Step -> root Header |

The logical JoinSig child `Body -> Header` row is not copied as a physical
edge. It is expanded into the child subloop, child After -> parent resume,
parent resume -> root Step, and root Step -> root Header. Predecessor seals
must name exact topology edges/ports, never physical IDs. Carrier destinations
must keep root `i`/`sum` visible through the named `ParentBodyResume` forwarding
port and child `j` local to the child recurrence; `j` must not be emitted into
the parent tail. `Root.After` is the outer exit publication port, not the child
normal-resume destination.

The source-role mapping is sealed before topology construction: root
initializer `i`/`sum` claims root entry, root predicate claims root Header,
root body/child entry claims root Body (and its explicit child Preheader alias),
child predicate claims child Header, child updates claim child body/step roles,
root `i` update claims root Step, and root predicate-false claims root After.
The symbolic predecessor seals use only these named port/edge references. A
later canonical adapter will compare the topology owner/frame with the
`CanonicalSsaFunctionSessionV2` owner/frame and issue physical predecessor
proofs; D4 itself never borrows that session.

## Explicitly out of scope

- no Nested Predicate `MirBuilder` mutation, PHI/SSA writer, physicalizer, or
  production route caller;
- no route selection, Retry/Option fallback, Generic, JoinIR scheduler, or
  legacy composer/CorePlan/PlanLowerer integration;
- no promotion of `LoopPhiMaterializerV1`, `phi_input_materializer`, or route
  local materializers;
- no Recipe schema or whole-MIR/block-argument redesign.

## Landed caller-zero slice

The caller-zero issuer is now present in
`src/mir/compiler/nested_predicate_topology.rs` and is covered by four focused
tests in `nested_predicate_topology_tests.rs`. The product is non-`Clone`,
contains symbolic ports/edges/expansions/seals/carrier destinations, and has
zero production consumers. It does not allocate physical IDs, borrow a
canonical session, or write PHI/SSA state. The issuer consumes the one-time
`VerifiedNestedPhysicalSourceHandoffV1` through `into_topology_input()`.

The focused structural slice proves the ten ports, eleven edges, explicit
child-preheader alias, child-after parent resume, expanded root backedge,
carrier visibility (`i`/`sum` parent-visible through `ParentBodyResume` and `j`
child-local), source-role bindings, and non-empty predecessor seals.
Owner/frame comparison with
`CanonicalSsaFunctionSessionV2`, exact missing-predecessor rejection, and
physical predecessor/PHI emission remain the next adapter's responsibility;
D4 does not claim those proofs early.

## Acceptance gates for the design stop

1. A single topology table maps every source/JoinSig role to one physical port,
   edge role, predecessor witness, and carrier destination.
2. Focused fixtures cover child normal resume, the expanded root backedge,
   symbolic predecessor non-emptiness, and illegal parent-tail use of `j`.
   Missing-predecessor rejection and owner/frame mismatch are deferred to the
   canonical-session adapter because D4 intentionally has no physical-session
   authority.
3. The topology product has zero production consumers and no physical IDs;
   it is only a sealed input for the later canonical-session adapter.
4. Existing D2-B/D2-C/D2-D, DirectAccum pilot, PHI lifecycle, and scope gates
   remain green; all touched files stay below 800 lines.

After this handoff slice lands, the caller-zero topology issuer is complete.
The next frontier is the design stop in
`joinir-loop-nested-predicate-d4-physical-emission-design-2026-08-04.md`:
preserve the Recipe/JoinSig pair, issue exact binding/prefix claims, map
symbolic ports to candidate-local physical blocks, and consume the existing
CanonicalSsaFunctionSessionV2 -> CanonicalCfgSessionV1 + BindingSsaBuilderV1 +
PhiTxn chain without introducing another authority.
