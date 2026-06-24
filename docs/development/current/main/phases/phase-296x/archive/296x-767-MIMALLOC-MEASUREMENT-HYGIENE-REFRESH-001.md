---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001
Scope: Re-run the canonical direct-exact body timing pair repeatedly after
  296x-766 selected measurement hygiene, then decide whether the old 79.586x
  body gap is reproducible enough to justify implementation.
Related:
  - docs/development/current/main/phases/phase-296x/296x-766-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001

## Result

```text
output_contract=hako-mimalloc-measurement-hygiene-refresh-v0
source_evidence=296x-766,296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
measurement_tmpdir=/tmp/hakorune_row767_hygiene.ED76rn
measurement_profile=canonical_direct_exact_pair_v0
sample_count=5
sample_0_hako_body_elapsed_ns=7000000
sample_0_c_body_elapsed_ns=3290003
sample_0_body_elapsed_ratio=2.128
sample_1_hako_body_elapsed_ns=6000000
sample_1_c_body_elapsed_ns=3321992
sample_1_body_elapsed_ratio=1.806
sample_2_hako_body_elapsed_ns=6000000
sample_2_c_body_elapsed_ns=3996764
sample_2_body_elapsed_ratio=1.501
sample_3_hako_body_elapsed_ns=7000000
sample_3_c_body_elapsed_ns=3302945
sample_3_body_elapsed_ratio=2.119
sample_4_hako_body_elapsed_ns=8000000
sample_4_c_body_elapsed_ns=3386232
sample_4_body_elapsed_ratio=2.363
hako_body_elapsed_ns_min=6000000
hako_body_elapsed_ns_median=7000000
hako_body_elapsed_ns_max=8000000
c_body_elapsed_ns_min=3290003
c_body_elapsed_ns_median=3321992
c_body_elapsed_ns_max=3996764
body_elapsed_ratio_min=1.501
body_elapsed_ratio_median=2.119
body_elapsed_ratio_max=2.363
previous_outlier_body_elapsed_ratio=79.586
previous_outlier_reproduced=0
refreshed_gap_owner=measurement_harness
refreshed_gap_confidence=low
measurement_state_drift_detected=1
fresh_high_confidence_implementation_owner_selected=0
selected_owner=none
selected_owner_reason=previous_large_body_gap_not_reproduced_under_canonical_direct_exact_pair
selected_next_action=measurement_state_provenance_inventory
implementation_allowed=0
measurement_required=1
winner_claim=0
startup_lane_reopened=0
source_hako_changed=0
mirbuilder_object_management_enabled=0
product_default_changed=0
provider_active=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Evidence

Command shape:

```bash
for i in 0 1 2 3 4; do
  bash tools/allocator/hako_mimalloc_direct_exact_pair.sh \
    --out /tmp/hakorune_row767_hygiene.ED76rn/pair.$i.out
  awk -F= ... /tmp/hakorune_row767_hygiene.ED76rn/pair.$i.out
done
```

The current canonical direct-exact pair uses:

```text
direct_exact_env_contract=mimalloc-direct-exact-env-v0
NYASH_FEATURES=rune
NYASH_DISABLE_PLUGINS=1
NYASH_SKIP_TOML_ENV=1
NYASH_GC_MODE=off
NYASH_SCHED_POLL_IN_SAFEPOINT=0
HAKO_TYPED_OBJECT_STORE=direct_slot_exact
HAKO_ARRAY_SLOT_STORE=direct_array_i64_exact
worker_front_mismatch_guard=1
```

## Decision

Do not open an optimization implementation row. The old 79.586x body gap from
296x-753 does not reproduce under the current canonical direct-exact pair:

```text
previous_outlier_body_elapsed_ratio=79.586
body_elapsed_ratio_median=2.119
body_elapsed_ratio_max=2.363
previous_outlier_reproduced=0
```

This is a measurement-state problem before it is an optimization-owner problem.
The next row must inventory the provenance difference between row 753 and the
current canonical direct-exact wrapper: command path, generated EXE inputs,
environment contract, temp artifacts, and whether stale/mismatched measurement
state was captured in row 753.

## Stop Line

```text
do not implement from this hygiene row
do not reopen LocalSSA, PHI-edge, block-entry, receiver, or arg forwarding
do not add broad copy coalescing
do not special-case source names, helper names, or benchmark names
do not move Box/Object management into MIRBuilder
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001:
  compare row753 and current canonical direct-exact measurement provenance,
  classify the 79.586x body ratio as stale, mismatched, or still unexplained,
  and only then decide whether a fresh optimization owner can reopen
```
