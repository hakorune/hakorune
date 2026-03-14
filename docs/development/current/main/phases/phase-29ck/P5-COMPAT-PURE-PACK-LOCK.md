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
  - tools/smokes/v2/profiles/integration/core/phase2120/README.md
  - tools/smokes/v2/profiles/integration/core/phase2120/run_all.sh
  - tools/selfhost/run_all.sh
  - tools/selfhost/run_hako_llvm_selfhost.sh
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
     - `tools/smokes/v2/profiles/integration/core/phase2120/run_all.sh`
     - `tools/selfhost/run_all.sh`
     - `tools/selfhost/run_hako_llvm_selfhost.sh`
   - required env:
     - `NYASH_LLVM_USE_CAPI=1`
     - `HAKO_V1_EXTERN_PROVIDER_C_ABI=1`
     - `HAKO_CAPI_PURE=1`
   - meaning:
     - historical pure-lowering and old selfhost helper routes
   - non-goal:
     - current backend-zero acceptance / promotion owner ではない

## Entry Rule

1. compat-only pack を回す時は script 名か log で historical だと明示する
2. mainline proof の failure/success と compat pack の failure/success を混ぜて解釈しない
3. new backend-zero work は compat pack を owner にしてはいけない

## Script Contract

1. `tools/selfhost/run_all.sh`
   - compatibility wrapper only
   - must print that it delegates to the compat pure pack
2. `tools/selfhost/run_hako_llvm_selfhost.sh`
   - compatibility wrapper only
   - must print that it is a historical pure selfhost helper
3. `tools/smokes/v2/profiles/integration/core/phase2120/run_all.sh`
   - historical pure/TM pack entry
   - must self-identify as compat-only
   - category map lives in `tools/smokes/v2/profiles/integration/core/phase2120/README.md`

## Retirement Trigger

compat-only pack は次を全部満たしたら retire 候補になる。

1. historical pure-lowering canary を mainline/native proof へ移管済み
2. old selfhost helper route が current docs から不要になっている
3. `HAKO_CAPI_PURE=1` を current docs で compat-only ではなく removed/no-op に落とせる

## Rule

- compat-only pack に新しい acceptance を積まない
- mainline proof を通すための workaround を compat pack に隠さない
- route widening が必要なら `phase-29ck` を reopen する
