# hako-alloc-provider-activation-dry-run-unsupported-behavior-proof

Row: MIMAP-378A

This proof app exercises provider activation dry-run unsupported behavior. It
consumes one accepted explicit input bundle, records an unsupported dry-run
outcome, and verifies reject reasons for missing bundle, rejected bundle,
invalid request token, invalid activation mode, unsupported evidence, and closed
execution leakage.

Stop lines: no provider activation, no provider API call, no host allocator
replacement, no hooks, no backend matcher, no worker/TLS behavior, and no source
concurrency surface.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_dry_run_unsupported_behavior_guard.sh --level L2
```
