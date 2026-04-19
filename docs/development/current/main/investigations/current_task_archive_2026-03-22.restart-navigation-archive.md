## Quick Restart (After Reboot)

1. `git status -sb`
2. `sed -n '1,220p' CURRENT_TASK.md`
3. `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
4. `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`

## Read-First Navigation

- root pointer: `CURRENT_TASK.md`（このファイル）
- compiler map SSOT: `docs/development/current/main/design/compiler-task-map-ssot.md`
- cleanliness SSOT: `docs/development/current/main/design/compiler-cleanliness-campaign-ssot.md`
- planner gate SSOT: `docs/development/current/main/design/joinir-planner-required-gates-ssot.md`
- ai/debug contract SSOT: `docs/development/current/main/design/ai-handoff-and-debug-contract.md`

## Quick Entry: Selfhost Migration

1. `docs/development/current/main/design/selfhost-parser-mirbuilder-migration-order-ssot.md`
2. `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
3. `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
4. `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`

## Daily Commands

- fast gate:
  - `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_fast_gate_vm.sh --only bq`
- planner-required packs:
  - `bash tools/smokes/v2/profiles/integration/joinir/scan_split_planner_required_pack_vm.sh`
  - `bash tools/smokes/v2/profiles/integration/joinir/bool_predicate_accum_planner_required_pack_vm.sh`
- probe:
  - `tools/dev/direct_loop_progression_sweep.sh --profile phase29x-probe --allow-emit-fail`
- allowlist guard:
  - `tools/dev/check_loop_pattern_context_allowlist.sh` (`loop_pattern_context` is a legacy filename token; script contract is still current)

## Next Exact Steps

1. keep the stop line fixed:
  - `bash tools/selfhost/run_lane_a_daily.sh`
  - do not reopen bootstrap closure unless fresh semantic mismatch appears
2. open the `llvmlite -> .hako` daily-route pivot:
  - start from `src/runner/modes/llvm/object_emitter.rs`
  - keep target boundary fixed to `lang/src/shared/backend/llvm_backend_box.hako` + `lang/c-abi/shims/hako_aot.c`
3. keep `llvmlite` / `native_driver` compat-probe keep only while the boundary-owned daily route is tightened:
  - do not reopen by-name; that surface is already retired
  - re-evaluate backend-zero defaults only after the boundary route is thin and green

## Archive

- full historical log (2111 lines, archived 2026-03-04):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-04.md`
- compiler cleanup handoff archive (2026-03-06):
  - `docs/development/current/main/investigations/current_task_archive_2026-03-06_compiler_cleanup.md`
- policy:
  - 長文の時系列ログは以後 archive 側へ追記し、`CURRENT_TASK.md` は再起動用の薄い入口を維持する。
  - 2026-03-18 perf/exe wave:
  - route contract is fixed to `.hako -> ny-llvmc(boundary) -> C ABI`; perf lane must fail-fast on `llvmlite/native/harness`
  - `kilo_micro_substring_concat` asm-guided slice:
    - `SUBSTRING_VIEW_MATERIALIZE_MAX_BYTES: 8 -> 0`
    - short `substring_hii` results now stay `StringViewBox` until container/materialize boundary
    - targeted kernel tests stay green
  - fresh micro result:
    - `kilo_micro_substring_concat -> ny_aot_cycles=265476416, ny_aot_ms=67`
    - previous checkpoint was `ny_aot_cycles=295536812, ny_aot_ms=76`
  - fresh asm top:
    - `substring_hii 34.56%`
    - `Registry::alloc 26.13%`
    - `BoxBase::new 14.86%`
    - `string_len_from_handle 7.01%`
    - `concat3_hhh 5.37%`
    - `string_handle_from_owned` is no longer a top owner after the short-slice view shift
  - note on non-target inventory:
    - worker inventory found a likely `loop self-carry PHI` ptr-provenance loss under `src/llvm_py/**`
    - that is useful diagnostic evidence only; it is not the next edit target in this exe optimization wave
    - the next active slices stay on kernel/runtime/C-boundary owners until asm top symbols move off `substring_hii` / `Registry::alloc` / `BoxBase::new`
  - active support slice:
    - `apps/tests/mir_shape_guard/substring_concat_loop_pure_min_v1.mir.json` and `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_substring_concat_loop_min.sh` now pin the active boundary pure substring-concat-loop seed
    - the current pilot stays backend-local inside `lang/c-abi/shims/hako_llvmc_ffi.c` and does not widen runtime/VM/plugin token surfaces
  - next narrow follow-up:
    - `indexof_line` is the next supported boundary-local string-search seed; keep it on the same C-boundary pure-path seam and do not reopen `src/llvm_py/**`
    - current exact seed shape: `apps/tests/mir_shape_guard/indexof_line_pure_min_v1.mir.json` with `tools/smokes/v2/profiles/integration/apps/phase29ck_boundary_pure_indexof_line_min.sh`
