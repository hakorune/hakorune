# 296x-1656: C Shim Prepass Fact Owner Drain

Status: Complete
Date: 2026-06-24
Token: C-SHIM-PREPASS-FACT-OWNER-DRAIN-001

## Decision

Stop treating C shim prepasses as semantic fact owners.

The prepasses may validate facts and build local lookup tables for efficient
emission, but they should not invent value class, origin, variant binding, or
route policy facts from names, payload spellings, or neighboring instructions.

## Scope

```text
current files:
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc
  lang/c-abi/shims/hako_llvmc_ffi_mir_call_prepass.inc

expected shape:
  published MIR JSON / lowering plan rows own semantic facts
  C prepasses verify and index those rows
  C prepasses do not infer value class from box names or payload spelling
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
new route descriptor = 0
same-module fusion plan = 0
object storage plan migration = 0
runtime fallback = 0
```

## First Step

```text
inventory exact remaining fact derivations in the three prepass files
classify each as:
  value_class_derivation
  origin_derivation
  variant_binding_derivation
  route_policy
  verification_only
choose one P0 derivation to replace with a published owner
```

## Initial Inventory

| File | Current behavior | Category | First action |
| --- | --- | --- | --- |
| `hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc` | `variant_make` creates local variant binding from tag/payload fields; `variant_project` recovers payload alias from that binding | `variant_binding_derivation` | selected first |
| `hako_llvmc_ffi_same_module_prepass.inc` | repeats the same `variant_make` / `variant_project` binding derivation for same-module lowering | `variant_binding_derivation` | selected first |
| `hako_llvmc_ffi_mir_call_prepass.inc` | still falls back from missing lowering-plan need rows to callee/classification helpers | `route_policy` | park until variant binding owner is drained |
| `hako_llvmc_ffi_mir_call_prepass.inc` | promotes array/string origins from local receiver/argument observations | `value_class_derivation` | park until route-policy fallback is separated |

## Selected First Drain

```text
selected_derivation:
  variant_binding_derivation

current duplicated owners:
  hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  hako_llvmc_ffi_same_module_prepass.inc

intended owner:
  MIR finalize / lowering-plan fact row for VariantMake local binding

required fact shape:
  dst
  tag_const
  tag_reg
  payload_reg
  has_payload
  enum_name
  boxed_sum_abi_plan_id when available

prepass responsibility after drain:
  verify row exists for selected VariantMake sites
  index row by dst ValueId
  propagate row through copy only
```

## Acceptance

```text
one concrete prepass fact derivation is moved to a named upstream owner
prepass consumes or verifies the named owner only
no behavior-only cleanup without owner movement
no payload_type spelling fallback reintroduced
no enum-name-only boxed sum site lookup reintroduced
metadata_context_region_parent_backend=green
rust_mirbuilder_converter_matrix_guard=green
runtime_try_hako_then_rust_fallback=0
```

## Closeout

```text
selected_derivation=variant_binding_derivation
upstream_owner=variant_make.variant_binding JSON fact row
generic_prepass_consumes_fact_row=1
same_module_prepass_consumes_fact_row=1
copy_propagation_remains_prepass_local=1
payload_type_spelling_fallback_reintroduced=0
enum_name_only_boxed_sum_site_lookup_reintroduced=0
cargo_check=green
cargo_test_variant_make_emits=green
cargo_test_boxed_sum_site=green
metadata_context_region_parent_guard=green
rust_mirbuilder_converter_matrix_guard=green
current_state_pointer_guard=green
runtime_try_hako_then_rust_fallback=0
```
