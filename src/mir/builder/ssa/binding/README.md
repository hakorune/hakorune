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
owned rollback operations and poisons the instance; the enclosing unpublished
function transaction must then be discarded.

SSA-S1 keeps this module disconnected from production. Production activation
must be one whole-function owner cutover; no old-map synchronization bridge is
allowed.
