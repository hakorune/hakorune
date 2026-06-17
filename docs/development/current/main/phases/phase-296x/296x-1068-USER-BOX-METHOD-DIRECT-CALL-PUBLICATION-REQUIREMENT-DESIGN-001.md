Status: Done
Date: 2026-06-17
Scope: user-box method direct-call publication requirement split
Previous:
  - docs/development/current/main/phases/phase-296x/296x-1067-USER-BOX-METHOD-PHI-PUBLICATION-OWNER-DESIGN-001.md

# USER-BOX-METHOD-DIRECT-CALL-PUBLICATION-REQUIREMENT-DESIGN-001

## Purpose

Decide whether `KnownReceiverDirectCall` needs
`publication_state=Unpublished` when the lowering only replaces dynamic method
dispatch with a direct same-module method call.

## Evidence

The active target has no user-box `LocalFastPathFact` rows:

```text
local_fastpath_fact_count=0
known_receiver_direct_method_without_fact_count=19
```

However the same target already has route-positive user-box method metadata:

```text
user_box_method_routes=19
thin_entry_selections(surface=user_box_method)=19
thin_entry_selections(manifest_row=user_box_method.known_receiver)=19
```

The LLVM lowering path for user-box methods already has a carrier-compatible
direct-call route:

```text
method_call.py:
  thin_entry_prefers_known_receiver_method(...)
  -> try_lower_known_box_method_call(..., recv_h=recv_h, ...)

direct_box_method.py:
  try_lower_known_box_method_call(...)
  -> builder.call(callee, [recv_h, args...])
```

This path passes the same product-compatible receiver handle. It does not
perform storage bypass, HostHandle bypass, Arc retirement, or local object
layout reinterpretation.

## Decision

Adopt B-lite:

```text
call-route direct dispatch does not require Unpublished.
storage/local-object fastpaths still require Unpublished.
```

This means the responsibilities are split:

```text
user_box_method_routes + thin_entry_selections:
  owner of product-compatible direct user-box method calls
  accepts published/public-runtime receiver carriers

LocalFastPathFact:
  owner of local-first/storage-sensitive fastpaths
  still requires Unpublished for facts whose semantics depend on deferred
  publication or local representation
```

Do not force user-box method direct calls through `LocalFastPathFact` merely to
count them as optimized. That would mix call-route proof with storage
publication proof and recreate the vocabulary bloat this lane is trying to
avoid.

## Implementation Consequence

The immediate implementation is report cleanup, not backend lowering:

```text
fastpath_gap_inventory:
  count user-box method routes covered by thin-entry known receiver selection
  distinguish:
    without_local_fastpath_fact
    covered_by_thin_entry_direct_call
    truly_uncovered
```

For the current target, the expected result is:

```text
known_receiver_direct_method_route_count=19
known_receiver_direct_method_without_fact_count=19
known_receiver_direct_method_thin_entry_covered_count=19
known_receiver_direct_method_uncovered_count=0
```

This preserves the existing `LocalFastPathFact` count while preventing the
report from treating product-compatible user-box direct-call routes as missing
local storage facts.

## Contract

```text
output_contract=user-box-method-direct-call-publication-requirement-design-v0

selected_policy=B-lite

known_receiver_direct_call_requires_unpublished=0
known_receiver_direct_call_requires_product_compatible_receiver=1
known_receiver_direct_call_storage_bypass_enabled=0
known_receiver_direct_call_hosthandle_bypass_enabled=0
known_receiver_direct_call_arc_retirement_enabled=0

local_storage_fastpath_requires_unpublished=1
local_field_access_fastpath_requires_unpublished=1
map_local_i64_storage_fastpath_requires_unpublished=1

user_box_method_direct_call_owner=user_box_method_routes_plus_thin_entry_selections
local_fastpath_fact_user_box_method_required=0
local_fastpath_fact_storage_sensitive_only=1

backend_lowering_changed=0
route_priority_changed=0
publication_classifier_retained_for_storage_sensitive_facts=1

next_task=USER-BOX-METHOD-THIN-ENTRY-COVERAGE-INVENTORY-001
summary=ok
```

## Stop Lines

```text
do not produce user-box LocalFastPathFact for product-compatible direct calls
do not relax Unpublished for storage/HostHandle/Arc bypass fastpaths
do not reinterpret published receiver layout
do not bypass HostHandle from this decision
do not change backend route priority in this row
do not remove the publication classifier; it remains needed for storage-sensitive facts
```
