---
Status: Done
Date: 2026-06-06
Scope: MIR-FMEM-006 producer-neutral parity gate for FastMemory replacement-front reports.
Related:
  - docs/development/current/main/phases/phase-296x/296x-443-PYTHON-C-BRIDGE-RETIREMENT-GATE.md
  - docs/development/current/main/phases/phase-296x/296x-444-MIR-FASTMEM-LLVM-PRIMARY-PRODUCER.md
  - docs/development/current/main/design/mimalloc-hako-port-capability-gap-inventory-ssot.md
---

# 296x-445 MIR FastMem Producer-Neutral Parity

## Decision

Add a report/check gate that compares the current
`python_template_c_bridge` baseline against a `mir_to_llvm_lowering`
candidate using producer-neutral `report.kv` fields.

This row does not retire the Python-template C bridge. It proves that the next
producer can be checked by the same contract before deletion starts.

## Contract

```text
baseline:
  replacement_front_producer=python_template_c_bridge
  replacement_front_backend_artifact=c

candidate:
  replacement_front_producer=mir_to_llvm_lowering
  replacement_front_backend_artifact=llvm_ir|object|exe

both:
  replacement_front_producer_taxonomy_v0=1
  replacement_front_python_template_c_semantic_ssot=0
  replacement_front_python_template_c_retirement_required=1
  replacement_front_mirbuilder_representation_only=1
  replacement_front_mirbuilder_route_decision_count=0
  type_abi_hot_lookup_count=0
  provider_abi_hot_dispatch_count=0
  product_activation=0
  hook_install=0
  global_allocator_claim=0
  winner_claim=0
```

The parity tool compares an explicit field allowlist. It must not compare
timing, throughput, producer name, or backend artifact as equality fields.

## Output Fields

```text
output_contract=hako-check-fastmem-producer-parity-v0
tool_surface=hako_check_fastmem_producer_parity
observation_only=1
benchmark_run_executed=0
producer_neutral_report_schema=0|1
producer_neutral_parity_pass=0|1
producer_neutral_compared_field_count
producer_neutral_mismatch_count
producer_neutral_missing_field_count
baseline_replacement_front_producer
candidate_replacement_front_producer
python_template_c_bridge_runtime_dependency_count
failure_count
failure_N_reason
summary=ok|failed
```

## Stop Line

```text
Do not delete the Python-template C bridge in MIR-FMEM-006.
Do not use the bridge as hidden fallback after MIR-FMEM-007.
Do not infer product activation from parity.
Do not compare benchmark timings as producer-neutral parity.
```

## Next

```text
MIR-FMEM-007:
  retire python_template_c_bridge semantic/runtime dependency after parity is green.
```
