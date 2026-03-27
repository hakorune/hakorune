# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-27
Scope: repo root の再起動入口。詳細の status/phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。
- naming cleanup lane の単一正本は `docs/development/current/main/phases/phase-29cs/README.md`。
- substrate capability ladder lane の単一正本は `docs/development/current/main/phases/phase-29ct/README.md`。
- current-task history archive の単一正本は `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current blocker (SSOT)

- runtime lane is parked/monitor-only again; there is no active `vm-hako` throughput blocker.
- current `vm-hako` remains a parked monitor/debug/bootstrap-proof lane only.
- `phase29ck_vmhako_llvm_backend_runtime_proof.sh` is manual/non-blocking monitor evidence, not mainline acceptance.
- `phase-29cj` has completed its near-thin-floor reinventory and formal close sync.
- `phase-29cu` has completed its formal close sync for the narrow Rune v0 scope.
- `phase-29ci` has completed its formal close sync for the current boundary-retirement scope.
- active implementation lane is `phase-29bq`:
  - selfhost `.hako` migration stays `mirbuilder first / parser later`
  - current blocker is `none`
  - `JIR-PORT-08` is landed; keep the lane failure-driven and promote a new exact leaf only when the next blocker is captured
- secondary exact blocker lane is `phase-29ck`:
  - `Stage0 = llvmlite` keep lane / `Stage1 = ny-llvmc(boundary pure-first)` mainline lane split is now locked
  - current route-correction blocker is retired for the current kilo entry
  - backend-zero thin-up is landed for the daily owner split:
    - `.hako` daily route is now visible as `pure-first + compat_replay=none`
    - explicit keep route remains `pure-first + compat_replay=harness`
    - `phase29ck_boundary_pure_first_min.sh` now pins the `compat_replay=none` route trace directly
  - current exact front is `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
  - current facts are:
    - `P17` staged `AOT-Core` proof vocabulary lock is landed
    - keep a rolling reject ledger for array substrate experiments
    - integer-heavy `ArrayBox` representation splits that add extra read crossings are currently rejected
    - `ArrayBox.items` lock-implementation swap (`parking_lot` -> `std::sync`) is also rejected in the current wave
    - `host_handles.table` lock-implementation swap (`parking_lot` -> `std::sync`) is also rejected in the current wave
    - rejected adjacent fused-leaf read is now explained by live no-replay route evidence:
      - current `kilo_micro_array_getset` live MIR window is semantic `get -> copy* -> const 1 -> add -> set`
      - earlier trigger miss was partly obscured by PHI-origin loss; current route trace now follows `scan_origin`
    - current micro route is now proven end-to-end on the same artifact:
      - `array_rmw_window result=hit`
      - lowered IR contains `nyash.array.rmw_add1_hi`
      - built binary exports `nyash.array.rmw_add1_hi`
    - current `kilo_kernel_small` source route now proves one direct main hit on the same artifact:
      - `array_string_len_window result=hit count=1`
      - lowered IR contains `nyash.array.string_len_hi`
      - built binary exports `nyash.array.string_len_hi`
    - rejected follow-up:
      - same-artifact `array_string_indexof_window result=hit` was proven
      - lowered IR still contained both `nyash.array.slot_load_hi` and `nyash.array.string_indexof_hih`
      - stable main regressed to `853 ms`
      - `kilo_micro_indexof_line` regressed to `9 ms`
    - current main route still has two accepted observer misses:
      - `array_string_len_window reason=post_len_uses_consumed_get_value`
      - `array_string_len_window reason=next_noncopy_not_len`
    - next exact code cut order is fixed:
      - `leaf-proof micro`
      - `micro kilo`
      - `main kilo`
    - `tools/perf/run_kilo_leaf_proof_ladder.sh` is the first acceptance lane for new observer/mutator leaves
    - current `leaf-proof micro` facts are:
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
      - current asm/perf comparison is now fixed:
        - `C = 4 ms / 7.2M cycles`, daily `Nyash AOT seed owner = 7 ms / 22.3M cycles`, forced generic probe `= 9 ms / 33.0M cycles` (`warmup=1 repeat=5`)
        - daily seed-owner asm is already near the C shape (`and $0x3f`, `test $0xf`, direct `strstr@plt`, raw flip store), so the remaining daily gap is small glue around a mostly-native loop
        - forced generic probe no longer spends the bulk of its cycles in `Registry::with_handle` / `Registry::with_str_pair`; the narrow fast path now reads array slots directly, caches the const needle string per thread, and leaves the hot route dominated by local `array_string_indexof_by_index` work plus a small `set_hih` tail
      - the block-26 interleaved branch/select family is therefore fully observable on the same artifact, and the earlier registry-boundary perf blocker is retired for the current micro route
      - current exact post-cut reading is fixed:
        - same-artifact probe bundle still reports `owner_route=generic_probe first_blocker=array_rmw_window:const_not_1`
        - lowered IR still contains only `nyash.array.string_indexof_hih` for the accepted observer path, and hot-block residue stays `slot_load_hi=0`, `generic_box_call=0`, `hostbridge=0`
        - current probe perf top is now local `array_string_indexof_by_index` work, with `array_set_by_index` / `slot_store_box_raw` as the visible remaining tail
        - do not reopen the rejected direct slot-string leaf rewrite without fresh same-artifact evidence
      - current `main kilo` reading is now fixed:
        - `kilo_kernel_small_hk` is back to `pure-first + compat_replay=none + aot_status=ok`
        - whole-program main bundle now builds green with both live-after-get routes accepted, and the lowered IR carries `nyash.array.string_len_hi`, `nyash.array.string_indexof_hih`, and `nyash.array.set_his`
        - the former compile stoppers `array_string_len_window reason=post_len_uses_consumed_get_value` and the accepted-branch undefined `%r55` are retired by the new live-after-get pins
        - remaining main residue is no longer route acceptance; it is consumer cost around the two surviving `slot_load_hi` sites plus `nyash.string.concat_hh` / `memmove` / alloc pressure on the edit loop and branch loop
        - first post-green store cut is landed: generic pure lowering now emits `nyash.array.set_his` for proven `ORG_STRING` array writes instead of generic `nyash.array.set_hih`
        - first post-green concat cut is landed: `nyash.string.concat_hh` now prefers `host_handles::with_str_pair(...)` before the slower span/materialize fallbacks
        - concat3 parity cut is landed: boundary smoke `phase29ck_boundary_pure_string_concat3_extern_min.sh` now proves `Extern nyash.string.concat3_hhh` is pure-first accepted, and the daily main edit loop folds `prefix + "xx" + suffix` down to `concat3_hhh` on the lowered IR
        - the next exact concat cut is landed: string-concat defer now recognizes a `concat3` consumer even when the third operand arrives through an intervening string-preserving `copy`, so the edit loop no longer emits a dead intermediate `nyash.string.concat_hh` before `concat3_hhh`
        - recent main rechecks now sit in the `715-724 ms` band on the daily route (`tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), down from the earlier `736 ms` read after pure concat3 parity; keep the explicit `NYASH_MIR_CONCAT3_CANON=1` lane as a probe only
        - the separate direct-emit owner red is retired: `phase21_5_concat3_assoc_contract_vm.sh` is green again after pure-first accepted `ArrayBox.birth()` as an initializer marker and the minimal `ret const` fallback was narrowed back to honest single-block const/ret only
        - the branch-loop consumer cut is landed: `current + "ln"` now lowers to `nyash.string.concat_hs` and the hot path no longer materializes a loop-local concat pair before `nyash.array.set_his`
        - rejected first leaf shape: direct `concat_hs` materialization regressed stable main to `1162 ms`, so the kept route caches the const suffix as a handle and forwards to the existing `concat_hh` fast path inside the leaf
	        - the edit-loop producer cut is now landed: the adjacent `substring(0, split)` / `substring(split, len)` window plus concat chain now collapses to `nyash.string.insert_hsi`, `string_insert_mid_window result=hit` is fixed in route trace, and `phase21_5_perf_kilo_text_concat_contract_vm.sh` now rejects any surviving `nyash.string.substring_hii` in `ny_main`
	        - daily main perf stays `aot_status=ok`, but the reading is still noisy (`715 ms` best recent recheck, `757 ms` latest spot check via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), so treat this landing as structure-first until leaf cost moves
	        - the next exact perf cut is now leaf quality inside `nyash.string.insert_hsi` plus the existing `nyash.string.concat_hs` / `nyash.array.set_his` tail, not more MIR route acceptance; keep MIR concat3 canon as a probe lane for now and do not promote it beyond the current owner proof
	        - `P0-attrs` is now landed conservatively on proven read-only array/map observer aliases (`slot_load_hi` / `string_len_hi` / `string_indexof_hih` / `slot_len_h` / `probe_hh` / `entry_count_h`); do not stamp hookable or mutating exports like `nyash.string.len_h` / `nyash.string.indexOf_hh` / `nyash.array.set_his`
	        - current app contract now pins those attrs directly and rejects accidental `readonly` on `nyash.array.set_his`
	        - latest attrs spot-check was noisy (`831 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`), so treat `P0-attrs` as IR-quality groundwork only; no wall-clock win is claimed yet
	        - `P0-copy-fold` is now landed in emit-side generic pure lowering:
	          - `copy` and `StringBox(arg0)` passthrough now register copy aliases instead of emitting identity `add i64 0, %rX` / `or i1 %rX, false`
	          - emit helpers (`phi` / `icmp` / `branch` / `ret` / call arg refs / select / binop) now resolve alias chains before printing SSA operands
	          - `phase21_5_perf_kilo_text_concat_contract_vm.sh` now rejects any surviving copy-style `add i64 0, %rN` / `or i1 %rN, false` noise in `ny_main`
	          - latest spot-check is back to `750 ms` via `tools/perf/run_kilo_hk_bench.sh diagnostic 1 3`; treat this as IR cleanup with a small recovery, not as the final main-kilo perf cut
	        - `P1-bool-i1` is now landed conservatively on compare/copy/phi/branch merges:
	          - prepass now marks `compare` results as `T_I1`, carries that type through `copy`, and emits `phi i1` when all incoming values are bools / bool consts
	          - new boundary canary `phase29ck_boundary_pure_bool_phi_branch_min.sh` proves `compare -> phi(bool) -> branch` lowers as `phi i1` plus direct `br i1`, and the `phase29ck-boundary` suite now carries that pin
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
          - keep Rust `ny-llvmc` topology thin; `main.rs` / `driver_dispatch.rs` / `native_ir.rs` stay transport/driver seams, not new policy owners
          - keep MIR(JSON) as the explicit debug/proof seam between `.hako`/boundary and native lowering
          - keep docs/SSOT/AI-handoff discipline as a first-class asset; structure changes stay docs-first
        - external evaluation items intentionally not promoted into the current exact lane:
          - broad `native_ir.rs` generic-lowering migration is a future design lane, not this cleanup series
          - unboxed value representation is a future performance design topic, not a `phase-29ck` exact blocker
          - replacing `llc` shell-out is worthwhile but separate from the current route/cleanup owner proof
        - `AOT-Core MIR` implementation policy is fixed for this wave:
          - do not open a full new `AOT-Core MIR` layer before the current registry-boundary blocker is cut
          - if the same blocker family repeats after the narrow fast-path cut, promote only an analysis-only `AOT-Core facts/recipe view` (`value_class` / `borrowed-or-raw` / `observer_op` / `effect_mask` / `reject_reason`), not a serializer-carrying new IR layer
        - future design backlog order after the current exact lane:
          - first: replace `llc` shell-out with a thinner in-process/object-emitter seam once the current route/cleanup owner proof is stable
          - second: stage a gradual generic-lowering migration from `lang/c-abi/shims/hako_llvmc_ffi_pure_compile.inc` into `crates/nyash-llvm-compiler/src/native_ir.rs` without collapsing the MIR(JSON) proof seam or promoting Rust into a new policy owner
          - third: open unboxed value-representation design only after route ownership and generic lowering contracts are stable; this stays design-doc-first because it is the widest performance/semantic change
        - keep daily seed owner, probe lane, and current acceptance rows unchanged during that cleanup
      - `RuntimeDataBox` stays protocol/facade only in this wave; do not reopen broad generic peel/widen before the same blocker family recurs
    - explicit compat-keep cleanup residue is retired:
      - `phase29ck_boundary_compat_keep_min.sh` is green again
      - direct `target/release/ny-llvmc --driver harness --in apps/tests/mir_shape_guard/method_call_only_small.prebuilt.mir.json ...` writes object again on the explicit keep lane
      - `llvmlite` keep-lane parse residue is retired without changing the Stage1 daily route policy
    - optimization return point after that cleanup stays unchanged:
      - return directly to `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
      - keep the current fixed order `leaf-proof micro -> micro kilo -> main kilo`; current resume point is `micro kilo`
      - the forced generic observer micro gap is now materially reduced; fixed-order return may advance to `main kilo`
      - if the micro lane reopens, the next exact blocker is the local `nyash.array.string_indexof_hih` closure/update tail rather than registry-boundary lookup glue
      - do not reopen broader keep-lane work once the explicit compat keep pin is green again
    - current non-blocking residue to ignore for this lane:
      - `build_stage1.sh --artifact-kind stage1-cli` capability check remains red
      - `phase29ci_stage1_cli_exact_emit_contract_vm.sh` remains red at compat route probe
      - treat both as separate Stage1/selfhost residue, not as `phase-29ck` owner proof blockers
    - do not reopen a direct `indexOf` observer that still leaves `slot_load_hi` behind

## Current Priority

1. active implementation lane: `phase-29bq`
   - status: `active (failure-driven; blocker=none)`
   - scope: selfhost `.hako` migration under `mirbuilder first / parser later`
   - working rule:
     - `JIR-PORT-08` is done and the fast gate is back to green
     - current exact implementation leaf is `none` while blocker=`none`
     - capture the next exact blocker before promoting any broader lane work
     - keep daily gate / probe / checklist operation active
   - read in this order:
     - `docs/development/current/main/phases/phase-29bq/README.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-113-hako-recipe-first-migration-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-114-hako-cleanup-integration-prep-lane.md`
     - `docs/development/current/main/phases/phase-29bq/29bq-115-selfhost-to-go-checklist.md`
   - latest landed blocker:
     - fixture: `apps/tests/phase29bq_selfhost_blocker_parse_program2_nested_loop_if_else_fallthrough_join_else_return_blockexpr_min.hako`
     - result: green after planner-required BlockExpr value-prelude parity
2. reopened exact blocker lane: `phase-29ck`
  - status: `active follow-up / docs-first exact front`
  - scope: `future AOT-Core MIR is locked as staged proof vocabulary now; current exact perf cut is narrowed to live-route debug bundle + semantic window proof before more array fixed-cost work, and any near-term AOT-Core follow-up stays analysis-only facts/recipe view rather than a full new IR layer`
   - exact front:
     - `docs/development/current/main/phases/phase-29ck/P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
     - `docs/development/current/main/design/stage2-aot-core-proof-vocabulary-ssot.md`
     - `docs/development/current/main/design/stage2-optimization-debug-bundle-ssot.md`
     - `docs/development/current/main/investigations/phase29ck-array-substrate-rejected-optimizations-2026-03-27.md`
  - working rule:
    - keep `llvmlite` in Stage0 keep lane only
    - keep `pure-first + compat_replay=none` as the only acceptable Stage1 mainline/perf route
    - do not introduce a distinct new IR layer in this wave
    - if the same blocker family repeats after the next narrow fast-path cut, consider only an analysis-only `AOT-Core facts/recipe view`, not a serializer-carrying full MIR layer
    - optimize the real Stage1 owner; do not drift back into keep-lane fixes
    - explicit compat-keep residue is retired; keep lane stays compat/canary evidence only
    - do not pull `vm-hako` or reduced-artifact Stage1 red paths into the current `micro kilo` / `main kilo` return
    - prefer analysis-only recipe/canonical-subset work on existing MIR over runtime smartening or backend-only tweaks
    - keep `RuntimeDataBox` facade-only; a new peel/widen is allowed only if the same blocker family repeats after the direct-path exact cut
    - accepted direct observer recipe rows must fail if standalone `slot_load_hi`, `generic_box_call`, or `hostbridge` still survives in the hot block
    - do not keep a new leaf unless the live route bundle proves MIR window -> IR -> symbol on the same artifact
    - on WSL, do not treat a single main bench delta as proof when the bundled main IR/symbol path is unchanged
    - keep rejected array-substrate attempts in the rolling ledger instead of shell history
3. close-synced boundary-retire lane: `phase-29ci`
   - status: `formal-close-synced`
   - current scope is complete for boundary retirement + caller-audit under the accepted keep set
   - explicit keep / monitor-only set:
     - `phase2044/*` thin wrapper family
     - `phase2160/*` thin wrapper families
     - `phase2170/hv1_mircall_*`
   - reopen only if:
     - a new exact caller/helper gap appears
     - or hard delete / broad internal removal explicitly resumes
4. close-synced Rune lane: `phase-29cu`
   - status: `formal-close-synced`
   - accepted narrow-scope current truth:
     - declaration-local `attrs.runes`
     - Rust direct MIR carrier
     - `.hako` source-route root-entry carrier via a real `defs[].Main.main.attrs.runes` entry
     - `.hako` compiler/mirbuilder generic function-rune carrier from `defs[].attrs.runes`
     - selected-entry `ny-llvmc` `Symbol` / `CallConv` semantics
   - future reopen only if `.hako` declaration-local full carrier parity resumes
5. close-synced mainline lane: `phase-29cj`
   - status: `formal-close-synced`
   - reopen only if a new exact disappearing leaf appears above the Rust stop-line or if deletion-prep explicitly resumes
6. close-synced by-name retire lane: `phase-29cl`
   - status: `formal-close-synced`
   - current accepted keep set is complete for the present by-name retirement scope
   - helper-side current truth:
     - `tools/hakorune_emit_mir.sh`: monitor-only
     - `tools/selfhost/selfhost_build.sh`: monitor-only
     - `tools/smokes/v2/lib/test_runner.sh`: thin loader / monitor-only
   - reopen only if:
     - a new exact `by_name` caller/helper gap appears
     - or hard delete / broad internal removal explicitly resumes
7. parked / stop-line
   - `phase-29y`: parked monitor-only
     - daily/mainline remains `llvm-exe` + `rust-vm` recovery/compat
     - `vm-hako` stays blocker-driven only; future interpreter discussion is a separate reopen
   - `phase-29ct`: stop-line reached
  - `phase-21_5` perf reopen: exact docs-first front now runs under `phase-29ck/P17` after `P16` landed
   - `phase-29cs`: parked
- runtime lane: `phase-29y / parked`. current blocker: `none`.

- compiler lane: `phase-29bq / none`（active: blocker none after JIR-PORT-08）
  - current blocker: `none`
  - lane A mirror sync helper:
    - `bash tools/selfhost/sync_lane_a_state.sh`
  - task SSOT:
    - `docs/development/current/main/design/joinir-port-task-pack-ssot.md`
    - `docs/development/current/main/design/joinir-extension-dual-route-contract-ssot.md`
  - done: `JIR-PORT-00`（Boundary Lock, docs-first）
  - done: `JIR-PORT-01`（Parity Probe）
  - done: `JIR-PORT-02`（if/merge minimal port）
  - done: `JIR-PORT-03`（loop minimal port）
  - done: `JIR-PORT-04`（PHI / Exit invariant lock）
  - done: `JIR-PORT-05`（promotion boundary lock）
  - done: `JIR-PORT-06`（monitor-only boundary lock）
  - done: `JIR-PORT-07`（expression parity seed lock: unary+compare+logic）
  - done: `JIR-PORT-08`（nested-loop BlockExpr value-prelude parity）
  - next: `none`（failure-driven steady-state）

## Unified Vocabulary

- `Stage1InputContractBox`: shared input/env contract
- `Stage1ProgramJsonMirCallerBox`: checked Program(JSON)->MIR handoff
- `Stage1ProgramJsonTextGuardBox`: Program(JSON) text guard
- `Stage1SourceMirAuthorityBox`: source authority
- `Stage1ProgramResultValidationBox`: Program JSON validation
- `Stage1MirResultValidationBox`: shared MIR validation
- `Stage1ProgramJsonCompatBox`: compat quarantine
- `nyash.plugin.invoke_by_name_i64`: compat-only plugin dispatch export

## Main Workstream

- active implementation front: `phase-29bq`
- active selfhost rule:
  - `.hako` migration stays `mirbuilder first / parser later`
  - current blocker is `none`
  - current operation mode is failure-driven / blocker-none steady-state
  - do not auto-create a broader leaf until that blocker is judged
- close-synced boundary-retire lane: `phase-29ci`
- reopened perf/mainline blocker lane: `phase-29ck`
- close-synced Rune lane: `phase-29cu`
- close-synced bootstrap-retire lane: `phase-29cj`

## Next Task

1. keep `phase-29bq` as the active selfhost lane
2. read blocker / daily operation from:
   - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
   - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
3. keep the active `29bq` reading failure-driven with `blocker=none` until the next exact blocker is captured
4. keep `phase-29ck` focused on `P18-LIVE-ROUTE-DEBUG-BUNDLE-LOCK.md`
   - immediate resume point for observer/perf work:
     - keep the fixed order `leaf-proof micro -> micro kilo -> main kilo`; resume at `micro kilo`
     - next exact shape is direct `get -> indexOf(const) -> compare -> select` as an analysis-only recipe sidecar cut
     - treat `vm-hako` as parked/frozen monitor-only while doing so
5. reopen `phase-29ci` only if a new exact boundary-retirement gap appears or hard delete resumes
6. keep `phase-29cl` formally closed unless a fresh exact `by_name` caller/helper gap reappears
7. keep `phase-29cu` / `phase-29cj` formally closed unless an exact gap reappears

## Lane Pointers

- Workstream map: `docs/development/current/main/15-Workstream-Map.md`
- Docs mirror: `docs/development/current/main/10-Now.md`
- Active selfhost lane: `docs/development/current/main/phases/phase-29bq/README.md`
- Perf/backend blocker lane: `docs/development/current/main/phases/phase-29ck/README.md`
- Boundary retire lane: `docs/development/current/main/phases/phase-29ci/README.md`
- By-name retire lane: `docs/development/current/main/phases/phase-29cl/README.md`
- Mainline phase: `docs/development/current/main/phases/phase-29cj/README.md`
- Rune lane: `docs/development/current/main/phases/phase-29cu/README.md`
- Runtime lane: `docs/development/current/main/phases/phase-29y/README.md`
- Substrate lane: `docs/development/current/main/phases/phase-29ct/README.md`
- Execution/artifact policy:
  - `docs/development/current/main/design/execution-lanes-and-axis-separation-ssot.md`
  - `docs/development/current/main/design/artifact-policy-ssot.md`

## Archive

- current-task history: `docs/development/current/main/investigations/current_task_archive_2026-03-22.md`
