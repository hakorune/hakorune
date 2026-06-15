---
Status: Landed
Date: 2026-06-16
Task: LOCAL-FIRST-DIRECT-PILOT-SELECTION-001
Scope: Select the next local-first direct pilot after Array.length preflight blocked.
Related:
  - docs/development/current/main/phases/phase-296x/296x-813-OBJECT-PUBLICATION-INVENTORY-001.md
  - docs/development/current/main/phases/phase-296x/296x-814-LOCAL-OBJECT-SHADOW-001.md
  - docs/development/current/main/phases/phase-296x/296x-815-LOCAL-DIRECT-ARRAY-LEN-PILOT-001.md
---

# LOCAL-FIRST-DIRECT-PILOT-SELECTION-001

## Purpose

Close the current-front Array.length pilot and select the next local-first
implementation direction from actual target-front evidence.

The target facade body does not contain a pre-publication Array.length
candidate. It does contain a `page` local receiver candidate with three
pre-publication calls. Therefore the next pilot is not Array.length and is not
page-specific storage lowering. The next pilot is:

```text
local known receiver direct call
```

## Selection Report

```text
output_contract=hako-local-first-direct-pilot-selection-v0
source_evidence=296x-815,296x-814,296x-813
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

array_length_pilot_closed_for_current_front=1
array_length_direct_candidate_count=0
pre_publication_array_length_candidate_count=0

selected_next_pilot=local_known_receiver_direct_call
first_target_receiver=page
first_target_call_count=3
first_target_methods=acquire_usize,reuse
pilot_scope=direct_call_only

page_is_first_target_not_rule=1
page_specific_rule_enabled=0
method_name_special_case_enabled=0
storage_direct_enabled=0
hosthandle_bypass_enabled=0
arc_retirement_enabled=0
product_default_changed=0
implementation_started=0

next_task=LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001
summary=ok
```

## Selected Shape

This row selects Tier 1 only:

```text
Tier 1:
  pre-publication known receiver direct call

Tier 2:
  local object storage / field layout direct

Tier 3:
  HostHandle / Arc bypass
```

Only Tier 1 is open for the next probe. Tier 2 and Tier 3 remain closed.

## Why Page Is Not The Rule

The current front evidence says:

```text
page:
  local identity candidate
  three pre-publication calls
  publication through recordLastAllocPage
```

The first target receiver is therefore `page`, with observed methods
`acquire_usize` and `reuse`. That is evidence for a first pilot target, not a
backend rule.

The implementation rule, if later opened, must be based on:

```text
ObjectPlan says receiver is pre-publication
RoutePlan says method target is closed-world direct
```

It must not be based on:

```text
receiver variable name == page
method name == acquire_usize or reuse
helper symbol name
benchmark name
```

## Stop Line

```text
do not implement page-specific branch
do not special-case acquire_usize or reuse
do not infer from helper symbol
do not open storage direct route
do not bypass HostHandle
do not retire Arc
do not change product default runtime behavior
do not reopen Array.length pilot for current front
```

## Next Tasks

```text
1. LOCAL-PAGE-RECEIVER-CANDIDATE-PROBE-001
   Confirm the page receiver shape:
   page_from_queue_selection, page_type_known, route known, publication sites,
   call-before/after publication, and no storage/HostHandle bypass requirement.

2. LOCAL-KNOWN-RECEIVER-DIRECT-CALL-GUARD-SURFACE-001
   Define conditions for direct call:
   pre-publication receiver + known type + known route + no plugin/dynamic
   ambiguity. Keep storage_direct_required=0.

3. LOCAL-KNOWN-RECEIVER-DIRECT-CALL-SHADOW-001
   Report-only shadow route for the three page calls. No behavior change.

4. LOCAL-KNOWN-RECEIVER-DIRECT-CALL-PILOT-001
   Implement only if the shadow report proves a generic ObjectPlan + RoutePlan
   rule. No receiver-name or method-name branch.

5. LOCAL-KNOWN-RECEIVER-DIRECT-CALL-MEASUREMENT-001
   Measure keeper/nonkeeper. Winner claim requires target counter shrink or
   meaningful body win with product_default_changed=0.
```
