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

LCL0-L0 adds one disconnected located Local selector. Ordinary initializers
are requested only through the existing `LocalInitializer(index)` source role,
so expression descent retains the exact site and ledger order. Typed-array and
record hooks require exact inactive-subtree proof before specialized effects;
an active element or constructor argument fails closed. The adapter does not
catch `RowsUnderPrefix` to probe a route, reconstruct a site, or publish a
production located root caller.

The canonical source grammar currently admits at most one initialized binding
per `Local`, so its reachable located role is `LocalInitializer(0)`. The shared
driver deliberately keeps an indexed associated-input contract; its synthetic
port fixture fixes `LocalInitializer(0)` then `LocalInitializer(1)` ordering.
Located LCL0 binds every requested index to the exact source role, but does not
invent a malformed multi-initializer source fixture to widen the parser law.

## Variable-target Assignment associated-input descent

`variable_assignment_descent.rs` owns one disconnected exact Variable-target
Assignment boundary. Its input carries only the already-selected variable name
and RHS; field/index target syntax is structurally absent. It observes the name
once and runs the existing declared-binding preflight before requesting the
RHS. The RHS is lowered once through the shared recursive expression port,
then the existing `build_assignment_from_value` owner repeats the declaration
check and performs contracts, ownership effects, and binding publication.

ASN0-S0 must not inspect or reconstruct target AST, admit field, index,
compound, or grouped assignment targets, inspect callable-result rows, or
publish a binding itself. An undeclared target rejects before RHS input or
effects, and an RHS failure leaves the previous assignment binding unchanged.
The second completion-time declaration check is retained.

ASN0-I0 selects this raw driver exactly once from the exact Variable branch of
the existing `exprs.rs` target selector. Field/index targets and compound
assignment retain their existing owners. `GroupedAssignmentExpr` remains
parked on a dedicated legacy facade because sharing its old facade would widen
ASN0 indirectly. The raw adapter does not inspect the target AST or add a
second selector. The parity reference and located `AssignmentValue` navigation
remain disconnected at I0.

ASN0-P0 retains the pre-I0 exact Variable orchestration only in a cfg(test)
reference: declared-binding preflight, raw RHS lowering, then the existing
from-value completion. Selected and reference paths compare result/error,
ordered MIR, transient facts, bindings/scopes, local and typed-array contracts,
slot/cached state, allocators, failure effects, and same-Builder reuse. The
reference rejects Grouped, field, index, and compound surfaces and has no
production caller. Located `AssignmentValue` navigation remains disconnected
until ASN0-L0.
