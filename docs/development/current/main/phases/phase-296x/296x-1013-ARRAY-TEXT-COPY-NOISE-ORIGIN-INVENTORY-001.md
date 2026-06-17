# 296x-1013 ARRAY-TEXT-COPY-NOISE-ORIGIN-INVENTORY-001

Status: Landed
Date: 2026-06-17
Scope: investigation / no behavior change

## Contract

```text
output_contract=hako-array-text-copy-noise-origin-inventory-v0
source_evidence=296x-1011,296x-1012,target/fresh-compiler-owner-selection-1008
row_kind=inventory

target_front=kilo_meso_substring_concat_array_set
target_block=23
target_route_proof=array_get_lenhalf_insert_mid_dest_slot_len_only

route_skip_instruction_count=19
route_skipped_copy_count=7
route_tolerated_unrelated_copy_count=2

skipped_copy_len_carrier_count=2
skipped_copy_left_substring_carrier_count=3
skipped_copy_right_substring_carrier_count=2
unrelated_copy_destination_receiver_carrier_count=2

copy_noise_primary_cause=localssa_method_operand_materialization
copy_noise_secondary_cause=source_local_binding_carriers
copy_noise_bug_classification=normal_ssa_carrier_noise
copy_skip_contract=same_root_only
copy_skip_broadening_required=0
copy_elimination_implementation_allowed=0

receiver_materialization_owner=MirBuilder::finalize_call_operands/receiver::finalize_method_receiver
pin_slot_owner=MirBuilder::pin_to_slot(@recv)
localssa_receiver_owner=mir::builder::ssa::local::recv

selected_next_owner=none
next_task=FRESH-COMPILER-OWNER-SELECTION-002
summary=ok
```

## Purpose

Check whether the Copy values tolerated by the 296x-1011/1012
`array_text_edit_routes` matcher are a symptom of a deeper compiler bug or
normal SSA carrier noise.

The concern was valid: if the matcher only hides a bad producer, the same shape
will reappear elsewhere.  This row therefore inventories the matched block
without changing code.

## MIR Window

For the selected route:

```text
block=23
get_instruction_index=7
set_instruction_index=26
result_len_value=52
skip_instruction_indices=8..18,21..28
```

Relevant block shape:

```text
07 KEEP  %42 = src.get(row)
08 SKIP  %43 = line.length()
09 SKIP  %71 = copy %43
11 SKIP  %75 = copy %71
13 SKIP  %74 = %75 / 2
14 SKIP  %47 = line.substring(0, split)
15 SKIP  %48 = line.substring(split, len)
16 SKIP  %77 = copy %47
17 SKIP  %80 = copy %77
18 SKIP  %82 = copy %48
19 KEEP  %84 = copy %7
20 KEEP  %83 = copy %84
21 SKIP  %87 = copy %80
22 SKIP  %88 = "xx"
23 SKIP  %86 = %87 + %88
24 SKIP  %89 = copy %82
25 SKIP  %85 = %86 + %89
26 SKIP  dst.set(row, out)
27 SKIP  %113 = 2
28 SKIP  %52 = %113 + %43
```

## Classification

Skipped copies:

```text
%43 -> %71 -> %75:
  len carrier for split / result length

%47 -> %77 -> %80 -> %87:
  left substring carrier for concat operands

%48 -> %82 -> %89:
  right substring carrier for concat operands
```

Tolerated but not skipped:

```text
%7 -> %84 -> %83:
  destination array receiver carrier for dst.set(row, out)
```

The destination receiver carrier appears before the concat expression because
method-call lowering materializes receiver / argument carriers before the final
call emission.  This is normal LocalSSA safety behavior, not a route-specific
semantic owner.

## Source Owner

The relevant code-side ownership is already explicit:

```text
src/mir/builder/emit_guard/mod.rs:
  finalize_call_operands(...)

src/mir/builder/receiver.rs:
  finalize_method_receiver(...)
  pin_to_slot(r, "@recv")
  ssa::local::recv(...)

src/mir/builder/builder_emit.rs:
  final receiver materialization for MethodCall

src/mir/builder/control_flow/plan/lowerer/effect_emission.rs:
  LocalSSA receiver/args materialized in current block
```

This matches the observed `%7 -> %84 -> %83` destination receiver chain.

## Decision

```text
copy_skip_is_symptom_only=0
copy_skip_is_normal_carrier_tolerance=1
broad_copy_coalescing_reopen=0
array_text_matcher_same_root_tolerance_kept=1
```

The matcher is allowed to tolerate Copy carriers only when `same_root(...)`
proves the value is one of the route values.  Unrelated Copy instructions may
be stepped over to reach the route-local `"xx"` constant, but they are not
included in `skip_instruction_indices` unless they belong to the matched route.

## Stop Line

```text
do not broaden LocalSSA copy coalescing from this evidence
do not remove receiver materialization from MethodCall lowering
do not let array_text matcher skip unrelated Copy instructions
do not change backend consumer behavior
do not infer a new optimization owner from Copy noise alone
```

## Follow-Up Policy

Open a new implementation row only if a later front shows:

```text
same_copy_family_blocks_route_matching=1
or
unrelated_copy_count_grows_across_fronts>=2
or
copy_noise_becomes_hot_owner=1
```

Otherwise continue with:

```text
FRESH-COMPILER-OWNER-SELECTION-002
```
