# JOINIR-LOOP-NESTED-PREDICATE-D4-PHYSICAL-TOPOLOGY

Status: design stop opened after the DirectAccum canonical pilot closeout.
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

The topology issuer may observe the already sealed source projection and
existing canonical role vocabulary, but it must not reread AST or legacy
facts, synthesize root input constants, or map by source name.

## Explicitly out of scope

- no Nested Predicate `MirBuilder` mutation, PHI/SSA writer, physicalizer, or
  production route caller;
- no route selection, Retry/Option fallback, Generic, JoinIR scheduler, or
  legacy composer/CorePlan/PlanLowerer integration;
- no promotion of `LoopPhiMaterializerV1`, `phi_input_materializer`, or route
  local materializers;
- no Recipe schema or whole-MIR/block-argument redesign.

## Acceptance gates for the design stop

1. A single topology table maps every source/JoinSig role to one physical port,
   edge role, predecessor witness, and carrier destination.
2. Positive and negative fixtures cover child normal resume, missing
   predecessor, owner/frame mismatch, and illegal parent-tail use of `j`.
3. The topology product has zero production consumers and no physical IDs;
   it is only a sealed input for the later canonical-session adapter.
4. Existing D2-B/D2-C/D2-D, DirectAccum pilot, PHI lifecycle, and scope gates
   remain green; all touched files stay below 800 lines.

After this card closes, the next implementation is a caller-zero topology
issuer only. Physical emission remains a separate card requiring the existing
CanonicalSsaFunctionSessionV2 -> CanonicalCfgSessionV1 + BindingSsaBuilderV1 +
PhiTxn chain.
