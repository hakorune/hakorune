---
Status: Active
Decision: accepted
Date: 2026-03-15
Scope: `HAKO_CAPI_PURE=1` を必要とする historical pure-lowering routes を compat-only pack として mainline backend-zero proof から分離し、入口と撤去条件を固定する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ck/README.md
  - docs/development/current/main/phases/phase-29ck/P3-THIN-BACKEND-CUTOVER-LOCK.md
  - docs/reference/environment-variables.md
  - tools/smokes/v2/profiles/integration/compat/pure-keep/README.md
  - tools/smokes/v2/profiles/integration/proof/phase2120-legacy-cluster/README.md
  - tools/compat/legacy-codegen/run_compat_pure_pack.sh
---

# P5: Compat Pure Pack Lock

## Goal

- `HAKO_CAPI_PURE=1` を使う historical pure-lowering routes を compat-only pack として mainline proof から切り離す。
- `.hako VM -> LlvmBackendBox -> C-API -> exe` の phase-29ck proof と、phase2120 pure canary/selfhost helper を導線レベルで混線させない。

## Official Meaning

1. mainline backend-zero proof
   - command:
     - `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`
   - required env:
     - `NYASH_LLVM_USE_CAPI=1`
     - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
   - non-goal:
     - `HAKO_CAPI_PURE=1` を要求しない
2. compat-only pure pack
   - owner pack:
     - `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`
     - `tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh`
     - `tools/compat/legacy-codegen/run_compat_pure_pack.sh`
     - `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
   - required env:
     - `NYASH_LLVM_USE_CAPI=1`
     - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
     - `HAKO_CAPI_PURE=1`
  - meaning:
    - historical pure-lowering and old selfhost helper routes
    - `HAKO_CAPI_PURE=1` is a historical alias only when no explicit backend recipe is present; explicit `HAKO_BACKEND_COMPILE_RECIPE=*` keeps precedence
    - current phase2120 active pure canaries are the two live keep pins (`array_set_get`, `loop_count`), locked by `tools/smokes/v2/suites/integration/compat/pure-keep.txt`; the historical archive-backed pins are locked by `tools/smokes/v2/suites/archive/pure-historical.txt`
  - non-goal:
    - current backend-zero acceptance / promotion owner ではない

## Entry Rule

1. compat-only pack を回す時は script 名か log で historical だと明示する
2. mainline proof の failure/success と compat pack の failure/success を混ぜて解釈しない
3. new backend-zero work は compat pack を owner にしてはいけない

## Script Contract

1. `tools/selfhost/run_all.sh`
   - retired alias; do not reintroduce
2. `tools/compat/legacy-codegen/run_compat_pure_pack.sh`
   - canonical historical compat pack wrapper
   - shells into `tools/smokes/v2/profiles/integration/compat/pure-keep/run_pure_keep.sh`, `tools/smokes/v2/profiles/archive/pure-historical/run_pure_historical.sh`, and `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
   - pack orchestration only; not a separate proof owner
3. `tools/compat/legacy-codegen/run_compat_pure_selfhost.sh`
   - compatibility wrapper only
   - archive-later compat wrapper, not a daily owner
   - transport-only shell shim around `tools/compat/legacy-codegen/hako_llvm_selfhost_driver.hako`
   - still depends on the legacy `CodegenBridgeBox` example caller
   - root-first replacement proof exists only on the separate `vm-hako -> LlvmBackendBox` owner lane and is not a drop-in replacement for this wrapper
4. `tools/smokes/v2/profiles/integration/proof/phase2120-legacy-cluster/run_all.sh`
   - full legacy-cluster entry
   - orchestrates the pure keep bucket, archive historical bucket, VM-adapter legacy cluster, and native reference bucket as separate child runners
   - not the canonical compat pure-pack owner anymore
   - must self-identify as compat-only
   - category map lives in `tools/smokes/v2/profiles/integration/proof/phase2120-legacy-cluster/README.md`
   - pure C-API canaries in this pack must use `boundary_pure_helper.sh -> ny-llvmc --driver boundary`; retired direct `hostbridge.extern_invoke("env.codegen", ...)` is outside the pack contract
5. `tools/smokes/v2/suites/integration/compat/pure-keep.txt`
   - canonical suite manifest for the two active pure C-API keep pins
   - keeps the live keep bucket explicit without re-promoting the compat pack to a mainline owner

## Retirement Trigger

compat-only pack は次を全部満たしたら retire 候補になる。

1. historical pure-lowering canary を mainline/native proof へ移管済み
2. old selfhost helper route が current docs から不要になっている
3. `HAKO_CAPI_PURE=1` を current docs で compat-only ではなく removed/no-op に落とせる

## Rule

- compat-only pack に新しい acceptance を積まない
- mainline proof を通すための workaround を compat pack に隠さない
- route widening が必要なら `phase-29ck` を reopen する
