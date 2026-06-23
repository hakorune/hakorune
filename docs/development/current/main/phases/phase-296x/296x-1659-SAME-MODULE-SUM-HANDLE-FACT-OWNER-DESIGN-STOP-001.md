---
Status: Active
Date: 2026-06-24
Token: SAME-MODULE-SUM-HANDLE-FACT-OWNER-DESIGN-STOP-001
Scope: C ABI shim responsibility cleanup / boxed-sum handle proof owner
---

# 296x-1659 SAME-MODULE-SUM-HANDLE-FACT-OWNER-DESIGN-STOP-001

## Decision Needed

The next C ABI shim cleanup target is the remaining boxed-sum handle inference
in:

```text
lang/c-abi/shims/hako_llvmc_ffi_same_module_value_metadata.inc
```

The current path publishes a sum handle from a runtime box-name prefix:

```text
__hako_sum_*
```

That is a name-spelling proof. Before implementation, choose the MIR-owned fact
that proves a value/register is a boxed-sum handle.

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

## Design Question

Which fact is the SSOT for "this value/register is a boxed-sum handle"?

```text
A. variant_binding row/table only
   Use the existing VariantMake binding proof as the only boxed-sum handle
   source. Non-VariantMake metadata without a binding is not a sum handle.

B. explicit value metadata only
   Extend value_types / value metadata with an explicit boxed-sum field such as
   boxed_sum_abi_plan_id or sum_handle=true. C consumes that field and never
   reads the __hako_sum_ spelling.

C. boxed_sum_abi_plan_id site facts propagated into value metadata
   Keep boxed-sum ABI plan identity as the owner and require semantic refresh /
   MIR JSON emission to thread it to all metadata consumers.

D. split responsibility
   Variant-producing sites use variant_binding. Generic value metadata uses an
   explicit boxed-sum value fact. The prefix is never a proof in either path.
```

## Recommendation To Validate

Prefer D if the current MIR model needs both site-local and propagated metadata:

```text
VariantMake / copy / phi values:
  variant_binding is the proof.

generic value metadata / result-origin publication:
  explicit boxed-sum value fact is the proof.

box_name prefix:
  diagnostic string only, never proof.
```

## Acceptance After Decision

Implementation may start only after the proof owner is selected.

Required acceptance for the follow-up implementation:

```text
__hako_sum_ prefix inference in same_module_value_metadata = 0
C shim does not infer sum handles from box names
missing explicit proof fails closed or remains non-sum by documented contract
existing boxed-sum I64/handle/unit probes stay green
metadata_context_region_parent AOT stays green
generic_method_get_policy prefix use is either parked with a card or drained
runtime fallback = 0
```

## Non-Claims

```text
no boxed-sum ABI redesign
no new canonical MIR instruction
no Option / MetadataContext / RegionObserver backend special case
no runtime type-name fallback
```
