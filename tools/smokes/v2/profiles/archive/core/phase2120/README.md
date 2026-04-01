# Archive: Phase 2120 Pure C-API Pins

This directory holds historical pure C-API canary pins that were retired from the active
`integration/core/phase2120` lane.

## Contents

- `core/phase2120/s3_link_run_llvmcapi_pure_ternary_collect_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`

## Root-First Replacements

- `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_ternary_collect_runtime_proof.sh`
- `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile archive --filter "core/phase2120/<basename>"
```

These pins are historical evidence only. They are not part of the active integration gate.
