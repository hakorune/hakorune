# hako-alloc-segment-arena-backing-no-escape-address-capability-diagnostics-proof

Row: MIMAP-245A

Purpose: prove observer-only diagnostics for the segment arena backing
no-escape address capability inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-245A
```

This proof observes MIMAP-244A inventory counters and last report facts. It
does not record new capability rows from the diagnostic owner or open real
pointer residence.
