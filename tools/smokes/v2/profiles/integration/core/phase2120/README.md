# Phase 2120 Compat Pack

このディレクトリは backend-zero の mainline proof ではなく、`HAKO_CAPI_PURE=1` を前提にした historical/compat canary 群の置き場だよ。
純粋な pure C-API canary のうち、いくつかは archive profile に退避済みだよ。

## Categories

1. active pure C-API keep pins
   - `core/phase2120/s3_link_run_llvmcapi_pure_array_set_get_canary_vm.sh`
   - `core/phase2120/s3_link_run_llvmcapi_pure_loop_count_canary_vm.sh`
   - canonical manifest: `tools/smokes/v2/profiles/integration/core/phase2120/pure_keep.txt`
   - dedicated suite manifest: `tools/smokes/v2/suites/integration/phase2120-pure-keep.txt`
   - `HAKO_CAPI_PURE=1` 必須
   - historical pure-lowering evidence
   - no exact root-first replacement exists yet, so these two remain keep
   - caller path is `boundary_pure_helper.sh -> ny-llvmc --driver boundary`; do not depend on the retired direct `hostbridge.extern_invoke("env.codegen", ...)` lane here
   - symbol-changing slices must fail fast on stale `target/release/libnyash_kernel.a` instead of surfacing as opaque link errors
2. archive-backed pure C-API historical pins
   - `core/phase2120/s3_link_run_llvmcapi_pure_ternary_collect_canary_vm.sh`
   - `core/phase2120/s3_link_run_llvmcapi_pure_map_set_size_canary_vm.sh`
   - `core/phase2120/s3_link_run_llvmcapi_pure_array_get_ret_canary_vm.sh`
   - `core/phase2120/s3_link_run_llvmcapi_pure_map_get_unbox_ret_canary_vm.sh`
   - `core/phase2120/s3_link_run_llvmcapi_pure_map_set_get_has_canary_vm.sh`
   - canonical manifest: `tools/smokes/v2/profiles/archive/core/phase2120/pure_historical.txt`
   - dedicated suite manifest: `tools/smokes/v2/suites/archive/phase2120-pure-historical.txt`
   - `phase29ck` root-first replacements:
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_ternary_collect_runtime_proof.sh`
     - `tools/smokes/v2/profiles/integration/apps/phase29ck_llvm_backend_map_set_size_runtime_proof.sh`
   - profile entry is `./tools/smokes/v2/run.sh --profile archive --suite phase2120-pure-historical`
3. VM adapter canaries
   - `s3_vm_adapter_*.sh`
   - Hako VM adapter / state alias の観測
   - pure-lowering owner ではないが、phase2120 compat pack と同居している legacy cluster
4. native backend canaries
   - `native_backend_*.sh`
   - `NYASH_LLVM_BACKEND=native` の最小参考 canary
   - `run_all.sh` の owner pack には含めない

## Official Entry

- full legacy cluster entry:
  - `tools/smokes/v2/profiles/integration/core/phase2120/run_all.sh`
  - orchestrates:
    - `tools/smokes/v2/profiles/integration/core/phase2120/run_pure_capi_canaries.sh`
    - `tools/smokes/v2/profiles/integration/core/phase2120/run_vm_adapter_legacy_cluster.sh`
- historical compat pure-pack entry:
  - `tools/smokes/v2/profiles/integration/core/phase2120/run_pure_capi_canaries.sh`
  - active keep pins now run via `--profile integration --suite phase2120-pure-keep`
  - archive-backed pins now run via `--profile archive --suite phase2120-pure-historical`
- shell wrapper:
  - `tools/selfhost/run_compat_pure_pack.sh`
  - old alias `tools/selfhost/run_all.sh` is retired
- SSOT:
  - `docs/development/current/main/phases/phase-29ck/P5-COMPAT-PURE-PACK-LOCK.md`

## Non-goals

- backend-zero mainline acceptance owner になること
- `.hako VM -> LlvmBackendBox -> C-API -> exe` proof を代替すること
- 新しい backend-zero workaround の避難先になること
