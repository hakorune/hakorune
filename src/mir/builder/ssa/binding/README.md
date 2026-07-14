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

`read` defines a provisional PHI before recursive predecessor reads. `seal`
completes open-block PHIs from the exact witness. Any PHI failure attempts all
still-pending provisional rollback operations and poisons the instance. A PHI
that was already patched is part of the poisoned unpublished draft and is not
individually undone; the enclosing function transaction discards the whole
draft.

SSA-M0 adds one borrowed real-MIR adapter over `MirBuilder` and `PhiTxn`.
It is mechanical only: allocation, provisional definition, exact input patch,
dominance/reachability verification, and pending rollback. Open and patched
PHIs both retain `MirType::Unknown`; the accepted fact-refinement set is empty.
Every predecessor set comes from `CanonicalCfgSessionV1`, including Return
blocks. The adapter has no production caller.

SSA-S1 keeps this module disconnected from production. Production activation
must be one whole-function owner cutover; no old-map synchronization bridge is
allowed.
