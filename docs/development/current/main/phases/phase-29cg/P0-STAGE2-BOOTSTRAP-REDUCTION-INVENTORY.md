---
Status: Accepted
Decision: accepted
Date: 2026-03-09
Scope: `phase-29cg` の初手として、Stage2 default-bootstrap dependency の exact owner と exact condition を固定する。
Related:
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29cg/29cg-10-stage2-bootstrap-reduction-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/selfhost_identity_check.sh
  - tools/selfhost/build_stage1.sh
---

# P0: Stage2 Bootstrap Reduction Inventory

## Purpose

- Stage2 build がどこで default bootstrap に落ちるのかを exact にする
- `stage1-cli` artifact と `launcher-exe` artifact の違いを contract として固定する
- reduction 対象を 1 blocker に絞る

## Exact owner

| Area | Current owner | Reason |
| --- | --- | --- |
| Stage2 build dispatch | [tools/selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) | `launcher-exe` のときだけ `NYASH_BIN=<stage1>` を Stage2 build に渡す |
| Artifact kind decision | [tools/selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh) | `cli-mode=stage0` なら `launcher-exe`、それ以外は `stage1-cli` |
| Artifact production | [tools/selfhost/build_stage1.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/build_stage1.sh) | `launcher-exe` と `stage1-cli` の entry/output 契約を分けている |

## Exact condition

1. `BUILD_ARTIFACT_KIND=launcher-exe`
- Stage2 build は `NYASH_BIN=$STAGE1_BIN` を受け取る

2. `BUILD_ARTIFACT_KIND=stage1-cli`
- Stage2 build は `NYASH_BIN=$STAGE1_BIN` を受け取らない
- note:
  - `stage1-cli artifact is emit-route entry only; using default bootstrap for Stage2 build`

## Capability probe (fixed)

1. raw direct contract is not valid for `stage1-cli`
- `target/selfhost/hakorune.stage1_cli emit program-json apps/tests/hello_simple_llvm.hako` -> `97`
- `target/selfhost/hakorune.stage1_cli --emit-mir-json /tmp/out apps/tests/hello_simple_llvm.hako` -> `97`

2. stage1-bridge helper contract is only partially valid for `stage1-cli`
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-program ...` -> Program(JSON v0)
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir ...` -> MIR(JSON v0)
- `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` -> `emit_program_rc=0 emit_mir_rc=0 llvm_rc=0 verify_rc=0`

3. implication
- next reduction target is not `stage1-cli` as raw `NYASH_BIN`
- next reduction target is `stage1-cli` as a stage1-bridge emit provider inside Stage2 build path

## Current probe result

- `build_stage1.sh` now has an explicit `stage1-cli bridge-first` path when `NYASH_BIN` itself is a `stage1-cli` artifact
- exact probe result:
  - Stage1 bridge emits Program(JSON) successfully
  - for `lang/src/runner/stage1_cli_env.hako`, that Program(JSON) now materializes same-file helper defs across `Main` + `Stage1InputContractBox` + `Stage1ProgramResultValidationBox` + `Stage1ProgramJsonTextGuardBox` + `Stage1ProgramJsonMirCallerBox` + `Stage1SourceMirAuthorityBox` + `Stage1MirResultValidationBox` + `Stage1ProgramJsonCompatBox`
  - `stage1_contract_exec_mode ... emit-mir ...` now succeeds and returns MIR(JSON)
  - with `HAKO_STAGE1_MODULE_DISPATCH_TRACE=1`, `lang.mir.builder.MirBuilderBox.emit_from_program_json_v0` is hit and returns `output_bytes=213003` / `output_handle=97`
  - direct kernel/plugin proof accepts the same `stage1_cli_env.hako` Program(JSON v0) and returns MIR(JSON) with `user_box_decls=[Main, Stage1InputContractBox, Stage1MirResultValidationBox, Stage1ProgramJsonCompatBox, Stage1ProgramJsonMirCallerBox, Stage1ProgramJsonTextGuardBox, Stage1ProgramResultValidationBox, Stage1SourceMirAuthorityBox]`
  - bridge/runtime extern-like names (`env.*`, `nyash.*`) are classified as `Callee::Extern` without depending on `HAKO_MIR_BUILDER_CALL_RESOLVE`
  - clean `build_stage1.sh` bridge-first probe is now green, and the stale-artifact failure mode can be recognized by missing same-file `Stage1*` box defs rather than by route drift
- therefore the helper/source closure bucket is closed for the reduced `stage1_cli_env.hako` proof source, and the handoff target is `phase-29ch` MIR-direct authority tightening rather than more surrogate closure work

## Reduction target

- target:
  - `stage1-cli` artifact 時の Stage2 default-bootstrap dependency
- not target yet:
  - Stage0/auto compat route
  - VM fallback compat lane
  - `launcher-exe` run artifact contract

## Acceptance

- exact owner / exact condition / exact target が 1 枚で読める
- checklist がこの inventory を参照して進められる
- raw direct probe and helper-driven probe are both fixed so the next reduction cannot drift into the wrong contract
- bridge-first probe failure point is fixed so the next execution slice can target stage1 surrogate helper/source closure, not route plumbing or helper-def materialization
