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

The first LCL0-S0 slice is disconnected. Raw selection, normalized parity,
and located `LocalInitializer(index)` acceptance belong to later LCL0 rows.
