---
Status: Accepted (formal-close-sync-ready)
Decision: accepted
Date: 2026-03-13
Scope: `phase-29ci` closeout-ready 後に、Rust-owned `Program(JSON v0)` bootstrap boundary の本体 retirement を 1 owner ずつ進める separate phase pointer。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-29ci/README.md
  - docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md
  - docs/development/current/main/phases/phase-29ci/P1-FUTURE-RETIRE-BRIDGE-DELETE-ORDER.md
  - docs/development/current/main/design/selfhost-bootstrap-route-ssot.md
  - docs/development/current/main/design/selfhost-compiler-structure-ssot.md
  - crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs
  - src/runner/stage1_bridge/program_json/mod.rs
---

# Phase 29cj: Rust-Owned Program JSON v0 Retirement Wave

## Goal

`phase-29ci` で caller/delete-order を closeout-ready に固定したあと、

- `build surrogate keep`
- `future-retire bridge`

の Rust-owned buckets を 1 owner-local wave ずつ薄くして、
`Program(JSON v0)` bootstrap boundary の本体 retirement を進める。

この phase は helper/smoke-tail collapse を再実行する場所ではない。
shared helper / smoke-tail 側は `phase-29ci` で closeout-ready に固定し、
ここでは Rust-owned boundary 本体だけを主語にする。

## Entry Conditions

1. `phase-29ci` が closeout-ready
   - shared helper / smoke-tail collapse is documented
   - remaining explicit registry keep is direct-lower probe only
2. proof bundle is still green
   - `bash tools/dev/phase29cg_stage2_bootstrap_phi_verify.sh`
   - `bash tools/selfhost_identity_check.sh --mode {smoke,full} --skip-build`
3. no new `.hako` workaround or shell-contract widening is introduced

## Fixed Order

1. `build surrogate keep`
   - [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs)
2. `future-retire bridge`
   - [src/runner/stage1_bridge/program_json/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/mod.rs)
   - [src/runner/stage1_bridge/program_json_entry/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/mod.rs)
3. only after Rust-owned buckets shrink, reconsider boundary deletion itself

## Non-goals

- reopening `phase-29ci` helper/smoke-tail collapse
- pulling `registry_optin_method_arraymap_direct_canary_vm.sh` into helper retirement
- widening `.hako` live/bootstrap caller contracts
- mixing authority migration back into `phase-29ch`

## Immediate Next

1. formal close sync
   - freeze `phase-29cj` as `formal-close-sync-ready`
   - keep the remaining live Rust stop-line wording pinned to `src/host_providers/mir_builder.rs`, with targeted proof centered on `module_to_mir_json(...)`
2. keep the strict source-authority and surrogate buckets frozen
   - `src/stage1/program_json_v0/authority.rs`
   - `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
   - these are exact owners, but no longer active phase fronts
3. keep bridge and `.hako` helper waves closed
   - `program_json/` and `program_json_entry/` stay near thin floor
   - `.hako` owner/helper local cleanup stays `closeout-ready`
4. keep the direct-lower probe as explicit evidence until the formal close sync lands
5. do not confuse this phase close sync with the primary pure-`.hako` blocker
   - the real current blocker is still the Rust stop-line `src/host_providers/mir_builder.rs::module_to_mir_json(...)`
   - `src/host_providers/mir_builder/lowering.rs` is test-only evidence, not the live phase front
6. after close sync, the next real movement is authority replacement above the stop-line
   - first `.hako` replacement owner remains `lang/src/mir/builder/MirBuilderBox.hako`
   - runner owners follow
   - `lang/src/compiler/build/build_box.hako` stays behind them because of blast radius

## Status Lock

- `.hako` owner/helper local thinning wave: `closeout-ready`
- `phase-29cj` overall: `formal-close-sync-ready`
- the remaining live Rust stop-line before close sync is concentrated in `src/host_providers/mir_builder.rs`, with targeted proof centered on `module_to_mir_json(...)`
- frozen exact owners after the latest stop-line audit:
  1. `src/stage1/program_json_v0/authority.rs`
  2. `crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs`
- do not reopen `.hako` local thinning only because those Rust-owned owners are still live

## Retreat Finding

- `phase-29ci` already closed the helper-side collapse, so further progress now depends on Rust-owned buckets moving, not on more shell cleanup
- `registry_optin_method_arraymap_direct_canary_vm.sh` is no longer “cleanup debt”; it is an explicit probe keep and should stay outside the shared-helper accounting
- the first productive slice already removed the shared route-table keep by moving surrogate route matching into `build_surrogate.rs`; current review treats that bucket as near thin floor rather than the next automatic shrink target
- the latest exact leaf on that same owner keeps route match, source-handle decode, stage1 emit, and result encode behind same-file helpers in `build_surrogate.rs`; after this, keep treating that owner as near thin floor unless another exact disappearing leaf appears
- after that `build_surrogate.rs` slice, the phase front may switch to `future-retire bridge`; the first bridge-entry leaf is `src/runner/stage1_bridge/program_json_entry/request.rs`, while `src/stage1/program_json_v0/authority.rs` stays frozen as the strict source-authority core
- `future-retire bridge` is now smaller on both sides: `program_json/emit_payload.rs`, `program_json/pipeline.rs`, and `program_json_entry/exit.rs` are gone, so the remaining inner bridge leaves are concentrated in `program_json/mod.rs` and `program_json_entry/request.rs`
- because `program_json_entry/request.rs` still touches env alias precedence and outer-caller-facing request extraction, it is not the default next slice; prefer bridge-local-only collapse before touching that contract leaf
- the latest bridge-entry leaf keeps emit-flag presence and out-path extraction behind shared helper `emit_program_json_out_path_ref(...)` in `program_json_entry/request.rs`; after this, treat that owner as near thin floor unless another exact disappearing leaf appears first
- after that bridge-entry tightening, `program_json_entry/request.rs` is no longer the active phase front; freeze it as near thin floor and return active slices to the Rust stop-line in `src/host_providers/mir_builder.rs`
- current authority is now exact enough to avoid hand-wavy blocker accounting: `src/host_providers/mir_builder.rs` owns the source-route handoff, explicit Program(JSON) route, shared `user_box_decls` shaping, and live MIR(JSON) emission stop-line, while `src/host_providers/mir_builder/lowering.rs` is now the test-only Program(JSON)->MIR evidence seam; `src/stage1/program_json_v0/authority.rs` remains the strict source-authority owner behind them
- the latest exact leaf on that stop-line now makes `src/host_providers/mir_builder.rs::module_to_mir_json(...)` read as `emit_module_to_temp_mir_json(...)` -> `finalize_temp_mir_json_output(...)`, with temp-file read, cleanup, and JSON canonicalization closed behind that same-file tail; the Rust stop-line itself is unchanged
- the latest exact leaf on the explicit Program(JSON) route also keeps MIR JSON parse/root mutation behind `parse_mir_json_value(...)` and `insert_user_box_decls(...)`; the façade owner still holds explicit-route `user_box_decls` shaping
- the latest explicit-route leaf also keeps Program(JSON) parse / box-name collect / decl materialization behind `parse_program_json_value(...)`, `collect_stage1_user_box_decl_names(...)`, and `stage1_user_box_decl_from_name(...)`; the owner still holds explicit-route shaping, but its inner tail is thinner
- the latest source-authority leaf also keeps duplicate strict-source Program(JSON) emission behind same-file helper `emit_strict_program_json_for_source(...)`; the façade owner still holds the strict source route above the Rust stop-line
- the latest explicit-entry leaf also keeps Program(JSON) module parse behind `parse_program_json_module(...)`, and defs iteration behind `insert_stage1_def_box_names(...)`; the active front stays in `src/host_providers/mir_builder.rs`
- the latest explicit-entry leaf also keeps `module_to_mir_json(...)` plus user-box finalize handoff behind same-file helper `emit_mir_json_with_user_box_decls(...)`; the active front still stays in `src/host_providers/mir_builder.rs`
- the latest source-route leaf also keeps the final Program(JSON text) -> explicit-route call behind same-file helper `emit_mir_json_from_program_json_text(...)`
- the latest strict-source public/test leaf also keeps Program(JSON) emission plus shared source-pair helper `emit_program_and_mir_json_for_source(...)`, with guarded/plain MIR handoff exposed through `emit_program_and_guarded_mir_json_for_source(...)` and `emit_program_and_plain_mir_json_for_source(...)`; the live stop-line still stays in `src/host_providers/mir_builder.rs`
- the latest source/explicit-route handoff leaf now keeps the shared env guard behind same-file helper `emit_guarded_mir_json_from_program_json(...)`
- the latest explicit-route finalize leaf also keeps `Program(JSON)` parse/build separate from MIR JSON value-build/serialize at `finalize_mir_json_with_stage1_user_box_decls(...)` -> `build_stage1_user_box_decls_from_program_json(...)` -> `inject_user_box_decls_into_mir_json(...)` -> `build_mir_json_with_user_box_decls(...)`
- worker order decision is now pinned: retire the dedicated `src/host_providers/mir_builder/authority.rs` adapter, fold the extra shared shaping leaf into `src/host_providers/mir_builder.rs`, and stop the kernel Program(JSON) route at thin floor unless an exact disappearing route leaf appears
- the test-only transient `(Program JSON, MIR JSON)` tuple helper still lives only in the `src/host_providers/mir_builder.rs` façade test surface
- the dedicated `src/host_providers/mir_builder/authority.rs` adapter is retired from the active owner surface, the extra `user_box_decls.rs::source_to_mir_json_with_user_box_decls(...)` leaf is gone, and shared Program(JSON) shaping is now folded into `src/host_providers/mir_builder.rs`; live source-route callers now enter through that façade directly
- imports-bearing `program_json_to_mir_json_with_imports(...)` is now test-only in `src/host_providers/mir_builder.rs`; live imports-bearing lowering stays off the façade surface
- plain `program_json_to_mir_json(...)` is now also test-only in `src/host_providers/mir_builder.rs`; the live explicit Program(JSON) route stays on `program_json_to_mir_json_with_user_box_decls(...)`
- imports-bearing lowering is also test-only inside `src/host_providers/mir_builder/lowering.rs`
- live source + explicit Program(JSON) callers now both stay in `src/host_providers/mir_builder.rs`, and cross the shared Rust seam only at `module_to_mir_json(...)`
- the extra `lower_program_json_to_module(...)` leaf is retired, and `src/host_providers/mir_builder/lowering.rs` now keeps only evidence/test seams around that path
- worker consensus now keeps `src/host_providers/mir_builder.rs` as the only active phase front until the remaining handoff/finalize leaves above `module_to_mir_json(...)` are thin enough to freeze
- worker consensus also treats `src/stage1/program_json_v0/authority.rs` as frozen strict source-authority core; the next real movement is authority replacement above the Rust stop-line in `src/host_providers/mir_builder.rs`
- worker consensus on `src/host_providers/mir_builder/lowering.rs`: the remaining helpers there are evidence-only, while `module_to_mir_json(...)` is the real shared seam and now lives in `src/host_providers/mir_builder.rs`
- the latest test-only source-evidence leaf now keeps plain `Program(JSON)` -> MIR handoff behind same-file helper `emit_plain_mir_json_from_program_json_text(...)`
- the latest explicit-route finalize leaf now keeps `Program(JSON)` parse/build and MIR JSON mutation separated behind same-file helpers `build_stage1_user_box_decls_from_program_json(...)`, `parse_program_json_value(...)`, `build_stage1_user_box_decls(...)`, and `inject_user_box_decls_into_mir_json(...)`
- the latest explicit/source handoff bundle now keeps shared route detail behind `Stage1ProgramJsonModuleHandoff`, `SourceProgramJsonHandoff`, and `with_phase0_mir_json_env(...)`, so both live entries read as owner-local handoff -> `module_to_mir_json(...)`
- the latest explicit-route authority cut now keeps root `user_box_decls` payload authoritative when it is already present, through `resolve_stage1_user_box_decls_from_program_json(...)` -> `resolve_stage1_user_box_decls(...)` -> `explicit_stage1_user_box_decls(...)`, then maps that payload into `MirModule.metadata.user_box_decls` via `with_stage1_user_box_decls(...)`; defs-mining remains compat fallback only
- that same cut retires the old emitted-MIR reparse/root-mutation tail from the active route; explicit Program(JSON) now reaches `module_to_mir_json(...)` through module-metadata passthrough instead of post-emit JSON splice
- the latest stop-line cut after that also retires temp MIR file round-trip from `module_to_mir_json(...)`: the owner now emits through `src/runner/mir_json_emit/mod.rs::emit_mir_json_string_for_harness_bin(...)`, so the active route no longer writes temp MIR, rereads it, deletes it, and canonicalizes it back to compact JSON
- the latest explicit-route shaping cut now keeps Program(JSON) `user_box_decls` parse-entry and explicit-vs-compat precedence behind `Stage1UserBoxDeclHandoff::resolve_user_box_decls()`, `explicit_user_box_decls()`, and `compat_user_box_decls()`, so `stage1_program_json_module_handoff(...)` now reads as module parse + same-owner decl handoff while `insert_stage1_def_box_names(...)` remains the compat fallback seam
- the latest explicit-route finalize cut now also keeps the module metadata splice inside `Stage1ProgramJsonModuleHandoff::into_module_with_user_box_decls()`, retiring the old free finalize helpers and leaving the remaining active leaf on the `stage1_user_box_decl_map()` metadata-map seam above `module_to_mir_json(...)`
- the latest explicit-route finalize cut also retires that metadata-map seam by moving it behind `Stage1UserBoxDecls::into_metadata_map()`, so the remaining active leaf is now the module-entry seam at `stage1_program_json_module_handoff(...)` -> `parse_program_json_module(...)`
- the latest explicit-route shaping cut also retires the remaining raw decl JSON seam by changing `Stage1UserBoxDecls` to hold typed `Stage1UserBoxDecl` entries, so explicit/compat decl shaping and metadata-map finalize no longer pass through `Vec<serde_json::Value>`
- the latest explicit-route entry cut now also folds the free module-entry pair into `Stage1ProgramJsonInput::into_module_handoff()` / `parse_module()`, so the remaining active leaf is the same-owner split between `Stage1ProgramJsonInput` and `Stage1UserBoxDeclHandoff::parse(...)`
- the latest explicit-route entry cut now also folds the free module-entry wrapper into `Stage1ProgramJsonModuleHandoff::parse(...)`, and folds the free source-entry wrapper into `SourceProgramJsonHandoff::for_source(...)`; the remaining active leaf is now the same-owner split between `Stage1ProgramJsonInput::into_module_handoff()` and `Stage1UserBoxDeclHandoff::parse(...)`
- the latest explicit-route value-entry cut now also folds Program(JSON) parse for decl resolution into `Stage1ProgramJsonInput::{resolve_user_box_decls,parse_user_box_decl_handoff,parse_value}`, so the remaining active leaf is the typed parsed-value handoff at `Stage1UserBoxDeclHandoff::from_program_value(...)`
- the latest explicit-route value-entry cut also folds that typed parsed-value handoff into `Stage1ProgramJsonValue::{parse,resolve_user_box_decls}`, retiring `Stage1UserBoxDeclHandoff`; the remaining active leaf is the thin parsed-value shim at `Stage1ProgramJsonInput::parse_value()`
- proof for that cut is `cargo test user_box_decls -- --nocapture`, including both the host-provider route and the runner `--program-json-to-mir` file route
- worker audit also raised the next non-Rust wave order after the current Rust-owned front: `lang/src/mir/builder/MirBuilderBox.hako` first, then runner owners `lang/src/runner/{stage1_cli_env.hako,stage1_cli.hako,launcher.hako}`, with shared producer `lang/src/compiler/build/build_box.hako` immediately behind that same wave; touching `build_box.hako` before those owner-local callers would be the highest-blast-radius move
- owner-role lock for this wave:
  - `authority owner`: live owner that decides input acceptance, route selection, fail-fast tags, and final handoff for the compiler boundary
  - `thin facade`: route/decode/encode/orchestration-only owner that should stop growing once the contract is readable
  - `compat keep`: historical/probe/helper lane retained for exact evidence, not for new authority logic
- therefore, the next wave should keep spending slices on `MirBuilderBox.hako`, runner owners, and helper-local shell callers above the Rust stop-line; do not reopen thin facades or compat keeps just because they still exist
- the kernel `emit_from_program_json_v0` / `emit_from_source_v0` pair now also shares same-file gate/decode/freeze helpers, so the remaining kernel work is explicitly thin-floor support code rather than a fresh authority-removal front
- the nearby future-retire bridge shim is now split out to `src/stage1/program_json_v0/bridge_shim.rs`, so `src/stage1/program_json_v0/authority.rs` no longer mixes bridge-specific error wrapping with strict source authority
- the first landed `.hako` authority-replacement slice now lives in `lang/src/runner/stage1_cli_env.hako`: `Stage1SourceMirAuthorityBox` owns the source-entry `BuildBox.emit_program_json_v0(...)` shim locally and delegates only Program(JSON) -> MIR to `MirBuilderBox.emit_from_program_json_v0(...)`
- the next landed tightening on the same owner keeps the direct `MirBuilderBox.emit_from_program_json_v0(...)` checked path behind same-file helper `Stage1ProgramJsonMirCallerBox`, shared by `Stage1SourceMirAuthorityBox` and `Stage1ProgramJsonCompatBox`
- the next landed tightening on that same helper keeps `Stage1ProgramJsonMirCallerBox` itself on the checked-contract split `_coerce_program_json_text_checked(...)` -> `_emit_mir_from_program_json_text_checked(...)`, so source authority and explicit Program(JSON) compat keep no longer share a mixed input-check + MirBuilder-call body
- the next landed tightening on the same owner keeps `Stage1ProgramJsonCompatBox` on the same shape too: `_coerce_program_json_text_checked(...)` now owns explicit Program(JSON) compat input validation before reusing `Stage1ProgramJsonMirCallerBox`
- the next landed tightening on the same owner also keeps `Stage1MirResultValidationBox` behind `_materialize_mir_text_with_debug(...)`, `_debug_print_mir_state(...)`, and `_validate_mir_text_checked(...)`, so the result lane no longer mixes materialization/debug with structural MIR validation inline
- the next landed tightening on the same owner also keeps compat/result tiny leaves behind `_has_explicit_program_json_text(...)`, `_fail_mixed_source_mode(...)`, `_print_validated_mir_result_checked(...)`, and `_fail_invalid_mir_text(...)`, so `Stage1ProgramJsonCompatBox` and `Stage1MirResultValidationBox` are closer to pure checked handoff owners
- the next landed `.hako` authority-replacement slice now lives in `lang/src/mir/builder/MirBuilderBox.hako`: the delegate branch of `emit_from_program_json_v0(...)` finalizes MIR locally by injecting `user_box_decls` before normalization, instead of leaving that shaping solely to Rust-owned provider surfaces
- `MirBuilderBox.hako` now also reads internal/delegate gate decisions via `lang/src/mir/builder/internal/builder_config_box.hako`, which is the last safe structural split on this front; `emit_from_source_v0(...)` stays a live compat seam for kernel route + route-evidence probes and should not be retired in the same wave
- the next tightening on that same owner keeps the source-entry compat seam behind owner-local helpers `_coerce_source_text_checked(...)`, `_emit_program_json_from_source_checked(...)`, and `_emit_mir_from_source_program_json_checked(...)`, so `emit_from_source_v0(...)` is now a thin shim without touching `emit_from_program_json_v0(...)` policy itself
- the next tightening on that same owner keeps the internal unsupported tail behind `_fail_internal_unsupported(...)` and `_program_json_has_ternary(...)`, so `_emit_internal_program_json(...)` now shows only loop-force / registry / fallback / fail-fast route order
- the normal registry-first `Program(JSON v0) -> MIR(JSON)` authority block now lives in `lang/src/mir/builder/internal/registry_authority_box.hako`
- the remaining non-registry/internal fallback chain now lives in `lang/src/mir/builder/internal/fallback_authority_box.hako`
- the next landed tightening on the same owner fixes the route contract itself: `BuilderConfigBox.internal_on()/registry_on()` now return numeric `1/0`, and the stage1 module registry/export now includes `lower_loop_count_param_box`, `registry_authority_box`, and `fallback_authority_box`; as a result `tools/hakorune_emit_mir_mainline.sh lang/src/runner/{stage1_cli.hako,stage1_cli_env.hako}` is green on selfhost-first + no-delegate
- the next landed tightening on `lang/src/runner/stage1_cli.hako` keeps source/program-json orchestration behind same-file helpers (`_resolve_emit_program_source_text(...)`, `_resolve_program_json_for_emit_mir(...)`, `_resolve_program_json_for_run(...)`, `_load_program_json_from_path_or_source(...)`), and `stage1_main(...)` now reuses `_resolve_mode/_resolve_source/_resolve_program_json_path/_resolve_backend` instead of re-reading the env contract inline
- the next landed tightening on `lang/src/runner/stage1_cli.hako` keeps the raw subcmd emit-mir checked contract behind `_coerce_program_json_for_emit_mir_checked(...)`, `_emit_mir_from_program_json_text_checked(...)`, and `_coerce_mir_output_checked(...)`, so Program(JSON) validation, MirBuilder call, and MIR validation are no longer mixed inline
- the next landed tightening on `lang/src/runner/stage1_cli.hako` also keeps raw/subcmd emit-program checked tail behind `_emit_program_json_raw_with_debug(...)`, `_fail_emit_program_json_null(...)`, and `_coerce_program_json_output_checked(...)`, so the future-retire raw lane no longer mixes BuildBox call, null fail-fast, and Program(JSON) validation inline
- the next landed tightening on `lang/src/runner/launcher.hako` keeps caller-side source/program-json choreography behind same-file helpers (`_emit_program_json_from_source_path_checked(...)`, `_emit_mir_json_from_source_path_checked(...)`, `_load_program_json_from_path_or_source(...)`, `_print_or_write_output(...)`), so `cmd_build_exe(...)`, `cmd_emit_program_json(...)`, and `cmd_emit_mir_json(...)` no longer repeat the read→Program→MIR→write tail inline
- the next landed tightening on `lang/src/runner/launcher.hako` keeps the `emit mir-json` checked contract behind `_coerce_program_json_for_emit_mir_checked(...)`, `_emit_mir_from_program_json_text_checked(...)`, and `_coerce_mir_output_checked(...)`, so Program(JSON) validation, MirBuilder call, and MIR validation are no longer mixed inline there either
- the next landed tightening on `lang/src/runner/launcher.hako` also keeps the `emit program-json` checked tail behind `_emit_program_json_raw(...)` and `_coerce_program_json_output_checked(...)`, so the launcher lane no longer mixes BuildBox call and Program(JSON) validation inline
- the next landed tightening on `lang/src/runner/launcher.hako` also keeps program-json path load and stdout/file output tails behind `_load_program_json_from_path_checked(...)`, `_print_output_checked(...)`, and `_write_output_checked(...)`, so the launcher lane no longer mixes readback/output side effects inline
- the next landed tightening on `lang/src/compiler/build/build_box.hako` keeps shared producer sequencing behind owner-local helpers too: `_bundle_inputs_requested(...)` / `_resolve_scan_src_from_bundle_ctx(...)` now isolate bundle resolve decision + merged `scan_src` materialization, `_emit_program_json_from_scan_src(...)` now owns outer producer sequencing, `_parse_program_json_from_scan_src(...)` now owns parse-source narrowing plus parser call, `_build_defs_fragment_json(...)` now owns defs-scan plus defs-fragment build, and `_inject_stageb_fragments_json(...)` keeps the defs→imports post-parse tail
- the next landed tightening on `lang/src/compiler/build/build_box.hako` also keeps imports-fragment build behind `_build_imports_fragment_json(...)`, so `_inject_imports_json(...)` now reads as fragment build -> inject only
- the next landed tightening on `lang/src/compiler/build/build_box.hako` also keeps defs gate/inject tail behind `_defs_scan_enabled(...)` and `_inject_defs_fragment_if_present(...)`, so `_inject_defs_json(...)` now reads as gate -> defs build -> inject only
- accepted proof for that launcher slice stays on `tools/hakorune_emit_mir_mainline.sh lang/src/runner/launcher.hako ...`; the stricter `stage1_contract_exec_mode ... emit-program launcher.hako ...` path still hits the adjacent strict source-route reject (`dev-local-alias-sugar`), so do not treat that route as a regression introduced by the helper split
- the next landed tightening on the same owner keeps route sequencing behind owner-local helpers `_lower_func_defs_if_enabled(...)`, `_emit_internal_program_json(...)`, and `_emit_delegate_program_json(...)`, so the outer box no longer repeats raw env/hostbridge branching inline
- the next landed tightening on the same owner also keeps the Program(JSON) entry contract behind `_coerce_program_json_checked(...)` and `_emit_mir_from_program_json_text_checked(...)`, so `emit_from_program_json_v0(...)` now shows checked handoff + route dispatch instead of mixing header validation inline
- the next landed tightening on the same owner also keeps internal route leaves behind `_try_emit_loop_force_jsonfrag(...)`, `_try_emit_registry_program_json(...)`, and `_try_emit_fallback_program_json(...)`, so `_emit_internal_program_json(...)` now reads as a pure route table
- the next landed tightening on the same owner also moves the delegate compat gate/provider call into `lang/src/mir/builder/internal/delegate_provider_box.hako::BuilderDelegateProviderBox.try_emit(...)`, so `_emit_delegate_program_json(...)` now reads as internal gate/provider -> local finalize
- the next landed tightening on the same owner also keeps the shared finalize chain behind `_inject_func_defs_checked(...)`, `_methodize_if_enabled_checked(...)`, and `_normalize_jsonfrag_if_enabled_checked(...)`, so `_norm_if_apply(...)` now reads as pure finalize order instead of mixing postprocess leaves inline
- the next landed tightening on the same owner also keeps defs-toggle/source-entry compat tails behind `_func_defs_toggle_on(...)`, `_coerce_func_defs_json(...)`, and `_emit_program_json_from_source_raw(...)`, so those tiny leaves no longer mix inline with checked handoff
- the next landed tightening on the same owner also keeps Program(JSON) fail-fast tiny leaves behind `_program_json_input_present(...)` and `_program_json_header_present(...)`, so `_coerce_program_json_checked(...)` now reads as input-present -> header-present -> handoff only
- consequence: `MirBuilderBox.hako` now keeps route sequencing, generic unsupported/no-match decision, and the remaining local finalize/compat tails around those internal authority owners
- the delegate-only probe now resolves `BuilderDelegateProviderBox` successfully and reaches the separate pre-existing `Unknown Box type: hostbridge` residue; treat that as adjacent subset-check debt, not as a regression from the delegate-provider split
- direct `phase2034/mirbuilder_internal_if_canary_vm.sh` is not promoted into accepted proof yet; it still hits the separate `vm-hako subset-check` blocker on `newbox(hostbridge)` before this owner split becomes observable
- the next pure-`.hako-only` removal wave should not start by shaving `build_surrogate.rs` more; it should keep shrinking the `.hako` owner chain and helper-local shell callers above the Rust stop-line in `src/host_providers/mir_builder.rs`
- runtime/plugin `env.mirbuilder.emit` is now concentrated in `src/runtime/mirbuilder_emit.rs`; `extern_provider.rs` and `plugin_loader_v2/enabled/extern_functions.rs` are thin callers, and `calls/global.rs` no longer owns a separate direct lowering branch
- runtime/plugin `env.mirbuilder.emit` also no longer counts as a live caller of `src/host_providers/mir_builder/lowering.rs`; that helper now lowers through `runner::json_v0_bridge::parse_json_v0_to_module_with_imports(...)` and reuses only shared MIR(JSON) emission
- worker audit agreed the safest next Rust-owned slice was the kernel/plugin Program(JSON) route in `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`; that narrowing is now landed, and the remaining kernel-side leaf is no longer the local `user_box_decls` splice because that responsibility now lives in shared owner `src/host_providers/mir_builder.rs`
- live source + explicit Program(JSON) callers now parse Program(JSON) inside `src/host_providers/mir_builder.rs` and cross the shared seam only at `module_to_mir_json(...)`; the imports-free plain lowering helper in `src/host_providers/mir_builder/lowering.rs` is test-only evidence now
- worker design review now fixes the stop-line: `module_to_mir_json(...)` is the Rust host seam that should remain Rust-owned, while the next `.hako` wave should own `Program(JSON v0) -> MIR(JSON)` above that seam; do not try to move `MirModule` ownership into `.hako`
- after this slice, the kernel/plugin Program(JSON) route is close to thin floor: route-local gate/decode/encode remain, but host-provider call selection and `user_box_decls` shaping no longer live there
- `tools/hakorune_emit_mir.sh` now also keeps the direct `MirBuilderBox.emit_from_program_json_v0(...)` checked path behind a generated wrapper-local `_emit_mir_checked(...)` helper, so the shell/helper wave has started without touching `selfhost_build.sh` or `test_runner.sh`
- `tools/hakorune_emit_mir.sh` now also keeps generated runner stdout -> MIR payload extraction behind `extract_mir_payload_from_stdout_file()` / `persist_mir_payload_from_stdout_file()`, so selfhost/provider helper lanes no longer duplicate `[MIR_OUT_BEGIN]...[MIR_OUT_END]` parsing inline
- `tools/hakorune_emit_mir.sh` now also keeps explicit direct-emit exit and loop-force JSONFrag MIR assembly behind `exit_after_forced_direct_emit()`, `extract_loop_force_limit_from_program_json()`, and `write_loop_force_jsonfrag_mir_json()`, so the helper-local tail is now mostly delegate/fallback route order
- `tools/hakorune_emit_mir.sh` now also keeps that remaining non-direct route order behind `emit_mir_json_via_non_direct_routes()`, so the script top-level is now closer to pure Stage-B -> route handoff orchestration
- the latest shell-helper tightening now also keeps selfhost-first gate/fail-fast, loop-force route, and provider/legacy delegate sequencing behind `try_selfhost_builder_first_route()`, `try_loop_force_jsonfrag_route()`, and `emit_mir_json_via_delegate_routes()`, so `emit_mir_json_via_non_direct_routes()` is down to a small route table
- the latest runner tightening now also keeps `Stage1ProgramJsonCompatBox` on explicit Program(JSON) checked handoff through `_emit_mir_from_text_checked(...)`, while `Stage1MirResultValidationBox` keeps result-materialize/debug/print behind `_coerce_materialized_mir_text_checked(...)`, `_debug_print_selected_input(...)`, `_debug_print_materialized_mir(...)`, and `_emit_validated_mir_text_checked(...)`
- `tools/selfhost/selfhost_build.sh` now also keeps its generated `BuildBox.emit_program_json_v0(...)` checked path behind wrapper-local `_emit_program_json_checked(...)`, so the explicit `HAKO_USE_BUILDBOX=1` keep stays helper-local instead of repeating the checked path inline
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR step behind `emit_mir_json_from_program_json_v0()`, so the downstream EXE helper reads as MIR generation -> ny-llvmc emission instead of mixing both inline
- `tools/selfhost/selfhost_build.sh` now also keeps the MIR(JSON)->EXE step behind `emit_exe_from_mir_json()`, so the downstream EXE helper is now mostly route/env orchestration
- `tools/selfhost/selfhost_build.sh` now also keeps top-level post-emit route order behind `dispatch_stageb_downstream_outputs()`, so the script tail is closer to pure Stage-B -> downstream handoff orchestration
- the latest shell-helper tightening now also keeps the remaining consumer tail behind `announce_program_json_output_if_requested()`, `emit_requested_mir_output_if_needed()`, `exe_output_requested()`, `emit_requested_exe_output()`, `run_program_json_requested()`, `run_requested_program_json()`, and `print_program_json_path_result()`, so `dispatch_stageb_downstream_outputs()` is down to a small downstream route table
- immediate next helper-local order after that selfhost-build slice:
  1. `tools/smokes/v2/lib/test_runner.sh` residual helper-local verify lanes
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the remaining verify-tail policy behind `verify_builder_no_fallback_requested()`, `cleanup_verify_builder_logs()`, and `handle_verify_builder_emit_result()`, so `verify_program_via_builder_to_core()` is closer to a pure emission entry plus tail-policy handoff
- `tools/smokes/v2/lib/test_runner.sh` now also keeps built-MIR route leaves behind `run_built_mir_json_via_hv1_route()`, `run_built_mir_json_via_hako_core_route()`, and `run_built_mir_json_via_core_v0_route()`, so `run_built_mir_json_via_verify_routes()` is down to a small route table
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared phase2044 builder/core env stack behind `run_verify_program_via_builder_to_core_with_env()`, so the phase2044 wrappers are down to thin flag wrappers
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared phase2160 temp-wrapper + vm-launch contract behind `run_program_json_via_builder_module_vm_with_env()`, so the phase2160 builder-min / registry wrappers are down to thin flag wrappers too
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the remaining builder-min no-functions-check lane and the plain/preinclude registry builder-vm lanes behind `run_builder_module_tag_canary()` and `run_registry_builder_tag_canary()`, so the only explicit `phase2160` keeps left are the diag/direct probes
- `tools/smokes/v2/lib/test_runner.sh` now also keeps Rust CLI fallback file-shape check and MIR-file execution behind `mir_json_file_looks_like_v0_module()` and `run_built_mir_json_file_via_core_v0()`, so the fallback lane is closer to pure convert -> verify/run -> cleanup orchestration
- `tools/smokes/v2/lib/test_runner.sh` now also keeps rc assertion and pass/fail formatting for the `phase2044/hako_primary_no_fallback_*` caller bucket behind `run_hako_primary_no_fallback_canary_and_expect_rc()`, so those scripts are now thin fixture wrappers
- `tools/smokes/v2/lib/test_runner.sh` now also keeps rc assertion and pass/fail formatting for the stable single-case `phase2044/mirbuilder_provider_*` caller bucket behind `run_preferred_mirbuilder_canary_and_expect_rc()`, so those scripts are now thin fixture wrappers too while `array_length_alias` / `array_push_size_rc` stay explicit keeps
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the duplicated embedded `MirBuilderBox.emit_from_program_json_v0(...)` checked path behind generator helper `builder_module_program_json_runner_code()`, so both shared module-vm helper lanes reuse the same generated `_emit_mir_checked(...)` contract
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared rc assertion/formatting contract for those `phase2044` callers behind `run_verify_canary_and_expect_rc()`, so the primary-no-fallback and preferred-mirbuilder wrappers are both thin runner adapters
- `tools/smokes/v2/lib/test_runner.sh` now also keeps stable builder/registry stdout-tag skip/pass cleanup behind `run_stdout_tag_canary()`, `cleanup_stdout_file()`, and `stdout_file_has_tag_match()`, with `basic` / `extended` / `fixed` matcher modes separated instead of reimplementing grep policy inline
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the shared registry arraymap exec/tag/functions front behind `prepare_registry_tagged_mir_canary_stdout()`, so `run_registry_method_arraymap_canary()` only owns the remaining method/args/`mir_call` token checks while the explicit diag/direct probe scripts stay outside the helper-local collapse
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the explicit registry arraymap diag probe behind `run_registry_builder_diag_canary()`, so `registry_optin_method_arraymap_get_diag_canary_vm.sh` is a thin fixture wrapper and the only remaining explicit `phase2160` keep is the direct lower probe script
- `tools/smokes/v2/lib/test_runner.sh` now also keeps the explicit registry arraymap direct lower probe behind `run_direct_lower_box_canary()`, `run_direct_lower_box_vm_to_stdout_file()`, and `direct_lower_box_runner_code()`, so `registry_optin_method_arraymap_direct_canary_vm.sh` is a thin fixture wrapper too and the helper-local shell wave is now near thin floor
- after that shell/helper collapse, return active slices to the Rust stop-line `src/host_providers/mir_builder.rs`; do not reopen `test_runner.sh` or the `phase2160` probe wrappers unless another exact disappearing leaf appears first
- the latest shell-helper tightening now also keeps built-MIR builder-only vs preferred-VM route order behind `run_built_mir_json_via_builder_only_route()` / `run_built_mir_json_via_preferred_vm_routes()`, keeps emit-result fallback/success tail behind `run_verify_builder_emit_rust_cli_fallback()` / `cleanup_verify_builder_logs_and_run_built_mir()`, and keeps builder/registry stdout->MIR extraction behind `run_builder_module_vm_to_stdout_file()`, `run_registry_builder_module_vm_to_stdout_file()`, `extract_builder_mir_from_stdout_file()`, and `stdout_file_has_functions_mir()`
- the `future-retire bridge` inner cluster is also thinner now: `src/runner/stage1_bridge/program_json/payload.rs` owns the bridge-local owner-1 payload emission, leaving `program_json/mod.rs` as read->emit->write orchestration only
- the `future-retire bridge` entry cluster is thinner too: `src/runner/stage1_bridge/program_json_entry/exit.rs` now owns exact success/error process-exit formatting, leaving `program_json_entry/mod.rs` as request-build + dispatch only
