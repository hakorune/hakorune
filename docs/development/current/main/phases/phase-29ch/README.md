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
   - env-mode `emit-mir` が source text から `_build_program_json()` を internal transient として組み立て、`MirBuilderBox.emit_from_program_json_v0(...)` へ渡す
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
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` -> PASS (`Program JSON v0` raw match; `MIR JSON v0` canonical match with raw diff retained at `/tmp/g1_mir_diff.txt.raw`)
- exact raw diff probe is fixed to `bash tools/dev/phase29ch_raw_mir_diff_probe.sh [entry]` (default: `lang/src/compiler/entry/compiler_stageb.hako`)
- route-mode branchpoint probe is fixed to `bash tools/dev/phase29ch_route_mode_matrix.sh [entry]`
- same-route repeatability probe is fixed to `bash tools/dev/phase29ch_same_route_repeat_probe.sh [entry]`
- fixed-Program repeatability probe is fixed to `bash tools/dev/phase29ch_fixed_program_mir_repeat_probe.sh [entry]`
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
- Raw exact MIR equality remains the follow-up target after `G1 full` is green again.

Current branch point (2026-03-11):
- the last solved reduction slice is `launcher-exe`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --out target/selfhost/hakorune.launcher_from_stage1_cli --force-rebuild` -> PASS
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli.next emit-program lang/src/runner/launcher.hako "$(cat lang/src/runner/launcher.hako)"` now emits Program(JSON v0) with `defs_boxes=[HakoCli]` and bare-using imports including `MirBuilderBox`
- `... emit-mir ...` now emits `user_box_decls=[HakoCli, Main]` and lowers `HakoCli.run/1` on the current reduced authority route
- the former active blocker was G1 full MIR exact diff on `compiler_stageb.hako`; it is now downgraded to tightening evidence because the canonical compare is green and still reports the raw diff
- therefore the current preferred order is: keep `stage1-env-program` + `stage1-env-mir-source` as the only reduced authority evidence, keep `run_stage1_cli.sh` as a compatibility wrapper over that contract, and decide whether raw MIR determinism needs tightening before widening the next bootstrap slice
- current raw determinism note: the mismatch is not owned by a late `jsonfrag_normalizer_box.hako` text reorder pass. `compiler_stageb.hako` had diverged across `default`, `internal-only`, and `delegate-only` route modes on a single Stage1 binary, so the authority shell contract is now pinned to `internal-only` by default. After that pin, same-route repeatability still diverges for repeated `default`, repeated `internal-only`, and repeated `delegate-only` emits, so the first owner is now execution-path nondeterminism rather than route selection alone. Repeated `emit-program` is raw exact-match, but repeated `emit-mir` from a fixed saved Program(JSON v0) payload still diverges at `StageBArgsBox.resolve_src/1` block 8. The impossible gate (`HAKO_SELFHOST_NO_DELEGATE=1 HAKO_MIR_BUILDER_DELEGATE=0 HAKO_MIR_BUILDER_INTERNAL=0`) no longer emits MIR after the fix in `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`; compiled stage1 artifacts had been bypassing `.hako` MirBuilder gate semantics there. Therefore the current blocker has moved back to raw MIR ordering under the Rust provider path (`src/host_providers/mir_builder.rs` / `runner::json_v0_bridge`) that module dispatch actually executes. `stage1_contract_exec_mode` owns fail-fast validation for silent-success emits. Delegate remains explicit compat-only and is still the retire-target route after MIR-direct authority is stable.
- current owner order remains fixed:
  1. `tools/selfhost/lib/identity_compare.sh`
  2. `tools/selfhost/lib/mir_canonical_compare.py`
  3. generator-order stabilization only after the narrow compare policy is proven in green G1 full

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
