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

## Cleanup Rule

- docs should treat the llvmlite trio as a distinct keep bucket
- do not describe the whole `phase2044/` directory as one llvmlite lane
- if paths are ever split physically, update discovery filters and archive references together
