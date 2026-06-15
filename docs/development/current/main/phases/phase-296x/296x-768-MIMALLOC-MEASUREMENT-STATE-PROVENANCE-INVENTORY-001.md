---
Status: Landed
Date: 2026-06-15
Task: MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001
Scope: Compare the row753 79.586x measurement with current canonical
  direct-exact measurements and classify whether the old large gap is stale,
  mismatched, or still unexplained before reopening optimization.
Related:
  - docs/development/current/main/phases/phase-296x/296x-767-MIMALLOC-MEASUREMENT-HYGIENE-REFRESH-001.md
  - docs/development/current/main/phases/phase-296x/296x-766-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-CALL-OPERAND-CLOSEOUT-001.md
  - docs/development/current/main/phases/phase-296x/296x-753-MIMALLOC-BODY-TIMING-REBASELINE-AFTER-LOCAL-SSA-NONKEEPER-001.md
  - docs/development/current/main/design/perf-owner-first-optimization-ssot.md
---

# MIMALLOC-MEASUREMENT-STATE-PROVENANCE-INVENTORY-001

## Result

```text
output_contract=hako-mimalloc-measurement-state-provenance-inventory-v0
source_evidence=296x-767,296x-766,296x-753
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
row753_measurement_tmpdir=/tmp/hakorune_row753_rebaseline.52TJFW
row767_measurement_tmpdir=/tmp/hakorune_row767_hygiene.ED76rn
provenance_probe_tmpdir=/tmp/hakorune_row768_provenance_probe.SLXrTE
row753_runtime_config_profile=root
row753_hako_body_elapsed_ns=374000000
row753_c_body_elapsed_ns=4699328
row753_body_elapsed_ratio=79.586
row753_hako_external_elapsed_ms=400
row753_external_peak_rss_bytes=9482240
row753_canonical_env_contract_recorded=0
row753_worker_front_mismatch_guard_recorded=0
row767_measurement_profile=canonical_direct_exact_pair_v0
row767_body_elapsed_ratio_median=2.119
row767_body_elapsed_ratio_max=2.363
row767_previous_outlier_reproduced=0
root_reprobe_hako_body_elapsed_ns=7000000
root_reprobe_external_elapsed_ms=10
root_reprobe_external_peak_rss_bytes=3588096
empty_reprobe_hako_body_elapsed_ns=6000000
empty_reprobe_external_elapsed_ms=10
empty_reprobe_external_peak_rss_bytes=3530752
runtime_config_root_reproduces_outlier=0
runtime_config_mismatch_explains_outlier=0
row753_small_alloc_block_count=21
current_small_alloc_block_count=21
row753_small_alloc_inst_count=157
current_small_alloc_inst_count=157
row753_small_alloc_copy_count=51
current_small_alloc_copy_count=51
row753_small_alloc_call_count=19
current_small_alloc_call_count=19
mir_shape_count_mismatch=0
old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier
old_large_gap_allowed_as_optimization_owner=0
current_reliable_body_ratio_floor=about_2x
fresh_high_confidence_implementation_owner_selected=0
selected_owner=none
selected_owner_reason=old_large_gap_not_reproduced_and_current_2x_gap_needs_boundary_inventory
selected_next_action=runtime_boundary_inventory_for_current_2x_gap
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

Existing row753 artifacts still exist and show the old outlier:

```text
row753_hako_body_elapsed_ns=374000000
row753_hako_external_elapsed_ms=400
row753_external_peak_rss_bytes=9482240
row753_runtime_config_profile=root
```

The current canonical pair records the direct-exact environment explicitly and
repeats near the 2x body ratio floor:

```text
body_elapsed_ratio_median=2.119
body_elapsed_ratio_max=2.363
previous_outlier_reproduced=0
```

A direct root/empty reprobe using the current runner did not reproduce the old
root-config outlier:

```text
root_reprobe_hako_body_elapsed_ns=7000000
root_reprobe_external_elapsed_ms=10
empty_reprobe_hako_body_elapsed_ns=6000000
empty_reprobe_external_elapsed_ms=10
runtime_config_root_reproduces_outlier=0
```

MIR count-level shape is the same for the selected method:

```text
row753_small_alloc_block_count=21
current_small_alloc_block_count=21
row753_small_alloc_inst_count=157
current_small_alloc_inst_count=157
row753_small_alloc_copy_count=51
current_small_alloc_copy_count=51
row753_small_alloc_call_count=19
current_small_alloc_call_count=19
```

## Decision

The 79.586x row753 body ratio must not drive another compiler-lowering
implementation. It is not reproduced by either current canonical direct-exact
pair measurement or a current root-config reprobe. The safest classification is:

```text
old_large_gap_classification=stale_or_transient_hako_runner_measurement_outlier
old_large_gap_allowed_as_optimization_owner=0
```

The active optimization question is now the current stable-ish 2x body gap, not
the stale 79.586x gap. That is a design boundary: before implementation, the
lane should inventory runtime/object/generated-runtime boundary costs under the
current canonical measurement floor.

## Stop Line

```text
do not implement from the stale 79.586x gap
do not reopen LocalSSA, PHI-edge, block-entry, receiver, or arg forwarding
do not add broad copy coalescing
do not special-case source names, helper names, or benchmark names
do not move Box/Object management into MIRBuilder
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
MIMALLOC-RUNTIME-BOUNDARY-INVENTORY-FOR-CURRENT-2X-GAP-001:
  inventory runtime/object/generated-runtime boundary costs under the current
  canonical direct-exact pair, treat the stale 79.586x gap as closed evidence,
  and select no implementation owner unless a fresh high-confidence boundary
  owner appears
```
