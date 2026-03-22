# Phase 2120 Compat Pack

このディレクトリは backend-zero の mainline proof ではなく、`HAKO_CAPI_PURE=1` を前提にした historical/compat canary 群の置き場だよ。

## Categories

1. pure C-API canaries
   - `s3_link_run_llvmcapi_pure_*.sh`
   - `HAKO_CAPI_PURE=1` 必須
   - historical pure-lowering evidence
   - caller path is `boundary_pure_helper.sh -> ny-llvmc --driver boundary`; do not depend on the retired direct `hostbridge.extern_invoke("env.codegen", ...)` lane here
   - symbol-changing slices must fail fast on stale `target/release/libnyash_kernel.a` instead of surfacing as opaque link errors
2. VM adapter canaries
   - `s3_vm_adapter_*.sh`
   - Hako VM adapter / state alias の観測
   - pure-lowering owner ではないが、phase2120 compat pack と同居している legacy cluster
3. native backend canaries
   - `native_backend_*.sh`
   - `NYASH_LLVM_BACKEND=native` の最小参考 canary
   - `run_all.sh` の owner pack には含めない

## Official Entry

- historical compat pack entry:
  - `tools/smokes/v2/profiles/integration/core/phase2120/run_all.sh`
  - filter contract inside the pack is `--profile integration --filter 'core/phase2120/...sh'`
- shell wrapper:
  - `tools/selfhost/run_compat_pure_pack.sh`
- SSOT:
  - `docs/development/current/main/phases/phase-29ck/P5-COMPAT-PURE-PACK-LOCK.md`

## Non-goals

- backend-zero mainline acceptance owner になること
- `.hako VM -> LlvmBackendBox -> C-API -> exe` proof を代替すること
- 新しい backend-zero workaround の避難先になること
