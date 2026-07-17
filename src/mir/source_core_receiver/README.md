# Source Core receiver proof

This module owns one disconnected source-only representation proof for a
future Core-method call site. Its first grammar is deliberately narrow:

```text
StringReceiver ::= StringLiteral
                 | Add(left = StringReceiver, right = AnySourceExpression)
```

`ExactStringOnSuccess` means only that, if evaluation produces a value, that
value is a String. It does not prove that evaluation succeeds, terminates, is
pure, or returns a value. In particular, the right operand is not inspected:
its evaluation and effects remain owned by the ordinary source/lowering
semantics.

The verifier walks only the left `Add` spine with an iterative cursor. It does
not infer String variables, call results, right-String additions, `BlockExpr`
results, or Builder/MIR/runtime types. The proof borrows the exact source
expression for its lifetime and publishes no persistent expression catalog.

This S0 module has no production producer or consumer. A later co-seal may
invoke it on the exact receiver expression while combining independently
verified call-target and Core-result rows.
