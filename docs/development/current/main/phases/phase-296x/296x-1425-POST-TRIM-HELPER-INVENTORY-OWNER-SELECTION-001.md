# 296x-1425 POST-TRIM-HELPER-INVENTORY-OWNER-SELECTION-001

Status: open
Date: 2026-06-20

## Purpose

Select the next lifecycle owner after `CarrierInfo.trim_helper` inventory
confirms that trim metadata remains route-specific and denied by
resolver/verifier/emitter fixtures.

## Selected By

```text
296x-1424-TRIM-HELPER-CARRIER-LIFECYCLE-INVENTORY-001
```

## Candidate Owners

```text
A. trim_helper producer probe
   value: fixture-guard TrimRouteInfo::to_carrier_info as the producer of
   trim_helper=Some(TrimLoopHelper)
   risk: can expand into all trim route lowering semantics

B. promoted_body_locals lifecycle inventory
   value: separates owned promoted-name metadata from trim helper payload
   risk: can expand into body-local promotion route design

C. Expand emitter probe to parser/MIR-checkable surface
   value: moves beyond comment-level fixture after denied boundaries are named
   risk: can become generated-program claim or converter rewrite
```

## Recommended Direction

```text
recommended=A-lite
reason=after inventory, the smallest implementation-capable owner is the
actual trim_helper producer surface: TrimRouteInfo::to_carrier_info. Keep this
as a producer fixture only; do not implement trim route lowering semantics.
```

## Acceptance

```text
next_owner_selected=1
selected_owner_scope_documented=1
non_selected_owners_parked=1
implementation_started=0
backend_behavior_changed=0
full_VariableContext_parity_claim=0
MirBuilder_wide_lifecycle_claim=0
```

Checks:

```bash
git diff --check
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

```text
do_not_mix_trim_helper_producer_with_promoted_body_locals=1
do_not_expand_emitter_before_selection=1
do_not_claim_trim_route_lowering_complete=1
do_not_make_converter_core_policy_owner=1
```

