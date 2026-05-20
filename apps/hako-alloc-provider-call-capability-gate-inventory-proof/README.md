# hako-alloc-provider-call-capability-gate-inventory-proof

Row: MIMAP-382A

This proof app exercises the provider-call capability gate inventory after the
provider activation modeled-open pilot. It proves that provider calls remain
closed even when the modeled activation state is open and the explicit
provider-call capability gate is present.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_provider_call_capability_gate_inventory_guard.sh --level L2
```
