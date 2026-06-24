# 296x-1653: Boxed Sum Site ABI Plan ID

Status: Complete
Date: 2026-06-24
Token: BOXED-SUM-SITE-ABI-PLAN-ID-001

## Decision

Drain boxed-sum ABI selection out of C shim spelling inference.

The boxed-sum I64 payload ABI and the MetadataContext region-parent AOT reopen
are green. Before adding any further boxed-sum payload classes, each
`VariantMake` / `VariantTag` / `VariantProject` site must consume a resolved
boxed-sum ABI plan identity instead of searching by enum name and payload
storage in backend shims.

## Scope

```text
current files:
  lang/c-abi/shims/hako_llvmc_ffi_boxed_sum_abi_plan.inc
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc
  lang/c-abi/shims/hako_llvmc_ffi_pure_compile_generic_lowering_prescan.inc
  lang/c-abi/shims/hako_llvmc_ffi_same_module_prepass.inc

expected change:
  MIR / LoweringPlan site facts carry resolved abi_plan_id and
  payload_storage=None|I64|Handle.

  C shims consume explicit plan rows only.
```

## Non-Claims

```text
new boxed payload class = 0
new canonical MIR instruction = 0
Option-name backend branch = 0
MetadataContext-name backend branch = 0
runtime fallback = 0
same-module fusion cleanup = 0
const payload definition-index cleanup = 0
```

## Acceptance

```text
remove boxed_sum_payload_storage_from_type_name()
variant_make/tag/project plan lookup uses abi_plan_id
enum-name-only boxed-sum plan lookup = 0 for site lowering
payload_type spelling fallback = 0
unit / handle / I64 boxed-sum probes stay EXE/AOT green
metadata_context_region_parent_backend=green
runtime_try_hako_then_rust_fallback=0
```

## Progress

```text
variant_make site metadata:
  boxed_sum_abi_plan_id emitted when plan is unique
  boxed_sum_payload_storage emitted when plan is unique

variant_project site metadata:
  boxed_sum_abi_plan_id emitted when plan is unique
  boxed_sum_payload_storage emitted when plan is unique

variant_tag site metadata:
  boxed_sum_abi_plan_id emitted when the tag source resolves to a unique local
  VariantMake / Copy boxed-sum site plan

C shim consumption:
  generic variant_make/tag/project consumes explicit site plan metadata
  same-module variant_make/tag/project consumes explicit site plan metadata
  payload_type spelling fallback removed
  boxed_sum_payload_storage_from_type_name removed

focused checks:
  cargo test -q boxed_sum_site_plan_metadata = green
  cargo check -q = green
  rust_lifecycle_metadata_context_region_parent_derived_artifact_guard = green
  rust_mirbuilder_converter_matrix_guard = green
```

## Closeout

```text
result:
  site-lowering payload_type spelling fallback removed

evidence:
  boxed_sum_payload_storage_from_type_name = removed
  find_boxed_sum_abi_plan_index_for_payload_storage = removed
  generic/same-module variant_make/project use explicit site payload_storage
  generic/same-module variant_tag uses explicit site abi_plan_id when present

remaining non-site cleanup:
  local payloadless Variant equality still has enum-name plan lookup and is
  tracked outside site lowering.

next:
  BOXED-SUM-CONST-PAYLOAD-DEF-INDEX-001
```

## Open Boundary

```text
VariantTag has no payload_type/tag field in canonical MIR. Existing metadata
now resolves local make/copy sources, but boxed runtime tag reads that arrive
from non-local values may still only know enum_name. Because one enum can have
multiple boxed ABI shapes, assigning a single abi_plan_id from enum_name alone
would reintroduce inference.

next required data model:
  MIR-owned boxed-sum site facts keyed by function/block/instruction/surface.

tag rule:
  derive abi_plan_id only when the tag value resolves to a unique local
  VariantMake shape. Otherwise fail-fast / leave unannotated until a real
  site fact owner exists.

remaining non-site enum-name lookup:
  local payloadless Variant equality still uses boxed_sum_unit_binding_plan_index
  and must be drained by carrying plan_id in GenericPureVariantBinding before
  claiming enum-name lookup = 0 everywhere.
```

## Follow-up Cleanup Tasks

```text
next inventory:
  C-ABI-SHIM-RESPONSIBILITY-INVENTORY-001

reason:
  hako_llvmc_ffi_pure_compile_boxed_sum_emit.inc is behaviorally valid but
  still scans prior instructions to rediscover const payload definitions.
  Other .inc files also need classification before more backend capability is
  added.

initial inventory scope:
  P0 is limited to boxed-sum site metadata, const payload definition lookup,
  boxed-sum lowering facade, and prepass fact-owner drain. Same-module fusion,
  route descriptor completion, object-storage name inference, and exact seeds
  remain queued cleanup lanes.

tracked follow-ups:
  BOXED-SUM-CONST-PAYLOAD-DEF-INDEX-001
    replace prior-instruction linear scan with named ValueId definition facts

  BOXED-SUM-LOWERING-FACADE-001
    unify generic and same-module boxed-sum make/tag/project lowering

non-goal:
  do not add another backend special case while performing this cleanup.
```
