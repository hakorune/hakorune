---
Status: Accepted (closeout-ready)
Decision: accepted
Date: 2026-03-10
Scope: reduced selfhost bootstrap を `Program(JSON v0)` bridge authority から `parser -> selfhost mirbuilder -> MIR(JSON) -> backend/VM` authority へ移すための専用 phase。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29cg/README.md
  - docs/development/current/main/phases/phase-29ci/README.md
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
5. parity が安定してから、separate future-wave `phase-29ci` で `Program(JSON v0)` retirement を切る

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
  - `lang/src/runner/stage1_cli_env.hako::Stage1SourceProgramAuthorityBox` -> exact `stage1-env-program`
  - `lang/src/runner/stage1_cli_env.hako::Stage1SourceMirAuthorityBox` -> `MirBuilderBox.emit_from_source_v0(...)`
  - shared checked Program(JSON)->MIR handoff lives in `Stage1ProgramJsonMirCallerBox`
- explicit supplied `Program(JSON)` input remains compatibility-only:
  - monitor-only explicit compat keep: `stage1-env-mir-program`
    - minimal selfhost helper calling `MirBuilderBox.emit_from_program_json_v0(...)` is green
    - `stage1_cli_env.hako` now keeps shared input/env contract, exact source-only emit-program authority, checked Program(JSON)->MIR caller, text guard, source-mainline emit-mir authority, Program(JSON) validation, MIR result validation, and explicit-compat in separate same-file boxes (`Stage1InputContractBox` / `Stage1SourceProgramAuthorityBox` / `Stage1ProgramJsonMirCallerBox` / `Stage1ProgramJsonTextGuardBox` / `Stage1SourceMirAuthorityBox` / `Stage1ProgramResultValidationBox` / `Stage1MirResultValidationBox` / `Stage1ProgramJsonCompatBox`)
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
- exact reduced-artifact emit contract is a separate proof from `build_stage1.sh --artifact-kind stage1-cli`; monitor it through `run_stage1_cli.sh emit {program-json|mir-json}` and `tools/smokes/v2/profiles/integration/selfhost/phase29ci_stage1_cli_exact_emit_contract_vm.sh`
- delegate remains explicit compat-only / future retire target

Detailed evidence / solved slice log / diagnostics probes:
- `docs/development/current/main/phases/phase-29ch/29ch-20-route-evidence-and-probes.md`

## Final Owner Order (locked)

1. keep source-only `stage1-env-mir-source` as the current green authority path
2. keep explicit supplied Program(JSON) compat monitor-only and frozen
3. touch `lang/src/runner/stage1_cli_env.hako` only if a later execute-lane slice proves a Stage1-side shim is still needed
   - `MirBuilderBox.emit_from_program_json_v0(...)` itself is already green in minimal selfhost helper shape
   - `stage1_cli_env.hako` wrapper-level compat branching is now thin enough
   - raw direct `stage1-cli` lane absence is a separate future slice (`tools/dev/phase29ch_raw_direct_stage1_cli_probe.sh` + `tools/dev/phase29ch_program_json_helper_exec_probe.sh`, current `rc=97`)
4. move to the next actual reduction owner on the authority path
   - first: `src/stage1/program_json_v0.rs`
   - only if proof still demands it: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
   - latest owner-1 reduction: strict source parsing in `src/stage1/program_json_v0.rs` no longer accepts bare script-body fallback; the Rust surrogate now requires explicit parseable source shape instead of synthesizing `static box Main`
   - latest authority tightening: `src/host_providers/mir_builder.rs` now uses `emit_program_json_v0_for_strict_authority_source(...)` for `stage1-env-mir-source`, so current source authority does not depend on dev-local alias sugar preexpansion and does not reassemble the authority check locally
   - latest authority fail-fast tightening: `emit_program_json_v0_for_strict_authority_source(...)` rejects compat-only relaxed source shapes up front on the source authority path, with explicit relaxed-keep reason tags (`dev-local-alias-sugar`) instead of a generic parse failure
   - latest owner-1 API reduction: the old `source_to_program_json_v0(...)` alias is now gone, `source_to_program_json_v0_strict(...)` stays owner-local, relaxed dev-local alias / launcher keep is also owner-local on `source_to_program_json_v0_relaxed(...)`, future-retire `stage1_bridge/program_json/mod.rs` uses `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`, and cross-crate callers no longer need a standalone relaxed entrypoint
   - latest owner-2 minimal tightening: `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` no longer normalizes `NYASH_STAGE1_MODE` / `HAKO_STAGE1_MODE` / `STAGE1_EMIT_PROGRAM_JSON` locally; it delegates current-mode build surrogate selection to owner-1 `emit_program_json_v0_for_current_stage1_build_box_mode(...)`, while launcher/no-mode keep stays behind that helper as owner-local route policy
   - latest owner-1 surface thinning: source-shape reason helpers (`source_program_json_v0_relaxed_keep_reason` / `source_needs_program_json_v0_relaxed`) and direct route emit internals are now owner-local; cross-crate callers use `emit_program_json_v0_for_current_stage1_build_box_mode(...)`, while owner-local `emit_program_json_v0_for_stage1_build_box(...)` and build-box route emission / `select_program_json_v0_build_route(...)` / `ProgramJsonV0BuildRoute` stay routing-local inside `routing.rs`
   - latest env SSOT cleanup: current-mode build surrogate selection now reuses `crate::config::env::stage1::emit_program_json()`; legacy `STAGE1_EMIT_PROGRAM_JSON=1` still proves strict-authority mode through the same shared helper
   - latest owner-1 caller cleanup: cross-crate authority callers no longer read source-shape objects directly and no longer stage the authority check themselves; they use `emit_program_json_v0_for_strict_authority_source(...)`, while source-shape enum/info both stay crate-local
   - latest owner-1 contract cleanup: provider callers now reuse owner-1 fail-fast formatting (`strict_authority_rejection()`), and build-route trace no longer leaks as a cross-crate read model
   - latest owner-1 payload cleanup: build emissions are now owner-local only; borrowed payload access, route label/reason getters, and cross-crate route-summary reads are gone
   - latest proof refresh: after the shared env SSOT swap and build-emission narrowing, `build_stage1.sh` Stage1/Stage2 rebuild, `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`, and `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build` are green again
   - latest owner-1 structure cleanup: `src/stage1/program_json_v0.rs` is now a façade over `routing.rs` / `extract.rs` / `lowering.rs`, and parse/lower orchestration stays owner-local so cross-crate callers cannot bypass the explicit authority/compat/build surface
   - latest owner-2 compat narrowing: compiled `BuildBox.emit_program_json_v0(...)` no longer broad-relaxes every no-mode source; it now keeps the relaxed path only when the source shape still contains current dev-local alias sugar (`@local = ...` / launcher keep)
   - latest proof refresh: after the current-mode build surrogate shift, `build_stage1.sh` Stage1/Stage2 rebuild, `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`, and `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build` are green again
   - rejected narrowing for now: strict default cannot switch to `Main`-only helper defs yet; fresh Stage2 build loses same-file `Stage1*` box closure and fails link
   - owner-1 operation card:
      - authority source caller => `emit_program_json_v0_for_strict_authority_source(...)`
      - explicit compat keep => owner-local `source_to_program_json_v0_relaxed(...)` only; do not cross crate boundary
      - build surrogate caller => `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
      - anything else => keep owner-local; do not add a new cross-crate entrypoint
   - future-retire bridge note:
     - Stage1 bridge mode classification now stays in `src/runner/stage1_bridge/args.rs::Stage1ArgsMode`; `plan.rs` / `stub_emit.rs` no longer re-infer it from a bool + env reread
     - backend CLI hint extraction now stays in `src/runner/stage1_bridge/args.rs::Stage1Args::backend_cli_hint()`; child-env helpers do not parse raw argv windows themselves
     - bridge entry child/enable guard + trace logging now live in `src/runner/stage1_bridge/entry_guard.rs`; `mod.rs` no longer owns those checks inline
     - `src/runner/stage1_bridge/args.rs::Stage1Args::stub_exec_plan()` now carries stub capture-vs-delegate selection; `route_exec/stub.rs` no longer re-infers emit-vs-run from `Stage1ArgsMode` or a `stub_emit` helper
     - `src/runner/stage1_bridge/plan.rs::Stage1BridgePlan` now carries the exact execution plan; `route_exec/direct.rs` no longer branches on a second route enum copy
     - `src/runner/stage1_bridge/env.rs` is a thin child-env facade; runtime defaults / Stage1 alias propagation / parser+using toggles live in `env/runtime_defaults.rs` / `env/stage1_aliases.rs` / `env/parser_stageb.rs`
     - `src/runner/stage1_bridge/modules.rs` owns `HAKO_STAGEB_MODULES_LIST` / `HAKO_STAGEB_MODULE_ROOTS_LIST` payload generation and child-env apply; `parser_stageb.rs` no longer writes those keys inline
     - `src/runner/stage1_bridge/route_exec.rs` is now a thin facade; route-to-executor dispatch stays there, binary-only direct route execution + direct-route exit-code mapping live in `route_exec/direct.rs`, and Stage1 stub route facade lives in `route_exec/stub.rs`
     - `src/runner/stage1_bridge/direct_route/mod.rs` is a thin facade; MIR compile lives in `direct_route/compile.rs`, and emit output-path resolution / JSON write live in `direct_route/emit.rs`
     - `src/runner/stage1_bridge/emit_paths.rs` owns bridge-local MIR / Program(JSON) output-path resolution; `stub_emit.rs` and `direct_route/emit.rs` no longer duplicate the MIR env alias policy
     - `src/runner/stage1_bridge/stub_emit.rs` is a thin facade; stdout parse / validation live in `stub_emit/parse.rs`, and writeback policy lives in `stub_emit/writeback.rs`
     - `src/runner/stage1_bridge/program_json/mod.rs` is the only remaining crate-local non-routing strict-parse consumer and it must use `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)`
     - Stage1 stub entry resolution + child command/env assembly + prepare-failure mapping live in `src/runner/stage1_bridge/stub_child.rs`; `route_exec/stub.rs` no longer owns the prepare error log + `97` mapping
     - Stage1 stub plain delegate-status execution + child-spawn-failure mapping live in `src/runner/stage1_bridge/stub_delegate.rs`; `route_exec/stub.rs` now only selects `stub_exec_plan()` branch
     - bridge-local `emit-program-json-v0` file I/O lives in `src/runner/stage1_bridge/program_json/mod.rs`
     - `src/runner/stage1_bridge/stub_emit.rs` stays a thin facade; Stage1 stub `emit` stdout parsing / validation live in `stub_emit/parse.rs`, and output-path writeback lives in `stub_emit/writeback.rs`
     - `src/runner/stage1_bridge/mod.rs` stays a thin delegate and must not regain child/enable entry guard checks, child command/env assembly, or JSON line parsing / writeback policy
     - do not call `source_to_program_json_v0_strict(...)` from `src/runner/stage1_bridge/**`
5. do not spend the next slice on shell/probe boundary cleanup or raw direct lane revival
6. keep delegate as explicit compat-only / future retire target until MIR-direct authority is stable

## Acceptance

- reduced bootstrap proof が MIR-direct authority で説明できる
- `Program(JSON v0)` bridge は `temporary bootstrap boundary` としてだけ残る
- `phase-29cg` solved bucket を reopen しない
- JSON v0 deletion phase とは明確に分離されている

## Closeout Judgment

1. current reduced bootstrap proof is now explainable by MIR-direct authority, not by JSON v0 route authority
2. `Program(JSON v0)` bridge is fixed as `temporary bootstrap boundary` only for this phase
3. the next retirement work is cut as separate `phase-29ci`, so `phase-29ch` no longer needs to mix authority migration and JSON v0 deletion
