---
Status: Accepted
Decision: accepted
Date: 2026-03-10
Scope: reduced selfhost bootstrap を `Program(JSON v0)` bridge authority から `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM` authority へ移すための専用 phase。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md
---

# Phase 29ch: MIR-Direct Bootstrap Unification

## Goal

`phase-29cg` で固定した reduced `stage1-cli bridge-first bootstrap` authority を次段へ進め、

- `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM`

を bootstrap route authority に寄せる。

この phase の目的は authority 移行であり、`Program(JSON v0)` の削除そのものではない。

## Boundary

- in scope:
  - reduced bootstrap route の authority を `Program(JSON v0)` bridge から MIR-direct へ寄せる
  - bridge を `temporary bootstrap boundary` へ縮退させる
  - reduced case の proof source を MIR-direct authority へ移す
- out of scope:
  - `Program(JSON v0)` bridge の削除
  - generic cleanup や unrelated bridge refactor
  - `phase-29cg` solved bucket の reopen

## Fixed Order

1. `phase-29cg` solved reduced case を authority evidence として pin する
2. reduced bootstrap でどこまで MIR-direct に置換できるか owner/route を inventory 化する
3. one reduced proof source を MIR-direct authority へ移す
4. proof pair を green に保ったまま bridge を `temporary bootstrap boundary` へ縮退させる
5. parity が安定してから、別 phase で `Program(JSON v0)` retirement を切る

## Exact Inventory (reduced proof source)

Target source:
- `lang/src/runner/stage1_cli_env.hako`

Current reduced-case authority:
1. `tools/selfhost/build_stage1.sh`
   - artifact-kind=`stage1-cli` かつ bootstrap artifact-kind=`stage1-cli` のときだけ reduced bootstrap lane を選ぶ
2. `tools/selfhost/lib/stage1_contract.sh`
   - `stage1_contract_exec_mode <bin> emit-mir <entry> <source_text>` を single-step source→MIR contract として注入する
3. `lang/src/runner/stage1_cli_env.hako`
   - env-mode `emit-mir` は source-only authority input を `MirBuilderBox.emit_from_source_v0(...)` へ直接渡す
   - explicit supplied Program(JSON) text がある場合だけ `MirBuilderBox.emit_from_program_json_v0(...)` を compatibility input shape として使う
4. `tools/ny_mir_builder.sh`
   - MIR(JSON) -> backend/VM link だけを担当する

Known non-authority routes:
- `tools/selfhost_exe_stageb.sh` `stageb-delegate`
  - non-stage1-cli artifact build 用の compatibility/bootstrap lane
- `tools/selfhost_exe_stageb.sh` `direct`
  - Stage0 direct `--emit-mir-json` probe 用で、reduced proof source の authority ではない
- linked Rust Stage1 bridge (`src/runner/stage1_bridge/mod.rs`) + embedded `lang/src/runner/stage1_cli.hako`
  - current reduced artifact still links this lane, but it is not accepted as reduced-case authority evidence in `phase-29ch`
  - treat it as `future retire target` until a dedicated slice proves otherwise
- direct raw artifact invocation (`target/selfhost/hakorune.stage1_cli emit ...`)
  - current reduced artifact (`stage1_cli_env.hako`) では raw/subcmd contract を持たず `rc=97`
- `tools/selfhost/run_stage1_cli.sh ... emit ...`
  - compatibility wrapper only; it translates raw `emit` surface into the env mainline contract and is not accepted as reduced-case authority evidence
- explicit supplied Program(JSON) text (`HAKO_STAGE1_PROGRAM_JSON[_TEXT]` / `NYASH_*` / `STAGE1_*`)
  - compatibility-only input shape inside `stage1_cli_env.hako`
  - not accepted as separate authority evidence once source-only `stage1-env-mir-source` is green
- compiled stage1 artifact module dispatch (`crates/nyash_kernel/src/plugin/module_string_dispatch.rs`)
  - this is currently part of the reduced execution path for `BuildBox.emit_program_json_v0` / `MirBuilderBox.emit_from_program_json_v0`
  - it is not a separate authority route, but it is the first implementation owner for gate semantics on compiled stage1 artifacts

## Current Accepted State

- current reduced authority remains:
  - `stage1-env-program`
  - `stage1-env-mir-source`
- source-only authority input is accepted evidence:
  - `lang/src/runner/stage1_cli_env.hako` -> `MirBuilderBox.emit_from_source_v0(...)`
- explicit supplied `Program(JSON)` input remains compatibility-only:
  - live compat keep: `stage1-env-mir-program`
  - cold compat keeps: `stage1-env-mir-legacy`, `stage1-subcmd-mir-program`
    - diagnostics-only from the dedicated cold-compat probe; not part of live env-mainline compat fallback order
- current reduced route is green:
  - `smoke` PASS
  - `G1 full` PASS
  - raw-exact `Program JSON v0` and `MIR JSON v0` match on the current reduced authority route
- `run_stage1_cli.sh` remains a compatibility wrapper, not authority evidence
- delegate remains explicit compat-only / future retire target

Detailed evidence / solved slice log / diagnostics probes:
- `docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md`

## Next Owner Order

1. keep source-only `stage1-env-mir-source` as the current green authority path
2. thin explicit supplied Program(JSON) compat surface
3. touch `lang/src/runner/stage1_cli_env.hako` only if the compat input itself still needs a Stage1-side shim
4. choose the next reduction slice without widening authority
5. keep delegate as explicit compat-only / future retire target until MIR-direct authority is stable

## Acceptance

- reduced bootstrap proof が MIR-direct authority で説明できる
- `Program(JSON v0)` bridge は `temporary bootstrap boundary` としてだけ残る
- `phase-29cg` solved bucket を reopen しない
- JSON v0 deletion phase とは明確に分離されている
