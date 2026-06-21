# 296x-1540 EXPLICIT-CARRIER-ARRAYBOX-GET-ACCEPTANCE-001

Status: landed
Date: 2026-06-21

## Purpose

Close the explicit CarrierInfo owned-snapshot artifact without adding
key-name-specific type patches.

This row landed with the general `ArrayBox.get` route. No literal key-name
type override was added.

The current first backend failure is:

```text
artifact:
  variable_context_explicit_carrier_snapshot.hako

shape:
  local requested_name_copy = info.get("requested_names")
  requested_name_copy.get(0)

failure:
  unsupported pure shape
  reason=mir_call_no_route
  callee_symbol=get
  receiver_origin_box=ArrayBox
```

This is an acceptance gap for `ArrayBox.get` in the explicit carrier consumer
path, not permission to classify `requested_names` as `StringBox`.

## Scope

```text
BoxCount: one backend/MIR acceptance shape
owner: ArrayBox.get route or receiver-origin publication for explicit carrier data
input: OrderedMapBox.get("requested_names") -> ArrayBox, then ArrayBox.get(i)
output: explicit carrier snapshot derived artifact reaches EXE
```

## Required Checks

```text
bash tools/checks/rust_lifecycle_no_carrier_key_type_special_case_guard.sh
cargo test -q ordered_map_origin_plan::tests --lib
bash tools/checks/rust_lifecycle_variable_context_carrier_snapshot_derived_artifact_guard.sh
```

The explicit artifact guard is the closeout target, not the entry condition:

```text
bash tools/checks/rust_lifecycle_variable_context_explicit_carrier_snapshot_derived_artifact_guard.sh
```

## Anti-Special-Case Rule

The fix must be rejected if it contains any of these:

```text
requested_names -> StringBox
key == "requested_names" type override
seed_string_key / seed_text_key helper for carrier data
source-name fallback for CarrierInfo
runtime try-Hako-then-Rust fallback
```

Legitimate evidence must come from facts, verifier operations, or a general
accepted backend route. It must not come from the literal key name.

## Acceptance

```text
no_carrier_key_type_special_case=green
ordered_map_origin_plan_tests=green
variable_context_carrier_snapshot generated EXE green
variable_context_explicit_carrier_snapshot generated EXE green
runtime_data_get_for_carrier_arrays=0
full_variable_context_claim=0
runtime_try_hako_then_rust_fallback=0
```

## Stop Line

```text
do_not_classify_requested_names_as_StringBox_by_name=1
do_not_open_general_dependent_map_typing=1
do_not_change_CarrierInfo_contract_without_facts=1
do_not_treat_carrier_sensitive_alias_as_closed_by_backend_patch=1
```
