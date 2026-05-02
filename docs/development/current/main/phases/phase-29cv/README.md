---
Status: Active
Decision: accepted
Date: 2026-04-30
Scope: post-EXE-direct `Program(JSON v0)` keeper closeout after phase-29ci P26.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29ci/P26-NYLLVMC-ENTRY-ARGS-BIRTH-PURE-FIRST.md
  - tools/selfhost/README.md
---

# Phase 29cv: Program JSON v0 Keeper Closeout

## Goal

Finish the `Program(JSON v0)` cleanup after the normal
`tools/selfhost/selfhost_build.sh --exe` route stopped producing Stage-B
Program(JSON v0).

This phase is not a broad language or backend redesign. It is a BoxShape
closeout lane: inventory the remaining keepers, move each keeper behind the
right owner, and delete dead helper surface when the repo no longer calls it.

## Current Read

- `phase-29ci` closed the public wrapper and raw-compat caller cleanup, then
  P26 unblocked the direct source -> MIR(JSON) -> ny-llvmc EXE route.
- Normal `selfhost_build.sh --mir`, `--run`, and `--exe` are now direct
  MIR(JSON) routes.
- `selfhost_build.sh --keep-tmp` and `NYASH_SELFHOST_KEEP_RAW=1` are retired
  from the facade; explicit Program(JSON v0) artifact capture now lives in the
  dev probe.
- `Program(JSON v0)` remains as internal/compat/debug infrastructure only.
- Remaining live Program(JSON v0) surfaces are compat capsules: explicit,
  bounded owners that pin compatibility seams without becoming mainline proof.
- The remaining work is keeper classification plus small delete slices, not a
  new acceptance-shape expansion.
- Thin shell/test seam cleanup is effectively exhausted through P32. Remaining
  work is explicit keeper replacement or final delete-last cleanup.
- P101 refreshed the caller inventory.
- P102 moved `tools/selfhost_exe_stageb.sh` default to the MIR-first `direct`
  route. `stageb-delegate` remains available only as an explicit bridge
  compat capsule.
- P103 keeps that explicit bridge capsule backend-clean by normalizing
  `nyash.console.log` / `env.console.log` print calls to `mir_call Global print`
  in the shared Program(JSON)->MIR bridge output before ny-llvmc sees the MIR
  JSON.
- P104 archived the old standalone bridge-to-EXE dev probe after explicit
  `tools/selfhost_exe_stageb.sh` `stageb-delegate` replacement proof went green.
- P105 kept `phase29cg_stage2_bootstrap_phi_verify.sh` as a bridge keep but
  guarded it against the current reduced run-only `stage1-cli` artifact. The
  replacement remains `stage1_contract_exec_mode ... emit-mir` once an
  emit-capable Stage1 env artifact is green.
- P106 locked why that replacement is not ready: Stage-B mainline-only does
  not emit the full Stage1 env MIR yet, Rust direct MIR is diagnostic-only,
  pure-first stops on unplanned `env.get/1`, and the full Stage1 env direct EXE
  route still fails MIR dominance verification.
- P107 added MIR-owned `extern_call_routes` / `LoweringPlan` metadata for
  `env.get/1`.
- P108 consumes that `EnvGet` plan in ny-llvmc pure-first without adding a raw
  backend `env.get` matcher. Remaining replacement blockers are the
  emit-capable Stage1 env artifact and the direct EXE MIR dominance failure.
- P109 consumes canonical MIR `keepalive` as a pure-first no-op after P108
  moved the Stage1 env direct MIR stop-line from `env.get/1` to lifecycle
  intent. The next observed pure-first stop is
  `mir_call Global BuildBox.emit_program_json_v0/2`.
- P110 locks that `BuildBox.emit_program_json_v0/2` stop as a Stage1
  authority/route split issue, not a backend accept-shape target. The thin
  `entry/stage1_cli_env_entry.hako` remains a green run-only bootstrap entry;
  the full `stage1_cli_env.hako` remains the emit-capable authority cluster and
  must not be made green by adding a raw ny-llvmc `BuildBox` matcher.
- P111 restores full `stage1_cli_env.hako` `Main.main()` to the thin dispatcher
  shape described by the Stage1 env boxes. The source-to-Program authority stays
  in `Stage1SourceProgramAuthorityBox`, and the source-to-MIR authority stays in
  `Stage1SourceMirAuthorityBox`; `Main` no longer carries a stale inline
  `BuildBox.emit_program_json_v0/2` call. The next pure-first stop is now the
  typed user/global-call boundary at `Stage1ModeContractBox.resolve_mode/0`.
- P112 classifies that typed user/global-call boundary as
  `global_call_routes` / `LoweringPlan tier=Unsupported`. ny-llvmc still
  fail-fasts, but the reason moves from raw `mir_call_no_route` discovery to
  `lowering_plan_unsupported_global_call`.
- P113 enriches `global_call_routes` with MIR module target facts. The
  `Stage1ModeContractBox.resolve_mode/0` stop now reports
  `missing_multi_function_emitter`, which means the callee exists and arity
  matches, but generic pure still emits only the selected entry function.
- P114 makes the C generic pure program view module-shaped (`functions[]`,
  `function_count`, selected entry, `entry_index`) while preserving entry-only
  emission. This is the structural landing point for the next multi-function
  emitter card.
- P115 adds a C `LoweringPlanGlobalCallView` and routes the unsupported
  user/global-call diagnostic through that typed view. The stop-line remains
  `missing_multi_function_emitter`, but the backend failure site no longer
  hand-parses the global-call plan fields.
- P116 adds `target_symbol` for same-module `global_call_routes` and emits
  module function declarations in generic pure before the entry definition.
  `UserGlobalCall` is still unsupported; this only gives the next call-emitter
  card a typed symbol/declaration contract.
- P117 adds the C direct-target validator for plan-backed same-module global
  calls and traces `global_call_direct_target_pending` when a site is ready but
  still blocked on function-body emission.
- P118 adds the first narrow same-module global-call lowering slice:
  `numeric_i64_leaf` target functions are emitted as LLVM definitions and only
  then become `DirectAbi` `UserGlobalCall` targets. Non-leaf targets, including
  `Stage1ModeContractBox.resolve_mode/0`, remain on the
  `missing_multi_function_emitter` stop-line.
- P119 separates generic pure lowering into an explicit per-function state
  seam. P120 uses that seam for same-module `generic_pure_string_body`
  definitions, so `Stage1ModeContractBox.resolve_mode/0` no longer requires a
  raw backend matcher.
- P147-P148 tighten MIR-owned global-call classification for unknown-return
  string/void sentinel bodies and exact string return parameter passthrough.
  This keeps parameter inference narrow while advancing the source-execution
  route through the mode contract.
- P149 makes void-typed return-profile diagnostics propagate nested global-call
  blockers instead of hiding them behind sentinel metadata.
- P150 makes `BuildBox._resolve_parse_src/1` materialize fallback source text
  through an owner-local helper. The current pure-first stop is now the parser
  authority boundary:
  `BuildBox._parse_program_json/1 -> generic_string_unsupported_instruction`.
- P151 keeps that boundary non-lowerable but improves the MIR-owned diagnostic:
  the stop now propagates to
  `ParserBox.parse_program2 -> generic_string_unsupported_known_receiver_method`.
- P207a locks the Stage0 size guard for the remaining source-execution work:
  Stage0 may own generic MIR(JSON)->object/exe bootstrap and recovery plumbing,
  but it must not clone Stage1/selfhost parser, mirbuilder, normalizer, route,
  or canonical policy semantics. New blockers should prefer explicit MIR facts
  and a uniform MIR function emitter over new body-shape/C-shim semantics.

## Compat Capsule Rules

- A capsule must have a named entrypoint and a clear input/output boundary.
- A capsule may produce or consume Program(JSON v0) internally, but it must not
  be read as the day-to-day compiler route.
- `selfhost_build.sh` must stay MIR-first and must not source capsule routes as
  facade shortcuts.
- A capsule is deleted only after it has a MIR-first replacement or an archive
  owner.
- Passing a capsule probe proves the compat seam only; primary proof stays on
  MIR(JSON) mainline gates.

## Compat Capsule Buckets

1. Explicit Stage-B artifact diagnostic probe
   - `tools/dev/program_json_v0/stageb_artifact_probe.sh`
   - `tools/lib/program_json_v0_compat.sh`
   - Kept for deliberate Program(JSON v0) artifact capture only.
     `selfhost_build.sh` must not own or source this route.
2. Program(JSON)->MIR bridge capsule
   - `tools/selfhost/lib/stageb_program_json_capture.sh`
   - `tools/selfhost/lib/program_json_mir_bridge.sh`
   - `tools/selfhost_exe_stageb.sh` only when explicitly run with
     `HAKORUNE_STAGE1_EMIT_ROUTE=stageb-delegate`
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - Kept for explicit compat conversion work only.
   - `tools/selfhost_exe_stageb.sh` defaults to `direct`; explicit
     `stageb-delegate` is a bridge capsule.
   - The old standalone bridge-to-EXE dev probe is archived at
     `tools/archive/legacy-selfhost/engineering/phase29ci_selfhost_build_exe_consumer_probe.sh`.
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh` is now guarded so it
     cannot misread the reduced run-only `stage1-cli` artifact as an
     emit-capable Stage1 env artifact.
   - P108 proved the plan-backed `env.get/1` consumer and P109 removed
     `keepalive` as a backend blocker. P110 records the next full-env stop as
     `BuildBox.emit_program_json_v0/2`, which is Stage1 authority surface
     rather than a backend matcher target. This caller stays in the bridge
     capsule until an emit-capable Stage1 env artifact or narrower emit-MIR-only
     owner is green on the MIR-first replacement path.
   - This capsule is not a primary proof route and is not part of
     `selfhost_build.sh` mainline routing.
3. Stage1 contract keepers
   - `tools/selfhost/lib/stage1_contract.sh`
   - `tools/selfhost/compat/run_stage1_cli.sh`
   - Keep only for explicit contract/probe coverage.
   - Old root helpers `tools/stage1_debug.sh` and `tools/stage1_minimal.sh`
     are archived under `tools/archive/legacy-selfhost/stage1-cli/`.
4. JoinIR / MirBuilder fixture keepers
   - `tools/smokes/v2/lib/stageb_helpers.sh`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_program_json_contract_pin_vm.sh`
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_hako_mirbuilder_*`
   - Stage-B/Core smoke callers under `tools/smokes/v2/profiles/integration/stageb/`,
     `tools/smokes/v2/profiles/integration/core_direct/`, and related quick
     core fixture gates.
   - Keep while those tests explicitly assert the Program(JSON)->.hako
     MirBuilder or Stage-B/Core fixture contracts.
   - Stage-B stdout capture is shared through
     `tools/selfhost/lib/stageb_program_json_capture.sh`.
   - weaker phase2160 Stage-B shape canaries are archive-only after P35
5. Rust public compat delete-last surface
   - `--emit-program-json-v0`
   - `src/runtime/deprecations.rs`
   - `src/stage1/program_json_v0*`
   - `src/runner/stage1_bridge/**`
   - Delete only after every shell/test keeper has a replacement or an archive
     owner.

## Primary Proof Reading

- Mainline proof:
  - `selfhost_build.sh --mir`, `--run`, and `--exe`
  - `--emit-mir-json`
  - `--mir-json-file`
- Compat capsule proof:
  - Program(JSON v0) artifact capture
  - Program(JSON)->MIR bridge conversion
  - Stage1 contract compatibility
  - JoinIR / MirBuilder Program(JSON) fixture pins

Do not use a compat capsule PASS as evidence that Program(JSON v0) is still a
mainline artifact family.

## Non-goals

- Do not reintroduce mixed `--run` + Stage-B artifact execution. Use
  fail-fast and ask for either run or artifact diagnostics.
- Do not reintroduce mixed `--exe` + Stage-B artifact execution. Use direct
  EXE or artifact diagnostics.
- Do not reintroduce mixed `--mir` + Stage-B artifact output. Use direct MIR or
  artifact diagnostics.
- Do not reintroduce `selfhost_build.sh --keep-tmp` or
  `NYASH_SELFHOST_KEEP_RAW=1` as facade artifact routes. Use
  `tools/dev/program_json_v0/stageb_artifact_probe.sh` for explicit diagnostics.
- Do not expand ny-llvmc pure-first acceptance shapes here.
- Do not revive `--hako-emit-program-json` or other retired public aliases.
- Do not treat fixture-only Program(JSON) producers as day-to-day bootstrap
  route authority.

## Acceptance

Each card in this phase should keep this minimum proof bundle green unless the
card narrows it further:

```bash
bash tools/checks/current_state_pointer_guard.sh
SMOKES_ENABLE_SELFHOST=1 bash tools/smokes/v2/profiles/quick/selfhost/selfhost_build_exe_return.sh
git diff --check
```
