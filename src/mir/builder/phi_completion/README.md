# PHI completion vocabulary

This module owns the disconnected PHI0-S0 vocabulary for one semantic PHI
completion transaction:

```text
PhiDraftV1
  -> validate logical predecessor/input rows
  -> reuse phi_type_publication's prepared type decision
  -> PreparedPhiCompletionV1
  -> instruction success
  -> CompletedPhiV1
```

It does not own `MirBuilder`, `MirFunction`, `TypeContext`, CFG reachability,
dominance, input rematerialization, origin propagation, or raw fact-map
writes. Production consumers remain zero in S0. The future PHI0-I0 owner may
connect raw, final, patch, and batch Builder facades only after their timing
and failure matrix is sealed.

`PhiTransientTypeDecisionV1` remains the sole type-decision authority.
