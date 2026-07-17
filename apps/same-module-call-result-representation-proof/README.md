# same-module-call-result-representation-proof

This disconnected S0 fixture records when an ordinary same-module static call
result becomes available in the lowering-time type context.

It deliberately keeps the forward callee untyped and after its caller. The
reverse-order and explicitly typed cases are controls only. The fixture does
not authorize a result-representation producer, reorder declarations, add a
fallback, or change GenericLoop.

Run the direct checker through:

```bash
bash apps/same-module-call-result-representation-proof/test.sh
```

