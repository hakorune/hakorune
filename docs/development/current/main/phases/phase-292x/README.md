---
Status: Active
Date: 2026-04-22
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
  - docs/development/current/main/phases/phase-292x/292x-91-task-board.md
  - docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md
  - docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md
  - docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md
---

# Phase 292x: `.inc` codegen thin tag cleanup

- Status: Active
- Date: 2026-04-22
- Purpose: `.inc` を MIR JSON の形解析 owner から外し、MIR-owned
  pre-decided tag を読むだけの boundary glue に寄せる。
- First implementation target: `array_rmw_window` (landed)
- Landed second target: `array_string_len_window` len-only route
- Landed third target: `array_string_len_window` keep-live source reuse
- Landed fourth target: `array_string_len_window` source-only direct-set reuse
- Next implementation target: delete legacy `array_string_len_window` C analyzer
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
2. `docs/development/current/main/phases/phase-292x/292x-91-task-board.md`
3. `docs/development/current/main/phases/phase-292x/292x-92-inc-codegen-analysis-debt-ledger.md`
4. `docs/development/current/main/phases/phase-292x/292x-93-array-rmw-window-route-card.md`
5. `docs/development/current/main/phases/phase-292x/292x-94-array-string-len-window-route-card.md`
6. `docs/development/current/main/phases/phase-292x/292x-95-array-string-len-keep-live-route-card.md`
7. `docs/development/current/main/phases/phase-292x/292x-96-array-string-len-source-only-route-card.md`
8. `docs/development/current/main/phases/phase-292x/292x-97-array-string-len-c-analyzer-deletion-card.md`
9. `docs/development/current/main/investigations/phase137x-inc-codegen-thin-tag-inventory-2026-04-22.md`

## Current Rule

- docs-first before code
- route legality belongs to MIR metadata, not `.inc`
- `.inc` may only consume tags, validate fields, emit, skip, or fail fast
- no benchmark-name or helper-name semantic ownership in C
- no new `.inc` raw MIR scan debt beyond the no-growth baseline
- old C analyzers are fallback-only during migration and must be removed family by family

## Implementation State

Landed guardrail:

- `tools/checks/inc_codegen_thin_shim_guard.sh`
- baseline: 30 `.inc` files, 324 analysis-debt lines
- `tools/checks/dev_gate.sh quick` runs the guard

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

Next open card:

```text
array_string_len_window C analyzer deletion
  -> delete analyze_array_string_len_window_candidate
  -> keep only metadata validation / emit / skip / fail-fast
```
