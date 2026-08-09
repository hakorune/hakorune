# Dynamic carrier contract

This module owns the neutral lifecycle vocabulary shared by Dynamic invocation
and Dynamic operators.

```text
DynamicCarrierLifecycleObligationV1
  EndExactlyOnceUnlessForwarded
```

It does not decide whether an operation publishes a carrier. Invocation and
operator semantic envelopes own that Normal/Fault decision separately and may
only borrow this vocabulary.

This module does not own Home classification, runtime payload tags, provider
selection, cleanup placement, physical release/end mechanics, retry, or
fallback.
