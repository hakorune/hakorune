# hako-alloc-provider-activation-input-bundle-inventory-proof

Row: MIMAP-376A

This proof app exercises the provider activation input bundle inventory owner.
It consumes an accepted unsupported-outcome report plus explicit request token
and activation mode inputs, records one accepted bundle, and verifies reject
reasons for missing outcome, rejected outcome, invalid provider candidate,
invalid provider kind, invalid request token, invalid activation mode,
unsupported evidence, and closed execution leakage.

Stop lines: no provider activation, no provider API call, no host allocator
replacement, no hooks, no backend matcher, no worker/TLS behavior, and no source
concurrency surface.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_activation_input_bundle_inventory_guard.sh --level L2
```
