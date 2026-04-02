# Pure Legacy Cluster Index

This directory is the index-only orchestrator for the pure legacy cluster.
The actual coverage lives in semantic homes; this file only points at them.

## Categories

1. active pure C-API keep pins
   - `compat/pure-keep/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`
   - `compat/pure-keep/s3_link_run_llvmcapi_pure_loop_count_canary_vm.sh`
   - canonical manifest: `tools/smokes/v2/profiles/integration/compat/pure-keep/pure_keep.txt`
   - dedicated suite manifest: `tools/smokes/v2/suites/integration/compat/pure-keep.txt`
   - `HAKO_CAPI_PURE=1` 必須
   - historical pure-lowering evidence
   - no exact root-first replacement exists yet, so these two remain keep
   - caller path is `boundary_pure_helper.sh -> ny-llvmc --driver boundary`; do not depend on the retired direct `hostbridge.extern_invoke("env.codegen", ...)` lane here
   - symbol-changing slices must fail fast on stale `target/release/libnyash_kernel.a` instead of surfacing as opaque link errors
2. archive-backed pure C-API historical pins
   - `archive/pure-historical/s3_link_run_llvmcapi_pure_ternary_collect_canary_vm.sh`
   - `archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh`
   - `archive/pure-historical/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`
   - `archive/pure-historical/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`
   - `archive/pure-historical/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`
   - canonical manifest: `tools/smokes/v2/profiles/archive/pure-historical/pure_historical.txt`
   - dedicated suite manifest: `tools/smokes/v2/suites/archive/pure-historical.txt`
   - `phase29ck` root-first replacements:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_ternary_collect_runtime_proof.sh`
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh`
3. VM adapter canaries
   - `proof/vm-adapter-legacy/s3_vm_adapter_*.sh`
   - Hako VM adapter / state alias の観測
   - pure-lowering owner ではないが、phase2120 legacy cluster と同居している proof bucket
4. native backend canaries
   - `proof/native-reference/native_backend_*.sh`
   - `NYASH_LLVM_BACKEND=native` の最小参考 canary
   - `run_all.sh` の owner pack には含めない

## Official Entry

- canonical suite manifest:
  - `tools/smokes/v2/suites/integration/pure-legacy-cluster.txt`
- full legacy cluster entry:
  - `tools/smokes/v2/profiles/integration/proof/pure-legacy-cluster/run_all.sh`
  - orchestrates:
    - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`
    - `tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh`
    - `tools/smokes/v2/profiles/integration/proof/vm-adapter-legacy/run_vm_adapter_legacy_cluster.sh`
    - `tools/smokes/v2/profiles/integration/proof/native-reference/run_native_reference_bucket.sh`
- compat pure-pack entry:
  - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`
  - active keep pins now run via `--profile integration --suite compat/pure-keep`
- archive historical entry:
  - `tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh`
  - archive-backed pins now run via `--profile archive --suite pure-historical`
- shell wrapper:
  - `tools/compat/legacy-codegen/run_compat_pure_pack.sh`
  - old alias `tools/selfhost/run_all.sh` is retired
- SSOT:
  - `docs/development/current/main/phases/phase-29ck/P5-COMPAT-PURE-PACK-LOCK.md`

## Non-goals

- backend-zero mainline acceptance owner になること
- `.hako VM -> LlvmBackendBox -> C-API -> exe` proof を代替すること
- 新しい backend-zero workaround の避難先になること
