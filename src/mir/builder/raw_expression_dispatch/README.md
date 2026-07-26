# Raw expression dispatch

This folder owns the **one** Raw AST body/statement/expression match tree.

## Authority

`build_expression_impl_with_port_v1` is the sole generic dispatcher. It owns
statement-surface classification and regular-expression dispatch, but it does
not own source navigation, call-result policy, `ValueId` publication, or
backend execution.

`build_expression_impl` is the legacy AST facade. It creates one
`RawLegacyChildLoweringPortV1` and delegates once to the generic dispatcher;
it must not grow a second matcher.

## RAW-EXPRESSION-DISPATCH-CURSOR0-I0

This series turns the dispatch inputs into a view boundary without changing
the legacy AST surface. The existing catalog-backed Raw cursor supplies exact
body/statement/expression site evidence only to the selected candidate prefix;
it must not introduce a Builder registry, AST rewalk, call writer, type
publication, route fallback, or a second AST match tree.

The physical `MethodCall` handoff remains a later row. In this folder, a
cursor may carry an exact site through structural descent, but no source
contract is paired with a call destination.
