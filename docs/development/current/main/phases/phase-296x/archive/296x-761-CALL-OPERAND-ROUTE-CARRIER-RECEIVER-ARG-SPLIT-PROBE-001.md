---
Status: Landed
Date: 2026-06-15
Task: CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001
Scope: Extend the current local-SSA copy-position probe to split call-operand
  route-carrier residue into receiver and arg surfaces, then select the next
  non-implementation owner row.
Related:
  - docs/development/current/main/phases/phase-296x/296x-760-CALL-OPERAND-ROUTE-CARRIER-REVALIDATION-GUARD-SURFACE-001.md
  - tools/allocator/mir_local_ssa_copy_position_probe.py
---

# CALL-OPERAND-ROUTE-CARRIER-RECEIVER-ARG-SPLIT-PROBE-001

## Result

```text
output_contract=hako-mimalloc-call-operand-route-carrier-receiver-arg-split-probe-v0
source_evidence=296x-760
probe_tool=tools/allocator/mir_local_ssa_copy_position_probe.py
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1
copy_count=51
backend_route_carrier_copy_count=19
route_carrier_residual_copy_count=13
call_operand_route_carrier_copy_count=13
call_operand_receiver_route_carrier_copy_count=2
call_operand_arg_route_carrier_copy_count=11
call_operand_receiver_route_carrier_sample_count=2
call_operand_arg_route_carrier_sample_count=11
dominant_call_operand_surface=arg
receiver_post_target=0
receiver_post_target_met=0
arg_forwarding_enabled=0
arg_forwarding_policy=closed_until_explicit_arg_owner_selection
selected_next_action=call_operand_receiver_residue_classification
implementation_allowed=0
measurement_required=0
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

## Evidence Command

```bash
tmp=$(mktemp -d /tmp/hakorune_row761_probe.XXXXXX)
NYASH_FEATURES=rune NYASH_DISABLE_PLUGINS=1 target/release/hakorune \
  --backend mir \
  --emit-mir-json "$tmp/app.mir.json" \
  apps/hako-alloc-mimalloc-comparison-in-process-object-lifecycle-small-block-proof/main.hako >/dev/null
python3 tools/allocator/mir_local_ssa_copy_position_probe.py \
  --mir-json "$tmp/app.mir.json" \
  --method HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1 \
  --topn 8 \
  --out "$tmp/report.out"
cat "$tmp/report.out"
```

## Decision

The coarse call-operand role is not purely receiver residue:

```text
call_operand_receiver_route_carrier_copy_count=2
call_operand_arg_route_carrier_copy_count=11
dominant_call_operand_surface=arg
```

Arg residue is dominant, but arg forwarding remains explicitly closed. Receiver
residue is also non-zero, which means the previous receiver keeper cannot be
reused blindly. The next row must classify the two receiver samples before any
implementation owner reopens.

## Probe Change

`tools/allocator/mir_local_ssa_copy_position_probe.py` now preserves the
existing coarse role:

```text
call_operand_route_carrier_copy_count
```

and additionally emits:

```text
call_operand_receiver_route_carrier_copy_count
call_operand_arg_route_carrier_copy_count
call_operand_receiver_route_carrier_sample_count
call_operand_arg_route_carrier_sample_count
```

The change is observation-only. It does not change MIR, LocalSSA,
callsite-canonicalize, source `.hako`, or runtime behavior.

## Stop Line

```text
do not implement from this probe row
do not patch LocalSSA::ensure_fallback_copy
do not reopen arg forwarding despite arg being dominant
do not retry the prior receiver rewrite until the receiver residue is classified
do not special-case source names, helper names, or benchmark names
do not change PHI lifecycle or freshness contracts
do not change source .hako, product defaults, provider activation,
replacement, hooks, or global allocator
```

## Next

```text
CALL-OPERAND-RECEIVER-RESIDUE-CLASSIFICATION-001:
  classify the two receiver route-carrier samples and decide whether they are
  stale evidence, a missing post-target in the prior receiver keeper, or a new
  receiver surface requiring a separate design row
```
