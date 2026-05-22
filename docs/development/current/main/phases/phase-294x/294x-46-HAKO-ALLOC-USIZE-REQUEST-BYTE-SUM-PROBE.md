---
Status: Landed
Date: 2026-05-23
Scope: proof-only page request-size and byte-sum exact `usize` probe.
Related:
  - lang/src/hako_alloc/memory/usize_field_probe_box.hako
  - apps/hako-alloc-usize-field-probe/main.hako
  - lang/src/hako_alloc/memory/NUMERIC_FIELDS.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
---

# 294x-46 Hako Alloc Usize Request Byte Sum Probe

## Decision

Extend the isolated `HakoAllocUsizeFieldProbe` with exact `usize` request-size
and byte-sum fields before migrating production page-model size/byte storage:

- `block_size`;
- `request_accept_count`;
- `request_oversize_reject_count`;
- `byte_sum_count`.

The probe accepts a request only when `requested_size <= block_size`, then
adds the accepted request size into `requested_bytes`. Oversized requests
increment a reject counter and do not mutate the byte sum.

## Stop Line

This row does not migrate production `HakoAllocPageModel.block_size` or
`HakoAllocPageModel.requested_bytes`.

It does not open page identity, queue indexes, remote-free mailbox state,
provider activation, host allocator replacement, hooks, or global allocator
integration.

## Verification

```bash
bash apps/hako-alloc-usize-field-probe/test.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
