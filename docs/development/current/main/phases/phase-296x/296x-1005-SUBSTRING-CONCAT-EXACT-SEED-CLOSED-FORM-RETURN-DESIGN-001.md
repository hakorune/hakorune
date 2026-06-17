# 296x-1005 SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-DESIGN-001

Status: Landed
Date: 2026-06-17
Scope: exact-seed route design / guard surface before code

## Contract

```text
output_contract=hako-substring-concat-exact-seed-closed-form-return-design-v0
source_evidence=296x-1004,target/fresh-compiler-owner-selection-1003/kilo_micro_substring_concat.mir.json
row_kind=design
implementation_started=0

target_front=kilo_micro_substring_concat
selected_route=substring_concat_loop_ascii
selected_owner_family=exact_seed_dead_byte_copy_loop

existing_closed_form_emitter=1
existing_closed_form_predicate=stable_length_scalar_base
current_closed_form_predicate_reaches=0
current_loop_payload_root=35
current_source_root=20
current_relation_shape=phi_carry_base_to_stable_length_scalar
selected_keeper_shape=stable_length_scalar_via_phi_carry_base

benchmark_name_branch_allowed=0
source_name_branch_allowed=0
helper_name_inference_allowed=0
raw_mir_rescan_allowed=0
product_stringbox_storage_changed=0
runtime_helper_added=0
exact_seed_retired=0

next_task=SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-IMPLEMENTATION-001
summary=ok
```

## Purpose

Fix the exact-seed route shape without adding a new fastpath family.

The current route already has a closed-form emitter:

```text
hako_llvmc_emit_substring_concat_len_closed_form_ir(...)
```

but the predicate does not reach the current metadata shape.

## Current Metadata Shape

The selected route is:

```json
{
  "tag": "substring_concat_loop_ascii",
  "source_route": "string_kernel_plans.loop_payload",
  "proof": "string_kernel_plan_concat_triplet_loop_payload",
  "selected_value": 35
}
```

The string kernel plan for value `35` carries:

```text
loop_payload.loop_bound=300000
loop_payload.seed_length=16
known_length=2
```

The relevant relation shape is:

```text
string_corridor_relations["20"]:
  kind=phi_carry_base
  base_value=35
  window_contract=stop_at_merge

string_corridor_relations["20"]:
  kind=stable_length_scalar
  base_value=20
  window_contract=preserve_plan_window
```

The existing predicate only accepts:

```text
stable_length_scalar(base_value=35, window_contract=stop_at_merge)
```

That is why the route falls back to:

```text
hako_llvmc_emit_substring_concat_loop_ir(...)
```

and emits a stack byte-copy loop before returning the constant result.

## Decision

Allow this narrow relation bridge:

```text
stable_length_scalar_via_phi_carry_base:
  relation owner key = source_root
  relation A:
    kind=phi_carry_base
    base_value=<loop_payload_root>
    window_contract=<requested_window_contract>
  relation B on the same owner key:
    kind=stable_length_scalar
    base_value=<source_root>
    window_contract=preserve_plan_window
```

If both relations exist, the exact-seed route may use the existing closed-form
emitter.

## Implementation Surface

Allowed file:

```text
lang/c-abi/shims/hako_llvmc_ffi_string_metadata_fn_readers.inc
```

Allowed function:

```text
hako_llvmc_string_corridor_has_stable_length_scalar_base_fn(...)
```

The implementation may add helper functions in the same metadata reader file
to avoid duplicating relation scans.

## Stop Line

```text
do not branch by benchmark name
do not branch by source path
do not infer from helper names
do not scan raw MIR instructions
do not add a new runtime helper
do not change product StringBox storage
do not retire substring_concat_loop_ascii
do not change route priority
```

## Acceptance

```text
substring_concat_loop_ascii emits closed_form route for the current front
ny_main no longer contains the stack byte-copy loop
route trace includes stable_length_scalar
phase137x substring-concat exact seed smoke remains green
current-state pointer guard remains green
```

## Next

```text
SUBSTRING-CONCAT-EXACT-SEED-CLOSED-FORM-RETURN-IMPLEMENTATION-001
```
