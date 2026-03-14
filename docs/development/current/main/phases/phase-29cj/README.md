---
Status: Accepted (queued)
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

1. treat `build_surrogate.rs` as near thin floor unless an exact disappearing leaf is obvious first
2. continue `future-retire bridge` retirement inside the inner bridge cluster only
   - `program_json/` is now `mod.rs` + `read_input.rs` + `writeback.rs`
   - `program_json_entry/` is now `mod.rs` + `request.rs`
3. do not take `program_json_entry/request.rs` next unless bridge-local-only leaves are truly exhausted
4. keep the direct-lower probe as explicit evidence until one Rust-owned bucket actually disappears
5. while this phase keeps the Rust-owned retirement order, do not confuse it with the primary pure-`.hako` blocker
   - the real current blocker is now the exact `Program(JSON v0) -> MIR(JSON)` lowering owner in `src/host_providers/mir_builder/lowering.rs`
   - the real current blocker is now the exact `Program(JSON v0) -> MIR(JSON)` lowering leaf in `src/host_providers/mir_builder/lowering.rs`
   - pinned live callers:
     - `src/host_providers/mir_builder.rs`
     - `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`
     - `src/runtime/mirbuilder_emit.rs`
   - therefore, a future authority-removal slice should narrow those callers before broad cleanup elsewhere
6. exact next ladder after `authority.rs` retirement:
   - landed: `src/host_providers/mir_builder.rs::source_to_mir_json(...)` now owns the live source-route handoff directly
   - landed: `src/host_providers/mir_builder/lowering/program_json.rs::lower_program_json_to_module(...)` is absorbed into `src/host_providers/mir_builder/lowering.rs`
   - next target is the remaining live caller/shaping around that lowering owner
   - keep `src/stage1/program_json_v0/authority.rs` frozen as the strict source-authority core while those host-provider slices are still live
7. authority-replacement rule:
   - treat `src/host_providers/mir_builder.rs` as near thin floor once only `source_to_mir_json(...)`, `program_json_to_mir_json_with_user_box_decls(...)`, and `module_to_mir_json(...)` remain live
   - do not reopen kernel/plugin route cleanup once `crates/nyash_kernel/src/plugin/module_string_dispatch.rs` is down to thin gate/decode/encode support
   - after the current Rust lowering owner is sufficiently thin, switch the phase language from “leaf retirement” to “authority replacement”
   - first `.hako` replacement owner remains `lang/src/mir/builder/MirBuilderBox.hako`; runner owners follow, and `lang/src/compiler/build/build_box.hako` stays behind them because of blast radius
   - exact stop-line: if a Rust owner only holds route gate/decode/encode, source-route handoff glue, or compat evidence, freeze it and move the phase front elsewhere

## Retreat Finding

- `phase-29ci` already closed the helper-side collapse, so further progress now depends on Rust-owned buckets moving, not on more shell cleanup
- `registry_optin_method_arraymap_direct_canary_vm.sh` is no longer “cleanup debt”; it is an explicit probe keep and should stay outside the shared-helper accounting
- the first productive slice already removed the shared route-table keep by moving surrogate route matching into `build_surrogate.rs`; current review treats that bucket as near thin floor rather than the next automatic shrink target
- `future-retire bridge` is now smaller on both sides: `program_json/emit_payload.rs`, `program_json/pipeline.rs`, and `program_json_entry/exit.rs` are gone, so the remaining inner bridge leaves are concentrated in `program_json/mod.rs` and `program_json_entry/request.rs`
- because `program_json_entry/request.rs` still touches env alias precedence and outer-caller-facing request extraction, it is not the default next slice; prefer bridge-local-only collapse before touching that contract leaf
- current authority is now exact enough to avoid hand-wavy blocker accounting: `src/host_providers/mir_builder.rs` owns the source-route handoff, explicit Program(JSON) route, shared `user_box_decls` shaping, and live MIR(JSON) emission stop-line, while `src/host_providers/mir_builder/lowering.rs` is now the test-only Program(JSON)->MIR evidence seam; `src/stage1/program_json_v0/authority.rs` remains the strict source-authority owner behind them
- worker order decision is now pinned: retire the dedicated `authority.rs` adapter, fold the extra shared shaping leaf into `src/host_providers/mir_builder.rs`, and stop the kernel Program(JSON) route at thin floor unless an exact disappearing route leaf appears
- the test-only transient `(Program JSON, MIR JSON)` tuple helper still lives only in the `src/host_providers/mir_builder.rs` façade test surface
- the dedicated `src/host_providers/mir_builder/authority.rs` adapter is gone, the extra `user_box_decls.rs::source_to_mir_json_with_user_box_decls(...)` leaf is gone, and shared Program(JSON) shaping is now folded into `src/host_providers/mir_builder.rs`; live source-route callers now enter through that façade directly
- imports-bearing `program_json_to_mir_json_with_imports(...)` is now test-only in `src/host_providers/mir_builder.rs`; live imports-bearing lowering stays off the façade surface
- plain `program_json_to_mir_json(...)` is now also test-only in `src/host_providers/mir_builder.rs`; the live explicit Program(JSON) route stays on `program_json_to_mir_json_with_user_box_decls(...)`
- imports-bearing lowering is also test-only inside `src/host_providers/mir_builder/lowering.rs`
- live source + explicit Program(JSON) callers now both stay in `src/host_providers/mir_builder.rs`, and cross the shared Rust seam only at `module_to_mir_json(...)`
- the extra `lower_program_json_to_module(...)` leaf is retired, and `src/host_providers/mir_builder/lowering.rs` now keeps only evidence/test seams around that path
- worker consensus now treats `src/host_providers/mir_builder.rs` as near thin floor, not the next place to keep shaving
- worker consensus also treats `src/stage1/program_json_v0/authority.rs` as frozen strict source-authority core; the next real movement is authority replacement above the Rust stop-line in `src/host_providers/mir_builder.rs`
- worker consensus on `src/host_providers/mir_builder/lowering.rs`: the remaining helpers there are evidence-only, while `module_to_mir_json(...)` is the real shared seam and now lives in `src/host_providers/mir_builder.rs`
- worker audit also raised the next non-Rust wave order after the current Rust-owned front: `lang/src/mir/builder/MirBuilderBox.hako` first, then runner owners `lang/src/runner/{stage1_cli_env.hako,stage1_cli.hako,launcher.hako}`, with shared producer `lang/src/compiler/build/build_box.hako` immediately behind that same wave; touching `build_box.hako` before those owner-local callers would be the highest-blast-radius move
- the kernel `emit_from_program_json_v0` / `emit_from_source_v0` pair now also shares same-file gate/decode/freeze helpers, so the remaining kernel work is explicitly thin-floor support code rather than a fresh authority-removal front
- the nearby future-retire bridge shim is now split out to `src/stage1/program_json_v0/bridge_shim.rs`, so `src/stage1/program_json_v0/authority.rs` no longer mixes bridge-specific error wrapping with strict source authority
- the first landed `.hako` authority-replacement slice now lives in `lang/src/runner/stage1_cli_env.hako`: `Stage1SourceMirAuthorityBox` owns the source-entry `BuildBox.emit_program_json_v0(...)` shim locally and delegates only Program(JSON) -> MIR to `MirBuilderBox.emit_from_program_json_v0(...)`
- the next landed `.hako` authority-replacement slice now lives in `lang/src/mir/builder/MirBuilderBox.hako`: the delegate branch of `emit_from_program_json_v0(...)` finalizes MIR locally by injecting `user_box_decls` before normalization, instead of leaving that shaping solely to Rust-owned provider surfaces
- `MirBuilderBox.hako` now also reads internal/delegate gate decisions via `lang/src/mir/builder/internal/builder_config_box.hako`, which is the last safe structural split on this front; `emit_from_source_v0(...)` stays a live compat seam for kernel route + route-evidence probes and should not be retired in the same wave
- the normal registry-first `Program(JSON v0) -> MIR(JSON)` authority block now lives in `lang/src/mir/builder/internal/registry_authority_box.hako`
- the remaining non-registry/internal fallback chain now lives in `lang/src/mir/builder/internal/fallback_authority_box.hako`
- the next landed tightening on the same owner fixes the route contract itself: `BuilderConfigBox.internal_on()/registry_on()` now return numeric `1/0`, and the stage1 module registry/export now includes `lower_loop_count_param_box`, `registry_authority_box`, and `fallback_authority_box`; as a result `tools/hakorune_emit_mir_mainline.sh lang/src/runner/{stage1_cli.hako,stage1_cli_env.hako}` is green on selfhost-first + no-delegate
- the next landed tightening on `lang/src/runner/stage1_cli.hako` keeps source/program-json orchestration behind same-file helpers (`_resolve_emit_program_source_text(...)`, `_resolve_program_json_for_emit_mir(...)`, `_resolve_program_json_for_run(...)`, `_load_program_json_from_path_or_source(...)`), and `stage1_main(...)` now reuses `_resolve_mode/_resolve_source/_resolve_program_json_path/_resolve_backend` instead of re-reading the env contract inline
- the next landed tightening on the same owner keeps route sequencing behind owner-local helpers `_lower_func_defs_if_enabled(...)`, `_emit_internal_program_json(...)`, and `_emit_delegate_program_json(...)`, so the outer box no longer repeats raw env/hostbridge branching inline
- consequence: `MirBuilderBox.hako` now keeps route sequencing, generic unsupported/no-match decision, and compat tails around those internal authority owners
- direct `phase2034/mirbuilder_internal_if_canary_vm.sh` is not promoted into accepted proof yet; it still hits the separate `vm-hako subset-check` blocker on `newbox(hostbridge)` before this owner split becomes observable
- the next pure-`.hako-only` removal wave should not start by shaving `build_surrogate.rs` more; it should keep shrinking the `.hako` owner chain and helper-local shell callers above the Rust stop-line in `src/host_providers/mir_builder.rs`
- runtime/plugin `env.mirbuilder.emit` is now concentrated in `src/runtime/mirbuilder_emit.rs`; `extern_provider.rs` and `plugin_loader_v2/enabled/extern_functions.rs` are thin callers, and `calls/global.rs` no longer owns a separate direct lowering branch
- runtime/plugin `env.mirbuilder.emit` also no longer counts as a live caller of `src/host_providers/mir_builder/lowering.rs`; that helper now lowers through `runner::json_v0_bridge::parse_json_v0_to_module_with_imports(...)` and reuses only shared MIR(JSON) emission
- worker audit agreed the safest next Rust-owned slice was the kernel/plugin Program(JSON) route in `crates/nyash_kernel/src/plugin/module_string_dispatch.rs`; that narrowing is now landed, and the remaining kernel-side leaf is no longer the local `user_box_decls` splice because that responsibility now lives in shared owner `src/host_providers/mir_builder.rs`
- live source + explicit Program(JSON) callers now parse Program(JSON) inside `src/host_providers/mir_builder.rs` and cross the shared seam only at `module_to_mir_json(...)`; the imports-free plain lowering helper in `src/host_providers/mir_builder/lowering.rs` is test-only evidence now
- worker design review now fixes the stop-line: `module_to_mir_json(...)` is the Rust host seam that should remain Rust-owned, while the next `.hako` wave should own `Program(JSON v0) -> MIR(JSON)` above that seam; do not try to move `MirModule` ownership into `.hako`
- after this slice, the kernel/plugin Program(JSON) route is close to thin floor: route-local gate/decode/encode remain, but host-provider call selection and `user_box_decls` shaping no longer live there
- `tools/hakorune_emit_mir.sh` now also keeps the direct `MirBuilderBox.emit_from_program_json_v0(...)` checked path behind a generated wrapper-local `_emit_mir_checked(...)` helper, so the shell/helper wave has started without touching `selfhost_build.sh` or `test_runner.sh`
