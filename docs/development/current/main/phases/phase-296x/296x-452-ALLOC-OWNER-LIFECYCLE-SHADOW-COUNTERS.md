---
Status: Done
Date: 2026-06-06
Scope: add producer-side AllocOwner lifecycle shadow counters without opening reclaim behavior.
Blocker: MIM-FMEM-018C
Related:
  - docs/development/current/main/phases/phase-296x/296x-450-ALLOC-OWNER-LIFECYCLE-STATE-MACHINE.md
  - docs/development/current/main/phases/phase-296x/296x-451-ALLOC-OWNER-LIFECYCLE-REPORT-CHECK-FIELDS.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - tools/allocator/replacement_front_bins_templates.py
  - tools/allocator/replacement_front_bins_report_source.py
  - tools/hako_check/fastmem_capability_inventory_impl.py
---

# 296x-452 AllocOwner Lifecycle Shadow Counters

## Purpose

`MIM-FMEM-018B` made lifecycle fields and fail-fast gates visible. This row
connects the benchmark-only replacement-front producer to those fields with
shadow counters, so thread exit and abandoned-owner observations can be
checked before remote drain or abandoned reclaim behavior opens.

## Decision

The replacement-front diagnostic producer now emits lifecycle shadow counters
for:

```text
owner lifecycle/generation flags
owner activation count
thread-exit observation / flush support / flush count
ExitingFlush owner transition count
abandoned owner/page/live/empty/remote-candidate counts
reclaim-block observations
forbidden local_free / reclaim-with-remote-candidates counters
```

The owner token is represented as a packed 64-bit value with slot/generation
shape:

```text
high 32 bits: generation
low 32 bits: slot
```

The current diagnostic producer uses generation `1` and a monotonically
allocated TLS slot for owner identity. It does not reuse owner slots in this
row, so generation bump and stale-generation counters remain zero.

## Behavior Boundary

Accepted in this row:

```text
producer-side shadow evidence for lifecycle state
raw replacement-front counter input accepted by hako_check inventory
existing owner-shadow smoke extended with lifecycle shadow fixture
diagnostic baseline generated-C compile/run smoke for lifecycle counters
```

Still closed:

```text
AtomicRemoteHead drain behavior
abandoned reclaim success
owner slot reuse
product activation
hook install
global allocator claim
winner claim
full hako mimalloc algorithm claim
```

`remote_candidate_unhandled_reclaim_block_count` remains observation-only in
this row. It explains why reclaim must stay blocked; it does not itself fail
`fastmem-check`.

## Acceptance

Proof:

```bash
python3 -m py_compile \
  tools/allocator/replacement_front_bins_report_source.py \
  tools/allocator/replacement_front_bins_templates.py \
  tools/hako_check/fastmem_capability_inventory_impl.py \
  tools/hako_check/fastmem_check.py

bash tools/hako_check/fastmem_alloc_owner_shadow_counter_smoke.sh
```

Additional manual compile/run probe:

```text
generated page-bins HotCore/TLS/page-from-ptr/remote-free diagnostic baseline
emits lifecycle_state_machine=1, generation_enabled=1,
thread_exit_observed_count>0, abandoned_owner_count>0, and
page_reclaimed_with_remote_candidates=0.
```

## Stop Line

- no AtomicRemoteHead drain implementation yet
- no abandoned reclaim success
- no owner slot reuse
- no product allocator activation

Next row:

```text
MIM-FMEM-019 AtomicRemoteHead drain
```
