---
Status: Landed
Date: 2026-06-16
Task: LOCAL-DIRECT-ARRAY-LEN-PILOT-001
Scope: Preflight the local direct Array.length pilot after local object shadow.
Related:
  - docs/development/current/main/phases/phase-296x/296x-814-LOCAL-OBJECT-SHADOW-001.md
  - tools/allocator/hako_local_object_shadow.py
---

# LOCAL-DIRECT-ARRAY-LEN-PILOT-001

## Purpose

Attempt to open the requested local direct Array.length pilot from the
local-first object shadow evidence.

Result:

```text
pilot is not open
```

The target facade body has no pre-publication Array.length candidate. Opening
an implementation row would require inferring from `nyash_array_length_h` or
from older public ArrayBox fallback evidence, which this lane explicitly
forbids.

## Preflight Report

```text
output_contract=hako-local-direct-array-len-pilot-preflight-v0
source_evidence=296x-814,296x-813
target_front=object_lifecycle_body
target_method=HakoAllocObjectLifecycleFacade.objectLifecycleSmallAlloc/1

local_direct_array_len_pilot_requested=1
array_length_direct_candidate_count=0
pre_publication_array_length_candidate_count=0
local_direct_array_len_pilot_open=0
implementation_allowed=0
measurement_required_before_winner_claim=1

blocked_reason=no_array_length_candidate_in_target_facade_body
design_consultation_required=1
candidate_alternative=local_page_direct_call_pilot

do_not_infer_from_helper_symbol=1
do_not_reopen_array_receiver_residence_chain=1
object_plan_execution_enabled=0
backend_consumes_object_plan=0
product_default_changed=0
summary=blocked
```

## Why

The local-first chain proved a different candidate:

```text
page:
  local identity candidate
  three pre-publication direct calls
  two publication sites through recordLastAllocPage
```

It did not prove:

```text
pre-publication Array.length candidate
```

Therefore this pilot cannot be implemented honestly from the current target
evidence.

## Stop Line

```text
do not implement Array.length direct lowering from this evidence
do not infer Array.length candidacy from nyash_array_length_h
do not treat public ArrayBox fallback as local direct storage
do not reopen ArrayReceiverResidenceFact from fallback evidence
do not switch target to page direct calls without design consultation
do not change product default runtime behavior
```

## Design Consultation Point

Choose the next lane:

```text
A. retarget to LOCAL-PAGE-DIRECT-CALL-PILOT-001
   Use the actual local-first candidate from 296x-814.

B. find a new source/MIR front that has pre-publication Array.length
   Keep the Array.length pilot goal but change target evidence.

C. stop exact-AOT local-first implementation here
   Keep the model as docs/tooling until a fresh owner appears.
```
