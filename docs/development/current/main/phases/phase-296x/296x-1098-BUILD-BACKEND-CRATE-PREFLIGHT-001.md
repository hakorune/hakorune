Status: Done
Date: 2026-06-18
Scope: preflight the next backend-side crate split after mir-plans Stage 1
Related:
  - docs/development/current/main/phases/phase-296x/296x-1097-BUILD-CRATE-SPLIT-POST-STAGE1-MEASURE-001.md
  - docs/development/current/main/design/build-crate-split-plan-ssot.md
  - src/backend
  - src/runner/mir_json_emit
  - src/host_providers/llvm_codegen
  - src/runner/product/llvm

# BUILD-BACKEND-CRATE-PREFLIGHT-001

## Purpose

Decide what "backend crate split" should mean after Stage 1 moved the low-risk
passive MIR plan vocabulary but did not improve cold build time.

## Audit

```text
src_backend_rs_total_lines=19948
src_runner_mir_json_emit_rs_total_lines=10033
src_host_providers_llvm_codegen_rs_total_lines=1160
src_runner_product_llvm_rs_total_lines=1120
```

## Decision

Do not split `src/backend/**` wholesale as the next row.

```text
src_backend_wholesale_split_selected=0
reason=vm_semantic_interpreter_and_feature_gated_wasm_are_mixed_with_backend_name
vm_product_route_retired=1
wasm_backend_feature_gated=1
```

Select the MIR JSON emitter boundary as the next backend-side preflight.

```text
selected_next_boundary=runner_mir_json_emit
selected_next_task=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
reason=product_exe_route_uses_mir_json_emit_before_ny_llvmc
reason=large_enough_for_build_split_candidate
reason=clear_serialization_boundary
```

## Candidate Table

| Candidate | Size | Decision | Reason |
|---|---:|---|---|
| `src/backend/**` | 19948 | defer | Name is broad; contains VM semantic interpreter and feature-gated WASM. |
| `src/runner/mir_json_emit/**` | 10033 | select next preflight | Product EXE route serialization boundary; large enough and conceptually narrow. |
| `src/host_providers/llvm_codegen/**` | 1160 | defer | Thin tool boundary; too small to be first backend split. |
| `src/runner/product/llvm/**` | 1120 | defer | Runner orchestration; depends on parser/using/runtime logging, not pure backend. |
| `src/llvm_py/**` | n/a | out of Rust crate split | Python backend code; not a Rust compile-time split target. |

## Stop Lines

```text
do_not_move_vm_semantic_interpreter=1
do_not_move_wasm_backend_in_backend_preflight=1
do_not_move_runner_or_parser_or_using_resolution=1
do_not_create_backend_crate_with_main_crate_dependency_cycle=1
do_not_change_mir_json_schema=1
do_not_change_ny_llvmc_route=1
behavior_change_allowed=0
```

## Next

```text
next_task=BUILD-MIR-JSON-EMIT-CRATE-PREFLIGHT-001
purpose=audit dependencies and decide whether MIR JSON emission can become the first backend-side crate split
```
