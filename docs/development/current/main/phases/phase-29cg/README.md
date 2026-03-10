---
Status: Accepted (probing)
Decision: accepted
Date: 2026-03-09
Scope: `stage1-cli` artifact 時に Stage2 build が default bootstrap に落ちる依存を、docs-first で削減する。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cf/README.md
  - docs/development/current/main/phases/phase-29cf/P0-VM-FALLBACK-AND-BOOTSTRAP-BOUNDARY-INVENTORY.md
  - docs/development/current/main/phases/phase-29cc/29cc-260-derust-task-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - tools/selfhost_identity_check.sh
  - tools/selfhost/build_stage1.sh
---

# Phase 29cg: Stage2 Bootstrap Reduction

## Goal

`phase-29cf` で `future retire target` に固定した

- `stage1-cli artifact is emit-route entry only; using default bootstrap for Stage2 build`

を、実際に削減するための専用レーンを切る。

Final direction:
- bootstrap でも `Program(JSON v0)` bridge を常設の本線にしない
- 最終的には `stage1-cli` / selfhost mirbuilder から direct MIR へ寄せ、bridge は retire target にする

## Why a separate phase

1. `phase-29cf` は inventory / keep authority の正本であり、実削減とは責務が違う
2. Stage2 dependency は bootstrap boundary の具体的な 1 blocker なので、単独 phase にした方が追いやすい
3. `VM fallback compat lane` と混ぜると判断がぶれる

## Current Target

- current reduction target:
  - [selfhost_identity_check.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost_identity_check.sh)
  - artifact-kind=`stage1-cli` のとき、Stage2 build が default bootstrap に落ちる点
- non-goal:
  - `phase-29cf` の caller bucket をやり直すこと
  - `compat-fallback` lane をこの phase で触ること

## Fixed Order

1. Stage2 default-bootstrap dependency を exact owner / exact condition で inventory 化する
2. `stage1-cli` artifact で Stage2 build を stage1-first に寄せるための contract を定義する
3. reduction を 1 箇所だけ切る acceptance を決める

## Contract Snapshot

- `launcher-exe` artifact:
  - raw `NYASH_BIN=<stage1>` bootstrap contract is valid
  - `tools/selfhost_identity_check.sh` already uses this path for Stage2 build
- `stage1-cli` artifact:
  - raw direct invocation is not a valid bootstrap contract for Stage2 build
  - exact probe:
    - `target/selfhost/hakorune.stage1_cli emit program-json ...` returns `97`
    - `target/selfhost/hakorune.stage1_cli --emit-mir-json ...` returns `97`
  - helper-driven stage1 bridge contract is only partially valid
    - `stage1_contract_exec_mode <stage1-cli> emit-program ...` succeeds
    - the emitted Program(JSON v0) for `stage1_cli_env.hako` now materializes helper defs (`defs_len=22`), but they are entry-local `Main` helper defs only
    - `stage1_contract_exec_mode <stage1-cli> emit-mir ...` now succeeds and returns MIR(JSON)
    - `HAKO_STAGE1_MODULE_DISPATCH_TRACE=1` confirms `lang.mir.builder.MirBuilderBox.emit_from_program_json_v0` is hit and returns `output_bytes=213003` / `output_handle=97`
    - direct kernel proof also exists: the same `stage1_cli_env.hako` Program(JSON v0) is accepted by `nyash_plugin_invoke_by_name_i64(lang.mir.builder.MirBuilderBox, "emit_from_program_json_v0", ...)` and returns MIR(JSON)
    - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` now passes with `emit_program_rc=0 emit_mir_rc=0 llvm_rc=0 verify_rc=0`
  - experimental bootstrap probe:
    - `build_stage1.sh` can now attempt a `stage1-cli bridge-first` Stage2 build when `NYASH_BIN` itself is a `stage1-cli` artifact
    - `lang/src/runner/stage1_cli_env.hako` has a lower-risk `_find_matching_pair_inline` CFG now, helper defs are materialized, and bridge `emit-mir` itself is green
    - bridge/runtime extern-like names (`env.*`, `nyash.*`) are now classified as `Callee::Extern` even when `HAKO_MIR_BUILDER_CALL_RESOLVE` is off; the legacy toggle now only controls unqualified helper-name upgrades
    - Rust-side minimal safe-keep from mixed stash review is now applied in two places:
      - `src/mir/passes/callsite_canonicalize.rs`
      - `src/runner/json_v0_bridge/lowering/program.rs`
      - both now suffix unsuffixed dotted helper `Global` callees when a matching `name/arity` definition exists
    - current exact blocker in that path is surrogate helper/source closure in the stage1 bridge lane
    - kernel-direct `lang.compiler.entry.using_resolver_box.resolve_for_source` is intentionally stubbed empty in `module_string_dispatch.rs`
    - kernel-direct `lang.compiler.build.build_box.emit_program_json_v0` delegates to Rust `source_to_program_json_v0(...)`, which materializes entry-local `Main` helper defs only
    - as a result, the bridge-first Program(JSON v0) for `stage1_cli_env.hako` still has `defs_len=22`, `box=Main` only, and `imports` empty
    - the reduced Stage2 object therefore keeps imported helper owners unresolved:
      - `FuncScannerBox.*`
      - `StageBJsonBuilderBox.*`
      - `Stage1UsingResolverBox.*`
      - `BuildBox.emit_program_json_v0`
      - `MirBuilderBox.emit_from_program_json_v0`
      - `BoxTypeInspectorBox.*`
      - `StringHelpers.int_to_str`
    - the next reduction owner is therefore stage1 surrogate helper/source closure in:
      - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
      - `src/stage1/program_json_v0.rs`
      not bridge return-path, extern classification, or current LLVM PHI repair
- therefore `phase-29cg` does not treat `stage1-cli` as a drop-in `NYASH_BIN`; it targets a narrower reduction:
  - lift the stage1-bridge helper contract into the Stage2 build path for one reduced case
  - then retire the bridge dependency itself once direct MIR parity is available for the reduced case

## Acceptance

- `phase-29cf` とは別に、Stage2 reduction target が独立レーンとして読める
- `tools/selfhost_identity_check.sh` の current fallback note がどの条件で出るか docs から一意に読める
- checklist に `owner / blocker / acceptance / non-goal` が揃っている
- `stage1-cli` reduction target is stated as `bridge-first Stage2 build`, not as `raw NYASH_BIN replacement`
- current G1 route is unchanged until the bridge-first Stage2 object closes helper/runtime symbols for the reduced case
- exact next reduction focus is surrogate helper/source closure for the bridge-first Stage2 object, not bridge return-path, extern classification, or current LLVM PHI wiring
- mixed worker stash lanes outside that narrow Rust fix remain deferred until they can be split into single-owner patches
