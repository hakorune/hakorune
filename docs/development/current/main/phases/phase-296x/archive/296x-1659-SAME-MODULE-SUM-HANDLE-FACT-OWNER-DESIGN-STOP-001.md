---
Status: Accepted
Date: 2026-06-24
Token: SAME-MODULE-SUM-HANDLE-FACT-OWNER-DESIGN-STOP-001
Scope: C ABI shim responsibility cleanup / boxed-sum handle proof owner
---

# 296x-1659 SAME-MODULE-SUM-HANDLE-FACT-OWNER-DESIGN-STOP-001

## Decision

The next C ABI shim cleanup target is the remaining boxed-sum handle inference
in:

```text
lang/c-abi/shims/hako_llvmc_ffi_same_module_value_metadata.inc
```

The current path publishes a sum handle from a runtime box-name prefix:

```text
__hako_sum_*
```

That is a name-spelling proof. The selected design is:

```text
semantic authority:
  ValueRepresentationFact::BoxedSumHandle { abi_plan_id }

implementation split:
  variant_binding remains a site-local tag/payload tracking helper

forbidden proof:
  __hako_sum_ prefix
  enum_name
  box_type spelling
  raw i64 sign
```

C is the semantic authority. D is only the implementation layering. Do not treat
`variant_binding` and value metadata as independent proof sources.

## Current Evidence

Known producer-side facts already exist:

```text
variant_binding row
  emitted on VariantMake by src/runner/mir_json_emit/emitters/sum.rs

boxed_sum_abi_plan_id
  emitted on boxed-sum make/tag/project sites

value_types handle metadata
  currently carries box_type, which C uses by prefix
```

Known C-side consumers:

```text
same_module_function_publish_sum_handle_if_box
same_module_function_publish_result_origin_box
same_module_function_publish_handle_value_metadata
```

Related prefix use also exists outside same-module metadata:

```text
lang/c-abi/shims/hako_llvmc_ffi_generic_method_get_policy.inc
```

## Authority Chain

```text
BoxedSumAbiPlan
  plan_id / layout / runtime_type_id
        ↓
BoxedSumSitePlan
  VariantMake / Tag / Project site selected plan
        ↓
ValueRepresentationFact
  dst = BoxedSumHandle(plan_id)
        ↓
Copy / Phi / selected result propagation
        ↓
MIR JSON
        ↓
C shim local register cache
```

`variant_binding` proves only local tag/payload/enum tracking. It does not prove
runtime boxed-sum handle representation.

## Selected Next Slice

```text
EXPLICIT-BOXED-SUM-VALUE-FACT-SAME-MODULE-001
```

Include:

```text
ValueRepresentationFact::BoxedSumHandle { abi_plan_id }
semantic refresh / MIR JSON publication for selected same-module values
common C value-representation metadata reader/cache
same_module_value_metadata prefix inference removal
focused same-module call gate
```

Do not include:

```text
generic_method_get_policy prefix removal
all handle classes
boxed-sum ABI redesign
new canonical MIR instruction
Option-specific backend behavior
```

## Acceptance

Required acceptance for the follow-up implementation:

```text
__hako_sum_ prefix inference in same_module_value_metadata = 0
C shim does not infer sum handles from box names
missing explicit proof fails closed or remains non-sum by documented contract
existing boxed-sum I64/handle/unit probes stay green
metadata_context_region_parent AOT stays green
generic_method_get_policy __hako_sum_ prefix inference = 0
runtime fallback = 0
```

Closeout evidence:

```text
EXPLICIT-BOXED-SUM-VALUE-FACT-SAME-MODULE-001 = landed
value_representations JSON fact = present
same_module_value_metadata __hako_sum_ prefix inference = 0
generic_method_get_policy __hako_sum_ prefix inference = 0
metadata_context_region_parent AOT = green
mir_call_route_policy legacy generic_method_routes fallback = 0
mir_call_need_name_fallback = compatibility-audited
object_storage_plan_name_inference = drained
exact_seed_route_quarantine = drained
same_module_definition_edge_plan = drained
next = MIR-CALL-PREPASS-FACT-OWNER-DRAIN-001
```

Focused negative acceptance:

```text
variant_binding without boxed_sum_abi_plan_id is not a boxed handle
box_type="__hako_sum_fake" is not a boxed handle
invalid abi_plan_id fails closed
Phi with mixed boxed-sum abi_plan_id values fails closed
payload_storage=Handle alone is not boxed-sum proof
```

## Non-Claims

```text
no boxed-sum ABI redesign
no new canonical MIR instruction
no Option / MetadataContext / RegionObserver backend special case
no runtime type-name fallback
```
