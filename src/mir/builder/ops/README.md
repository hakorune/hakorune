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
