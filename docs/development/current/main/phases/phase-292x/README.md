---
Status: Active
Date: 2026-04-23
Scope: `.inc` codegen を pre-decided tag consumer だけに薄くする phase front。
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/05-Restart-Quick-Resume.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/15-Workstream-Map.md
  - docs/development/current/main/design/stage2-hako-owner-vs-inc-thin-shim-ssot.md
  - docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md
  - docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md
  - docs/development/current/main/phases/phase-292x/292x-STATUS.toml
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md
  - docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-97-array-string-len-c-analyzer-deletion-card.md
  - docs/development/current/main/phases/phase-292x/292x-98-array-rmw-c-analyzer-deletion-card.md
  - docs/development/current/main/phases/phase-292x/292x-99-string-direct-set-window-metadata-card.md
  - docs/development/current/main/phases/phase-292x/292x-100-generic-method-route-policy-metadata-card.md
  - docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md
  - docs/development/current/main/phases/phase-292x/292x-102-doc-update-simplification-card.md
  - docs/development/current/main/phases/phase-292x/292x-103-sum-variant-tag-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-104-sum-variant-project-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-105-userbox-point-local-scalar-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-106-userbox-flag-pointf-local-scalar-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-107-userbox-loop-micro-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-108-userbox-known-receiver-local-method-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-109-userbox-known-receiver-chain-micro-method-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-110-array-getset-micro-seed-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-111-pure-compile-minimal-paths-inventory-card.md
  - docs/development/current/main/phases/phase-292x/292x-112-pure-compile-minimal-ret-branch-deletion-card.md
  - docs/development/current/main/phases/phase-292x/292x-113-mapbox-duplicate-receiver-unified-dispatch-card.md
  - docs/development/current/main/phases/phase-292x/292x-114-hako-ll-stack-overflow-predelete-card.md
  - docs/development/current/main/phases/phase-292x/292x-115-pure-compile-minimal-map-array-deletion-card.md
---

# Phase 292x: `.inc` codegen thin tag cleanup

- Status: Active
- Date: 2026-04-23
- Purpose: `.inc` を MIR JSON の形解析 owner から外し、MIR-owned
  pre-decided tag を読むだけの boundary glue に寄せる。
- First implementation target: `array_rmw_window` (landed)
- Landed second target: `array_string_len_window` len-only route
- Landed third target: `array_string_len_window` keep-live source reuse
- Landed fourth target: `array_string_len_window` source-only direct-set reuse
- Landed fifth target: delete legacy `array_string_len_window` C analyzer
- Landed sixth target: delete legacy `array_rmw_window` C analyzer
- Landed seventh target: string concat / direct-set windows metadata-only
- Landed eighth target: `generic_method.has` route policy metadata
- Landed exact-seed slice: `array_getset_micro` route metadata
- Landed predelete fix: `mapbox_duplicate_receiver_unified_dispatch`
- Landed predelete fix: `hako_ll_provider_stack_overflow_predelete`
- Landed deletion: `pure_compile_minimal_paths` path #1/#2
- Landed deletion: `pure_compile_minimal_paths` path #4 Array
- Landed deletion: `pure_compile_minimal_paths` path #3 Map
- Landed deletion: `pure_compile_minimal_paths` String const-eval
- Landed deletion: string loop seed copy-graph helper
- Landed tightening: generic pure cross-block use lookup API
- Next cleanup target: live generic pure walker residual debt
- Status SSOT: `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`
- Sibling guardrail:
  - `docs/development/current/main/phases/phase-137x/README.md`
  - phase-137x remains observe-only unless this cleanup reopens a real app/perf blocker.

## Decision

`.inc` is glue, not planner.

```text
MIR metadata
  -> route_id / proof / block / instruction_index / skip_instruction_indices / operands

.inc boundary
  -> validate required fields
  -> emit the selected helper call
  -> mark covered instructions skipped
  -> fail fast on inconsistent metadata
```

`.inc` must not grow new raw MIR analysis. Legacy C-side analyzers may remain
only as temporary fallback while each family gets a MIR-owned route tag.

## Reading Order

1. `docs/development/current/main/phases/phase-292x/292x-90-inc-codegen-thin-tag-design-brief.md`
2. `docs/development/current/main/phases/phase-292x/292x-STATUS.toml`
3. `docs/development/current/main/phases/phase-292x/292x-91-task-board.md`
4. `docs/development/current/main/phases/phase-292x/292x-101-exact-seed-ladder-function-route-tags-card.md`
5. `docs/development/current/main/phases/phase-292x/292x-103-sum-variant-tag-seed-route-card.md`
6. `docs/development/current/main/phases/phase-292x/292x-104-sum-variant-project-seed-route-card.md`
7. `docs/development/current/main/phases/phase-292x/292x-105-userbox-point-local-scalar-seed-route-card.md`
8. `docs/development/current/main/phases/phase-292x/292x-106-userbox-flag-pointf-local-scalar-seed-route-card.md`
9. `docs/development/current/main/phases/phase-292x/292x-107-userbox-loop-micro-seed-route-card.md`
10. `docs/development/current/main/phases/phase-292x/292x-108-userbox-known-receiver-local-method-seed-route-card.md`
11. `docs/development/current/main/phases/phase-292x/292x-109-userbox-known-receiver-chain-micro-method-seed-route-card.md`
12. `docs/development/current/main/phases/phase-292x/292x-110-array-getset-micro-seed-route-card.md`
13. `docs/development/current/main/phases/phase-292x/292x-115-pure-compile-minimal-map-array-deletion-card.md`
14. `docs/development/current/main/phases/phase-292x/292x-112-pure-compile-minimal-ret-branch-deletion-card.md`
15. `docs/development/current/main/phases/phase-292x/292x-114-hako-ll-stack-overflow-predelete-card.md`
16. `docs/development/current/main/phases/phase-292x/292x-113-mapbox-duplicate-receiver-unified-dispatch-card.md`
17. `docs/development/current/main/phases/phase-292x/292x-111-pure-compile-minimal-paths-inventory-card.md`
18. `docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md`
19. `docs/development/current/main/phases/phase-292x/292x-102-doc-update-simplification-card.md`
20. Historical per-slice cards as needed:
   - `docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-97-array-string-len-c-analyzer-deletion-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-98-array-rmw-c-analyzer-deletion-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-99-string-direct-set-window-metadata-card.md`
   - `docs/development/current/main/phases/phase-292x/292x-100-generic-method-route-policy-metadata-card.md`
20. `docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md`

## Current Rule

- docs-first before code
- phase status first before mirror updates
- route legality belongs to MIR metadata, not `.inc`
- `.inc` may only consume tags, validate fields, emit, skip, or fail fast
- no benchmark-name or helper-name semantic ownership in C
- no new `.inc` raw MIR scan debt beyond the no-growth baseline
- old C analyzers are fallback-only during migration and must be removed family by family
- current mirrors should point to `292x-STATUS.toml`; do not duplicate full
  per-slice ledgers outside the active card

## Implementation State

Landed guardrail:

- `tools/checks/inc_codegen_thin_shim_guard.sh`
- baseline: 5 `.inc` files, 28 analysis-debt lines
- current inventory: 73 `.inc` files, 16,079 `.inc` lines
- `tools/checks/dev_gate.sh quick` runs the guard
- compact phase status lives in `292x-STATUS.toml`

Landed first card:

```text
array_rmw_window
  -> MIR-owned route metadata
  -> MIR JSON route tag
  -> .inc metadata-first lowering
  -> legacy analyzer fallback only
  -> route trace locks `mir_route_metadata`
```

Landed second card:

```text
array_string_len_window len-only
  -> MIR-owned route metadata
  -> MIR JSON route tag
  -> .inc metadata-first lowering
  -> legacy analyzer deletion remains a follow-up cleanup
```

Landed third card:

```text
array_string_len_window keep-live source reuse
  -> keep_get_live metadata route
  -> .inc emits slot_load + string_len from metadata
```

Landed fourth card:

```text
array_string_len_window source-only direct-set reuse
  -> source_only_insert_mid metadata route
  -> piecewise concat direct-set route tag
  -> source-only smokes require MIR metadata route
```

Landed fifth card:

```text
array_string_len_window C analyzer deletion
  -> delete analyze_array_string_len_window_candidate
  -> keep only metadata validation / emit / skip / fail-fast
```

Landed sixth card:

```text
array_rmw_window C analyzer deletion
  -> delete analyze_array_rmw_window_candidate
  -> keep only metadata validation / emit / skip / fail-fast
```

Landed seventh card:

```text
string concat / direct-set windows metadata-only
  -> move piecewise source-window matching to MIR metadata
  -> keep .inc substring/direct-set lowering as route-tag consumer glue
```

Landed eighth card:

```text
generic_method.has route policy metadata
  -> move has helper selection to MIR metadata
  -> keep .inc has lowering as metadata validation / helper emission glue
```

Completed exact-seed route card:

```text
exact seed ladders to function-level backend route tags
  -> first slice landed for array_string_store_micro
  -> second slice landed for concat_const_suffix_micro
  -> third slice landed for substring_views_only_micro
  -> fourth slice landed for substring_concat_loop_ascii with selected plan value
  -> fifth slice landed for array_rmw_add1_leaf whole-function metadata
  -> sixth slice landed for Sum variant_tag local/copy metadata
  -> seventh slice landed for Sum variant_project local/copy metadata
  -> eighth slice landed for UserBox Point local/copy scalar metadata
  -> ninth slice landed for UserBox Flag/PointF local/copy scalar metadata
  -> tenth slice landed for UserBox loop micro metadata
  -> eleventh slice landed for UserBox known-receiver local/copy method metadata
  -> twelfth slice landed for UserBox known-receiver chain/micro method metadata
  -> thirteenth slice landed for array_getset_micro metadata
  -> no hako_llvmc_match_*seed definitions remain
  -> pure_compile_minimal_paths inventory card is documented
  -> MapBox duplicate receiver predelete fix is landed
  -> Hako LL/provider stack overflow predelete fix is landed
  -> minimal paths #1/#2 deletion is landed
  -> minimal path #4 Array deletion is landed
  -> path #3 Map deletion is blocked on generic/Hako LL MapBox set-size ownership
```
