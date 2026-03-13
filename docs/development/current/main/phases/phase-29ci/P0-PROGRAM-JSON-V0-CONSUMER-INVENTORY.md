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
| `current authority` | [src/host_providers/mir_builder.rs](/home/tomoaki/git/hakorune-selfhost/src/host_providers/mir_builder.rs) | `emit_program_json_v0_for_strict_authority_source(...)` | current `stage1-env-mir-source` authority still materializes Program(JSON v0) before MIR(JSON); do not delete before replacing authority path |
| `build surrogate keep` | [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | `emit_program_json_v0_for_current_stage1_build_box_mode(...)` | compiled-stage1 `BuildBox.emit_program_json_v0` surrogate; public result is now payload `String` only |
| `build surrogate route table keep` | [crates/nyash_kernel/src/plugin/module_string_dispatch.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch.rs) | shared dispatch table includes `build_surrogate::BUILD_SURROGATE_ROUTE` | thin route-table owner only; surrogate module/method strings and the handler itself now stay in `build_surrogate.rs` |
| `build surrogate test keep` | [crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs) | same as above | route-selection plus invoke-by-name build-box/MIR handoff regression coverage are now fully co-located with the surrogate owner |
| `future-retire bridge` | [src/runner/stage1_bridge/program_json/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/mod.rs), [src/runner/stage1_bridge/program_json/pipeline.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/pipeline.rs), [src/runner/stage1_bridge/program_json/read_input.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/read_input.rs), [src/runner/stage1_bridge/program_json/emit_payload.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/emit_payload.rs), [src/runner/stage1_bridge/program_json/writeback.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json/writeback.rs) | `emit_program_json_v0_for_stage1_bridge_emit_program_json(...)` | `program_json/mod.rs` is now a thin facade; `pipeline.rs` owns the bridge-local read->emit->write chain, and `read_input.rs` / `emit_payload.rs` / `writeback.rs` own the detailed policies under the same future-retire bucket |
| `future-retire bridge entry` | [src/runner/stage1_bridge/program_json_entry/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/mod.rs), [src/runner/stage1_bridge/program_json_entry/request.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/request.rs), [src/runner/stage1_bridge/program_json_entry/exit.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/stage1_bridge/program_json_entry/exit.rs), [src/runner/emit.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/emit.rs), [src/runner/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/mod.rs) | `program_json_entry::{emit_program_json_v0_requested, emit_program_json_v0_and_exit}` | delegate-only entry; `mod.rs` is now a thin facade, `request.rs` owns request building/source-path precedence, and `exit.rs` owns bridge-specific success/error process-exit formatting while outer callers stay thin |
| `.hako` live/bootstrap callers | [lang/src/runner/stage1_cli_env.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli_env.hako), [lang/src/runner/stage1_cli.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/stage1_cli.hako), [lang/src/runner/launcher.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/runner/launcher.hako), [lang/src/mir/builder/MirBuilderBox.hako](/home/tomoaki/git/hakorune-selfhost/lang/src/mir/builder/MirBuilderBox.hako) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | live/bootstrap boundary on the `.hako` side; delete order must respect these callers separately from Rust host cleanup |
| `shell helper keep` | [tools/hakorune_emit_mir.sh](/home/tomoaki/git/hakorune-selfhost/tools/hakorune_emit_mir.sh), [tools/selfhost/selfhost_build.sh](/home/tomoaki/git/hakorune-selfhost/tools/selfhost/selfhost_build.sh), [tools/smokes/v2/lib/test_runner.sh](/home/tomoaki/git/hakorune-selfhost/tools/smokes/v2/lib/test_runner.sh) | `BuildBox.emit_program_json_v0(...)`, `MirBuilderBox.emit_from_program_json_v0(...)` | helper/canary route; must be caller-audited before delete slices touch shared shell contracts |
| `diagnostics/probe keep` | [tools/dev/phase29ch_program_json_helper_exec_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_program_json_helper_exec_probe.sh), [tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_stage1_cli_env_file_context_probe.sh), [tools/dev/phase29ch_selfhost_program_json_helper_probe.sh](/home/tomoaki/git/hakorune-selfhost/tools/dev/phase29ch_selfhost_program_json_helper_probe.sh) | `MirBuilderBox.emit_from_program_json_v0(...)` | diagnostics-only keep; should be retired after live callers, not before |

## Delete Order Guard

1. do not touch `current authority` until a non-JSON authority path exists
2. thin `build surrogate keep` and `future-retire bridge` as separate owner buckets
3. keep `.hako` live/bootstrap callers and diagnostics/probes out of the same patch as Rust host caller deletion

## Retreat Finding

- current compiled-stage1 `BuildBox.emit_program_json_v0` surrogate is still a real live bucket
- however, it is now physically isolated to [build_surrogate.rs](/home/tomoaki/git/hakorune-selfhost/crates/nyash_kernel/src/plugin/module_string_dispatch/build_surrogate.rs), and its route-selection/invoke-by-name regression tests plus surrogate route registration now live there too, so future retirement can remove one owner-local module without reopening shared MirBuilder dispatch logic
- shared route-table keep is now registration-only in practice; `BUILD_BOX_MODULE` no longer appears outside `build_surrogate.rs`
- the surrogate handler is now owner-local too; shared `module_string_dispatch.rs` only imports `BUILD_SURROGATE_ROUTE`
- `crates/nyash_kernel/src/tests.rs` no longer owns build-box receiver/method strings; compiled-stage1 build-surrogate regression coverage now stays in `build_surrogate.rs`
- root-level kernel regression no longer touches this bucket; launcher MIR handoff now stays in `build_surrogate.rs` with the surrogate route contract itself
- outside `src/runner/stage1_bridge/**`, direct `emit_program_json_v0` flag reading is now gone; the remaining outer caller contract is only [src/runner/mod.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/mod.rs) `skip_stage1_stub` route selection plus [src/runner/emit.rs](/home/tomoaki/git/hakorune-selfhost/src/runner/emit.rs) early-exit dispatch
- `ProgramJsonEntryOutcome` is gone from the outer caller surface; exact success/error process-exit formatting now stays in `program_json_entry/exit.rs`
- bridge-local source-path precedence (`stage1::input_path()` aliases first, CLI input fallback second) now stays in `program_json_entry/request.rs`, so `program_json/mod.rs` no longer depends on `CliGroups`
- bridge-local read->emit->write orchestration now stays in `program_json/pipeline.rs`, so `program_json/mod.rs` no longer carries the chain itself
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
- `tools/selfhost/selfhost_build.sh` still has an explicit `HAKO_USE_BUILDBOX=1` build-contract keep in code, but `apps/tests/hello_simple_llvm.hako` no longer proves a default-vs-BuildBox split: both emit-only lanes now exit `0` for `--json` and emit the same malformed Program(JSON v0) payload, so retire/delete claims must treat this as malformed-output debt rather than a JoinIR freeze/rescue split
- `tools/selfhost/selfhost_build.sh` now pins that live keep behind `buildbox_emit_only_keep_requested()`, so the exact `HAKO_USE_BUILDBOX=1` emit-only/no-EXE predicate is code-side SSOT instead of being re-read inline at the top level
- `tools/selfhost/selfhost_build.sh` now keeps its post-emit raw/extract contract behind `extract_program_json_v0_from_raw()`, `persist_stageb_raw_snapshot()`, and `exit_after_stageb_emit_failure()`, so remaining helper-local work can target downstream consumers (`--mir` / `--exe` / `--run`) without reopening the raw capture block
- `tools/selfhost/selfhost_build.sh` now keeps the source-direct `--mir` consumer behind `emit_mir_json_from_source()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane and the Core-Direct `--run` lane
- `tools/selfhost/selfhost_build.sh` now also keeps the Core-Direct `--run` consumer behind `run_program_json_v0_via_core_direct()`, so the remaining downstream helper-local work is the Program(JSON)->MIR->EXE lane alone
- `tools/selfhost/selfhost_build.sh` now also keeps the Program(JSON)->MIR->EXE consumer behind `emit_exe_from_program_json_v0()`, so all remaining downstream consumers are explicit owner-local helpers rather than top-level branches
- `tools/selfhost/selfhost_build.sh --mir` is now green on `apps/tests/hello_simple_llvm.hako`, but only because that lane is source-direct and bypasses the malformed Program(JSON) payload
- `tools/selfhost/selfhost_build.sh --run` now fails downstream on the malformed Program(JSON v0) payload with `JSON v0 bridge error: undefined variable: static`
- `tools/selfhost/selfhost_build.sh --exe` now fails on that same malformed Program(JSON v0) payload with `Program(JSON v0) parse error: undefined variable: static`
- exact producer-side root-cause evidence is now pinned in `P5-STAGEB-MALFORMED-PROGRAM-JSON.md`: raw snapshots show the malformed Program(JSON v0) is already printed by both `compiler.hako --stage-b --stage3` and `BuildBox.emit_program_json_v0(...)`, and `HAKO_STAGEB_DEBUG=1` shows `StageBBodyExtractorBox.build_body_src()` falling back to full source on `hello_simple_llvm`
- route split is now explicit for `apps/tests/phase29bq_selfhost_blocker_decode_escapes_if_idx12_min.hako`: direct CLI `--backend mir --emit-mir-json` now lowers in both default release and strict/dev shadow mode, while the Rust host-provider route and the language-level `lang.mir.builder.MirBuilderBox.emit_from_source_v0` surface (currently kernel-dispatch owned rather than pure `.hako`-internal proof) also lower the same fixture successfully; treat this as route/boundary evidence, not as proof that all owners are merged, and use `P4-MIRBUILDER-ROUTE-SPLIT.md` for the exact call-chain SSOT

## Immediate Next Slice

Prefer the smallest Rust-owned bucket first:

1. `build surrogate keep`
2. `future-retire bridge`
3. only then revisit `.hako` live/bootstrap callers
