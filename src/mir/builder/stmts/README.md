# MIR Builder statement boundary

This directory owns source-statement orchestration after the expression
dispatcher has selected a statement family.

## Existing semantic owners

- `variable_stmt.rs`: Local declaration preflight, specialized typed-array and
  record initializer lowering, binding publication, LocalSlot contracts, and
  result metadata.
- `return_stmt.rs`: Return preflight, cleanup, contracts, and emission.
- `block_stmt.rs` / `block_driver.rs`: statement order, scope, termination, and
  the existing suffix-router boundary.

## Local associated-input descent

`local_statement_descent.rs` is a child-demand boundary. It may:

1. borrow the Local declaration syntax once;
2. run the existing whole-declaration exact-numeric/typed-array preflight
   before initializer effects;
3. request ordinary initializer expressions in declaration order through the
   shared recursive child-lowering port;
4. request the existing typed-array or record initializer owner through an
   explicit specialized hook;
5. publish bindings once through the existing from-values completion owner
   after every initializer succeeds.

The specialized hooks are not permission to bypass located coverage. A future
located port must first prove the exact `LocalInitializer(index)` subtree
inactive before invoking either hook. Active array elements or record
arguments require their own associated-input row and fail closed in LCL0.

This boundary must not reconstruct source sites, own a caller ledger, infer
types, reimplement typed-array or record semantics, publish a binding before
all initializer values exist, or store its port/input in `MirBuilder`.

LCL0-I0 selects the owned raw Local input through the existing
`variable_stmt::build_local_statement` facade. That facade has one consumer of
the raw adapter; the old initializer loop is retired. The shared driver keeps
the existing preflight-success debug observation in its original position,
before initializer effects.

LCL0-P0 keeps one `cfg(test)` pre-I0 orchestration reference. Selected and
reference paths compare exact results plus normalized MIR, transient type and
origin facts, bindings/scopes, specialized array/record contracts, slot state,
allocator counters, partial failure state, and same-Builder reuse. The
reference must not call the selected driver or either descent port.

Located Local acceptance remains disconnected until LCL0-L0. Its port must
prove each exact `LocalInitializer(index)` subtree active or inactive before
delegating; P0 does not authorize a located implementation or selector.
