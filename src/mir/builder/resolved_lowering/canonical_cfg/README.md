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

Dynamic Loop P2A adds one migration-private caller-zero consumer. The one
canonical function session allocates distinct Enter, Header, body,
terminal-Backedge, and After roles and emits only the Enter-to-Header edge.
It does not expose a raw predecessor list or authorize PHI patching. P2B now
uses this same owner to emit the Header branch, body-to-terminal edge, and
terminal Backedge-to-Header edge. It seals Enter, body, terminal Backedge, and
Header; only the resulting exact Header witness authorizes canonical Binding
SSA to complete the already-open PHI. After remains open for its later owner.

P2C proves that failures before or after these seals never authorize local
CFG repair or same-session retry. The complete unpublished function session
is discarded and the caller context is restored once; a fresh session may
repeat the same semantic shape independently of numeric block IDs.
