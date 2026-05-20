# hako-alloc-provider-call-noop-execution-seam-pilot-proof

Row: MIMAP-390A

This proof app exercises the provider-call no-op execution seam after the
execution capability preflight. It proves that the explicit execution boundary
can be crossed while actual provider API execution remains closed.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_noop_execution_seam_pilot_guard.sh --level L2
```
