---
Status: Accepted
Decision: accepted
Date: 2026-03-12
Scope: `phase-29ch` の active evidence / diagnostics probes / solved slice log を README 本体から分離して保持する。
Related:
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
---

# 29ch-20 Route Evidence And Probes

## Purpose

`phase-29ch` README は current accepted truth を読む入口に保ち、
この文書は active evidence / diagnostics-only probes / solved slice ledger を保持する。

## Accepted Evidence Snapshot (2026-03-12)

- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir apps/tests/hello_simple_llvm.hako "$(cat apps/tests/hello_simple_llvm.hako)"` -> `rc=0`
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli emit-mir lang/src/runner/stage1_cli_env.hako "$(cat lang/src/runner/stage1_cli_env.hako)"` -> `rc=0`
- `bash tools/selfhost/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli emit program-json apps/tests/hello_simple_llvm.hako` -> `rc=0`
- `bash tools/selfhost/run_stage1_cli.sh --bin target/selfhost/hakorune.stage1_cli emit mir-json apps/tests/hello_simple_llvm.hako` -> `rc=0`
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli.next --force-rebuild` -> PASS
- `bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli --force-rebuild` -> PASS
- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind stage1-cli --out target/selfhost/hakorune.stage1_cli.stage2 --force-rebuild` -> PASS
- `bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` -> PASS
- `bash tools/selfhost_identity_check.sh --mode smoke` -> PASS
- `bash tools/selfhost_identity_check.sh --mode full --skip-build --bin-stage1 target/selfhost/hakorune.stage1_cli --bin-stage2 target/selfhost/hakorune.stage1_cli.stage2` -> PASS
  - `Program JSON v0`: raw match
  - `MIR JSON v0`: raw match on the current reduced authority route

## Diagnostics Probe Registry

- exact raw diff probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_raw_mir_diff_probe.sh [entry]`
  - default entry: `lang/src/compiler/entry/compiler_stageb.hako`
- route-mode branchpoint probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_route_mode_matrix.sh [entry]`
- same-route repeatability probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_same_route_repeat_probe.sh [entry]`
- fixed-Program repeatability probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_fixed_program_mir_repeat_probe.sh [entry]`
- transient-boundary probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_transient_boundary_probe.sh [entry]`
- source-route direct probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_source_route_direct_probe.sh [entry]`
  - diagnostics-only: builds a temporary helper artifact and calls `MirBuilderBox.emit_from_source_v0(...)` directly on a compiled artifact
- stage1 env file-context probe:
  - diagnostics-only: emits/runs temporary `stage1_cli_env`-shaped clones through Stage1/Stage2 and narrows where source-route promotion first turns red
- explicit Program(JSON) compat probe:
  - `bash tools/dev/phase29ch_program_json_compat_route_probe.sh --bin <stage1-cli>`
  - diagnostics-only: reports which explicit supplied-Program compat route is actually used on a compiled artifact
  - current owner: the probe itself + `stage1_contract_exec_program_json_text()`; no shared compat helper remains in `identity_routes.sh`
- explicit Program(JSON) cold compat probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh --bin <stage1-cli>`
  - diagnostics-only: reports whether legacy/subcmd cold compat routes are still accepted on a compiled artifact
- explicit Program(JSON) text-only probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh --bin <stage1-cli>`
  - diagnostics-only: proves whether the remaining compat resolver can accept `*_PROGRAM_JSON_TEXT` alone
- explicit Program(JSON) mode-gate probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh`
  - diagnostics-only: proves that plain `emit-mir` rejects mixed-in Program(JSON) text, exact-only `emit-mir-program` stays green, and legacy alias forms such as `emit_mir_program` are rejected
- Program(JSON) helper execution probe:
- archived evidence:
  `bash tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh`
  - diagnostics-only: proves that raw `stage1-cli` artifacts still return `rc=97` when asked to execute a helper source that would print `MirBuilderBox.emit_from_program_json_v0(...)`
- raw direct `stage1-cli` absence probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_raw_direct_stage1_cli_probe.sh`
  - diagnostics-only: proves that the generic raw direct `stage1-cli` lane is absent on green artifacts (`<bin> <source>` / `emit program-json` / `emit mir-json` all return `rc=97`)
- impossible-gate probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_impossible_gate_probe.sh [entry]`
- bridge-bypass probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_bridge_bypass_probe.sh [entry]`
- source-route materialization probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh`
- selfhost source-route helper probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_source_route_helper_probe.sh`
- selfhost source-route bisect probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_source_route_bisect_probe.sh`
- selfhost Program(JSON) helper probe:
  - archived evidence:
    `bash tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh`
  - diagnostics-only: proves that a minimal selfhost helper calling `MirBuilderBox.emit_from_program_json_v0(...)` stays green on Stage1/Stage2

## Current Compare Decision

- `phase-29ch` uses `semantic canonical match` for G1 MIR compare and keeps raw MIR exact diff as tightening evidence.
- compare rules SSOT:
  - `docs/development/current/main/design/selfhost-g1-mir-compare-policy-ssot.md`
- fast regression entry:
  - `python3 -m unittest tools.selfhost.lib.tests.test_mir_canonical_compare`
- Raw exact MIR equality has now been reached again for the current reduced authority route on `compiler_stageb.hako`.
- The canonical compare policy remains in place for future widenings and for narrowing future non-semantic noise without changing route authority.

## Solved Slice Ledger

### launcher-exe widening

- `NYASH_BIN=target/selfhost/hakorune.stage1_cli bash tools/selfhost/build_stage1.sh --artifact-kind launcher-exe --out target/selfhost/hakorune.launcher_from_stage1_cli --force-rebuild` -> PASS
- `stage1_contract_exec_mode target/selfhost/hakorune.stage1_cli.next emit-program lang/src/runner/launcher.hako "$(cat lang/src/runner/launcher.hako)"` emits Program(JSON v0) with `defs_boxes=[HakoCli]` and bare-using imports including `MirBuilderBox`
- `... emit-mir ...` emits `user_box_decls=[HakoCli, Main]` and lowers `HakoCli.run/1` on the current reduced authority route

### raw determinism closure

- The former active blocker was G1 full MIR exact diff on `compiler_stageb.hako`; that exact diff is now closed on the current reduced authority route.
- Effective repair owner:
  - `src/runner/json_v0_bridge/lowering/merge.rs`
  - `src/runner/json_v0_bridge/lowering/try_catch.rs`
- merge-variable name collection now uses `BTreeSet<String>` instead of `HashSet<String>`, stabilizing the copy/materialization order that had been drifting first at `StageBArgsBox.resolve_src/1` block 8.
- closure evidence:
  1. `bash tools/archive/legacy-selfhost/engineering/phase29ch_fixed_program_mir_repeat_probe.sh` was quiet/raw-exact for `lang/src/compiler/entry/compiler_stageb.hako`
  2. `bash tools/archive/legacy-selfhost/engineering/phase29ch_route_mode_matrix.sh` was quiet for the same source
  3. fresh `G1 full` is raw-exact green for both `Program JSON v0` and `MIR JSON v0`

### source-route promotion

- `MirBuilderBox.emit_from_source_v0(...)` is accepted as reduced-case authority evidence for source-only `stage1-env-mir-source`.
- The previously red env-wrapper/file-context cluster turned green after fixing the compiled-artifact Rust provider path under:
  - `src/runner/json_v0_bridge/lowering/if_else.rs`
  - `src/runner/json_v0_bridge/lowering/merge.rs`
- The focused case now emits `block 9: phi dst=31 incoming=[[11,8],[19,15]]` before `emit_from_source_v0(selected_input, null)`.
- The previous `[freeze:contract][stage1_mir_builder] source decode failed` path is gone on fresh Stage1/Stage2 artifacts.
- the following archived probes were green when this slice closed:
  - `bash tools/archive/legacy-selfhost/engineering/phase29ch_source_route_materialize_probe.sh`
  - `bash tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_source_route_helper_probe.sh`
  - `bash tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_source_route_bisect_probe.sh`

### compat-surface thinning

- `bash tools/dev/phase29ch_program_json_compat_route_probe.sh --bin target/selfhost/hakorune.stage1_cli`
  and `--bin target/selfhost/hakorune.stage1_cli.stage2`
  both report `compat_route=stage1-env-mir-program`.
- That explicit compat route uses text transport through `stage1_contract_exec_program_json_compat()` and the current live text SSOT is `STAGE1_SOURCE_TEXT`.
- `STAGE1_PROGRAM_JSON_TEXT` is now retained only for fail-fast diagnostics and archived cold compat observation; the archived explicit mode gate probe and archived cold compat probe inject it directly, while live shell helpers do not.
- retired path transport has been removed from `stage1_contract_exec_mode()` / `stage1_contract_run_bin_with_env()`;
  live shell compat now enters through `stage1_contract_exec_program_json_compat()` only.
- Raw `tools/selfhost/run_stage1_cli.sh ... emit mir-json --from-program-json <file>` is retired from the live wrapper surface.
- exact-only compat helper / mode / sentinel entry (`stage1_contract_exec_program_json_compat()` / `emit-mir-program` / `__stage1_program_json__`) are now centralized in `tools/selfhost/lib/stage1_contract.sh`.
- No separate cold supplied-Program compat lane remains on green artifacts.
- The cold supplied-Program diagnostics owner is now archived evidence, not
  `identity_routes.sh` / `stage1_contract.sh`.
- `tools/archive/legacy-selfhost/engineering/phase29ch_selfhost_program_json_helper_probe.sh`
  was green before archive:
  - `stage1_stage2_mir=exact-match`
  - runtime flags: `MIR_NONNULL`, `MIR_NONEMPTY`, `TEXT_NONEMPTY`, `LEN_POS`, `HEAD_OK`, `IDX_OK`
- therefore the explicit supplied-Program compat lane stays frozen/probe-owned for the current slice; do not reopen `stage1_cli_env.hako` wrapper-level compat branching while the authority path is still shrinking on Rust owner-1 / owner-2.
- current code-side quarantine owner for that branch is `lang/src/runner/stage1_cli_env.hako::Stage1ProgramJsonCompatBox` (mixed-input fail-fast gate + explicit compat call).
- `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_explicit_mode_gate_probe.sh`
  was green before archive:
  - `stage1.plain_rc=96`
  - `stage2.plain_rc=96`
  - `stage1.legacy_alias_rc=97`
  - `stage2.legacy_alias_rc=97`
  - explicit compat mode still emits MIR JSON on both bins
- `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_helper_exec_probe.sh`
  was green before archive:
  - `stage1.raw_exec_rc=97`
  - `stage2.raw_exec_rc=97`
  - therefore raw helper execution is not yet available as a shell-side replacement for the Stage1-side explicit compat dispatch
- `tools/archive/legacy-selfhost/engineering/phase29ch_raw_direct_stage1_cli_probe.sh`
  was green before archive:
  - `hakorune.stage1_cli.raw_source.rc=97`
  - `hakorune.stage1_cli.raw_emit_program.rc=97`
  - `hakorune.stage1_cli.raw_emit_mir.rc=97`
  - `hakorune.stage1_cli.stage2.raw_source.rc=97`
  - `hakorune.stage1_cli.stage2.raw_emit_program.rc=97`
  - `hakorune.stage1_cli.stage2.raw_emit_mir.rc=97`
  - therefore raw helper execution is one instance of a broader non-authority raw direct lane absence
- `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_cold_compat_probe.sh`
  was green before archive on `target/selfhost/hakorune.stage1_cli` and
  `target/selfhost/hakorune.stage1_cli.stage2`:
  - `legacy_env_program_json=none`
  - `raw_wrapper_program_json=none`
  - `explicit_helper_program_json=stage1-env-mir-program`
- `tools/archive/legacy-selfhost/engineering/phase29ch_program_json_text_only_probe.sh`
  was green before archive:
  - `target/selfhost/hakorune.stage1_cli` returned `text_only_rc=0`
  - `target/selfhost/hakorune.stage1_cli.stage2` returned `text_only_rc=0`
  - the live explicit compat route probe now covers this text transport through
    `stage1_contract_exec_program_json_compat()`

### owner-local surface thinning

- `src/stage1/program_json_v0.rs` cross-crate surface is now:
  - `emit_program_json_v0_for_strict_authority_source(...)`
  - `emit_program_json_v0_for_current_stage1_build_box_mode(...)`
- owner-local only:
  - `source_to_program_json_v0_strict(...)`
  - `source_to_program_json_v0_relaxed(...)`
  - `emit_program_json_v0_for_stage1_build_box(...)`
  - build-route/source-shape internals in `routing.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` no longer interprets stage1 mode env keys directly; current-mode build surrogate selection is delegated to owner-1.
- current-mode env interpretation now reuses `crate::config::env::stage1::emit_program_json()` as shared env SSOT, and legacy `STAGE1_EMIT_PROGRAM_JSON=1` still proves strict-authority mode through that path.
- future-retire bridge lane is thinner:
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
  - only remaining crate-local non-routing strict-parse consumer is `src/runner/stage1_bridge/program_json/mod.rs`
  - Stage1 stub entry resolution + child command/env assembly + prepare-failure mapping live in `src/runner/stage1_bridge/stub_child.rs`; `route_exec/stub.rs` no longer owns the prepare error log + `97` mapping
  - Stage1 stub plain delegate-status execution + child-spawn-failure mapping live in `src/runner/stage1_bridge/stub_delegate.rs`; `route_exec/stub.rs` now only selects `stub_exec_plan()` branch
  - bridge-local `emit-program-json-v0` file I/O lives in `src/runner/stage1_bridge/program_json/mod.rs`
  - Stage1 stub `emit` stdout parse / validation live in `src/runner/stage1_bridge/stub_emit/parse.rs`, and output-path writeback lives in `src/runner/stage1_bridge/stub_emit/writeback.rs` behind the thin `stub_emit.rs` facade
  - `src/runner/stage1_bridge/mod.rs` stays a thin delegate and no longer carries child/enable entry guard checks, child command/env assembly, or JSON line parsing / writeback logic
- proof refresh after the above narrowing:
  - `build_stage1.sh` Stage1/Stage2 rebuild: PASS
  - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`: PASS
  - `tools/selfhost_identity_check.sh --mode smoke --skip-build`: PASS
  - `tools/selfhost_identity_check.sh --mode full --skip-build`: PASS

## Route Guard Lock

- `tools/selfhost_identity_check.sh --mode full` must observe:
  - `program-json`: `stage1-env-program`
  - `mir-json`: `stage1-env-mir-source`
- `stage1-env-mir-program` is explicit compatibility-only and is not accepted as reduced-case authority evidence.
- only `stage1-env-mir-program` remains as the explicit supplied-Program compat route.
- `tools/selfhost/build_stage1.sh` stage1-cli capability probe and `identity_routes.sh` preflight share the same env-mainline capability helper.
- `tools/selfhost/build_stage1.sh --artifact-kind stage1-cli` capability probe must fail fast if the artifact only exposes compat/stale routes.
- `tools/selfhost/build_stage1.sh` bridge-first bootstrap body also uses the same shared env-mainline helper for actual source->MIR emission.
- route retirement rule:
  - when this phase discovers a non-authority route, document it immediately as exactly one of `compat-only keep` or `future retire target`
  - discovery alone must not create new authority evidence

## Detour Prevention

- `src/runner/modes/vm_hako/compile_bridge.rs` already contains a Rust direct source→MIR helper, but it is reference-only for `phase-29ch`.
- Do not promote it into current selfhost authority while choosing the next reduction slice.
