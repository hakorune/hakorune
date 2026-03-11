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
- explicit supplied Program(JSON) text (`STAGE1_PROGRAM_JSON_TEXT`)
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
  - `lang/src/runner/stage1_cli_env.hako::Stage1SourceMirAuthorityBox` -> `MirBuilderBox.emit_from_source_v0(...)`
- explicit supplied `Program(JSON)` input remains compatibility-only:
  - monitor-only explicit compat keep: `stage1-env-mir-program`
    - minimal selfhost helper calling `MirBuilderBox.emit_from_program_json_v0(...)` is green
    - `stage1_cli_env.hako` now keeps shared input/env contract, emit-program authority, emit-program validation, source-mainline, MIR result validation, and explicit-compat in separate same-file boxes (`Stage1InputContractBox` / `Stage1ProgramAuthorityBox` / `Stage1ProgramResultValidationBox` / `Stage1SourceMirAuthorityBox` / `Stage1MirResultValidationBox` / `Stage1ProgramJsonCompatBox`)
    - explicit compat MIR call and mixed-input fail-fast gate are quarantined in `Stage1ProgramJsonCompatBox` inside `lang/src/runner/stage1_cli_env.hako`
    - live text transport reuses the existing `STAGE1_SOURCE_TEXT` contract
    - exact-only compat helper / mode / sentinel entry (`stage1_contract_exec_program_json_compat()` / `emit-mir-program` / `__stage1_program_json__`) are centralized in `tools/selfhost/lib/stage1_contract.sh`
    - current caller inventory is probe/helper-owned only; this route is not part of reduced authority evidence
    - legacy `STAGE1_PROGRAM_JSON_TEXT` is now diagnostics-only / fail-fast only and is no longer injected by live shell helpers
    - `stage1_contract.sh` no longer carries retired path transport; live shell compat is exact-helper only
    - explicit mode is exact-only: `emit-mir-program`
    - plain `emit-mir` now fail-fast on mixed-in Program(JSON) text
    - legacy alias forms such as `emit_mir_program` are rejected
    - removal is still blocked because raw `stage1-cli` artifacts do not execute helper sources directly (`rc=97`)
    - generic raw direct `stage1-cli` lane is absent on green artifacts (`<bin> <source>` / `emit ...` / helper execute => `rc=97`)
  - no separate cold compat lane remains on the current green route
    - diagnostics-only from the dedicated cold-compat probe; legacy env shape now returns `none`, retired raw wrapper sugar also returns `none`, and only the explicit helper still reports `stage1-env-mir-program`
  - raw `run_stage1_cli.sh ... --from-program-json` is retired from the live wrapper surface
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
2. keep explicit supplied Program(JSON) compat monitor-only and frozen
3. touch `lang/src/runner/stage1_cli_env.hako` only if a later execute-lane slice proves a Stage1-side shim is still needed
   - `MirBuilderBox.emit_from_program_json_v0(...)` itself is already green in minimal selfhost helper shape
   - `stage1_cli_env.hako` wrapper-level compat branching is now thin enough
   - raw direct `stage1-cli` lane absence is a separate future slice (`tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` + `tools/dev/phase29ch_program_json_helper_exec_probe.sh`, current `rc=97`)
4. move to the next actual reduction owner on the authority path
   - first: `src/stage1/program_json_v0.rs`
   - only if proof still demands it: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - latest owner-1 reduction: `source_to_program_json_v0(...)` no longer accepts bare script-body fallback; the Rust surrogate now requires explicit parseable source shape instead of synthesizing `static box Main`
   - latest authority tightening: `src/host_providers/mir_builder.rs` now uses the strict default surrogate for `stage1-env-mir-source`, so current source authority does not depend on dev-local alias sugar preexpansion
   - latest owner-1 API reduction: `source_to_program_json_v0(...)` is now strict-by-default, and relaxed dev-local alias / launcher keep moved to explicit `source_to_program_json_v0_relaxed(...)`
   - latest owner-2 minimal tightening: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` routes normalized `emit-program` authority through the strict default surrogate, while launcher/no-mode keep stays on the explicit relaxed helper
   - rejected narrowing for now: strict default cannot switch to `Main`-only helper defs yet; fresh Stage2 build loses same-file `Stage1*` box closure and fails link
5. do not spend the next slice on shell/probe boundary cleanup or raw direct lane revival
6. keep delegate as explicit compat-only / future retire target until MIR-direct authority is stable

## Acceptance

- reduced bootstrap proof が MIR-direct authority で説明できる
- `Program(JSON v0)` bridge は `temporary bootstrap boundary` としてだけ残る
- `phase-29cg` solved bucket を reopen しない
- JSON v0 deletion phase とは明確に分離されている
