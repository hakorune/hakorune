# hako-alloc-host-replacement-explicit-preflight-inventory-proof

Row: MIMAP-420A

This proof app validates the host replacement explicit preflight inventory row.
It consumes real external provider API call first-pattern evidence and records
the explicit request, hook-plan, rollback-plan, and backend no-growth inputs
that would be required before any later optional host replacement row.

Run:

```bash
bash tools/checks/k2_wide_hako_alloc_host_replacement_explicit_preflight_inventory_guard.sh --level L2
```

Stop lines:

- no hook installation
- no backend matcher additions
- no process allocator replacement
- no `#[global_allocator]`
- no worker/thread execution
