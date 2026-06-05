---
Status: Done
Date: 2026-06-06
Scope: add producer-neutral replacement-front producer taxonomy before any MIR/FastMem lowering work.
Blocker: MIM-FMEM-017D
Related:
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
  - docs/development/current/main/workstreams/mimalloc-current.md
  - tools/hako_check/replacement_front_report.py
  - tools/hako_check/fastmem_capability_inventory.py
  - tools/hako_check/fastmem_check.py
---

# 296x-433 Replacement-Front Producer Taxonomy

## Purpose

`MIM-FMEM-017A..017C` proved that the current replacement-front bridge is
non-activating and tied to `.hako` allocator source truth. `MIM-FMEM-017D`
names the producer of that bridge before MIR/FastMem lowering begins.

This row is report/check-only.

## Decision

```text
replacement_front_producer_taxonomy_v0=1
generated_c_behavior_change=0
source_syntax_change=0
rust_parser_change=0
hako_parser_change=0
mir_lowering_behavior_change=0
product_activation=0
hook_installed=0
global_allocator_product_claim=0
winner_claim=0
```

Producer taxonomy is an evidence surface, not a lowering implementation.

```text
current:
  replacement_front_producer=python_template_c_bridge
  C is still the bridge implementation
  C is not semantic SSOT
  retirement is required

transition:
  replacement_front_producer=mir_to_c_lowering
  C may remain only as a backend artifact

final:
  replacement_front_producer=mir_to_llvm_lowering
  LLVM/object is the primary path
```

## Fields

```text
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge|mir_to_c_lowering|mir_to_llvm_lowering
replacement_front_backend_artifact=c|llvm_ir|object|exe
replacement_front_source_truth=hako_fastmem|hako_alloc.size_class_box|hako_alloc.page_box|unknown
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=0|1
replacement_front_mir_memop_enabled=0|1
replacement_front_mir_fastmem_region_enabled=0|1
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_transition_state=current_bridge|transition_backend_artifact|final_primary
```

## Acceptance

For the current bridge:

```text
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge
replacement_front_backend_artifact=c
replacement_front_source_truth=hako_alloc.size_class_box
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
replacement_front_mir_memop_enabled=0
replacement_front_mir_fastmem_region_enabled=0
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_transition_state=current_bridge
product_activation_ready=0
type_abi_hot_path_lookup_count=0
provider_dispatch_hot_path=0
```

Proof:

```bash
python3 -m py_compile tools/hako_check/replacement_front_report.py tools/hako_check/fastmem_capability_inventory.py tools/hako_check/fastmem_check.py
bash tools/hako_check/replacement_front_report_smoke.sh
bash tools/hako_check/fastmem_capability_inventory_smoke.sh
bash tools/hako_check/fastmem_check_smoke.sh
bash tools/checks/current_state_pointer_guard.sh
```

## Stop Line

- do not implement MIR lowering in this row
- do not remove the Python-template C bridge in this row
- do not change generated C malloc/free behavior
- do not let Python-template C become semantic SSOT
- do not fold LLVM runner cleanup into this row
- do not open product allocator activation
- do not install hooks, claim global allocator ownership, or make winner claims

## Follow-Up Phase Split

The LLVM runner cleanup is intentionally separate from this producer taxonomy:

```text
LLVM-PIPE-001:
  Inventory/report the current LLVM runner pipeline debt:
    NYASH_REWRITE_FUTURE env forcing
    method_id_injector no-op mutation seam
    joinir_experiment hook/fallback
    pyvm/harness/mock fallback route visibility

LLVM-PIPE-002:
  Add pipeline/report fields:
    mir_future_rewrite_route
    pipeline_joinir_experiment_enabled
    method_id_injector_mutation_count
    execution_backend
    llvm_fallback_used
    llvm_fallback_reason

LLVM-PIPE-003:
  Move env side effects and runner ad-hoc stages toward
  CompileOptions / PipelinePlan / LoweringPlan.
```

Those rows must not alter `MIM-FMEM-017D` producer taxonomy.

## Landed Evidence

```text
replacement_front_producer_taxonomy_v0=1
replacement_front_producer=python_template_c_bridge
replacement_front_backend_artifact=c
replacement_front_source_truth=hako_alloc.size_class_box
replacement_front_python_template_c_semantic_ssot=0
replacement_front_python_template_c_retirement_required=1
replacement_front_mir_memop_enabled=0
replacement_front_mir_fastmem_region_enabled=0
replacement_front_mirbuilder_representation_only=1
replacement_front_mirbuilder_route_decision_count=0
replacement_front_producer_transition_state=current_bridge
generated_c_behavior_change=0
mir_lowering_behavior_change=0
product_activation=0
```

Next row:

```text
LLVM-PIPE-001 LLVM runner pipeline debt inventory
```
