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

The sole raw/default `ASTNode::BinaryOp` selector lives in
`raw_expression_dispatch`. It creates one owned input and enters exactly one
generic owner:

```text
And / Or
  -> RawLegacyShortCircuitInputV1
  -> drive_short_circuit_expression_v1

all remaining operators
  -> RawLegacyBinaryInputV1
  -> drive_ordinary_binary_expression_v1
```

The partition is total and disjoint. There is no intermediate raw facade,
fallback, retry, or route reselection.

BIN0-P0 keeps one pre-I0 orchestration reference strictly inside a `#[cfg(test)]`
module. Its snapshot compares output/error, ordered MIR and terminators,
transient types, value kinds, origins, next ValueId, and recursion depth for the
ordinary operator matrix, MethodCall children, nested trees, failures, and
reuse. The reference is parity evidence only and is never a production
fallback or second lowering route. Located acceptance still belongs to BIN0-L0.

The detached located port uses the same two drivers and obtains children only
through PATH0 `BinaryLeft` / `BinaryRight`. Each child independently proves an
inactive prefix for raw whole-child delegation or continues located descent to
an exact claim. The port never catches `RowsUnderPrefix` to probe for another
route. Production located root callers and callable-result publishers remain
zero.

`short_circuit_expression_descent.rs` admits only `And` / `Or`, lowers the lhs
first, and gives the existing `logical_shortcircuit.rs` owner one deferred RHS closure.
That owner invokes the closure only after entering the eval-RHS block. Branch
layout, variable snapshots, PHIs, result type, and diagnostics remain in the
existing owner.

SC0-P0 retains the pre-I0 raw orchestration only as one `#[cfg(test)]`
reference. Fresh selected/reference Builders compare result or error, ordered
blocks and terminators, transient types, value kinds and origins, variable and
pin maps, current block, next ValueId, and recursion depth. The reference is
not a production selector or fallback. Located adapters and result publication
remain zero.

For short-circuit located descent, PATH0 remains the only `BinaryLeft` /
`BinaryRight` source; the lhs is demanded first and the rhs location is
requested only by the deferred closure inside the eval-RHS block. The adapter
adds no source rewalk, ledger probing, CFG/PHI/result authority, production
root caller, or fallback.
