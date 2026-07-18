# MIR Builder operator boundary

This directory owns operator lowering after the source expression dispatcher
has selected an operator family.

## Existing semantic owners

- `converters.rs`: source operator to MIR operator classification.
- `arithmetic.rs`: arithmetic/operator-box policy, destination allocation, and
  result representation facts.
- `comparison.rs`: comparison policy, operand normalization, destination
  allocation, and Bool result facts.
- `logical_shortcircuit.rs`: conditional RHS evaluation plus CFG/PHI creation
  for `And` and `Or`.
- `unary.rs`: unary operator lowering.

## Associated-input descent

`binary_expression_descent.rs` is a child-demand boundary, not another
operator semantic owner. It may:

1. observe one borrowed source `BinaryOperator`;
2. reject `And` and `Or` before child effects because SC0 owns them;
3. request the left child and then the right child through the shared
   recursive child-lowering port;
4. finish once through the existing `build_binary_op_from_values` owner.

It must not classify MIR operators, allocate a result, infer types, construct
MIR instructions, own short-circuit control flow, or reconstruct source
locations. It is stack-scoped and is never stored in `MirBuilder`.

BIN0-I0 selects the ordinary raw source entry through one owned raw input and
the shared raw child-lowering port. `And` / `Or` are rejected by this driver
and remain selected by `logical_shortcircuit.rs` before the raw adapter is
constructed. The adapter owns no recursion guard, location, ledger, route, or
result policy. Located `BinaryLeft` / `BinaryRight` acceptance remains the
later BIN0-L0 row; production located callers remain zero.

BIN0-P0 keeps one pre-I0 orchestration reference strictly inside a `#[cfg(test)]`
module. Its snapshot compares output/error, ordered MIR and terminators,
transient types, value kinds, origins, next ValueId, and recursion depth for the
ordinary operator matrix, MethodCall children, nested trees, failures, and
reuse. The reference is parity evidence only and is never a production
fallback or second lowering route. Located acceptance still belongs to BIN0-L0.

BIN0-L0 adds one disconnected located port. Every ordinary Binary encountered
by that session uses the same driver and obtains children only through PATH0
`BinaryLeft` / `BinaryRight`. Each child independently proves an inactive
prefix for raw whole-child delegation or continues located descent to an exact
claim. The port never catches `RowsUnderPrefix` to probe for another route.
Logical operators still reject before child effects, and production located
root callers and callable-result publishers remain zero.

`short_circuit_expression_descent.rs` is the disconnected SC0-S0 child-demand
boundary. It admits only `And` / `Or`, lowers the lhs first, and gives the
existing `logical_shortcircuit.rs` owner one deferred RHS closure. That owner
invokes the closure only after entering the eval-RHS block. Branch layout,
variable snapshots, PHIs, result type, and diagnostics remain in the existing
owner. Raw and located adapters, production selectors, and result publication
are still zero through SC0-S0.

SC0-I0 adds one owned raw short-circuit input and one implementation on the
existing raw child-lowering port. The existing `MirBuilder::build_binary_op`
selector still chooses only `And` / `Or`, then delegates once to the generic
driver. Ordinary Binary remains on BIN0. The raw adapter adds no operator,
recursion, CFG, PHI, type, result, location, ledger, or fallback authority;
located adapters and callable-result publication remain zero.
