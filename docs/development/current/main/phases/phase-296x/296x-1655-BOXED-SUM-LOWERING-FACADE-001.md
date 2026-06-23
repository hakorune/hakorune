# 296x-1655: Boxed Sum Lowering Facade

Status: Complete
Date: 2026-06-24
Token: BOXED-SUM-LOWERING-FACADE-001

## Decision

Unify boxed-sum make/tag/project lowering behind one backend helper facade.

The current C shim layout has two main emit paths:

```text
generic pure lowering
same-module typed-field / RMW lowering
```

Both paths can lower boxed-sum `VariantMake`, `VariantTag`, and
`VariantProject`. That duplicates plan lookup and payload-storage handling in
`.inc` emit files. The site metadata work removed the worst spelling fallback,
but the lowering responsibility is still physically split.

## Scope

```text
current files:
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_variant_dispatch.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_typed_field_rmw_emit.inc
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc

expected shape:
  one boxed-sum lowering facade consumes:
    opcode surface
    dst / source / tag / payload ValueIds
    resolved abi_plan_id
    resolved payload_storage
    existing local binding facts

  generic and same-module emitters call the facade.
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
new route descriptor = 0
new same-module fusion plan = 0
new prepass fact owner = 0
runtime fallback = 0
```

## Acceptance

```text
one boxed-sum opcode-lowering entry per opcode surface
generic and same-module paths share payload_storage behavior
payload_type spelling inference = 0
enum-name-only boxed sum site lookup = 0
duplicate boxed-sum make/project branches in same-module emitter = 0
unit / handle / I64 boxed-sum probes stay EXE/AOT green
metadata_context_region_parent_backend=green
rust_mirbuilder_converter_matrix_guard=green
runtime_try_hako_then_rust_fallback=0
```

## Parked

```text
C-SHIM-PREPASS-FACT-OWNER-DRAIN-001
SAME-MODULE-FUSION-PLAN-SSOT-001
GENERIC-ROUTE-DESCRIPTOR-FULL-GENERATION-001
```

## Closeout

```text
boxed_sum_variant_make_facade=green
boxed_sum_variant_tag_facade=green
boxed_sum_variant_project_facade=green
generic_path_uses_facade=1
same_module_path_uses_facade=1
payload_type_spelling_inference=0
duplicate_make_project_payload_dispatch=0
cargo_check=green
cargo_test_boxed_sum_site=green
metadata_context_region_parent_guard=green
rust_mirbuilder_converter_matrix_guard=green
current_state_pointer_guard=green
runtime_try_hako_then_rust_fallback=0
```
