# CURRENT_TASK (root pointer)

Status: SSOT
Date: 2026-03-28
Scope: repo root の再起動入口。詳細の status / phase 進捗は `docs/development/current/main/` を正本とする。

## Purpose

- root から最短で current blocker / active lane / next fixed order に到達する。
- 本ファイルは薄い入口に保ち、長い phase 履歴や retired lane detail は phase README / design SSOT へ逃がす。

## Quick Restart Pointer

- `docs/development/current/main/05-Restart-Quick-Resume.md`
- `docs/development/current/main/15-Workstream-Map.md`
- `git status -sb`
- `tools/checks/dev_gate.sh quick`

## Current Lanes

### phase-29bq

- status: `active (failure-driven; blocker=none)`
- scope: selfhost `.hako` migration (`mirbuilder first / parser later`)
- current SSOT:
  - `docs/development/current/main/phases/phase-29bq/README.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md`
  - `docs/development/current/main/phases/phase-29bq/29bq-92-parser-handoff-checklist.md`
- next exact leaf: `none` until the next blocker is captured

### phase-29x

- status: `active compare bridge retirement / archive decisions`
- scope: shrink the remaining compare bridge / archive wrapper surfaces
- current truth:
  - `archive-home is sufficient`
  - `delete-ready is none`
  - Hako front-door `env.codegen.compile_json_path` retirement is landed
  - launcher root-first transport cut is landed
  - builder-side `compile_json_path` recognition is retired
  - Rust runtime dispatcher `compile_json_path` branches are retired
  - route-env helper `lang/src/shared/backend/backend_route_env_box.hako` is retired from code
  - remaining live set is compare bridge / archive wrapper surfaces
  - dead wrapper `lang/src/shared/host_bridge/codegen_bridge_box.hako::compile_json_path_args` is retired in this slice
- fixed order:
  1. keep `.ll` as the Rust/LLVM tool seam
  2. thin compare bridge wrapper surfaces caller-by-caller
  3. review archive/delete only after the wrapper inventory reaches zero
- current prep SSOT:
  - `docs/development/current/main/design/backend-owner-cutover-ssot.md`
  - `docs/development/current/main/design/runtime-decl-manifest-v0.toml`
  - `docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md`
  - `docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md`
- next exact leaf:
  - keep archive-later compare wrapper inventory closed and do not reopen daily ownership
  - treat delete/archive review as blocked until the remaining wrapper inventory actually reaches zero

### phase-29ck

- status: `monitor/evidence only`
- current details stay in phase29ck docs

### perf-kilo

- status: `active micro/kilo optimization`
- scope: string materialization / array store memory motion
- current SSOT:
  - `docs/development/current/main/10-Now.md`
  - `docs/development/current/main/design/kilo-meso-benchmark-ladder-ssot.md`
  - `docs/development/current/main/design/recipe-scope-effect-policy-ssot.md`
  - `docs/development/current/main/design/string-birth-sink-ssot.md`
  - `docs/development/current/main/design/transient-text-pieces-ssot.md`
  - `docs/tools/README.md`
- current leaf status:
  - normalized transient text pieces (`TextPlan` / `PiecesN`) pilot landed
  - `micro -> meso -> kilo` observation ladder landed
- current sub-slice:
  - meso first reading is fixed: `len = 37 ms`, `array_set = 69 ms`, `loopcarry = 69 ms` (`warmup=1 repeat=3`)
  - the first large jump is `len -> array_set`, not `array_set -> loopcarry`
  - landed narrow store-boundary cut: `array_set_by_index_string_handle_value` now resolves the source handle in-place inside the write closure instead of cloning a temporary `Arc` before the hot path
  - latest store-boundary recheck: `kilo_meso_substring_concat_array_set = 66 ms`, `kilo_kernel_small_hk = 708 ms` (`warmup=1 repeat=3`, `aot_status=ok`)
  - shared store-ready string materialization boundary
  - string-specific store helper for array/string hot paths
  - single handle/span resolution in `concat_const_suffix_fallback`
  - follow-up design front: `freeze.str` as the single birth sink for `concat_hs` / `insert_hsi` / `concat3_hhh`
  - attempted canonical sink re-home: moving `freeze.str` into `string_store.rs` regressed stable main (`kilo_kernel_small_hk = 834 -> 909 ms` on back-to-back checks), so keep the explicit `freeze_text_plan(...)` sink helper in `string.rs` for now
  - landed planner cleanup: const-suffix / insert recipe helpers now live in `crates/nyash_kernel/src/exports/string_plan.rs`, leaving `string.rs` as the boundary/sink site
  - next fixed order is now:
    1. shrink `BorrowedSubstringPlan` into recipe-only / boundary-only placement
    2. keep `array_set` as the consumer boundary and avoid new `set_his` splits
    3. re-run meso/main proof before any further sink-local `Registry::get` tuning
    4. keep `BoxBase::new` out unless fresh asm evidence shows the object layout itself is the limiter
  - rejected follow-up:
    - direct `concat_hs` / `concat3` copy materialization regressed stable `kilo_kernel_small_hk` (`736 -> 757 ms`) and did not improve micro; keep `TextPlan`-backed concat routes until new asm evidence appears
    - piece-preserving `insert_inline` plus store/freeze restructuring regressed stable `kilo_kernel_small_hk` to `895 ms`; do not reopen that cut without a fresh `concat_hs` / `array_set_by_index_string_handle_value` reason
    - blanket `#[inline(always)]` on host registry / hako forward string wrappers held stable main around `740 ms` and did not beat the current `736 ms` line; keep that slice reverted
    - `concat_hs` duplicate span-resolution removal plus span-resolver inlining regressed stable `kilo_kernel_small_hk` to `796 ms`; keep the existing `TextPlan::from_handle(...)` route until a new asm reason appears
    - specialized `StringBox`-only store leaf under `nyash.array.set_his` regressed the kept store-boundary line (`kilo_meso_substring_concat_array_set = 66 -> 69 ms`, `kilo_kernel_small_hk = 708 -> 791 ms`); keep the generic string-source helpers and the in-place source borrow cut only
- notes:
  - generic optimization unit is `recipe family`, not benchmark name
  - keep the generalized scope/method machinery
  - keep docs-first alignment between the transient carrier and the existing string docs
  - the current pilot uses normalized `PiecesN` only for the targeted concat/insert path; keep the carrier backend-local and non-observable
  - avoid reopening route / fallback policy until the memory-motion slice is exhausted

## Immediate Next Task

- keep rejected `concat_hs` / `insert_inline` perf cuts documented and out of the active lane
- keep the landed meso benchmark ladder as the gate for the next string cut
- next exact code sequence is fixed:
  1. shrink `BorrowedSubstringPlan` into recipe-only / boundary-only placement
  2. keep `array_set` as the consumer boundary
  3. same-artifact meso/main proof
  4. only then narrow sink-local tuning around `Registry::get`, keeping `BoxBase::new` out unless new asm evidence appears
- rejected follow-up:
  - canonicalizing `freeze.str` in `string_store.rs` regressed `kilo_kernel_small_hk` to `834 ms` and `909 ms` on back-to-back checks; keep the shared `freeze_text_plan(...)` helper local to `string.rs` until new asm evidence appears
- do not reopen `set_his` helper splitting before the `freeze.str` canonicalization wave lands
- do not reopen loop-carry shaping before the `array_set` boundary gap shrinks
- keep genericization work on `recipe / scope / effect / policy`, not on benchmark-named branches
- keep the generalized cache/scope machinery intact while tightening the hot leaf path
- do not reopen `route.rs` / compare-bridge policy unless new evidence shows route cost dominates again
- keep the stage0 llvmlite lane and stage1 root-first mainline intact

## Notes

- `compile_json_path` / `mir_json_to_object*` are no longer daily-facing.
- No new delete-ready surface is known.
