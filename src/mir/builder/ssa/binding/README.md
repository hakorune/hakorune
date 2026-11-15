# Binding SSA

This box owns function-local `(BasicBlockId, BindingRefV1) -> ValueId`
reaching definitions and sealed-block PHI construction.

Allowed inputs:

- one `FunctionOwnerIdV1` fixed at construction;
- `BindingRefV1`, `BasicBlockId`, and `ValueId`;
- immutable `VerifiedPredecessorsV1` witnesses from canonical CFG;
- a narrow PHI lifecycle adapter.

Forbidden inputs and decisions:

- AST, Span, source sites, names, ScopeId, RegionId, or RegionFlow;
- If/Loop/exit policy and effect/carrier discovery;
- CFG repair or predecessor recomputation;
- type/representation inference from an incomplete PHI.
- Recipe policy or `LoopBindingKeyV1` issuance. Binding SSA consumes resolved
  `BindingRefV1` identities and owns only their physical `ValueId`/PHI
  realization.

`read` defines a provisional PHI before recursive predecessor reads. `seal`
completes open-block PHIs from the exact witness. Any PHI failure attempts all
still-pending provisional rollback operations and poisons the instance. A PHI
that was already patched is part of the poisoned unpublished draft and is not
individually undone; the enclosing function transaction discards the whole
draft.

SSA-M0 added one borrowed real-MIR adapter over `MirBuilder` and `PhiTxn`.
It is mechanical only: allocation, provisional definition, exact input patch,
dominance/reachability verification, and pending rollback. Open PHIs retain
`MirType::Unknown`. Successful patch completion delegates exact type publication
to the canonical PHI lifecycle owner, which checks the complete logical inputs;
the adapter does not infer types or use a PHI hint as authority.
Every predecessor set comes from `CanonicalCfgSessionV1`, including Return
blocks.

SSA-I1-T connects the adapter to one admitted trivial whole-function route.
That route has exactly one `BindingSsaBuilderV1`; no old-map synchronization
bridge is allowed. Non-admitted current canonical owners remain a separately
selected whole-unit A+ route and never retry after a Binding-SSA failure.

Current real-MIR adapter tests cover provisional PHI creation before exposure,
exact predecessor completion and type publication, rejected non-dominating
inputs, and late patch failure without draft repair. `phi_lifecycle_tests`
separately covers transaction rollback and cleanup failure retention.
