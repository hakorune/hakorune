# PHI completion vocabulary

This module owns the disconnected PHI0/PRED0 vocabulary for two distinct
PHI claims:

```text
PhiDraftV1
  -> generic input/type completion
  -> reuse phi_type_publication's prepared type decision
  -> PreparedPhiCompletionV1
  -> instruction success
  -> CompletedPhiV1

route-owned CFG witness
  -> exact predecessor/input row validation
  -> prepare_cfg_ready
```

`prepare_input_completion` is deliberately not CFG-ready: an unsealed
provisional patch can lawfully receive rows before the surrounding CFG exposes
its predecessor set. A non-Clone `CfgReadyPhiRowsV1` keeps exact expected and
incoming rows inseparable before `prepare_cfg_ready` may consume them. Its
constructor remains private in S0; a future CFGREADY0 route must expose only a
route-specific sealed witness, never generic raw rows.

The module does not own `MirBuilder`, `MirFunction`, `TypeContext`, CFG
reachability, dominance, input rematerialization, origin propagation, or raw
fact-map writes. Production consumers remain zero in PRED0-S0. A later PHI0-I0
may connect raw, final, patch, and batch only to generic input/type completion;
route-owned CFG-ready activation remains separate.

`PhiTransientTypeDecisionV1` remains the sole type-decision authority.
