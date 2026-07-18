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

The first BIN0-S0 slice is disconnected. Raw production selection, normalized
parity, and located `BinaryLeft` / `BinaryRight` acceptance belong to later
BIN0 rows.
