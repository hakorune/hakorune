# Phase 2044 Mixed Proof Buckets

This directory is not one semantic lane.

## Buckets

1. llvmlite monitor-only keep
   - `codegen_provider_llvmlite_canary_vm.sh`
   - `codegen_provider_llvmlite_compare_branch_canary_vm.sh`
   - `codegen_provider_llvmlite_const42_canary_vm.sh`
   - purpose:
     - provider-first llvmlite proof/canary coverage
     - integration discovery-live monitor-only keep
   - note:
     - this is the only `phase2044` surface tracked by `29x-98` as a remaining proof-only direct `env.codegen.emit_object` caller

2. `hako_primary_no_fallback_*`
   - separate core-exec proof bucket
   - not part of the llvmlite keep lane

3. `mirbuilder_provider_*`
   - separate mirbuilder-provider proof bucket
   - not part of the llvmlite keep lane

## Bucket Runners

- `tools/smokes/v2/profiles/integration/core/phase2044/run_llvmlite_monitor_keep.sh`
  - runs only the llvmlite monitor-only keep trio
- `tools/smokes/v2/profiles/integration/core/phase2044/run_hako_primary_no_fallback_bucket.sh`
  - runs only the `hako_primary_no_fallback_*` bucket
- `tools/smokes/v2/profiles/integration/core/phase2044/run_mirbuilder_provider_bucket.sh`
  - runs only the `mirbuilder_provider_*` bucket

## Cleanup Rule

- docs should treat the llvmlite trio as a distinct keep bucket
- do not describe the whole `phase2044/` directory as one llvmlite lane
- bucket runners separate the semantics without changing discovery paths
- if paths are ever split physically, update discovery filters and archive references together
