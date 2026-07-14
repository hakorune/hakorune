# Canonical CFG edge and seal boundary

This directory owns the D-prime canonical CFG edge/seal boundary.

Responsibilities:

- emit one checked Jump or Branch terminator together with its cached
  predecessor witness;
- derive predecessor truth directly from MIR terminators;
- compare cached successors and predecessors without repairing either cache;
- freeze a block's exact predecessor set and reject every later incoming edge;
- finish only after every function block has one verified seal witness.

Non-responsibilities:

- AST, source-site, ScopeId, RegionId, binding, or control-family policy;
- PHI placement or `BindingRefV1 -> ValueId` state;
- `update_cfg()` or missing-PHI repair;
- If/Loop acceptance policy or Binding SSA reaching-value decisions.

`VerifiedPredecessorsV1` is an immutable seal witness. MIR terminators remain
the CFG SSOT; the witness only proves that the cached graph matched that SSOT
at the moment the block was sealed.

SSA-I1-T has exactly one production consumer in the admitted trivial
whole-owner lowerer. Loop and non-admitted canonical owners remain outside
this connection.
