---
Status: SSOT
Date: 2026-03-31
Scope: main ラインの current summary と正本リンクだけを置く薄い mirror/dashboard。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/DOCS_LAYOUT.md
  - docs/development/current/main/20-Decisions.md
  - docs/development/current/main/30-Backlog.md
---

# Self Current Task — Now (main)

## Purpose

- この文書は docs 側の薄い mirror/dashboard だよ。
- 置くのは current summary、実行入口、正本リンクだけ。
- 進捗履歴や長文ログは `CURRENT_TASK.md`、phase README、design SSOT に逃がす。

## Root Anchors

- Root anchor: `CURRENT_TASK.md`
- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Quick restart: `docs/development/current/main/05-Restart-Quick-Resume.md`
- Layout contract: `docs/development/current/main/DOCS_LAYOUT.md`

## Immediate Resume

- current lane is docs/policy refresh for the kernel replacement axis.
- visible order:
  1. `Rune lane (parallel, compiler-contract side)`
  2. `K0 -> K-migration`
  3. `K2-core acceptance lock`
  4. `RawMap` deferred in `K2-wide`; map perf stays evidence/regression only
- stage axis:
  - `stage0 = bootstrap/recovery keep`
  - `stage1 = same-boundary swap proof`
  - `stage2-mainline = daily mainline / distribution lane`
  - `stage2+ = umbrella / end-state label`
- replacement axis:
  - `K0 = Boundary Lock`
  - `K1 = Semantic Owner Swap`
    - public reading: semantic kernel is complete on the `.hako` side
  - `K2 = Substrate Era`
    - `K2-core = RawArray first`
    - `K2-wide = RawMap second + capability widening + metal review`
- implementation note:
  - the engineering line after `K0` is one `K-migration` track
  - `K1` and `K2` remain separate gates / acceptance checkpoints
- current repo read:
  - collection wave (`Array -> Map -> RuntimeData cleanup`) is `K1 done-enough`
  - `Rune` stays a parallel compiler-contract lane, not a step inside `K-axis`
  - post-`K0` engineering line is read as one `K-migration` track
  - next structural step is `K2-core acceptance lock`
  - `K2-core` smoke/evidence gate is the existing `nyash_kernel` RawArray contract tests (`runtime_data_invalid_handle_returns_zero`, `runtime_data_array_round_trip_keeps_rawarray_contract`, `legacy_set_h_returns_zero_but_applies_value`, `hi_hii_aliases_keep_fail_safe_contract`, `slot_load_store_raw_aliases_keep_contract`, `slot_append_raw_alias_keeps_contract`)
  - `RawMap` is `K2-wide` second and stays deferred while `RuntimeDataBox` remains facade-only
  - same-boundary daily swap code should be called `.hako kernel module` / `.hako substrate module`; `plugin` remains cold loader lane vocabulary
  - default daily/distribution target is `zero-rust`, meaning non-Cargo user-facing normal operation; bootstrap/recovery/reference/buildability and native metal keep are explicit keeps
- evidence appendix below keeps the map/array perf snapshots as support only; they do not change the order above.
- next horizon inventory:
  - big: `Rune lane (parallel)`; `K-migration` with `K2-core acceptance lock`; policy stabilization; zero-rust default operationalization
  - parked big: `K2-wide` follow-up; broad `Map` structural expansion
  - small: docs ladder sync; Rune docs/tag sync; Map evidence bundle maintenance
  - lane-local cleanup candidates only:
    - Rune lane: `src/parser/runes.rs`, `src/parser/statements/helpers.rs`, `src/stage1/program_json_v0.rs`, `src/macro/ast_json/roundtrip.rs`
    - RawArray lane: `crates/nyash_kernel/src/plugin/handle_cache.rs`, `crates/nyash_kernel/src/plugin/runtime_data_array_dispatch.rs`, `crates/nyash_kernel/src/plugin/array_slot_load.rs`, `crates/nyash_kernel/src/plugin/array_slot_store.rs`, `crates/nyash_kernel/src/plugin/array_slot_append.rs`
    - broader `src/backend/*`, `src/bid/*`, and non-active `crates/nyash_kernel/*` cleanup stays parked
  - parked small: warning debt sweep; TODO cleanup / ignore triage; code-hotspot cleanup outside the active pilot boundary
  - execution order: `Rune lane (parallel)` plus `K0 -> K-migration`; `RawMap` remains deferred in `K2-wide`
- next exact docs:
  - `docs/development/current/main/design/kernel-replacement-axis-ssot.md`
  - `docs/development/current/main/design/rune-v1-metadata-unification-ssot.md`
  - `docs/development/current/main/design/optimization-hints-contracts-intrinsic-ssot.md`
  - `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
  - `docs/development/current/main/design/kernel-implementation-phase-plan-ssot.md`
  - `docs/development/current/main/design/de-rust-zero-buildability-contract-ssot.md`
- Already landed: `docs/private/papers-archive/paper-a-mir13-ir-design/out/mir13-paper.pdf` has been moved to `docs/private/out/`, `docs/private/roadmap2/CURRENT_TASK_2025-11-29_full.md` has been archived under `docs/private/roadmap2/archive/`, the root build scripts are shimmed to `tools/build/`, `src/runner/mir_json_v0.rs` has been split into helper/call/tests submodules, `src/backend/wasm/shape_table.rs` has been split into `native/p10/tests` submodules, `src/backend/mir_interpreter/handlers/calls/method.rs` has been split into `dispatch/tests` submodules, `src/runner/modes/vm_hako/tests/boxcall_contract.rs` has been split into `subset/compile` submodules, `src/bin/rc_insertion_selfcheck.rs` has been split into `helpers` plus `cases/{mod,basic,jump,misc}` submodules, `src/mir/passes/rc_insertion_helpers.rs` has been split into `cleanup/contracts/cycles/plan/apply/types/util` submodules, `src/mir/builder/control_flow/plan/composer/coreloop_v1_tests.rs` has been split into scenario submodules, `src/mir/optimizer.rs` has been split with a `diagnostics` submodule, `src/runner/modes/vm_hako/subset_check.rs` has been split into `shapes/boxcalls/externcalls` submodules, `src/mir/join_ir/lowering/loop_with_if_phi_if_sum.rs` has been split into `extract/tests` submodules, `src/mir/builder/control_flow/plan/features/loop_cond_bc_else_patterns.rs` has been split into `returns/breaks/guard_break` submodules, `src/mir/builder/control_flow/plan/composer/coreloop_v0_tests.rs` has been split into `simple_while/scan_with_init/split_scan` submodules, and `src/backend/mir_interpreter/handlers/extern_provider.rs` has been split into lane submodules.
- Already landed: `crates/nyash_kernel/src/plugin/runtime_data.rs` now routes array dispatch through handle-based RawArray substrate helpers, `runtime_data_array_dispatch.rs` short-circuits invalid handles before index resolution, and RawArray guard sites now share `array_guard::{valid_handle, valid_handle_idx}` across dispatch, slot, compat, and capacity/string helper boundaries; map any-key paths still materialize owned key strings before map ops so handle-registry borrow overlap is avoided.
- Also landed: `src/mir/control_tree/normalized_shadow/loop_true_break_once.rs` has its tests moved to `loop_true_break_once/tests.rs`, `src/macro/ast_json/joinir_compat.rs` has its helper functions moved to `joinir_compat/helpers.rs`, `src/mir/builder/control_flow/joinir/route_entry/registry/handlers.rs` has `generic` route functions moved to `handlers/generic.rs`, and `lang/src/runner/launcher.hako` has dispatch/input-contract helper boxes moved into `launcher/dispatch.hako` and `launcher/input_contract.hako`.
- tmp cleanup note: the zero-reference `apps/` `tmp_*.hako` files are deleted.
- Ignore cleanup note: `loop_routes` and bridge/debug harness comments were normalized; remaining `#[ignore]` work is now a candidate-by-candidate shelfing pass, not a blanket TODO cleanup.
- Keep `tools/selfhost/run_all.sh`, phase-local `run_all.sh`, and `apps/tests/` as compat/fixture surfaces for now.
- `lang/src/compiler/mirbuilder/mir_json_v0_shape_box.hako`, `lang/src/compiler/entry/func_scanner.hako`, `lang/src/compiler/mirbuilder/stmt_handlers/return_stmt_handler.hako`, and `lang/src/runner/stage1_cli.hako` are split already.
- `stage1_cli.stage2` exact emit compat probe is green again; `stage1_cli` itself remains the run-only bootstrap output.
- `launcher` is now split into dispatch/input-contract/artifact_io/payload_contract helpers; the thin bootstrap proof is being shifted to `launcher_native_entry.hako`, and the remaining cleanup slice is launcher compile facade thinning or the route-entry handler table.
- `src/runner/json_v1_bridge/parse.rs` has its tests moved to `parse/tests.rs`.
- `src/runner/modes/vm_hako/tests/boxcall_contract/subset.rs` has been split into topic submodules under `subset/`.
- `handlers` has the generic route leaf split out; the next cleanup slice is the remaining handler route table or `artifact_io` depending on which lane proves cheaper.
- Next step is to keep the map lookup cache reproducible on the Map micro lane, use the asm diff helper when the hot symbol changes, and only revisit `NYASH_NY_LLVM_LLC_FLAGS` if it shows a stable win.

## Evidence Appendix

- `kilo_leaf_map_get_missing 0` = `c_ms=3 / ny_aot_ms=46 / ratio_cycles=0.07`
- `kilo_leaf_map_getset_has -1` = `c_ms=2 / ny_aot_ms=87 / ratio_cycles=0.00`
- `kilo_micro_array_getset 1x7` = `c_ms=3 / ny_aot_ms=3 / ratio_cycles=0.94`
- `runtime_data` array dispatch now goes through handle-based RawArray substrate helpers, and map any-key paths materialize owned keys before map ops to avoid handle-registry borrow overlap

## Current Read

- Active lane: `policy-refresh`
  - status: `active`
  - purpose:
    - keep `stage` as build/distribution vocabulary
    - keep compressed `K-axis` as replacement progress vocabulary
    - read the post-`K0` implementation line as `K-migration`, not as one merged acceptance gate
    - keep `Rune` visible as a parallel compiler-contract lane
    - pin `K2-core = RawArray first truthful substrate pilot`
    - keep `RawMap` deferred in `K2-wide` and map perf as regression/evidence, not structural next step
- Active code lane: `phase-29bq`
  - status: `active (failure-driven; blocker=none)`
  - purpose:
    - keep selfhost `.hako` migration on `mirbuilder first / parser later`
    - keep the lane blocker-none until the next exact blocker is captured
    - keep daily lane checks and blocker evidence current
  - current read:
    - current exact implementation leaf is `none while blocker=none`
    - latest landed blocker fixture is `phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
    - landed fix is planner-required BlockExpr value-prelude parity in normalizer
    - operational SSOT is `phase-29bq/29bq-90-selfhost-checklist.md`
    - progress ledger is `phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
    - parser handoff ledger is `phase-29bq/29bq-92-parser-handoff-checklist.md`
    - current inner migration reading is `29bq-113` / `29bq-114` / `29bq-115`
- Structure-first implementation lane: `phase-29x`
  - status: `active owner-cutover prep`
  - current exact read:
    - `backend-owner-cutover-ssot.md` is now the structure-first parent SSOT
    - backend-private `runtime-decl-manifest-v0.toml` is the single compare-lane declare inventory
    - `.hako ll emitter` min v0 is now the daily owner for `ret const`, `bool phi/branch`, `Global print`, `StringBox.length`, `StringBox.indexOf`, and `concat3 extern`
    - explicit compare smoke now lives in archive suite only as `phase29x_backend_owner_hako_ll_compare_min.sh`; compare lane is bridge-only
    - new legacy/demotion queue is fixed in `phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
    - legacy C `.inc` stays daily owner only for unflipped shapes and silent fallback remains forbidden
    - `backend_daily_owner_policy_box.hako`, `backend.ll_emit.call_policy`, and `backend.ll_emit.call_selector` are already split out; `backend_route_env_box.hako` has been retired from code; `ll_emit_compare_driver.rs` carries compare/debug orchestration while `ll_emit_compare_vm.rs` carries VM spawn, `ll_emit_compare_stdout.rs` carries stdout extraction, and `ll_emit_compare_source.rs` carries source materialization; `provider_keep.rs` carries explicit provider keep lanes; `legacy_json.rs` carries the archive-later MIR(JSON) front door; LLVM tool execution is isolated in `src/host_providers/llvm_codegen/ll_tool_driver.rs`
    - `MirRootHydratorBox` and `MirBuilderBox.emit_root_from_{program_json,source}_v0(...)` are now landed as the compat root entry
    - daily `.hako ll emitter` profiles already compile via `root -> facts -> ll text -> env.codegen.compile_ll_text(...)`
    - launcher/root-first daily transport cut is landed:
      - mainline launcher now hydrates a root from source and enters `LlvmBackendBox.compile_obj_root(...)`
      - temp MIR JSON remains evidence/output only; it is not the daily compile transport
      - `compile_json_path(...)` has been retired from code
    - `route.rs` compare/archive shrink is landed; the Rust-side stage0 object-emit JSON round-trip is also retired, so the next fixed order is `keep .ll tool seam -> keep archive-later wrapper inventory closed -> review archive/delete only after wrapper inventory reaches zero`
    - archive/delete sweep wave 1, code-side legacy C daily demotion v1, and the `hello_simple_llvm_native_probe_v1` narrow owner flip are landed:
      - flipped `phase29ck` locks moved from the default `phase29ck-boundary` suite into `phase29ck-boundary-legacy`
      - compare bridge assets remain explicit bridge-only and now live in `phase29x-derust-archive.txt`
      - route payload now keeps `acceptance_case` / `legacy_daily_allowed` visible through the Rust bridge
      - `.hako ll emitter` is now also the daily owner for `runtime_data_string_length_ascii_min_v1`, `runtime_data_array_length_min_v1`, and `runtime_data_map_size_min_v1`
      - those three `RuntimeData` observer locks now live in `phase29ck-boundary-legacy`
      - the lookup family is landed; `RuntimeData` mutator `runtime_data_array_push_min_v1` is now also daily
      - remaining active owner-flip targets are 0 shapes; `indexof_line_pure_min_v1` and `substring_concat_loop_pure_min_v1` are now daily and their boundary locks are retired into `phase29ck-boundary-legacy.txt`
    - only structural perf is in scope during this prep (`attrs` SSOT, facts visibility, copy-transparency, verifier/compare ledger)

- Stage2-mainline lane
  - status: `active bounded-3 stop-line landed / entry-task-pack sync`
  - current exact read:
    - stage2-mainline is now read as mostly `.hako` authority / thin `.inc` shim / native metal keep; stage2+ remains the umbrella / end-state reading
    - collection substrate cleanup is tracked on the owner/substrate axis; stage1 remains bridge/proof and stage2-mainline remains the daily mainline
    - stage/artifact/build-conduit wording is now synced:
      - `execution-lanes-and-axis-separation-ssot.md` is the canonical vocabulary owner
      - `stage1` has current concrete build/invoke conduits
      - `stage2-mainline` is the daily mainline lane; `stage2+` is the umbrella / end-state distribution reading, not a standalone build-script family
      - `stage3` is a compare/sanity label only
    - parent task pack is now `stage2plus-entry-and-first-optimization-wave-task-pack-ssot.md`
    - current collection cleanup SSOT is `stage2-collection-substrate-cleanup-ssot.md`
    - Array phase closes only after the same-artifact `kilo_micro_array_getset` compare against the current Rust array baseline is acceptable
    - syntax audit: canonical metadata surface is `@rune`; legacy `@hint/@contract/@intrinsic_candidate` remain compat aliases during the Rune v1 window; current `.hako` collection code does not depend on this lane
    - surfaced v1 syntax gap remains the selfhost compiler `{ ident: expr }` / BlockExpr migration note; it does not block the current Array phase lane
    - the new SSOT is `stage2-hako-owner-vs-inc-thin-shim-ssot.md`
    - phase plan SSOT is `kernel-implementation-phase-plan-ssot.md`
    - `.hako` complete in this lane means authority/mainline completion, not native zero
    - standard distribution reading is `hakoruneup + self-contained release bundle`, not a single stage artifact
    - boundary truth is not owned by `.inc`; `.inc` remains thin artifact/shim space
    - `.inc` partitions are still mixed today, so the first task is classification, not code motion
    - first stage2-mainline optimization wave is fixed to `route/perf only` on `.hako -> ny-llvmc(boundary) -> C ABI`
- fast-smoke CI now does a clean rebuild (`cargo clean` before `cargo build --release --workspace`) and stages built plugin artifacts into `plugins/*`, so `ternary_basic` does not depend on cached build outputs or prebuilt plugin .so availability on fresh runners
- fast-smoke CI now also pins `NYASH_NY_LLVM_OPT_TOOL=opt-18` and `NYASH_NY_LLVM_LLC_TOOL=llc-18`; the pure-first shim resolves LLVM tools by env override or PATH fallback so GitHub runner tool naming does not break the `mem2reg -> llc` lane
    - stage2-mainline first perf wave is now explicitly `Array only`, and the fixed order is `leaf-proof micro -> micro kilo -> main kilo`
    - refreshed same-artifact `kilo_micro_array_getset` baseline is `c_ms=3 / ny_aot_ms=3 / ratio_instr=0.90 / ratio_cycles=0.68 / ratio_ms=1.00`
    - refreshed direct bundle is `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-refresh/`:
      - `mir_windows` stays on `Method:RuntimeDataBox.{push,get,set}`
      - `owner_route=seed first_blocker=empty`
      - `recipe_acceptance` remains empty
      - hot-block scan still shows no `slot_load_hi` / `generic_box_call` / `hostbridge` / `runtime_data` residue
      - `perf_top` is still dominated by `ny_main` (`92.61%`), so the current direct artifact does not expose a narrower route leaf yet
    - `tools/perf/trace_optimization_bundle.sh` now auto-saves `perf_top_symbol.txt`, `perf_top_annotate.txt`, `perf_top_objdump.txt`, `perf_top_hot_insns.txt`, `perf_top_opcode_hist.txt`, and `perf_top_group_summary.txt` for the hottest in-binary symbol plus grouped residue
    - probe bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-probe/` resolves that hottest symbol to `ny_main`; the annotate/objdump pair shows a tight stack-array loop (`cmp -> load -> inc -> store -> add -> inc`) with no surviving foreign calls in the hot block
    - 20-run observe bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-observe20/` still samples only `cmp` / `inc` (`54.45% cmp`, `45.55% inc`), so the current direct artifact does not expose a richer subsymbol leaf yet
    - grouped 3-run residue probe `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-groups/` reports `89.50% bundle / 5.98% loader / 1.47% runner`; use this grouped reading before opening any Array code slice on WSL
    - repeated 3-run bundles still vary meaningfully under WSL:
      - repeatA `92.66% bundle / 2.81% loader / 2.04% runner`
      - repeatB `89.84% bundle / 5.82% loader / 3.09% libc / 1.20% runner`
      - repeatC `74.02% bundle / 22.96% loader / 2.40% libc / 0.55% runner`
      - treat `perf_top_group_summary` as a noise detector, not as the sole acceptance gate
    - cold 1-run residue probe `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-cold1/` shifts more weight into loader/runner (`87.25% bundle / 6.90% loader / 5.84% runner`), so keep `3 runs + asm` as the decision gate and use 1-run only as startup-residue evidence
    - C baseline loop-shape check shows the expected scalar loop body with samples spread across `and / mov / inc / mov / cmp`, while the AOT direct probe still concentrates on `cmp / inc`; treat the remaining gap as outside the old route/helper residue story unless a new boundary blocker appears
    - llpath canonical emit contract landed: `lang/c-abi/shims/hako_llvmc_ffi_common.inc` now canonicalizes pure-first IR with `opt -passes=mem2reg` before `llc` in the current implementation, and the Array micro seed keeps the benchmark sink honest via explicit volatile `sum` accesses
    - landed proof bundle `target/perf_state/optimization_bundle/stage2plus-array-wave-direct-mem2reg-v2/` now shows `ny_main` registerizing the loop IV as SSA/PHI (`%i.1 = phi ...`) and the emitted asm drops the `%i` stack spill; sampled hot insns collapse to `and / inc` on the loop body, while `sum` remains the only intentional stack sink
    - the latest 3-run residue summary is `93.66% bundle / 3.02% loader / 0.56% runner`; keep `3 runs + asm` as the judge and treat the grouped summary as a WSL noise detector
    - Rune optimization metadata remains `parse/noop`; backend-active consumption is deferred beyond this first wave
    - `pure_compile` / `generic_method_lowering` / `string_concat_*` are the first semantic-owner-heavy candidates
    - `hako_llvmc_ffi_common.inc` stays thin boundary utility and native support
    - `stage1_cli_env` helper default now stays on `selfhost-first`; the strict stage1 probe is green and no longer falls back to `delegate:provider`
    - canonical selfhost-first promotion no longer leaks collapsed `functions_0`; `MirSchemaBox.module(...)` now keeps only canonical `functions[]`
    - explicit `HAKO_MIR_BUILDER_FUNCS=1` now lowers helper defs as a flat canonical `functions[]` splice
  - landed order / deferred end-state direction:
    1. docs-first owner/shim SSOT
    2. classify `.inc` partitions into semantic owner / compiler owner / thin shim / native leaf
    3. define compiler-state capability and lowering builder seam
    4. first code slice: extract emit primitives into `hako_llvmc_ffi_emit_seam.inc`
    5. second code slice: split generic method classification into `hako_llvmc_ffi_generic_method_match.inc`
    6. third code slice: extract compiler-state helpers into `hako_llvmc_ffi_compiler_state.inc`
    7. fourth code slice: split string concat emit helpers into `hako_llvmc_ffi_string_concat_emit.inc`
    8. first semantic-owner slice: land string-chain policy vocabulary in `.hako`
    9. fifth code slice: mirror string-chain route policy in `hako_llvmc_ffi_string_chain_policy.inc`
    10. second semantic-owner slice: land generic collection method vocabulary in `.hako`
    11. sixth code slice: mirror generic method policy in `hako_llvmc_ffi_generic_method_policy.inc`
    12. seventh code slice: mirror generic method len route in `hako_llvmc_ffi_generic_method_len_policy.inc`
    13. eighth code slice: mirror generic method push route in `hako_llvmc_ffi_generic_method_push_policy.inc`
    14. ninth code slice: mirror generic method has route in `hako_llvmc_ffi_generic_method_has_policy.inc`
    15. tenth code slice: mirror generic method substring route in `hako_llvmc_ffi_generic_method_substring_policy.inc`
    16. eleventh code slice: mirror generic method get fallback route in `hako_llvmc_ffi_generic_method_get_policy.inc`
    17. move remaining semantic owner decisions to `.hako`
    18. thin shim cleanup and README sync
  - note:
    - the list above is historical landed order plus deferred end-state direction; it is not the current active expansion plan
    - docs cleanup may refine boundary-truth wording, but it must not reopen broad stage2 owner expansion before perf returns
  - landed so far:
    - `hako_llvmc_ffi_emit_seam.inc`
    - `hako_llvmc_ffi_generic_method_match.inc`
    - `hako_llvmc_ffi_compiler_state.inc`
    - `hako_llvmc_ffi_string_concat_emit.inc`
    - `lang/src/runtime/kernel/string/chain_policy.hako`
    - `hako_llvmc_ffi_string_chain_policy.inc` (now mirrors route / retained-form / post-store observer names)
    - `lang/src/runtime/collections/method_policy_box.hako`
    - `hako_llvmc_ffi_generic_method_policy.inc`
    - `hako_llvmc_ffi_generic_method_len_policy.inc`
    - `hako_llvmc_ffi_generic_method_push_policy.inc`
    - `hako_llvmc_ffi_generic_method_has_policy.inc`
    - `hako_llvmc_ffi_generic_method_substring_policy.inc`
    - `hako_llvmc_ffi_generic_method_get_policy.inc`
    - `hako_llvmc_ffi_generic_method_get_window.inc`
    - `hako_llvmc_ffi_generic_method_get_lowering.inc`
    - `hako_llvmc_ffi_string_concat_window.inc`
    - `pure_compile.inc` dead GET-window remnants retired
    - `string_concat_match.inc` dead helper copy block retired
    - `runtime/meta/{mir_call_route_policy_box,mir_call_need_policy_box,mir_call_surface_policy_box}.hako`
    - `hako_llvmc_ffi_mir_call_route_policy.inc`
    - `hako_llvmc_ffi_mir_call_need_policy.inc`
    - `hako_llvmc_ffi_mir_call_surface_policy.inc`
    - `hako_llvmc_ffi_mir_call_dispatch.inc`
    - `pure_compile.inc` now routes `mir_call` through `hako_llvmc_ffi_mir_call_dispatch.inc`
    - `RuntimeDataBox` generic fallback routes now reuse `nyash.runtime_data.{get,set,has,push}` through the method-policy seams
    - `runtime_data_map_get_hh(...)` now preserves the mixed runtime i64/handle return contract, so map-get facade parity stays pinned after the route-policy split
    - manifest/inventory sync landed for the map observer seam:
      - canonical daily `MapBox.size/len` now points at `nyash.map.entry_count_i64` in both manifest roots
      - generated defaults and the ABI slice guard now enforce `entry_count_i64`; `entry_count_h` remains compat-only
    - hako-vm collection routing fix landed:
      - `MirCallV1HandlerBox` now routes `ArrayCoreBox` / `MapCoreBox` / `StringCoreBox` before the adapter gate, so `HAKO_ABI_ADAPTER=0` no longer forces `MapBox.set/get/has` into method stubs
      - quick `map_basic_get_set_vm.sh` now pins the adapter-on `MapBox.set/get` route explicitly; builtin no-adapter VM contract remains `MapBox.len/size` only
      - vm-hako payload normalize now keeps string-handle consts used by nested `mir_call.args` and rewrites `MapBox.{set,get,has,getField,setField,delete,size,len,length}` Method mir-calls into canonical `boxcall` payloads
      - source quick `MapBox.get("a")` and `MapBox.size()` are green again on the vm-hako route; the old `[vm/method/stub:get]` residual is closed
      - String source quick route is now green on vm-hako for `substring/indexOf`; `StringCoreBox` owns `substring/indexOf` directly and the old `[vm/method/stub:*]` residual is closed
      - Array quick route is now green across vm-hako -> rust-vm driver execution:
        - `hako.toml` / `nyash.toml` now expose `"selfhost.runtime" = "lang/src/runtime"` so the generated driver sees the collection/runtime roots
        - vm-hako `newbox` now allocates live `ArrayBox` / `MapBox` handles instead of filebox-only tokens
        - rust-vm extern dispatch now covers `nyash.{array,map}.birth_h`, array raw seams, and `nyash.box.from_i64`
        - `ArrayCoreBox.push` now distinguishes scalar vs handle args and boxes scalar i64 before `slot_append_hh`
        - quick `array_length_vm.sh`, `array_oob_set_tag_vm.sh`, and `array_empty_pop_tag_vm.sh` are green; the old `missing receiver for slot_append_any` residual is closed
    - collection stop-line regression pack:
      - active Array daily route is now mirrored across `.hako ll emit` and pure-first no-replay:
        - `get -> nyash.array.slot_load_hi`
        - `push -> nyash.array.slot_append_hh`
        - `has -> nyash.runtime_data.has_hh`
        - `set -> nyash.array.slot_store_hih / nyash.array.slot_store_hii` now uses raw substrate nouns on the active daily path
        - compat-only aliases are now explicit: `set_hih` / `set_hii` / `get_hi` / `has_hi` / `push_h*` are no longer part of the active daily path
      - `phase21_5_perf_kilo_runtime_data_array_route_contract_vm.sh` is green again under `pure-first + compat_replay=none`
    - stage1 closeout landed:
      - `stage1_cli_env` helper default now stays on `selfhost-first`; the strict stage1 probe is green and no longer falls back to `delegate:provider`
      - canonical selfhost-first promotion no longer leaks collapsed `functions_0`; `MirSchemaBox.module(...)` now keeps only canonical `functions[]`
      - explicit `HAKO_MIR_BUILDER_FUNCS=1` now lowers helper defs as a flat canonical `functions[]` splice
  - next exact slice:
    - treat `stage1_cli_env` selfhost-first strict green and explicit `HAKO_MIR_BUILDER_FUNCS=1` flat defs splice as landed evidence
    - sync the master task pack across dashboard/stage docs
    - close the `stage1 -> stage2-mainline` entry gate on `.hako` canonical MIR authority and thin Rust transport only
    - keep collection domains on regression-pack status
    - keep the collection quick-vm closeout (`MapBox.get/size`, `String substring/indexOf`, `Array length/oob/pop`) as a regression pack only; do not reopen owner semantics in this lane
    - reopen only the route/perf first wave, not broad authority expansion or Rune activation
    - do not cut a blind code leaf while the refreshed direct bundle still collapses into `ny_main`; the next accepted slice needs a measurable leaf below `ny_main` from the same-artifact bundle
    - do not reopen historical Rust helper splits from shell memory alone; the current direct probe no longer exposes them in the hot block
- Secondary exact blocker lane: `phase-29ck`
  - status: `monitor/evidence while phase-29x owner-cutover prep is active`
    - current exact result:
      - generic optimization unit is now fixed as `recipe family`, not benchmark name
      - `recipe / scope / effect / policy / leaf` split is now the preferred reading for future user-box and allocator optimization work
      - current perf slices may keep narrow leaf proofs, but they must not become permanent benchmark-keyed owners
      - judgment policy: `repeat < 3` is probe-only; keep/reject decisions require at least 3 runs plus a quick ASM probe; if WSL jitter or allocator-like noise remains, recheck with `repeat=20` before closing the lane
      - `Stage1 MIR dialect split` is retired for the current kilo entry
      - `kilo_kernel_small_hk` is back to `pure-first + compat_replay=none + aot_status=ok`
      - docs-first proof-vocabulary lock is landed
      - the current perf-kilo design front is now the normalized transient text pieces carrier, and the concat/insert pilot is landed
      - the string export surface is now split by responsibility: `string.rs` (entrypoints/sink), `string_debug.rs`, `string_search.rs`, `string_plan.rs`, and `string_view.rs`
      - the current proof lane remains the shared store-ready string materialization boundary
      - next exact proof is `concat3_hhh` / array-store motion on `PiecesN`, with `substring_hii` still kept as the read-only carrier check
      - rejected follow-up: authoritative `ArrayBox` integer-storage split did not improve `kilo_micro_array_getset` and regressed main `kilo`
      - rejected follow-up: `ArrayBox.items` `parking_lot::RwLock -> std::sync::RwLock` regressed both micro and main
      - rejected follow-up: `host_handles.table` `parking_lot::RwLock -> std::sync::RwLock` regressed both micro and main
      - rejected follow-up: backend-private adjacent fused `get -> +const -> set -> get` leaf is now explained as a route-shape miss, not a mysterious symbol miss
      - rejected follow-up: `StringViewBox::new(...)` stable-id derivation (to avoid `BoxBase::new()`) regressed stable main to `814 ms` under `repeat=3`, so keep the current atomic view birth until fresh evidence appears
      - rejected follow-up: widening `maybe_borrow_string_handle_with_epoch(...)` / `try_retarget_borrowed_string_slot_with_source(...)` to accept `StringViewBox` as a string source regressed stable main to `844 ms` under `repeat=3`, so keep the current StringBox-only borrow/retarget lane
    - current live no-replay array window is semantic `get -> copy* -> const 1 -> add -> set`
    - current micro route now proves the semantic window on the same artifact:
      - `array_rmw_window result=hit`
      - lowered IR contains `nyash.array.rmw_add1_hi`
      - built binary exports `nyash.array.rmw_add1_hi`
      - `kilo_micro_array_getset` is down to `37 ms` under `1x3`
    - current main route now has one same-artifact direct hit:
      - `array_string_len_window result=hit count=1`
      - lowered IR contains `nyash.array.string_len_hi`
      - built binary exports `nyash.array.string_len_hi`
      - stable main median moved `843 -> 822`
    - rejected follow-up:
      - same-artifact `array_string_indexof_window result=hit` was proven
      - lowered IR still contained both `nyash.array.slot_load_hi` and `nyash.array.string_indexof_hih`
      - stable main moved to `853 ms`
      - `kilo_micro_indexof_line = 9 ms`
    - current main route still has two accepted observer misses:
      - `array_string_len_window reason=post_len_uses_consumed_get_value`
      - `array_string_len_window reason=next_noncopy_not_len`
    - next exact work order is now placement-first:
      1. keep `retained-boundary-and-birth-placement-ssot.md` as the parent contract
      2. keep `array_set` as the first `Store` proof boundary while `post-store-observer-facts-ssot.md` owns the trailing `length()` observer
      3. use `concat3-array-store-placement-window-ssot.md` as the next exact rollout contract for `concat3_hhh -> array.set -> trailing length()`
      4. read compiler-local facts from `remember_string_concat_*`, `remember_string_substring_call(...)`, `remember_string_length_call(...)`, `has_direct_array_set_consumer(...)`, and `analyze_array_string_len_window_candidate(...)`
      5. only after a same-artifact improvement is visible, revisit code-side `RetainedForm` wiring
    - `tools/perf/run_kilo_leaf_proof_ladder.sh` remains the acceptance lane for new observer/mutator leaves, but it is no longer the active perf lane for the current string-birth wave
    - current `leaf-proof micro` facts are still documented, but they are stop-line evidence for this wave:
      - `kilo_leaf_array_rmw_add1 = 36 ms` (`aot_status=ok`)
      - `kilo_leaf_array_string_len = 12 ms` (`aot_status=ok`)
      - `kilo_leaf_array_string_indexof_const = 25 ms` (`aot_status=ok`)
      - narrow pure-first pins are now `apps/tests/mir_shape_guard/array_string_indexof_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_branch_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_cross_block_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_interleaved_branch_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_interleaved_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_len_live_after_get_min_v1.mir.json`, and `apps/tests/mir_shape_guard/array_string_indexof_branch_live_after_get_min_v1.mir.json`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_select_min.sh` proves `get -> indexOf("line") -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-select-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_branch_min.sh` proves `get -> indexOf("line") -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-branch-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh` proves `get -> indexOf("line") -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-cross-block-select-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-branch-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-select-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_len_live_after_get_min.sh` proves `get -> len` can stay accepted when the fetched string is still consumed by later `substring(...)`, and the visible `.hako` evidence row is `acceptance_case=array-string-len-live-after-get-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_branch_live_after_get_min.sh` proves `get -> indexOf("line") -> compare -> branch` can keep the original fetched string live into the then-block, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-branch-live-after-get-v1`
      - the exact leaf-proof pure-first acceptance gap is retired
      - fixed-order recheck after the landing is `kilo_micro_indexof_line = 7 ms`, and recent `kilo_kernel_small_hk` rechecks are back to `aot_status=ok` at `787-814 ms` (`warmup=1 repeat=3`)
    - current direct-path optimization reading is fixed:
      - battle order is `typed/recipe canonical subset -> generic pure lowering -> RuntimeData peel only on recurrence`
      - landed exact cuts are analysis-only recipe sidecars on existing MIR for `get -> indexOf(const) -> compare -> select|branch`, the cross-block `get -> indexOf(const) -> jump -> compare -> select` shape, and the interleaved producer-guard branch/select shapes, all lowered as `nyash.array.string_indexof_hih`
      - bundle evidence now includes `recipe_acceptance.txt` plus `hot_block_residue.txt`, and the accepted observer recipes leave `slot_load_hi`, `generic_box_call`, and `hostbridge` at zero on all five pinned fixtures
      - default same-artifact bundle for `kilo_micro_indexof_line` still shows `recipe_acceptance=empty`, route trace `select` only, and lowered IR remains `indexOf line loop ascii` with `strstr`
      - diagnostic same-artifact bundle can now force the generic route with `tools/perf/trace_optimization_bundle.sh --skip-indexof-line-seed`; on that probe lane the same artifact shows `array_string_indexof_interleaved_branch_window result=hit`, lowered IR contains `nyash.array.string_indexof_hih`, and hot-block residue stays zero
      - forced generic probe originally regressed `kilo_micro_indexof_line` to `27-29 ms`; after landing FAST const-string hoist in generic pure lowering it now measures `16 ms` (`warmup=1 repeat=5`), so the dedicated `indexOf line` seed still stays the daily/perf owner but the cost gap is materially smaller
      - current probe IR hoists string-const boxer calls into `bb0` under `NYASH_LLVM_FAST=1`, so loop-local boxer churn is retired while `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1` stays unchanged
      - rejected exact cut: direct `nyash.array.string_indexof_hih` slot-string leaf rewrite regressed the probe back to `19 ms`, so keep the cached string-pair route for now and do not reopen that leaf without new evidence
      - current asm/perf reading is now fixed:
        - `C = 4 ms / 7.2M cycles`, daily `Nyash AOT seed owner = 7 ms / 22.3M cycles`, forced generic probe `= 9 ms / 33.0M cycles` (`warmup=1 repeat=5`)
        - daily seed-owner asm is already near the C loop shape (`and $0x3f`, `test $0xf`, direct `strstr@plt`, raw flip store)
        - forced generic probe no longer spends the bulk of its cycles in `Registry::with_handle` / `Registry::with_str_pair`; the narrow fast path now reads array slots directly, caches the const needle string per thread, and leaves the hot route dominated by local `array_string_indexof_by_index` work plus a small `set_hih` tail
      - compiler-local placement trace is available under `NYASH_LLVM_ROUTE_TRACE=1`; use the narrow stages `string_direct_array_set_consumer`, `string_insert_mid_window`, and `string_concat_add_route` when you need to decide the next placement cut
      - Rust-side string trace is split into `placement`, `carrier`, `sink`, and `observer` lines under the same route-trace gate; it now shows `BoundaryKind` / `RetainedForm`, borrowed substring lineage, freeze/birth sinks, and post-store observer resolution without reopening leaf hacks
      - gate split note: test probes read `NYASH_LLVM_ROUTE_TRACE`, runtime reads `NYASH_VM_ROUTE_TRACE`; the bench compare harness suppresses both stdout and stderr, so the visible probe evidence comes from the unit contracts below
    - canonical probe entrypoint: `tools/perf/run_kilo_string_trace_probe.sh`
      - it collects the unit trace contracts into one summary without touching timing lanes
      - bench compare stays timing-only; do not make it carry trace output
    - trace+asm bundle entrypoint: `tools/perf/run_kilo_string_trace_asm_bundle.sh`
      - it keeps trace and asm in the same out-dir while leaving bench compare timing-only
      - the bundle resolves symbols from the perf report before annotate, so the asm notes no longer rely on stale Rust-path guesses
      - the current bundle hot symbols are `nyash.string.concat_hh`, `nyash.string.concat3_hhh`, `nyash.string.substring_hii`, `nyash.array.set_his`, `nyash.array.string_len_hi`, `nyash_kernel::exports::string::string_handle_from_owned`, and `nyash_rust::box_trait::BoxBase::new`
    - trace probe snapshot is frozen via unit contracts (`NYASH_LLVM_ROUTE_TRACE=1 cargo test -p nyash_kernel -- --nocapture`); `bench_compare_c_py_vs_hako.sh` still suppresses those lines, so use the unit traces for placement/carrier/sink/observer inspection:
      - `string_concat_hs_contract` prints `placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> placement=return_handle`
      - `string_insert_hsi_contract` prints `observer=fast_hit -> placement=keep_transient -> sink=freeze_plan -> sink=fresh_handle -> observer=fast_hit -> placement=return_handle`
      - `substring_hii_short_slice_materializes_under_fast_contract` prints `placement=must_freeze -> carrier=freeze_span -> sink=fresh_handle -> sink=span_materialize -> observer=fast_hit`
      - the block-26 interleaved branch/select family is therefore fully observable on the same artifact, and the earlier registry-boundary perf blocker is retired for the current micro route
      - current post-cut reading is fixed:
        - same-artifact probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
        - lowered IR still contains only `nyash.array.string_indexof_hih` for the accepted observer path, and hot-block residue stays zero
        - fixed-order return may now advance to `main kilo`
      - current `main kilo` reading is now fixed:
        - `kilo_kernel_small_hk` is back to `pure-first + compat_replay=none + aot_status=ok`
        - whole-program main bundle now builds green with both live-after-get routes accepted, and the lowered IR carries `nyash.array.string_len_hi`, `nyash.array.string_indexof_hih`, and `nyash.array.set_his`
        - the former compile stoppers `array_string_len_window reason=post_len_uses_consumed_get_value` and the accepted-branch undefined `%r55` are retired by the new live-after-get pins
        - remaining main residue is no longer route acceptance; it is consumer cost around the two surviving `slot_load_hi` sites plus `nyash.string.concat_hh` / `memmove` / alloc pressure on the edit loop and branch loop
        - first post-green store cut is landed: generic pure lowering now emits `nyash.array.set_his` for proven `ORG_STRING` array writes instead of generic `nyash.array.slot_store_hih`
        - first post-green concat cut is landed: `nyash.string.concat_hh` now prefers `host_handles::with_str_pair(...)` before the slower span/materialize fallbacks
        - concat3 parity cut is landed: legacy boundary lock `phase29ck_boundary_pure_string_concat3_extern_min.sh` still proves `Extern nyash.string.concat3_hhh` is pure-first accepted, and the daily main edit loop folds `prefix + "xx" + suffix` down to `concat3_hhh` on the lowered IR
        - the next exact concat cut is landed: string-concat defer now recognizes a `concat3` consumer even when the third operand arrives through an intervening string-preserving `copy`, so the edit loop no longer emits a dead intermediate `nyash.string.concat_hh` before `concat3_hhh`
        - recent main rechecks now sit in the `715-724 ms` band on the daily route (`tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), down from the earlier `736 ms` read after pure concat3 parity; keep the explicit `NYASH_MIR_CONCAT3_CANON=1` lane as a probe only
        - the separate direct-emit owner red is retired: `phase21_5_concat3_assoc_contract_vm.sh` is green again after pure-first accepted `ArrayBox.birth()` as an initializer marker and the minimal `ret const` fallback was narrowed back to honest single-block const/ret only
        - the branch-loop consumer cut is landed: `current + "ln"` now lowers to `nyash.string.concat_hs` and the hot path no longer materializes a loop-local concat pair before `nyash.array.set_his`
        - rejected first leaf shape: direct `concat_hs` materialization regressed stable main to `1162 ms`, so the kept route caches the const suffix as a handle and forwards to the existing `concat_hh` fast path inside the leaf
	        - the edit-loop producer cut is now landed: the adjacent `substring(0, split)` / `substring(split, len)` window plus concat chain now collapses to `nyash.string.insert_hsi`, `string_insert_mid_window result=hit` is fixed in route trace, and `phase21_5_perf_kilo_text_concat_contract_vm.sh` now rejects any surviving `nyash.string.substring_hii` in `ny_main`
	        - daily main perf stays `aot_status=ok`, but the reading is still noisy (`715 ms` best recent recheck, `757 ms` latest spot check via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), so treat this landing as structure-first until leaf cost moves
	    - the next exact perf cut is now leaf quality inside `nyash.string.insert_hsi` plus the existing `nyash.string.concat_hs` / `nyash.array.set_his` tail, not more MIR route acceptance; keep MIR concat3 canon as a probe lane for now and do not promote it beyond the current owner proof
	        - rejected 2026-03-28 follow-up: direct `concat_hs` / `concat3` copy materialization regressed stable `kilo_kernel_small_hk` from the `736 ms` line to `757 ms` and did not improve micro; keep the `TextPlan`-backed concat route
	        - rejected 2026-03-28 follow-up: piece-preserving `insert_inline` plus store/freeze reshaping regressed stable `kilo_kernel_small_hk` to `895 ms`; do not reopen that cut without a fresh asm-backed reason
	        - rejected 2026-03-28 follow-up: blanket `#[inline(always)]` on host registry / hako-forward string wrappers held stable main around `740 ms` and did not beat the current `736 ms` line; keep that slice reverted
	        - rejected 2026-03-28 follow-up: `concat_hs` duplicate span-resolution removal plus span-resolver inlining regressed stable `kilo_kernel_small_hk` to `796 ms`; keep the existing `TextPlan::from_handle(...)` route
	        - rejected 2026-03-28 follow-up: specialized `StringBox`-only store leaf under `nyash.array.set_his` regressed the kept store-boundary line (`kilo_meso_substring_concat_array_set = 66 -> 69 ms`, `kilo_kernel_small_hk = 708 -> 791 ms`); keep the generic string-source helpers and the in-place source borrow cut only
	        - rejected 2026-03-29 follow-up: direct array-slot insert helper (`nyash.array.string_insert_hisi` from `string_insert_mid_window`) regressed stable main to `1020 ms` on `repeat=3`, and the `repeat=20` recheck still stayed above the kept `668 ms` line at `716 ms`; keep the current helper-backed insert route and do not reopen the direct array-slot helper without fresh birth-density evidence
	        - `micro -> meso -> kilo` observation ladder is now landed (`substring+concat+len`, `+array_set`, `+loopcarry`)
	        - compile-time placement helper `crates/nyash_kernel/src/exports/string_birth_placement.rs` is now landed
	        - first meso reading (`warmup=1 repeat=3`) is fixed:
	          - `kilo_meso_substring_concat_len = 37 ms`
	          - `kilo_meso_substring_concat_array_set = 69 ms`
	          - `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`
	        - the first large gap opens at `len -> array_set`, so the next exact leaf stays on string store / `array_set_by_index_string_handle_value` before any loop-carry reopen
	          - landed narrow store-boundary cut:
	            - `nyash.array.set_his` no longer clones a temporary source `Arc` before entering the array write closure; the hot branch now resolves the source handle in place
	            - latest spot-check is `kilo_meso_substring_concat_array_set = 66 ms` and `kilo_kernel_small_hk = 708 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
	          - landed concat3 reuse-only specialization:
	            - `concat3_plan_from_spans(...)` is now fixed to the reuse-allowed lane, so the dead `allow_handle_reuse = false` branch is gone and span emptiness checks use byte-range length directly
	            - latest same-artifact proof after this specialization is `kilo_meso_substring_concat_len = 34 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 65 ms`, `kilo_kernel_small_hk = 668 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
	          - rejected small carrier cleanup retry:
	            - sending owned fast paths directly through `string_handle_from_owned(...)`, removing the `resolve_string_span_from_handle(...)` fallback after `TextPlan::from_handle(...)`, and using the relative range length directly inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `777 ms`; keep the span-backed / helper-backed current lane for now
	          - rejected pair span-length retry:
	            - changing `concat_pair_from_spans(...)` to use span byte lengths instead of `as_str().is_empty()` regressed stable main to `904 ms`; keep the existing span-read check there for now
	          - landed sink-local read-side cut: `Registry::get` now uses a direct clone path without the extra clone helper
	          - current optimization summary lives in `docs/development/current/main/investigations/perf-kilo-string-birth-hotpath-summary-2026-03-28.md`
	          - sink-local lane is exhausted; no further safe code cut is known without fresh upstream birth-density evidence
	          - compile-time placement helper is landed, so the next exact lane is upstream birth-density proof rather than more sink-local cuts
	          - docs-first parent split is now `retained-boundary-and-birth-placement-ssot.md`: `BoundaryKind` owns retained reason and `RetainedForm` owns retained result
	          - current fixed order is `retained-boundary parent -> array_set Store proof -> same-artifact meso/main proof -> only then code-side retained-form split`; the latest proof stayed flat, so code-side `RetainedForm` split remains deferred unless fresh asm evidence appears
	          - latest kept recheck after branch-check trim is `kilo_kernel_small_hk = 707 ms`, `kilo_meso_substring_concat_array_set = 68 ms` (`warmup=1 repeat=3`)
          - latest same-artifact proof after the retained-boundary parent split stayed flat: `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 68 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 760 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
          - accepted concat3 lock-safe fast path: `concat3_plan_from_fast_str(...)` and `concat_pair_from_fast_str(...)` now return a reuse-or-owned decision before freeze, so the registry read lock is no longer held across `freeze_text_plan(...)`; `resolve_string_span_triplet_from_handles(...)` plus `string_span_cache_get_triplet(...)` landed the triple-span route
          - latest same-artifact recheck after the concat3 lock-safe route is `kilo_meso_substring_concat_len = 36 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 67 ms`, `kilo_kernel_small_hk = 704 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
          - rejected follow-up: concat3 reuse-only alias to earlier insert birth regressed stable main to `754-755 ms` under `repeat=3/20`; keep the current canonical birth split as-is until fresh placement evidence says otherwise
          - rejected follow-up: canonical `concat3_hhh` birth with later reuse alias regressed stable main to `723 ms` on `repeat=3` and `777 ms` on `repeat=20`; keep the upstream placement lane open instead of forcing another birth-site alias
          - rejected follow-up: rewriting the insert-mid route to emit `concat3_hhh` directly still regressed main to `775 ms` and tripped `build_failed_after_helper_retry` on the ladder lane; keep the current helper-backed insert route for now and do not treat the concat3 rewrite as the canonical birth
          - accepted short-slice substring freeze cut: `BorrowedSubstringPlan` now returns `FreezeSpan(StringSpan)` for short freeze-only slices, and `substring_hii` materializes them directly via `string_handle_from_span(...)` instead of routing through `TextPlan::from_span(...).into_owned()`
          - latest same-artifact recheck after the short-slice freeze cut is `kilo_meso_substring_concat_len = 35 ms`, `kilo_meso_substring_concat_array_set = 67 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 69 ms`, `kilo_kernel_small_hk = 704 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
          - accepted array string-length observer cut: `array_string_len_by_index(...)` now uses `handle_cache::with_array_box(...)` instead of `host_handles::with_handle(...) + ArrayBox` downcast, so `nyash.array.string_len_hi` stays on the typed handle-cache lane
          - latest `repeat=3` proof after this observer cut is `35 / 68 / 69` with `kilo_kernel_small_hk = 721 ms`; latest `repeat=20` WSL recheck is `36 / 67 / 68` with `kilo_kernel_small_hk = 688 ms`, so keep the cut and keep using `repeat=20` before closing noisy lanes
          - rejected length-aware store-boundary classifier retry: changing `has_direct_array_set_consumer(...)` to classify `array.set` plus trailing `length()` as a combined store boundary regressed stable main to `746 ms` on `repeat=3` and `757 ms` on `repeat=20`; keep the direct-set-only guard for this wave
          - landed JSON artifact split: `src/runner/json_artifact/` now owns artifact-family convergence, `program_json_v0_loader.rs` owns the compat import-bundle merge/trace, and `core_executor::execute_json_artifact(...)` is the terminal execution owner while the thin compat alias `run_json_v0(...)` has been deleted
          - JSON artifact family lock is now `MIR(JSON)` mainline vs `Program(JSON v0)` compat/bootstrap-only retire target
          - route reading is fixed: `--json-file` = compat umbrella intake, `--mir-json-file` = mainline direct intake
          - migration order is fixed:
            1. docs lock on artifact families and route map (`landed`)
            2. internal API split to `load_mir_json(...)`, `load_program_json_v0(...)`, `load_json_artifact_to_module(...)`, `execute_json_artifact(...)` (`landed`)
            3. compat isolation for Program(JSON v0) import-bundle behavior (`landed`)
            4. archive/delete readiness sync plus caller-surface reduction under `phase-29ci` / `phase-29cj` (`current`)
            5. public-surface cleanup / hard delete only after compat caller inventory reaches zero
          - selfhost file-level inventory lock is the next exact structure front:
            - `BuildBox.emit_program_json_v0(...)` = sole `source -> Program(JSON v0)` authority
            - `compiler_stageb.hako` = Stage-B emit/adapter lane that should shrink toward entry-only behavior
            - `launcher.hako` = CLI facade/orchestration keep that should shrink away from pipeline-detail ownership
            - `stage1_cli_env.hako` = same-file stage1 env-entry authority cluster; forced file split is deferred
            - `tools/selfhost/build_stage1.sh` = strategy shell
            - `tools/selfhost/lib/stage1_contract.sh` = contract shell
          - fixed migration order for that structure lane is:
            1. authority unification: `compiler_stageb -> BuildBox`
            2. launcher facade extraction
            3. conditional `stage1_cli_env.hako` split only if steps 1/2 leave a real blocker
            4. shell strategy/contract split
            5. compat physical isolation
            6. naming cleanup last
          - launcher facade extraction is now partially landed:
            - `launcher.hako` no longer keeps separate Program(JSON) wrapper boxes; emit/build now route through `LauncherCompileFacadeBox` + payload-contract helpers directly
            - keep shrinking launcher toward CLI/request dispatch only
          - landed cleanup closures:
            - archive-ready monitor/probe/docs bucket is archive-only evidence now
            - `tools/smokes/v2/lib/test_runner_builder_helpers.sh` now has explicit direct-MIR detection + compat fallback helpers, so the mixed route probe bucket is closed
          - remaining cleanup bucket is now closed:
            - runner-side compat alias layer has been deleted
          - caller-surface rule is now:
            - direct `MIR(JSON)` file callers use `--mir-json-file`
            - remaining `--json-file` callers are compat-on-purpose only
          - landed direct-MIR rewrites:
            - `tools/smokes/v2/profiles/quick/core/gate_c_v1_file_vm.sh`
            - `tools/smokes/v2/profiles/quick/core/nyvm_wrapper_module_json_vm.sh`
          - landed comment cleanup:
            - `tools/smokes/v2/lib/stageb_helpers.sh` and the small Hako quick canaries now describe Stage-B output as `Program(JSON v0)`, not `MIR(JSON v0)`
          - next exact leaf:
            - land the file-level inventory SSOT and keep parent docs vocabulary-only
            - treat `src/runner/json_artifact/program_json_v0_loader.rs` as the compat loader owner for `--json-file`
            - keep `core_executor` as terminal execution owner only; do not reopen it as a compat boundary owner
            - keep `pipe_io` comment wording aligned with the loader split
            - do not remove CLI flags yet, and do not force a `stage1_cli_env.hako` file split in the same wave
          - rejected known-len propagation retry: threading `known_len` / post-store facts from `concat_hs` / `array.set` into `length()` lowering kept the lane flat-to-worse (`kilo_meso_substring_concat_len = 38 ms`, `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_meso_substring_concat_array_set_loopcarry = 70 ms`, `kilo_kernel_small_hk = 705 ms` on `repeat=3`; `692 ms` on `repeat=20`); keep `array_set` as the first Store boundary and keep trailing `length()` as a separate post-store observer fact
          - post-store observer reading is now separated into `post-store-observer-facts-ssot.md`: `length()` after `array.set` is observer-after-store, not the store boundary itself
          - next design front is now `concat3-array-store-placement-window-ssot.md`: treat `concat3_hhh -> array.set -> trailing length()` as one compiler-local placement window, while still keeping `array.set` and `length()` as separate semantic boundaries
          - direct compiler bundle now shows the placement fields after rebuilding `libhako_llvmc_ffi.so`: `string_direct_array_set_consumer` carries `producer_kind=Concat3` / `boundary_kind=Store` / `post_store_use=None` / `known_len=-1` on the concat3 store boundary, while `array_string_len_window` carries `producer_kind=ArrayGet` / `boundary_kind=Store` / `post_store_use=LenObserver` / `known_len=-1`
          - rejected short-slice owned materialize retry: changing the short freeze lane to `FreezeOwned(String)` and materializing inside `borrowed_substring_plan_from_handle(...)` regressed stable main to `866 ms`; keep the span-backed short freeze contract for now
          - rejected follow-up: borrowed triple-span miss resolution via `handles::with3(...)` plus local `StringViewBox` flattening kept meso flat (`67 -> 68 ms`) and regressed stable main (`704 -> 745 -> 819 ms` on back-to-back checks); keep the explicit uncached miss wave in `resolve_string_span_triplet_from_handles(...)`
          - rejected follow-up: widening the C-side direct-store consumer test to tolerate one trailing `length()` observer after `array.set` kept the lane flat-to-worse (`36 / 70 / 70`, `kilo_kernel_small_hk = 706 ms` under `repeat=3`); keep the stricter store-only consumer guard
          - rejected follow-up: direct-set-preferring `concat3_hhh` route ordering in `string_concat_add_route` looked promising in trace, but the timing-only 3-run regressed to `kilo_kernel_small_hk = 745 ms` (`c_ms = 74`, `aot_status=ok`); keep the existing fallback order and do not treat this route-order tweak as a win
          - rejected follow-up: compiler-side `string.length()` arithmetic lowering for the insert-shaped concat recipe improved meso (`33 / 63 / 65`) but still regressed stable main to `695 ms` versus the kept `668 ms` line on the same artifact pair; keep the runtime `nyash.string.len_h` observer until a future placement wave changes the retained boundary
          - rejected follow-up: combining the widened direct-store consumer window with insert-recipe `length()` arithmetic still regressed stable main to `732 ms` (`34 / 66 / 69` under `repeat=3`), so keep both slices closed until a future placement wave changes the retained/store boundary
          - rejected follow-up: making `borrowed_substring_plan_from_handle(...)` consult the span cache before `handles::with_handle(...)` lowered meso to `33 / 64 / 67` and cut `substring_hii` down to `6.57%`, but stable main still stayed at `706 ms`; keep the current planner shape until a future placement wave can remove more than one substring birth together
	          - rejected follow-up: flipping `string_len_from_handle(...)` / `string_is_empty_from_handle(...)` to consult `string_len_impl(...)` / `string_is_empty_impl(...)` before the direct fast-string observer path kept meso at `35 / 68 / 71` and regressed stable main to `764 ms`; keep the existing fast-str-first observer order for now
	          - rejected follow-up: widening `handle_cache` to latest+previous entries and routing `array_set_by_index_string_handle_value(...)` through a detached array cache path lowered meso to `35 / 65 / 69`, but stable main still stayed at `701 ms`; keep the current one-slot cache and direct `with_array_box(...) + handles::with_handle(...)` store route
	          - rejected follow-up: relaxing `has_direct_array_set_consumer(...)` to accept `array.set` as the first consumer even when `out.length()` stays afterward only reached `35 / 67 / 67` on meso and `698 / 697 ms` on back-to-back main checks; keep the stricter single-use predicate for now
	          - rejected follow-up: the `insert_hsi` one-resolve helper improved the first `repeat=3` read (`kilo_kernel_small_hk = 694 ms`) but drifted back to `727 ms` under `repeat=20`; keep the current helper-backed lane on WSL
	          - rejected follow-up: seeding `string_span_cache` at `materialize_owned_string(...)` birth improved the first `repeat=3` probe (`35 / 69 / 71`, `kilo_kernel_small_hk = 692 ms`) but regressed to `36 / 70 / 69`, `kilo_kernel_small_hk = 730 ms` under `repeat=20`; keep span-cache admission on resolve-side only for this wave
	          - code-side `RetainedForm` split remains deferred unless fresh asm evidence appears
	          - latest asm read puts `__memmove_avx512_unaligned_erms`, `nyash.string.concat_hs`, `Registry::get`, and `Registry::alloc` above `BoxBase::new`, so the next cut is upstream placement proof
	          - next exact cut stays on store-boundary birth/lookup cost only if new asm evidence appears; keep `BoxBase::new` out unless the object layout itself shows up as the limiter, not loop-carry shaping
	        - docs-first next design front is now `string-birth-sink-ssot.md`:
	          - `freeze.str` is the single birth sink
	          - `concat_hs` / `insert_hsi` / `concat3_hhh` should converge on the same `plan -> freeze` model
	          - `set_his` helper splits are no longer the primary design front
	          - landed planner cleanup: const-suffix / insert recipe helpers are now isolated in `crates/nyash_kernel/src/exports/string_plan.rs`
	          - current implementation order is fixed:
	            1. shrink `BorrowedSubstringPlan` into recipe-only / boundary-only placement
	            2. keep `array_set` as the consumer boundary
	            3. meso/main proof on the same artifact pair
	            4. only then narrow sink-local tuning further if new asm evidence appears; keep `BoxBase::new` out unless the object layout itself shows up as the limiter
	          - canonical sink re-home was attempted but rejected: moving `freeze.str` into `string_store.rs` regressed stable main (`kilo_kernel_small_hk = 834 -> 909 ms` on back-to-back checks), so keep the explicit `freeze_text_plan(...)` helper in `string.rs` for now
	        - `P0-attrs` is now landed conservatively on proven read-only array/map observer aliases (`slot_load_hi` / `string_len_hi` / `string_indexof_hih` / `slot_len_h` / `probe_hh` / `entry_count_i64`); do not stamp hookable or mutating exports like `nyash.string.len_h` / `nyash.string.indexOf_hh` / `nyash.array.set_his`
	        - current app contract now pins those attrs directly and rejects accidental `readonly` on `nyash.array.set_his`
	        - latest attrs spot-check was noisy (`831 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), so treat `P0-attrs` as IR-quality groundwork only; no wall-clock win is claimed yet
	        - `P0-copy-fold` is now landed in emit-side generic pure lowering:
	          - `copy` and `StringBox(arg0)` passthrough now use alias resolution instead of emitting identity `add i64 0, %rX` / `or i1 %rX, false`
	          - emit helpers now resolve copy chains before printing SSA operands, so `phi` / `icmp` / `branch` / `ret` / call args / `select` / `binop` all see the same terminal source
	          - `phase21_5_perf_kilo_text_concat_contract_vm.sh` now rejects any surviving copy-style noise in `ny_main`
	          - latest spot-check is `750 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`; treat this as IR cleanup with a small recovery, not the final leaf win
	        - `P1-bool-i1` is now landed conservatively on compare/copy/phi/branch merges:
	          - prepass now marks `compare` results as `T_I1`, carries that type through `copy`, and emits `phi i1` when all incoming values are bools / bool consts
	          - legacy boundary lock `phase29ck_boundary_pure_bool_phi_branch_min.sh` proves `compare -> phi(bool) -> branch` lowers as `phi i1` plus direct `br i1`, and `phase29ck-boundary-legacy` now carries that pin
	          - latest spot-check is `771 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`; treat this as correctness/IR cleanup only, not as a claimed wall-clock win
	        - hoisted string-const cleanup is now landed for the main leaf route:
	          - FAST hoist pre-scan now skips `StringBox` handle materialization when the const is proven raw-ptr-only for `nyash.string.insert_hsi` / `nyash.string.concat_hs`
	          - `phase21_5_perf_kilo_text_concat_contract_vm.sh` now rejects dead `nyash.box.from_i8_string_const` calls in `ny_main`, so the edit/branch loop raw-ptr route stays visible in IR
	          - the direct-emit concat3 owner canary is green again after global `print` lowering was made copy-transparent, resolving the `%r29/%r36` undefined-SSA regression
	          - latest spot-check is `760 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`; treat this as IR cleanup only, not as a claimed wall-clock win
	        - main-kilo exact next cut is now back on leaf quality:
	          - keep focusing on `nyash.string.insert_hsi` plus the surviving `nyash.string.concat_hs` / `nyash.array.set_his` tail
	          - do not open broader value-repr work until the current leaf-quality gap has a same-artifact reason
	        - keep the following out of the current exact lane for now:
	          - full `ptr`-typed handle lowering and broad value-repr changes
	          - broad LLVM pass-pipeline work / generic optimizer migration
	          - JSON single-parse cleanup, `llc` shell-out replacement, and VLA removal
	        - asm target is now fixed again: C already reaches `load -> strstr -> malloc/memcpy append -> store` with `%64/%8` folded to mask/test, and Hakorune now reaches `insert_hsi -> set_his` on the edit loop and `get_hi -> string_indexof_hih -> concat_hs -> set_his` on the branch loop, so the remaining hot delta is leaf quality rather than substring/concat route acceptance
      - temporary priority override is `clean-clean / BoxShape` before the next perf cut:
        - first cleanup target is `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc`
        - extracted `indexOf` observer state/trace helpers into `hako_llvmc_ffi_indexof_observer_state.inc` and `hako_llvmc_ffi_indexof_observer_trace.inc`
        - extracted direct `indexOf` observer detector helpers into `hako_llvmc_ffi_indexof_observer_direct_match.inc`
        - extracted cross-block / interleaved `indexOf` observer detector helpers into `hako_llvmc_ffi_indexof_observer_block_match.inc`
        - extracted `indexOf` observer lowering helpers into `hako_llvmc_ffi_indexof_observer_lowering.inc`
        - extracted FAST const-string hoist helper into `hako_llvmc_ffi_const_string_hoist.inc`
        - extracted `mir_call` prepass need-flag helpers into `hako_llvmc_ffi_mir_call_prepass.inc`
        - extracted non-`indexOf` generic method lowering helpers into `hako_llvmc_ffi_generic_method_lowering.inc`
        - extracted `mir_call` emit-shell helpers into `hako_llvmc_ffi_mir_call_shell.inc` so constructor/global emit and runtime route classification no longer stay inline in the lowering walk
        - extracted string concat chain / concat3 extern helpers into `hako_llvmc_ffi_string_concat_lowering.inc`
        - route summary remains unchanged after the split: probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
        - this BoxShape pass is now sufficient for returning to perf work; only reopen `pure_compile.inc` cleanup if a new exact readability/ownership blocker appears
        - `tools/perf/trace_optimization_bundle.sh` now emits `owner_route` / `first_blocker` in its bundle summary
        - `tools/build_hako_llvmc_ffi.sh` now serializes shared `libhako_llvmc_ffi.so` rebuilds with a small lock
        - smoke ownership cleanup is landed: `test_runner.sh` is now a thin loader over `test_runner_{builder,stdout_core,stdout,llvm}_helpers.sh`, and `phase29ck-boundary` suite now includes the concat3 extern canary explicitly
        - external evaluation positives to preserve:
          - keep Rust `ny-llvmc` topology thin; `main.rs` / `driver_dispatch.rs` / `native_ir.rs` stay transport/driver seams
          - keep MIR(JSON) as the explicit debug/proof seam for route and lowering evidence
          - keep docs/SSOT/AI-handoff discipline as a maintained strength
        - broad `native_ir.rs` migration, unboxed value representation, and `llc` shell-out replacement stay future design topics rather than the current exact cleanup lane
        - any near-term `AOT-Core` follow-up stays analysis-only facts/recipe view only; do not open a full new MIR layer before the current registry-boundary blocker is cut
        - keep daily seed owner, probe lane, and current acceptance rows unchanged during that cleanup
      - `RuntimeDataBox` stays protocol/facade only in this wave; do not reopen broad generic peel/widen before the same blocker family recurs
    - explicit compat-keep cleanup residue is retired:
      - `phase29ck_boundary_compat_keep_min.sh` is green again
      - direct `target/release/ny-llvmc --driver harness --in apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json ...` writes object again on the explicit keep lane
      - optimization return resumes at `micro kilo` while keeping the fixed order `leaf-proof micro -> micro kilo -> main kilo`
      - if the micro lane reopens, the next exact blocker is the local `nyash.array.string_indexof_hih` closure/update tail rather than registry-boundary lookup glue
    - do not reopen a direct `indexOf` observer that still leaves `slot_load_hi`
  - current exact front:
    - `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
    - `stage2-aot-core-proof-vocabulary-ssot.md`
    - `stage2-optimization-debug-bundle-ssot.md`
    - `phase29ck-array-substrate-rejected-optimizations-2026-03-27.md`
  - working rule:
    - keep `pure-first + compat_replay=none` pinned
    - optimize `ny-llvmc(boundary)` rather than `llvmlite`
    - do not introduce a distinct new IR layer in this wave
    - if the same blocker family repeats after the next narrow fast-path cut, consider only an analysis-only `AOT-Core facts/recipe view`, not a serializer-carrying full MIR layer
    - prefer analysis-only recipe/canonical-subset work on existing MIR over runtime smartening or backend-only tweaks
    - keep `RuntimeDataBox` facade-only; a new peel/widen is allowed only if the same blocker family repeats after the direct-path exact cut
    - do not broaden pure-first to permanent dual-dialect support
    - do not keep a new fused leaf without same-artifact route/window/IR/symbol proof
    - on WSL, do not treat a single main bench delta as proof when bundled main IR/symbol is unchanged
- Compiler lane: `phase-29bq`（JIR-PORT-00..08 done / active blocker=`none` / next=`none`）
- JoinIR port mode（lane A）: monitor-only（failure-driven）
- Boundary-retire lane: `phase-29ci`
  - status: `formal-close-synced`
  - current boundary-retirement scope is complete for the accepted keep set:
    - helper-local slices through W14 are landed
    - smoke-tail caller buckets through W18 are landed
    - `phase2044` / `phase2160` thin wrapper families are monitor-only keeps
    - `phase2170` default pack is landed
    - `phase2170/hv1_mircall_*` stays as explicit keep
  - reopen only if:
    - a new exact caller/helper gap appears
    - or hard delete / broad internal removal explicitly resumes
- By-name retire lane: `phase-29cl`
  - status: `formal-close-synced`
  - current accepted keep set is complete for the present `by_name` retirement scope
  - helper-side current truth:
    - `tools/hakorune_emit_mir.sh`: monitor-only
    - `tools/selfhost/selfhost_build.sh`: monitor-only
    - `tools/smokes/v2/lib/test_runner.sh`: thin loader / monitor-only
  - reopen only if:
    - a new exact `by_name` caller/helper gap appears
    - or hard delete / broad internal removal explicitly resumes
- Rune lane: `phase-29cu`
  - status: `formal-close-synced`
  - narrow-scope current truth:
    - declaration-local `attrs.runes`
    - Rust direct MIR carrier
    - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
    - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
    - `.hako` parser statement/program routes fail fast on Rune invalid placement
    - Rust function-target placement / ABI-facing verifier contract
    - `.hako` root-entry carrier value-contract parity for `CallConv("c")` / `Ownership(owned|borrowed|shared)`
    - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
    - `Program(JSON v0)` remains no-widen
  - latest landed carrier cut:
    - `.hako` compiler/mirbuilder state now carries a generic function-rune map instead of `entry_runes_json`
    - `.hako` MIR attrs injection is function-name driven instead of `main` hardcode
    - `.hako` Stage-B source route now carries root-entry Rune attrs through a real `Main.main` def instead of a synthetic transport shim
    - parser/roundtrip/MIR bridge tests now pin the canonical `@rune` surface for `Public/FfiSafe/ReturnsOwned/FreeWith/Symbol/CallConv/Hint/Contract/IntrinsicCandidate`
    - RawArray dispatch now short-circuits invalid handles before index resolution, keeping the facade-only contract intact
  - planned future reopen only:
    - `.hako` declaration-local full Rune carrier parity beyond root-entry transport
- Bootstrap-retire lane: `phase-29cj`
  - status: `formal-close-synced`
  - current stop-line is still `src/host_providers/mir_builder.rs::module_to_mir_json(...)`
  - latest landed `.hako` cuts now cover `BuilderUnsupportedTailBox`, `Stage1MirPayloadContractBox`, `Stage1CliProgramJsonInputBox`, `Stage1CliRawSubcommandInputBox`, `LauncherArtifactIoBox`, and `LauncherPayloadContractBox`
  - `MirBuilderBox.hako`, `stage1_cli_env.hako`, `stage1_cli.hako`, and `launcher.hako` are now treated as near-thin-floor / close-synced owners
- Runtime lane: `phase-29y`
  - parked
  - operational reading is `llvm-exe` daily / `vm-hako` reference-debug-bootstrap-proof / `rust-vm` bootstrap-recovery-compat
  - active acceptance is `phase29y_vm_hako_caps_gate_vm.sh` only
  - `phase29ck_vmhako_llvm_backend_runtime_proof.sh` is manual monitor evidence only, not a blocking acceptance smoke
- Substrate lane: `phase-29ct`
  - stop-line reached
- JSON v0 reading
  - `Program(JSON v0)` is retire/no-widen and no longer the target external/bootstrap boundary
  - `MIR(JSON v0)` is the current external/bootstrap interchange / gate boundary
  - allowed keep:
    - internal compat/test/bootstrap-only routes
    - `.hako` mirbuilder internal input until later delete waves

## Clean-Shape Status

1. `stage1/stage2` artifact semantics の整理（landed）
2. `ABI/export manifest + generated shim` 化（landed）
3. `hako_alloc` root の物理再編（landed）
4. transitional Rust export の daily-path 退役（landed）
5. handle/provider/birth の substrate-only 化（docs-locked）
6. `Stage3` gate 追加（landed）
   - build lane compares re-emitted Program/MIR payload snapshots from a known-good seed plus `.artifact_kind`
   - skip-build lane compares an explicit prebuilt pair

## Exact Links

- Mainline workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Execution lane policy: `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
- Execution lane task pack: `docs/development/current/main/design/execution-lanes-migration-task-pack-ssot.md`
- Execution lane legacy inventory: `docs/development/current/main/design/execution-lanes-legacy-retirement-inventory-ssot.md`
- Bootstrap route SSOT: `docs/development/current/main/design/selfhost-bootstrap-route-ssot.md`
- Compiler structure SSOT: `docs/development/current/main/design/selfhost-compiler-structure-ssot.md`
- Stage axis SSOT: `docs/development/current/main/design/de-rust-stage-and-owner-axis-ssot.md`
- Rune final shape SSOT: `docs/development/current/main/design/rune-and-stage2plus-final-shape-ssot.md`
- Rune v0 rollout SSOT: `docs/development/current/main/design/rune-v0-contract-rollout-ssot.md`
- Stage3 same-result gate: `tools/selfhost/stage3_same_result_check.sh`
- ABI inventory: `docs/development/current/main/design/abi-export-inventory.md`
- JSON v0 inventory: `docs/development/current/main/phases/phase-29ci/P0-PROGRAM-JSON-V0-CONSUMER-INVENTORY.md`
- Route split note: `docs/development/current/main/phases/phase-29ci/P4-MIRBUILDER-ROUTE-SPLIT.md`
- Phase 29ci close-sync: `docs/development/current/main/phases/phase-29ci/README.md`
- Active selfhost lane: `docs/development/current/main/phases/phase-29bq/README.md`
- By-name retire lane: `docs/development/current/main/phases/phase-29cl/README.md`

## Restart Reminder

- 最初に `git status -sb` を見る。
- 次に `CURRENT_TASK.md` を読む。
- その次に `15-Workstream-Map.md` で lane 順を確認する。
- 詳細は `10-Now.md` を増やさず、phase README / design SSOT を開く。
