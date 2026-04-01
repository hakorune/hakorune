# Archive: Phase 2120 Pure C-API Pins

This directory holds historical pure C-API canary pins that were retired from the active
`integration/core/phase2120` lane.

## Contents

- `core/phase2120/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`
- `core/phase2120/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile archive --filter "core/phase2120/<basename>"
```

These pins are historical evidence only. They are not part of the active integration gate.
