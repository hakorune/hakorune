# hako-alloc-provider-call-dry-run-unsupported-behavior-proof

Row: MIMAP-384A

This proof app exercises the provider-call dry-run unsupported behavior after
the provider-call capability gate inventory. It proves that the dry-run records
an unsupported provider-call outcome without executing provider APIs.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_dry_run_unsupported_behavior_guard.sh --level L2
```
