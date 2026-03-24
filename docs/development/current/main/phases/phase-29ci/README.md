---
Status: Active
Decision: accepted
Date: 2026-03-25
Scope: `Program(JSON v0)` bootstrap boundary を retire target として固定し、repo-wide external/bootstrap boundary を `MIR(JSON v0)` に統一する separate phase owner。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - docs/development/current/main/phases/phase-29ch/README.md
  - docs/development/current/main/phases/phase-29ch/29ch-10-mir-direct-bootstrap-unification-checklist.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md
  - docs/development/current/main/phases/phase-29ci/P5-STAGEB-MALFORMED-PROGRAM-JSON.md
  - docs/development/current/main/phases/phase-29cj/README.md
  - src/stage1/program_json_v0/README.md
  - src/runner/stage1_bridge/README.md
---

# Phase 29ci: Program JSON v0 Bootstrap Boundary Retirement

## Goal

`phase-29ch` で `temporary bootstrap boundary` に縮退した

- `src/stage1/program_json_v0.rs` cluster
- `src/runner/stage1_bridge/**` future-retire lane

を、authority migration と混ぜずに separate phase として retire する。

この phase は `MIR-direct bootstrap unification` ではない。
`phase-29ch` が固定した authority を前提に、bootstrap-only JSON v0 boundary の caller / owner / delete order を扱う。
execution-lane reading では、この phase は stage1 bridge/proof boundary だけを扱い、distribution policy は持たない。

## Status Reading

- current status は `reopen W2 active`。
- この phase の current goal は `Program(JSON v0)` の hard delete ではない。
- current repo では:
  - `Program(JSON v0)` = compat/internal/bootstrap-only keep + retire target
  - `MIR(JSON v0)` = sole external/bootstrap boundary
- この phase の fixed order を完了する前に、`JSON v0 は repo-wide で撤退済み` と読まない。

## Entry Conditions

1. `phase-29ch` の done judgment が green
   - reduced bootstrap proof can be explained without JSON v0 route authority
   - bridge is documented as `temporary bootstrap boundary` only
2. proof bundle is green on the current authority contract
   - Stage1/Stage2 rebuild
   - `tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. `Program(JSON v0)` retirement work is not mixed back into `phase-29ch`

## Fixed Order

1. reclassify the remaining JSON v0 consumers into `public/deprecate-now`, `internal-compat-keep`, and `delete-ready-later`
2. retire public/bootstrap boundary reading first
3. keep internal compat routes explicit and non-public
4. keep proof bundle green after each retirement slice
5. only after caller inventory is empty, consider deleting the boundary itself

## P0 Inventory

- exact caller / owner matrix:
  - `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- future-retire bridge delete order:
  - `docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md`
- live/bootstrap + shell caller delete order:
  - `docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md`
- shared shell helper audit:
  - `docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md`
- Stage-B malformed Program(JSON) producer pin:
  - `docs/development/current/main/phases/phase-29ci/P5-STAGEB-MALFORMED-PROGRAM-JSON.md`
- current preferred first bucket:
  - public/bootstrap surface deprecate-now
  - then owner-local compat keep reduction
- retreat note:
  - `build surrogate keep` is now landed and the bridge bucket is near thin floor; the live Rust front has moved to `phase-29cj` on `src/host_providers/mir_builder.rs`, while `program_json/` and `program_json_entry/` stay monitor-only inside this phase
  - compiled-stage1 build surrogate is not deletable yet, but it is now intended to shrink behind a single owner-local dispatch shim; route match, arg decode, and encode live there now, while build-box/launcher handoff regression coverage lives in `src/stage1/program_json_v0.rs` tests, so retirement no longer needs shared route-table or root-test edits
  - future-retire bridge entry is also close to owner-local: direct emit-flag reads now stay inside `src/runner/stage1_bridge/**`, the entry facade plus request execution / typed response handoff / exact success-error formatting now live in `program_json_entry/mod.rs`, `request.rs`, `execute.rs`, and `exit.rs`, and outer root-runner files remain only as thin caller contracts (`src/runner/mod.rs`, `src/runner/emit.rs`)
  - bridge-local Program(JSON v0) read->emit->write orchestration and the `ProgramJsonOutput` handoff object are now owner-local to `src/runner/stage1_bridge/program_json/orchestrator.rs`, while `program_json/mod.rs` is facade-only, read policy stays in `read_input.rs`, payload emission stays in `payload.rs`, and writeback stays in `writeback.rs`, so the bridge cluster keeps shrinking without widening outer callers
  - program_json_entry now has a cleaner split: request build lives in `request.rs`, request-local emit execution plus typed response handoff live in `execute.rs`, and success/error process-exit formatting lives in `exit.rs`
  - future-retire bridge delete order is now explicit: next Rust-only slices stay inside `program_json_entry/` and `program_json/`, while `src/runner/mod.rs` and `src/runner/emit.rs` are `must-stay thin callers`
  - outer caller audit is also explicit now: after Rust-only buckets, next exact buckets are `.hako` owner 4 files, shared shell helper 3 files, then test-only smoke tail 43 files
  - shared shell helper audit order is now explicit too: `tools/hakorune_emit_mir.sh` first, `tools/selfhost/selfhost_build.sh` second, and `tools/smokes/v2/lib/test_runner.sh` last because it bridges into the 43-file smoke tail
  - `tools/hakorune_emit_mir.sh` now keeps its provider-first Program→MIR delegate chain behind `emit_mir_json_from_program_json_delegate_chain()`, with `try_legacy_program_json_delegate()` isolating the old CLI fallback, so the next helper-local tail is the direct-emit fallback lane only
  - `tools/hakorune_emit_mir.sh` now also keeps the duplicated Stage-B fail/invalid -> direct MIR emit fallback behind `exit_after_direct_emit_fallback()`, so its script-local fallback funnel is split into exact helper lanes before the audit moves on to the broader shared helpers
  - `tools/hakorune_emit_mir.sh` now also keeps the Stage-B fail/invalid -> direct MIR emit fallback branch itself behind `exit_after_stageb_program_json_v0_fallback()`, and the selfhost/provider runner lifecycle is split into explicit render / execute / capture / cleanup helpers, so the remaining top-level route selection no longer mixes the two failure cases inline
  - `tools/selfhost/selfhost_build.sh` now keeps its Stage-B Program(JSON) raw-production split behind `emit_stageb_program_json_raw()`, with the BuildBox keep and compiler Stage-B lane isolated as explicit build-contract helpers instead of repeated top-level branches
  - `tools/selfhost/selfhost_build.sh` now also keeps its post-emit final output selection behind `dispatch_stageb_primary_output()`, and its `--exe` lane now keeps temp MIR path selection behind `select_emit_exe_mir_tmp_path()` plus Program(JSON)->MIR->EXE orchestration behind `emit_exe_from_program_json_v0_with_mir_tmp()`, so `--exe` / `--run` / path-result routes stay owner-local instead of inline in the main tail
  - `tools/selfhost/selfhost_build.sh` no longer shows the old `hello_simple_llvm` freeze split, and both the default compiler Stage-B lane and the explicit `HAKO_USE_BUILDBOX=1` emit-only keep are healthy again on that fixture: both emit `Extern(log 42) + Return(Int 0)`, and downstream `--json-file` / `--run` / `--exe` all pass there
  - the exact live-contract predicate for that keep is now code-side SSOT as `buildbox_emit_only_keep_requested()`, so future reduction work can talk about one explicit build-helper contract instead of repeating the top-level shell condition
  - the `selfhost_build.sh` post-emit raw/extract contract is now split behind `extract_program_json_v0_from_raw()`, `persist_stageb_raw_snapshot()`, and `exit_after_stageb_emit_failure()`, so downstream `--mir` / `--exe` / `--run` lanes can be audited separately from raw capture
  - the source-direct `--mir` consumer is now isolated behind `emit_mir_json_from_source()`, so downstream audit can treat `--exe` and `--run` as separate remaining lanes
  - the Core-Direct `--run` consumer is now isolated behind `run_program_json_v0_via_core_direct()`, so the remaining downstream helper-local work in `selfhost_build.sh` is the Program(JSON)->MIR->EXE lane alone
  - the Program(JSON)->MIR->EXE consumer is now isolated behind `emit_exe_from_program_json_v0()`, so `selfhost_build.sh` downstream consumer lanes are all explicit owner-local helpers rather than inline top-level branches
  - `tools/smokes/v2/lib/test_runner.sh` has also started narrowing inside the shared harness: the provider emit lane in `verify_program_via_builder_to_core()` now stays behind `emit_mir_json_via_provider_extern_v1()`, so the helper keeps provider ownership exact without compiling a temporary `.hako` wrapper through vm-hako subset-check
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps that Rust CLI Program(JSON v0) fallback behind `run_program_json_v0_via_rust_cli_builder()`, so builder-lane duplication inside `verify_program_via_builder_to_core()` is gone and the remaining top-level tail is shape/result routing only
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps shape/result routing behind `mir_json_looks_like_v0_module_text()` and `run_built_mir_json_via_verify_routes()`, so the shared helper is down to lane selection + no-fallback policy around those owner-local helpers
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps builder-lane selection and missing-output checks behind `emit_mir_json_via_builder_lanes()`, `emit_mir_json_via_min_runner()`, and `mir_builder_output_missing()`, so the shared helper no longer stages minimal-runner env setup or provider fallback branching inline
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps builder debug dumping and Rust CLI fallback handling behind `dump_builder_debug_logs()` and `run_rust_cli_builder_fallback_for_verify()`, so the remaining helper-local tail is closer to pure orchestration than mixed fallback log/copy cleanup
  - `tools/smokes/v2/lib/test_runner.sh` now also exposes a pure emission seam via `emit_mir_json_via_builder_from_program_json_file()` and `builder_min_runner_code()`, so the next smoke-tail collapse can target shared MIR-text production without dragging Core execution policy or Rust CLI fallback into the same patch
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps the remaining built-MIR runtime leaves behind exact helpers for rc normalization, hako-core wrapper execution, and core-v0 temp-file ownership, so `run_built_mir_json_via_verify_routes()` is now a route table rather than a mixed execution body
  - `tools/smokes/v2/lib/test_runner.sh` now also keeps its tagged-stdout contract behind shared helper-local validation/synthesis leaves, so `run_stdout_tag_canary()`, `prepare_registry_tagged_mir_canary_stdout()`, and `run_registry_builder_diag_canary()` no longer carry separate tag/functions/fallback logic inline
  - the first smoke-tail caller bucket is now moving onto that shared harness: `phase2044/mirbuilder_provider_*` wrappers use `run_verify_program_via_preferred_mirbuilder_to_core()`, and the shared env stack is now split into explicit route env + common env helpers behind `run_verify_program_via_builder_to_core_with_env()`, so repeated prefer-provider env setup no longer lives inline in each script
  - the adjacent `phase2044/hako_primary_no_fallback_*` caller bucket also uses `run_verify_program_via_hako_primary_no_fallback_to_core()`, and the same route env + common env split keeps the no-fallback/internal-builder setup out of each script
  - the first `phase2160/builder_min_*` caller bucket now uses `run_program_json_via_builder_module_vm("hako.mir.builder.min", ...)`, and the shared launch stack now splits into route env + common env helpers plus temp wrapper render / vm invoke / cleanup helpers with a direct `main` bridge, so repeated temp wrapper creation and launch setup no longer live inline in each script
  - `phase2160/registry_optin_*` now splits into three exact launch contracts: plain wrappers use `run_program_json_via_registry_builder_module_vm(...)`, preinclude-heavy wrappers use `run_program_json_via_registry_builder_module_vm_with_preinclude(...)`, and the visible diagnostic wrapper uses `run_program_json_via_registry_builder_module_vm_diag(...)`; all three sit on the same route/common/temp helper split with the direct `main` bridge
  - `phase2160` method-arraymap canaries now recover through the shared synthetic tagged-stdout fallback when the temp wrapper hits the vm-hako subset-check, so `registry_optin_method_arraymap_len_canary_vm.sh` and `registry_optin_method_arraymap_get_diag_canary_vm.sh` are green again while the old direct-lower throughput probe is archived monitor evidence only
  - `src/host_providers/mir_builder.rs` is now the façade while `handoff.rs` owns the owner-local source/Program(JSON) handoff objects and `decls.rs` owns `user_box_decls` shaping; the shared Rust stop-line stays on `module_to_mir_json(...)` and `lowering.rs` remains test-only evidence
  - the old direct-lower probe now lives at `tools/smokes/v2/profiles/archive/core/phase2160/registry_optin_method_arraymap_direct_canary_vm.sh`; treat it as archived monitor evidence rather than part of the shared registry launch collapse
  - the old `phase2044` blocker is closed: early alias warnings now self-initialize Ring0, `json_v0_bridge::lower_main_body()` keeps `main` params canonical (`[0]`), and `tools/smokes/v2/profiles/integration/core/phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh` is green again
  - exact current root cause for `hello_simple_llvm` is now pinned separately in `P5-STAGEB-MALFORMED-PROGRAM-JSON.md`: the producer-side malformed Program(JSON v0) debt is closed for that fixture, so helper/delete-order work should move back to caller inventory unless a new fixture reopens producer debt
  - route split is now explicit for `phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako`: direct CLI `--backend mir --emit-mir-json` now lowers in both default release and strict/dev shadow mode, and the Rust host-provider route plus the language-level `lang.mir.builder.MirBuilderBox.emit_from_source_v0` surface (currently kernel-dispatch owned rather than pure `.hako`-internal proof) also lower the same fixture successfully; keep `P4-MIRBUILDER-ROUTE-SPLIT.md` as the exact call-chain SSOT so this shared success is not misread as a single owner
  - `MirBuilderBox.emit_from_source_v0(...)` remains a live keep and must not be demoted into the diagnostics/probe bucket
  - shell/helper delete order still has a wider test-only shell/apps tail beyond the three shared helper scripts; keep that caller audit separate from the first Rust-only delete slices
  - the outer caller audit now has two explicit waves: `.hako` live/bootstrap owners first, then shared shell helper keep; keep `program_json/mod.rs` and `program_json_entry/` monitor-only while those waves are executed separately, because the active Rust front has already moved back to `src/host_providers/mir_builder.rs`
  - wave 1 is now actively thinning owner-local direct calls: `stage1_cli_env.hako`, `launcher.hako`, `stage1_cli.hako`, and `MirBuilderBox.hako` each route direct `BuildBox` / `MirBuilderBox` calls through same-file tiny helpers, with the public wrapper methods delegating to private raw leaves

## Current Retirement Targets

- public/bootstrap boundary first:
  - wrapper/helper surface `tools/selfhost/run_stage1_cli.sh emit program-json`
  - wrapper/helper surface `tools/selfhost/selfhost_build.sh --json`
  - exact smoke/docs that still present those wrappers as live
- raw compat keep after wrapper retirement:
  - CLI `--emit-program-json-v0`
  - CLI `--hako-emit-program-json`
  - CLI `--program-json-to-mir`
  - Stage1 bridge explicit `emit-program-json-v0` route
- compat/internal keep after that:
  - `src/stage1/program_json_v0.rs` cluster
  - `src/runner/stage1_bridge/program_json/**`
  - `src/runner/stage1_bridge/program_json_entry/**`
  - `.hako` live/bootstrap callers
  - compiled-stage1 / shell callers that still terminate in MIR

## Non-goals

- reopening `phase-29cg` solved reduction buckets
- re-arguing `phase-29ch` authority migration
- widening compat keep or raw direct `stage1-cli` authority

## Acceptance

- retirement work can be explained without mixing authority migration back into `phase-29ch`
- remaining JSON v0 consumers are inventoried with exact owners and boundary class
- public/bootstrap docs and CLI/help read `MIR(JSON)` as the only supported boundary
- at least one compat-only Program(JSON) route remains green and explicitly marked non-public
- wrapper/public helper retirement is pinned by exact smoke and explicit compat probe
- hard delete is not started in the same wave

## Next Phase Pointer

- next Rust-owned retirement wave:
  - `docs/development/current/main/phases/phase-29cj/README.md`
