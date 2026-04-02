# Phase 2120 Compat Pure Keep

This bucket is the semantic home for the active phase2120 pure C-API keep pins.
It lives under `integration/compat` because it is keep-only proof coverage, not owner code.

## Bucket

- `s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`
- `s3_link_run_llvmcapi_pure_loop_count_canary_vm.sh`

## Canonical Manifest

- `tools/smokes/v2/suites/integration/compat/pure-keep.txt`

## Runner

Use:

```bash
./tools/smokes/v2/run.sh --profile integration --suite compat/pure-keep
```

These pins are discovery-live historical evidence. They are not part of the active owner proof lane.
