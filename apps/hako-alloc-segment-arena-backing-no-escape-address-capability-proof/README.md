# hako-alloc-segment-arena-backing-no-escape-address-capability-proof

Row: MIMAP-244A

Purpose: prove the segment arena backing no-escape address capability inventory.

Run:

```bash
bash tools/checks/run_proof_app.sh --only MIMAP-244A
```

This proof keeps the future raw pointer boundary scalar/model-only. It records
owner/lifetime/address-carrier facts and rejects every escape or closed
substrate requirement before real pointer residence opens.
