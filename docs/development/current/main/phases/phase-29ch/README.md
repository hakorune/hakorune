---
Status: Accepted
Decision: accepted
Date: 2026-03-10
Scope: reduced selfhost bootstrap を `Program(JSON v0)` bridge authority から `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM` authority へ移すための専用 phase。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
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

Evidence (2026-03-11):
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir apps/tests/hello_simple_llvm.hako "$(cat apps/tests/hello_simple_llvm.hako)"` -> `rc=0`
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir lang/src/runner/stage1_cli_env.hako "$(cat lang/src/runner/stage1_cli_env.hako)"` -> `rc=0`
- `bash tools/selfhost/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli emit program-json apps/tests/hello_simple_llvm.hako` -> `rc=0`
- `bash tools/selfhost/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli emit mir-json apps/tests/hello_simple_llvm.hako` -> `rc=0`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli.next --force-rebuild` -> PASS
- `bash tools/selfhost_identity_check.sh --mode smoke` -> PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` -> PASS (`Program JSON v0` raw match; `MIR JSON v0` raw match on the current reduced authority route)
- exact raw diff probe is fixed to `bash tools/dev/phase29ch_raw_mir_diff_probe.sh [entry]` (default: `lang/src/compiler/entry/compiler_stageb.hako`)
- route-mode branchpoint probe is fixed to `bash tools/dev/phase29ch_route_mode_matrix.sh [entry]`
- same-route repeatability probe is fixed to `bash tools/dev/phase29ch_same_route_repeat_probe.sh [entry]`
- fixed-Program repeatability probe is fixed to `bash tools/dev/phase29ch_fixed_program_mir_repeat_probe.sh [entry]`
- transient-boundary probe is fixed to `bash tools/dev/phase29ch_transient_boundary_probe.sh [entry]`
- source-route direct probe is fixed to `bash tools/dev/phase29ch_source_route_direct_probe.sh [entry]`
  - diagnostics-only: builds a temporary helper artifact and calls `MirBuilderBox.emit_from_source_v0(...)` directly on a compiled artifact
  - not accepted as reduced-case authority evidence
- stage1 env file-context probe is fixed to `bash tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh`
  - diagnostics-only: emits/runs temporary `stage1_cli_env`-shaped clones through Stage1/Stage2 and narrows where the source-route promotion first turns red
- impossible-gate probe is fixed to `bash tools/dev/phase29ch_impossible_gate_probe.sh [entry]`
- bridge-bypass probe is fixed to `bash tools/dev/phase29ch_bridge_bypass_probe.sh [entry]`
- current authority shell contract now pins `stage1_contract_exec_mode` to `HAKO_SELFHOST_NO_DELEGATE=1` + `HAKO_MIR_BUILDER_DELEGATE=0` by default; delegate route is explicit compat only
- `tools/selfhost/lib/stage1_contract.sh` now fail-fast rejects `rc=0` emit calls that do not actually return Program/MIR JSON payloads
- compiled stage1 artifacts currently satisfy `BuildBox` / `MirBuilderBox` calls via `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`; impossible-gate semantics must therefore hold there too
- current branch point is no longer whether to land canonical compare; it is whether `.hako` MirBuilder ordering should later be tightened until raw-text MIR also converges

Current compare decision (2026-03-11):
- `phase-29ch` now uses `semantic canonical match` for G1 MIR compare and keeps raw MIR exact diff as tightening evidence.
- compare rules SSOT: `docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md`
- fast regression entry: `python3 -m unittest tools.selfhost.lib.tests.test_mir_canonical_compare`
- Raw exact MIR equality has now been reached again for the current reduced authority route on `compiler_stageb.hako`; the canonical compare policy remains in place for future widenings and for narrowing future non-semantic noise without changing route authority.

Current branch point (2026-03-11):
- the last solved reduction slice is `launcher-exe`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --out target/selfhost/hakorune.launcher_from_stage1_cli --force-rebuild` -> PASS
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli.next emit-program lang/src/runner/launcher.hako "$(cat lang/src/runner/launcher.hako)"` now emits Program(JSON v0) with `defs_boxes=[HakoCli]` and bare-using imports including `MirBuilderBox`
- `... emit-mir ...` now emits `user_box_decls=[HakoCli, Main]` and lowers `HakoCli.run/1` on the current reduced authority route
- the former active blocker was G1 full MIR exact diff on `compiler_stageb.hako`; that exact diff is now closed on the current reduced authority route
- raw determinism closure note: the effective repair owner was the compiled-artifact Rust provider path under module dispatch, specifically `src/runner/json_v0_bridge/lowering/merge.rs` and `src/runner/json_v0_bridge/lowering/try_catch.rs`, where merge-variable name collection now uses `BTreeSet<String>` instead of `HashSet<String>`. This stabilizes the copy/materialization order that had been drifting first at `StageBArgsBox.resolve_src/1` block 8.
- current evidence after that repair:
  1. `bash tools/dev/phase29ch_fixed_program_mir_repeat_probe.sh` is quiet/raw-exact for `lang/src/compiler/entry/compiler_stageb.hako`
  2. `bash tools/dev/phase29ch_route_mode_matrix.sh` is quiet for the same source
  3. fresh `G1 full` is raw-exact green for both `Program JSON v0` and `MIR JSON v0`
- therefore the current preferred order is now: keep `stage1-env-program` + `stage1-env-mir-source` as the only reduced authority evidence, keep `run_stage1_cli.sh` as a compatibility wrapper over that contract, and use the first true bootstrap reduction slice to promote source->MIR directly without widening authority
- next owner order remains fixed:
  1. keep source-only `stage1-env-mir-source` as the current green authority path
  2. thin explicit supplied Program(JSON) text to a smaller compat-only surface (`tools/selfhost/lib/identity_routes.sh` -> `tools/selfhost/run_stage1_cli.sh` -> `tools/selfhost/lib/stage1_contract.sh`)
  3. touch `lang/src/runner/stage1_cli_env.hako` only if the compat input itself still needs a Stage1-side shim
  4. choose the next reduction slice without widening authority
  5. keep delegate as explicit compat-only / future retire target until MIR-direct authority is stable
- transient-boundary proof rule: `bash tools/dev/phase29ch_transient_boundary_probe.sh [entry]` must stay raw-exact quiet for current reduced sources. It compares source-only authority `emit-mir` against the same saved Program(JSON v0) supplied explicitly, so the next reduction slice can prove the transient boundary is semantically transparent before shrinking it.
- source-route promotion note (2026-03-11): `MirBuilderBox.emit_from_source_v0(...)` is now accepted as reduced-case authority evidence for source-only `stage1-env-mir-source`. The previously red env-wrapper/file-context cluster turned green after fixing the compiled-artifact Rust provider path under `src/runner/json_v0_bridge/lowering/if_else.rs` -> `src/runner/json_v0_bridge/lowering/merge.rs` to use PHI-unified `if` joins. `bash tools/dev/phase29ch_source_route_materialize_probe.sh`, `bash tools/dev/phase29ch_selfhost_source_route_helper_probe.sh`, and `bash tools/dev/phase29ch_selfhost_source_route_bisect_probe.sh` stay green, and `bash tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh` is now green for `env_source_only`, `env_mode_no_supplied`, `env_branch_literal_empty`, `env_branch_helper_empty`, `env_branch_helper_env_text`, `env_branch_select_then_call`, `env_branch_same_callee_two_calls`, `mini_env`, `full`, `thin`, and `thin_imports`. The focused case now emits `block 9: phi dst=31 incoming=[[11,8],[19,15]]` before `emit_from_source_v0(selected_input, null)`, and the previous `[freeze:contract][stage1_mir_builder] source decode failed` path is gone on fresh Stage1/Stage2 artifacts.
- detour prevention for the next slice: `src/runner/modes/vm_hako/compile_bridge.rs` already contains a Rust direct source→MIR helper, but it is reference-only for `phase-29ch`. Do not promote it into current selfhost authority while choosing the next reduction slice.

Route guard lock:
- `tools/selfhost_identity_check.sh --mode full` must observe
  - `program-json`: `stage1-env-program`
  - `mir-json`: `stage1-env-mir-source`
- `stage1-env-mir-program` / `stage1-env-mir-legacy` / `stage1-subcmd-mir-program` are compatibility-only and are not accepted as reduced-case authority evidence
- `tools/selfhost/build_stage1.sh` stage1-cli capability probe and `identity_routes.sh` preflight share the same env-mainline capability helper; the reduced authority is checked once and reused
- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` capability probe also uses the same env-mainline contract and must fail fast if the artifact only exposes compat/stale routes
- `tools/selfhost/build_stage1.sh` bridge-first bootstrap body also uses the same shared env-mainline helper for actual source->MIR emission; manual `stage1_contract_exec_mode ... emit-mir` + local marker checks are no longer the mainline authority path
- route retirement rule: when this phase discovers a non-authority route, the route must be documented immediately as exactly one of `compat-only keep` or `future retire target`. Discovery alone must not create new authority evidence.

## Acceptance

- reduced bootstrap proof が MIR-direct authority で説明できる
- `Program(JSON v0)` bridge は `temporary bootstrap boundary` としてだけ残る
- `phase-29cg` solved bucket を reopen しない
- JSON v0 deletion phase とは明確に分離されている
