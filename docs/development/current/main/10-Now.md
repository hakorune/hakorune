---
Status: SSOT
Date: 2026-03-27
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

## Current Read

- Active lane: `phase-29bq`
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
- Secondary exact blocker lane: `phase-29ck`
  - status: `active follow-up / docs-first exact front`
  - current exact result:
    - `Stage1 MIR dialect split` is retired for the current kilo entry
    - `kilo_kernel_small_hk` is back to `pure-first + compat_replay=none + aot_status=ok`
    - docs-first proof-vocabulary lock is landed
    - rejected follow-up: authoritative `ArrayBox` integer-storage split did not improve `kilo_micro_array_getset` and regressed main `kilo`
    - rejected follow-up: `ArrayBox.items` `parking_lot::RwLock -> std::sync::RwLock` regressed both micro and main
    - rejected follow-up: `host_handles.table` `parking_lot::RwLock -> std::sync::RwLock` regressed both micro and main
    - rejected follow-up: backend-private adjacent fused `get -> +const -> set -> get` leaf is now explained as a route-shape miss, not a mysterious symbol miss
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
    - next exact work order is fixed:
      - `leaf-proof micro`
      - `micro kilo`
      - `main kilo`
    - `tools/perf/run_kilo_leaf_proof_ladder.sh` is the first acceptance lane for new observer/mutator leaves
    - current `leaf-proof micro` facts:
      - `kilo_leaf_array_rmw_add1 = 36 ms` (`aot_status=ok`)
      - `kilo_leaf_array_string_len = 12 ms` (`aot_status=ok`)
      - `kilo_leaf_array_string_indexof_const = 25 ms` (`aot_status=ok`)
      - narrow pure-first pins are now `apps/tests/mir_shape_guard/array_string_indexof_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_branch_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_cross_block_select_min_v1.mir.json`, `apps/tests/mir_shape_guard/array_string_indexof_interleaved_branch_min_v1.mir.json`, and `apps/tests/mir_shape_guard/array_string_indexof_interleaved_select_min_v1.mir.json`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_select_min.sh` proves `get -> indexOf("line") -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-select-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_branch_min.sh` proves `get -> indexOf("line") -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-branch-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_cross_block_select_min.sh` proves `get -> indexOf("line") -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-cross-block-select-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_branch_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> compare -> branch` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-branch-v1`
      - boundary smoke `phase29ck_boundary_pure_array_string_indexof_interleaved_select_min.sh` proves `get -> indexOf("line") -> (%16==0) guard -> jump -> compare -> select` without harness fallback, and the visible `.hako` evidence row is `acceptance_case=array-string-indexof-interleaved-select-v1`
      - the exact leaf-proof pure-first acceptance gap is retired
      - fixed-order recheck after the landing is `kilo_micro_indexof_line = 7 ms`, `kilo_kernel_small_hk = 824 ms` (`warmup=1 repeat=3`)
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
      - the block-26 interleaved branch/select family is therefore fully observable on the same artifact, and the earlier registry-boundary perf blocker is retired for the current micro route
      - current post-cut reading is fixed:
        - same-artifact probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
        - lowered IR still contains only `nyash.array.string_indexof_hih` for the accepted observer path, and hot-block residue stays zero
        - fixed-order return may now advance to `main kilo`
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
        - route summary remains unchanged after the split: probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
        - this BoxShape pass is now sufficient for returning to perf work; only reopen `pure_compile.inc` cleanup if a new exact readability/ownership blocker appears
        - `tools/perf/trace_optimization_bundle.sh` now emits `owner_route` / `first_blocker` in its bundle summary
        - `tools/build_hako_llvmc_ffi.sh` now serializes shared `libhako_llvmc_ffi.so` rebuilds with a small lock
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
    - `tools/smokes/v2/lib/test_runner.sh`: near-thin-floor / monitor-only
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
