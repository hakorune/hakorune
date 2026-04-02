# llvmlite monitor-only keep

This bucket is the semantic home for the llvmlite keep trio.

It lives under `integration/compat` because it is keep-only proof coverage, not owner code.

- Read this bucket as `compat/probe keep`.
- Do not read it as `llvm/exe` product-mainline evidence.
- Green here means the explicit llvmlite keep surface still works; it does not
  mean the product lane runs through llvmlite.

## Bucket

1. llvmlite monitor-only keep
   - `codegen_provider_llvmlite_canary_vm.sh`
   - `codegen_provider_llvmlite_compare_branch_canary_vm.sh`
   - `codegen_provider_llvmlite_const42_canary_vm.sh`
   - purpose:
     - provider stop-line llvmlite proof/canary coverage
     - integration discovery-live monitor-only keep
   - note:
     - this is the only live keep bucket in this semantic home
     - current route is `vm-hako -> LlvmBackendEvidenceAdapterBox.compile_obj_provider_stopline(...) -> compat/provider stop-line -> llvmlite keep`
     - `compare_branch` and `const42` are merge-later candidates only; they are not exact duplicates today
     - no file in this trio is archive-ready on current replacement coverage

## Bucket Runners

- `tools/smokes/v2/suites/integration/compat/llvmlite-monitor-keep.txt`
  - dedicated suite manifest for the llvmlite keep trio
- `tools/smokes/v2/profiles/integration/compat/llvmlite-monitor-keep/run_llvmlite_monitor_keep.sh`
  - runs only the llvmlite monitor-only keep trio via the dedicated suite manifest
- `tools/smokes/v2/profiles/integration/proof/hako-primary-no-fallback/run_hako_primary_no_fallback_bucket.sh`
  - separate core-exec proof bucket
- `tools/smokes/v2/profiles/integration/proof/mirbuilder-provider/run_mirbuilder_provider_bucket.sh`
  - separate mirbuilder-provider proof bucket

## Cleanup Rule

- docs should treat the llvmlite trio as a distinct keep bucket
- do not re-hang the bucket under `phase2044/` as a live semantic home
- the dedicated suite manifest separates the semantics without changing discovery paths
- the remaining proof buckets live under `integration/proof/`
- if paths are ever split again, update discovery filters and archive references together
