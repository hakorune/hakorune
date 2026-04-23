---
Status: Active
Date: 2026-04-23
Scope: Phase 292x task board for `.inc` thin tag cleanup.
Related:
  - docs/development/current/main/phases/phase-292x/README.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
---

# 292x-91: Task Board

## Guardrails

- [x] G1 `.inc` analysis-debt no-growth guard
  - command: `tools/checks/inc_codegen_thin_shim_guard.sh`
  - integrated into: `tools/checks/dev_gate.sh quick`
- [x] G2 route-family deletion accounting
  - when a C analyzer is retired, reduce
    `tools/checks/inc_codegen_thin_shim_debt_allowlist.tsv` in the same commit

## Completed Cards

- [x] A1 `array_rmw_window` MIR-owned route tag
  - design: `292x-93-array-rmw-window-route-card.md`
  - state: MIR metadata is emitted; `.inc` reads metadata first and treats the old C analyzer as
    temporary fallback only
  - trace proof: `[llvm-route/trace] stage=array_rmw_window result=hit reason=mir_route_metadata`

- [x] A1b delete legacy `array_rmw_window` C analyzer
  - design: `292x-98-array-rmw-c-analyzer-deletion-card.md`
  - state: `analyze_array_rmw_window_candidate` and its fallback branch are
    deleted; `.inc` keeps metadata validation / emit / skip / fail-fast for
    the migrated family

- [x] A2a `array_string_len_window` len-only MIR-owned route tag
  - design: `292x-94-array-string-len-window-route-card.md`
  - state: MIR metadata is emitted for len-only get/copy*/length windows;
    `.inc` reads metadata and the old analyzer was retired in A2d
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata`

- [x] A2b `array_string_len_window` keep-live MIR-owned route tag
  - design: `292x-95-array-string-len-keep-live-route-card.md`
  - state: `keep_get_live` mode is MIR metadata-owned; `.inc` emits
    slot-load + string-len from metadata
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata ... keep_get_live=1`

- [x] A2c `array_string_len_window` source-only direct-set route tag
  - design: `292x-96-array-string-len-source-only-route-card.md`
  - state: `source_only_insert_mid` mode is MIR metadata-owned; source-only
    insert-mid and piecewise direct-set smokes require
    `reason=mir_route_metadata`
  - trace proof: `[llvm-route/trace] stage=array_string_len_window result=hit reason=mir_route_metadata ... source_only_insert_mid=1`

- [x] A2d delete legacy `array_string_len_window` C analyzer
  - design: `292x-97-array-string-len-c-analyzer-deletion-card.md`
  - state: `analyze_array_string_len_window_candidate` and its fallback branch
    are deleted; `.inc` keeps metadata validation / emit / skip / fail-fast
    for the migrated family

- [x] A4 string concat / direct-set windows metadata-only consumption
  - design: `292x-99-string-direct-set-window-metadata-card.md`
  - state: `StringDirectSetWindowRoute` owns the piecewise source-window
    direct-set proof; `.inc` reads metadata and no longer rediscovers
    `substring + substring + substring_concat3_hhhii` shapes

- [x] A3 generic method route policy metadata
  - design: `292x-100-generic-method-route-policy-metadata-card.md`
  - state: `GenericMethodRoute` owns the first `has` route-policy leaf;
    `.inc` reads `generic_method.has` metadata before falling back to legacy
    compatibility classification
  - trace proof: `[llvm-route/trace] stage=generic_method_has_route result=hit reason=mir_route_metadata`

- [x] A6 phase docs update simplification
  - design: `292x-102-doc-update-simplification-card.md`
  - state: `292x-STATUS.toml` is the compact phase status SSOT; current
    mirrors should only carry lane/blocker/next-slice summaries instead of
    repeating every slice result

- [x] A5 exact seed ladders to function-level backend route tags
  - design: `292x-101-exact-seed-ladder-function-route-tags-card.md`
  - compact status: `292x-STATUS.toml`
  - first slice landed: `array_string_store_micro` now uses
    `metadata.exact_seed_backend_route` to select the already-proven
    `array_string_store_micro_seed_route`
  - second slice landed: `concat_const_suffix_micro` now uses the same
    function-level tag to select `concat_const_suffix_micro_seed_route`
  - third slice landed: `substring_views_only_micro` now uses the same
    function-level tag to select `substring_views_micro_seed_route`
  - fourth slice landed: `substring_concat_loop_ascii` uses the same
    function-level tag plus `selected_value` to select a concrete
    `string_kernel_plans.loop_payload` entry
  - fifth slice landed: `array_rmw_add1_leaf` gets a whole-function
    `array_rmw_add1_leaf_seed_route` layered over the existing
    `array_rmw_window_routes` inner proof
  - sixth slice landed: `292x-103` moved the Sum `variant_tag` local/copy
    seed family behind `sum_variant_tag_seed_route`
  - seventh slice landed: `292x-104` moved the Sum `variant_project`
    local/copy seed family behind `sum_variant_project_seed_route`
  - eighth slice landed: `292x-105` moved the UserBox `Point`
    local/copy scalar pair behind `userbox_local_scalar_seed_route`
  - ninth slice landed: `292x-106` moved `Flag` / `PointF` local/copy scalar
    seeds behind `userbox_local_scalar_seed_route`
  - tenth slice landed: `292x-107` moved the multi-block UserBox
    `point_add_micro` / `flag_toggle_micro` pair behind
    `userbox_loop_micro_seed_route`
  - eleventh slice landed: `292x-108` moved the `Counter.step` and
    `Point.sum` local/copy known-receiver method seeds behind
    `userbox_known_receiver_method_seed_route`
  - twelfth slice landed: `292x-109` moved the remaining `Counter.step_chain`,
    `Counter.step` micro, and `Point.sum` micro known-receiver method seeds
    behind `userbox_known_receiver_method_seed_route`
  - thirteenth slice landed: `292x-110` moved the unrelated
    `array_getset_micro` whole-function seed behind
    `array_getset_micro_seed_route`
  - final state: no exact seed matchers remain; the next bucket is
    `pure_compile_minimal_paths`

- [x] A7 pure compile minimal paths inventory
  - design: `292x-111-pure-compile-minimal-paths-inventory-card.md`
  - state: the remaining 40-line debt bucket is split into six paths:
    ret-const, compare-branch, MapBox set-size, ArrayBox push-len,
    StringBox length const-fold, and StringBox indexOf const-fold

- [x] A9 MapBox duplicate receiver unified dispatch
  - design: `292x-113-mapbox-duplicate-receiver-unified-dispatch-card.md`
  - state: Rust VM method dispatch now strips duplicate BoxRef receiver aliases
    before invoking MapBox/StringBox surface methods

- [x] A10 Hako LL stack overflow predelete
  - design: `292x-114-hako-ll-stack-overflow-predelete-card.md`
  - state: ArrayBox/MapBox clone recursion is fixed, ArrayBox fast bridge
    duplicate receiver aliases are stripped arity-aware, daily Hako LL and
    llvmlite monitor canaries are green

- [x] A8 pure compile minimal ret/branch deletion
  - design: `292x-112-pure-compile-minimal-ret-branch-deletion-card.md`
  - state: path #1/#2 deleted; allowlist pruned; guard is now 5 files / 34
    analysis-debt lines

- [x] A11 pure compile minimal Map/Array deletion
  - design: `292x-115-pure-compile-minimal-map-array-deletion-card.md`
  - state: path #4 ArrayBox push-len and path #3 MapBox set-size deleted; guard is now 5 files / 21
    analysis-debt lines

- [x] A12 pure compile minimal String const-eval decision
  - design: `292x-116-pure-compile-minimal-string-const-eval-card.md`
  - state: paths #5/#6 and `hako_llvmc_ffi_pure_compile_minimal_paths.inc`
    deleted; A13 later reduced the guard to 3 files / 4 analysis-debt lines

- [x] A13 generic pure walker residual debt
  - design: `292x-117-generic-pure-walker-residual-debt-card.md`
  - state: copy-graph helper deleted and cross-block use API tightened; guard
    is now 3 files / 4 analysis-debt lines

## Active Card

- [ ] A14 generic pure walker view extraction
  - design: `292x-118-generic-pure-walker-view-extraction-card.md`
  - state: GenericPureProgramView and GenericPureBlockView shells are landed;
    guard is now 3 files / 3 analysis-debt lines
  - next families: split cross-block use facts or declaration-needs behind the
    named view

## Done Definition

- `.inc` has no active route-legality owner for the migrated family
- MIR JSON carries the pre-decided route tag
- `.inc` validates and emits, but does not rediscover the shape
- smoke covers both behavior and route selection
- no new `.inc` analysis-debt baseline rows are introduced
