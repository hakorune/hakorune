---
Status: Accepted
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` で扱う `Program(JSON v0)` bootstrap boundary の残存 consumer を exact owner 付きで固定する。
Related:
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P2-LIVE-CALLER-DELETE-ORDER.md
  - docs/development/current/main/phases/phase-29ci/P3-SHARED-SHELL-HELPER-AUDIT.md
  - docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md
  - CURRENT_TASK.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - src/stage1/program_json_v0.rs
  - src/runner/stage1_bridge/program_json/mod.rs
---

# P0 Program JSON v0 Consumer Inventory

## Goal

`phase-29ci` の delete/reduction slice を caller audit から始められるように、
`Program(JSON v0)` bootstrap boundary の残存 consumer を

- current authority
- build surrogate keep
- future-retire bridge
- `.hako` live/bootstrap callers
- diagnostics / probe keep

の bucket で固定する。

## Consumer Matrix

| Bucket | Current owner / caller | Surface | Note |
| --- | --- | --- | --- |
| `current authority` | [src/host_providers/mir_builder/authority.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/authority.rs), [src/host_providers/mir_builder/lowering/program_json.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/lowering/program_json.rs) | `emit_program_json_v0_for_strict_authority_source(...)`, `program_json_to_mir_json(...)` | current `stage1-env-mir-source` authority still materializes Program(JSON v0) before MIR(JSON); `authority.rs` owns `source -> Program(JSON v0)` while `lowering/program_json.rs` owns the real `Program(JSON v0) -> MIR(JSON)` lowering blocker; exact live callers of that lowering owner are now only the source authority itself plus [crates/nyash_kernel/src/plugin/module_string_dispatch.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch.rs) `emit_from_program_json_v0(...)`; runtime/plugin `env.mirbuilder.emit` still uses Rust, but now lowers via [src/runtime/mirbuilder_emit.rs](/home/tomoaki/git/hakorune-selfhost/src/runtime/mirbuilder_emit.rs) and direct [src/runner/json_v0_bridge/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/json_v0_bridge/mod.rs) imports support before reusing only shared MIR(JSON) emission |
| `legacy AST JSON compat keep` | [src/host_providers/mir_builder/lowering/ast_json.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder/lowering/ast_json.rs) | `program_json_to_mir_json(...)` legacy AST fallback branch | Phase-0 AST JSON compat route; keep separate from the pure `.hako` blocker accounting and retire only after the primary Program(JSON v0) lowering path no longer needs it |
| `build surrogate keep` | [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | `emit_program_json_v0_for_current_stage1_build_box_mode(...)` | compiled-stage1 `BuildBox.emit_program_json_v0` surrogate; public result is now payload `String` only |
| `build surrogate test keep` | [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | same as above | route-selection plus invoke-by-name build-box/MIR handoff regression coverage are now fully co-located with the surrogate owner |
| `future-retire bridge` | [src/runner/stage1_bridge/program_json/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/mod.rs), [src/runner/stage1_bridge/program_json/read_input.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/read_input.rs), [src/runner/stage1_bridge/program_json/writeback.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/writeback.rs) | `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)` | `program_json/mod.rs` is now a thin facade plus the bridge-local read->emit->write chain and owner-1 payload emission, while `read_input.rs` / `writeback.rs` own the remaining detailed policies under the same future-retire bucket |
| `future-retire bridge entry` | [src/runner/stage1_bridge/program_json_entry/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/mod.rs), [src/runner/stage1_bridge/program_json_entry/request.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/request.rs), [src/runner/emit.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/emit.rs), [src/runner/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/mod.rs) | `program_json_entry::{emit_program_json_v0_requested, emit_program_json_v0_and_exit}` | delegate-only entry; `mod.rs` is now a thin facade plus exact bridge-specific success/error process-exit formatting, `request.rs` owns request building/source-path precedence, and outer callers stay thin |
| `.hako` live/bootstrap callers | [lang/src/runner/stage1_cli_env.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli_env.hako), [lang/src/runner/stage1_cli.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako), [lang/src/runner/launcher.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/launcher.hako), [lang/src/mir/builder/MirBuilderBox.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/mir/builder/MirBuilderBox.hako) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | live/bootstrap boundary on the `.hako` side; delete order must respect these callers separately from Rust host cleanup |
| `shell helper keep` | [tools/hakorune_emit_mir.sh](/home/tomoaki/git/hakorune-selfhost/tools/hakorune_emit_mir.sh), [tools/selfhost/selfhost_build.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/selfhost_build.sh), [tools/smokes/v2/lib/test_runner.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/lib/test_runner.sh) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | helper/canary route; must be caller-audited before delete slices touch shared shell contracts |
| `diagnostics/probe keep` | [tools/dev/phase29ch_program_json_helper_exec_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_program_json_helper_exec_probe.sh), [tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh), [tools/dev/phase29ch_selfhost_program_json_helper_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_selfhost_program_json_helper_probe.sh) | `MirBuilderBox.emit_from_program_json_v0(...)` | diagnostics-only keep; should be retired after live callers, not before |

## Delete Order Guard

1. do not touch `current authority` until a non-JSON authority path exists
2. thin `build surrogate keep` and `future-retire bridge` as separate owner buckets
3. keep `.hako` live/bootstrap callers and diagnostics/probes out of the same patch as Rust host caller deletion

## Retreat Finding

- current compiled-stage1 `BuildBox.emit_program_json_v0` surrogate is still a real live bucket
- however, it is now physically isolated to [build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs), and its route-selection/invoke-by-name regression tests plus route match now live there too, so future retirement can remove one owner-local module without reopening shared MirBuilder dispatch logic
- shared route-table keep is gone: `module_string_dispatch.rs` now probes `build_surrogate::try_dispatch(...)` before the shared dispatch table, and `BUILD_BOX_MODULE` no longer appears outside `build_surrogate.rs`
- the surrogate handler and route match are both owner-local now; shared `module_string_dispatch.rs` no longer owns surrogate registration
- `crates/nyash_kernel/src/tests.rs` no longer owns build-box receiver/method strings; compiled-stage1 build-surrogate regression coverage now stays in `build_surrogate.rs`
- root-level kernel regression no longer touches this bucket; launcher MIR handoff now stays in `build_surrogate.rs` with the surrogate route contract itself
- outside `src/runner/stage1_bridge/**`, direct `emit_program_json_v0` flag reading is now gone; the remaining outer caller contract is only [src/runner/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/mod.rs) `skip_stage1_stub` route selection plus [src/runner/emit.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/emit.rs) early-exit dispatch
- `ProgramJsonEntryOutcome` is gone from the outer caller surface; exact success/error process-exit formatting now stays in `program_json_entry/mod.rs`
- bridge-local source-path precedence (`stage1::input_path()` aliases first, CLI input fallback second) now stays in `program_json_entry/request.rs`, so `program_json/mod.rs` no longer depends on `CliGroups`
- bridge-local read->emit->write orchestration now stays in `program_json/mod.rs`, so the bridge cluster no longer needs a separate orchestration leaf for this route
- therefore, do not spend the next delete slice on root-runner reshaping yet; treat those two files as `must-stay thin callers` until the bridge bucket itself is ready to retire, and use `P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md` as the exact delete-order SSOT for that bucket
- `MirBuilderBox.emit_from_source_v0(...)` is still a live keep, not a diagnostics-only probe bucket; do not collapse it into shell/probe cleanup planning yet
- outside the helper trio in the `shell helper keep` row, there is a test-only smoke/apps tail with 43 direct caller files; record that tail as caller-audit work rather than mixing it into the first Rust-only delete slices, and use `P2-LIVE-CALLER-DELETE-ORDER.md` as the delete-order SSOT for that outer bucket
- `.hako` owner thinning can proceed one file at a time; `lang/src/runner/launcher.hako` now groups its checked Program(JSON) / MIR direct call sites behind owner-local helpers instead of keeping three duplicated checked paths
- `.hako` owner thinning is also valid for `lang/src/runner/stage1_cli.hako`; its direct `BuildBox.emit_program_json_v0(...)` checked path now stays behind an owner-local helper instead of remaining duplicated between `emit_program_json(...)` and `_mode_emit_program(...)`
- `.hako` owner thinning is also valid for `lang/src/runner/stage1_cli_env.hako`; its authority box now keeps direct `BuildBox.emit_program_json_v0(...)` behind a same-file helper shared by authority emit and defs-synthesis lowering
- `.hako` owner thinning is also valid for `lang/src/mir/builder/MirBuilderBox.hako`; `emit_from_source_v0(...)` now keeps its direct `BuildBox.emit_program_json_v0(...)` source-entry shim behind an owner-local helper, while `emit_from_program_json_v0(...)` remains the separate owner policy surface
- shared shell helper keep is not homogeneous; use `P3-SHARED-SHELL-HELPER-AUDIT.md` for the exact helper order (`tools/hakorune_emit_mir.sh` -> `tools/selfhost/selfhost_build.sh` -> `tools/smokes/v2/lib/test_runner.sh`) and keep the 43-file smoke tail separate from helper-local slices
- `tools/hakorune_emit_mir.sh` now keeps its embedded selfhost/provider runner generation behind helper-local shell functions, so the next helper audit can stay inside that script without touching `selfhost_build.sh` or `test_runner.sh`
- `tools/hakorune_emit_mir.sh` still remains a helper-local `Stage-B Program(JSON) production + imports normalize + Program→MIR fallback funnel`; retire/order work should thin the Stage-B Program(JSON) production block before attempting to remove direct-emit fallback or legacy delegate lanes
- `tools/hakorune_emit_mir.sh` now keeps Stage-B Program(JSON) production + imports normalize behind `emit_stageb_program_json_v0()`, so the remaining helper-local work is the direct-emit fallback / delegate tail rather than raw production wiring
- `tools/hakorune_emit_mir.sh` now keeps the provider-first Program→MIR delegate funnel behind `emit_mir_json_from_program_json_delegate_chain()`, with `try_legacy_program_json_delegate()` isolating the old CLI fallback, so the next safe helper-local slice is the direct-emit fallback lane alone
- `tools/hakorune_emit_mir.sh` now keeps the Stage-B fail/invalid -> direct MIR emit fallback behind `exit_after_direct_emit_fallback()`, so the helper-local fallback funnel is split into exact lanes and further helper-local work no longer needs duplicated top-level fallback branches
- `tools/selfhost/selfhost_build.sh` now keeps its Stage-B Program(JSON) raw-production split behind `emit_stageb_program_json_raw()`, with `emit_program_json_v0_via_buildbox()` and `emit_program_json_v0_via_stageb_compiler()` isolating the two live build-contract lanes while leaving `HAKO_USE_BUILDBOX=1` as an explicit keep
- `tools/selfhost/selfhost_build.sh` still has an explicit `HAKO_USE_BUILDBOX=1` build-contract keep in code, but `apps/tests/hello_simple_llvm.hako` no longer justifies it as a malformed-producer rescue: the default compiler Stage-B lane and the BuildBox keep both emit `Extern(log 42) + Return(Int 0)` there
- `tools/selfhost/selfhost_build.sh` now pins that live keep behind `buildbox_emit_only_keep_requested()`, so the exact `HAKO_USE_BUILDBOX=1` emit-only/no-EXE predicate is code-side SSOT instead of being re-read inline at the top level
- `tools/selfhost/selfhost_build.sh` now keeps its post-emit raw/extract contract behind `extract_program_json_v0_from_raw()`, `persist_stageb_raw_snapshot()`, and `exit_after_stageb_emit_failure()`, so remaining helper-local work can target downstream consumers (`--mir` / `--exe` / `--run`) without reopening the raw capture block
- `tools/selfhost/selfhost_build.sh` now keeps the source-direct `--mir` consumer behind `emit_mir_json_from_source()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane and the Core-Direct `--run` lane
- `tools/selfhost/selfhost_build.sh` now also keeps the Core-Direct `--run` consumer behind `run_program_json_v0_via_core_direct()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane alone
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR->EXE consumer behind `emit_exe_from_program_json_v0()`, so all remaining downstream consumers are explicit owner-local helpers rather than top-level branches
- `tools/selfhost/selfhost_build.sh --mir` is green on `apps/tests/hello_simple_llvm.hako` and still bypasses Program(JSON) production
- `tools/selfhost/selfhost_build.sh --run` is green on the repaired default Stage-B payload
- `tools/selfhost/selfhost_build.sh --exe` is green on that same repaired payload
- exact producer-side root-cause evidence is now pinned in `P5-STAGEB-MALFORMED-PROGRAM-JSON.md`: the malformed Program(JSON v0) producer debt exposed by `hello_simple_llvm` is closed for both default Stage-B and BuildBox keep, so the next delete-order work should not treat that fixture as a producer-side blocker anymore
- `tools/smokes/v2/lib/test_runner.sh` is now being thinned inside the shared harness itself: the provider emit lane inside `verify_program_via_builder_to_core()` now lives behind `emit_mir_json_via_provider_extern_v1()`, so the helper keeps provider ownership exact while avoiding temporary `.hako` wrappers that would reopen vm-hako subset debt
- `tools/smokes/v2/lib/test_runner.sh` now also keeps that Rust CLI Program(JSON v0) fallback behind `run_program_json_v0_via_rust_cli_builder()`, so builder-lane duplication is gone and the remaining helper-local top-level tail is shape/result routing only
- `tools/smokes/v2/lib/test_runner.sh` now also keeps that shape/result routing behind `mir_json_looks_like_v0_module_text()` and `run_built_mir_json_via_verify_routes()`, so the shared harness no longer repeats hv1/core/result routing inline in `verify_program_via_builder_to_core()`
- `tools/smokes/v2/lib/test_runner.sh` now also keeps builder-lane selection and missing-output checks behind `emit_mir_json_via_builder_lanes()`, `emit_mir_json_via_min_runner()`, and `mir_builder_output_missing()`, so the shared harness is down to orchestration / no-fallback policy / final route dispatch rather than staging minimal-runner env setup inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps builder debug dumping and Rust CLI fallback handling behind `dump_builder_debug_logs()` and `run_rust_cli_builder_fallback_for_verify()`, so shared-harness cleanup no longer mixes fallback log/copy/removal details into the orchestration path
- `tools/smokes/v2/lib/test_runner.sh` now also exposes a pure emission seam via `emit_mir_json_via_builder_from_program_json_file()` and `builder_min_runner_code()`, so future smoke-tail reduction can target a shared MIR-text producer before touching Core execution routing or Rust CLI fallback
- the first exact smoke-tail caller bucket is now partially collapsed: `phase2044/mirbuilder_provider_*` wrappers use `run_verify_program_via_preferred_mirbuilder_to_core()`, so repeated prefer-provider env setup is no longer owned by individual canary scripts
- the adjacent `phase2044/hako_primary_no_fallback_*` bucket now also uses `run_verify_program_via_hako_primary_no_fallback_to_core()`, so repeated no-fallback/internal builder env setup is no longer owned by individual canary scripts either
- the first `phase2160/builder_min_*` bucket now uses `run_program_json_via_builder_module_vm("hako.mir.builder.min", ...)`, so repeated temp wrapper construction and default stage3/using vm launch setup are no longer owned by individual canary scripts
- the adjacent `phase2160/registry_optin_*` bucket now collapses through three exact helper contracts: plain wrappers use `run_program_json_via_registry_builder_module_vm(...)`, preinclude-heavy wrappers use `run_program_json_via_registry_builder_module_vm_with_preinclude(...)`, and the visible diagnostic wrapper uses `run_program_json_via_registry_builder_module_vm_diag(...)`
- retreat note: `registry_optin_method_arraymap_direct_canary_vm.sh` remains the only explicit registry legacy keep because it still carries a direct-lower probe contract; do not mix that file into the shared registry collapse or helper-retirement accounting
- even after that split, do not mix `test_runner.sh` helper-local work with the 43-file smoke tail; keep the harness owner and the caller tail as separate delete-order buckets
- the old phase2044 runtime/entry blocker is closed: `warn_alias_once()` now self-initializes Ring0 on early alias warnings, `json_v0_bridge::lower_main_body()` keeps `main` params canonical, and `tools/smokes/v2/profiles/integration/core/phase2044/mirbuilder_provider_emit_core_exec_canary_vm.sh` is green again
- route split is now explicit for `apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako`: direct CLI `--backend mir --emit-mir-json` now lowers in both default release and strict/dev shadow mode, while the Rust host-provider route and the language-level `lang.mir.builder.MirBuilderBox.emit_from_source_v0` surface (currently kernel-dispatch owned rather than pure `.hako`-internal proof) also lower the same fixture successfully; treat this as route/boundary evidence, not as proof that all owners are merged, and use `P4-MIRBUILDER-ROUTE-SPLIT.md` for the exact call-chain SSOT
- current Rust authority is now physically split: `src/host_providers/mir_builder.rs` is only a thin façade, `authority.rs` owns `source -> Program(JSON v0)`, `lowering/program_json.rs` owns the real `Program(JSON v0) -> MIR(JSON)` blocker, and `lowering/ast_json.rs` is explicit legacy compat keep; use that split when deciding the next pure-`.hako` blocker slice
- source-route Program(JSON) tuple leakage is gone from kernel dispatch: cross-crate source callers stay on `source_to_mir_json(...)`, while `source_to_program_and_mir_json(...)` is test-only owner-local evidence, and `user_box_decls` injection is now shared inside `src/host_providers/mir_builder/user_box_decls.rs`
- exact `Program(JSON v0) -> MIR(JSON)` lowering caller accounting is now pinned: the remaining live caller owners are `authority.rs` (current source authority), `module_string_dispatch.rs` (explicit Program(JSON) kernel route), and the shared runtime/plugin bridge owner `src/runtime/mirbuilder_emit.rs`; this is narrower than “all of mir_builder” and should guide the next de-Rust slice
- worker audit on that caller trio said the safest next owner-local slice was the kernel/plugin route in `module_string_dispatch.rs`; both the host-provider call narrowing and the `user_box_decls` splice move are now landed, so the remaining kernel-side leaf is only route-local gate/decode/encode shape
- runtime/plugin `env.mirbuilder.emit` is now concentrated behind `src/runtime/mirbuilder_emit.rs`; `extern_provider.rs` and `plugin_loader_v2/enabled/extern_functions.rs` are thin callers, and `calls/global.rs` delegates instead of keeping a second direct lowering branch
- `src/stage1/program_json_v0/authority.rs` should still be treated as the strict source-authority owner, not as generic cleanup debt; the future-retire bridge shim is now split to `src/stage1/program_json_v0/bridge_shim.rs`, so the remaining owner is even closer to the real authority core

## Immediate Next Slice

Prefer the smallest Rust-owned bucket first:

1. `build surrogate keep`
2. `future-retire bridge`
3. only then revisit `.hako` live/bootstrap callers
