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
- `tools/selfhost/run_stage1_cli.sh ... emit mir-json ...`
  - current reduced artifact (`stage1_cli_env.hako`) では raw/subcmd contract を持たず `rc=97`

Evidence (2026-03-10):
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir apps/tests/hello_simple_llvm.hako "$(cat apps/tests/hello_simple_llvm.hako)"` -> `rc=0`
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir lang/src/runner/stage1_cli_env.hako "$(cat lang/src/runner/stage1_cli_env.hako)"` -> `rc=0`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli.next --force-rebuild` -> PASS
- `bash tools/selfhost_identity_check.sh --mode smoke` -> PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` -> PASS
- `bash tools/selfhost_identity_check.sh --mode full` -> PASS

Current branch point (2026-03-10):
- the next reduction slice is `launcher-exe`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --out target/selfhost/hakorune.launcher_from_stage1_cli --force-rebuild` -> PASS
- but `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli.next emit-program lang/src/runner/launcher.hako "$(cat lang/src/runner/launcher.hako)"` still emits Program(JSON v0) without `defs`, and `... emit-mir ...` still emits `newbox HakoCli` with empty `user_box_decls`
- direct scanner probe `target/release/hakorune --backend vm lang/src/compiler/tests/funcscanner_launcher_probe.hako` currently freezes in `FuncScannerBox._scan_methods/4` with `[joinir/reject_detail] box=generic_loop_v0 reason=no_valid_loop_var_candidates`
- therefore the current preferred next move is `BoxCount` on compiler expressivity for `FuncScannerBox.scan_all_boxes(...)` / `_scan_methods/4`; bootstrap-only Rust surrogate expansion is the fallback branch and must be kept in a separate commit series

Route guard lock:
- `tools/selfhost_identity_check.sh --mode full` must observe
  - `program-json`: `stage1-env-program`
  - `mir-json`: `stage1-env-mir-source`
- `stage1-env-mir-program` / `stage1-env-mir-legacy` / `stage1-subcmd-mir-program` are compatibility-only and are not accepted as reduced-case authority evidence
- `tools/selfhost/build_stage1.sh` stage1-cli capability probe and `identity_routes.sh` preflight share the same env-mainline capability helper; the reduced authority is checked once and reused
- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` capability probe also uses the same env-mainline contract and must fail fast if the artifact only exposes compat/stale routes
- `tools/selfhost/build_stage1.sh` bridge-first bootstrap body also uses the same shared env-mainline helper for actual source->MIR emission; manual `stage1_contract_exec_mode ... emit-mir` + local marker checks are no longer the mainline authority path

## Acceptance

- reduced bootstrap proof が MIR-direct authority で説明できる
- `Program(JSON v0)` bridge は `temporary bootstrap boundary` としてだけ残る
- `phase-29cg` solved bucket を reopen しない
- JSON v0 deletion phase とは明確に分離されている
