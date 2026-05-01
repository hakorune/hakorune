---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: consume LoweringPlan v0 for MapLen DirectAbi.
Related:
  - docs/development/current/main/design/lowering-plan-json-v0-ssot.md
  - docs/development/current/main/phases/phase-29cv/P74-LOWERING-PLAN-EMIT-KIND-VIEW-SSOT.md
  - lang/c-abi/shims/hako_llvmc_ffi_mir_call_need_policy.inc
  - apps/tests/mir_shape_guard/lowering_plan_map_len_directabi_min_v1.mir.json
---

# P75 LoweringPlan MapLen DirectAbi Consume

## Goal

Add the next single LoweringPlan v0 accepted shape: `MapLen` lowered through the
direct ABI helper `nyash.map.entry_count_i64`.

This card follows P73/P74. The plan route and emit-kind consumers already accept
`MapLen`; the remaining plan-only gap is the need prepass declaration path.

## Decision

- Add a plan-only `MapLen` fixture with no `generic_method_routes`.
- Map a valid `LoweringPlan` view for `generic_method.len` / `MapLen` /
  `map_entry_count` to the existing `MAP_SIZE` need kind.
- Keep legacy route metadata as migration fallback.

## Non-goals

- no `ArrayLen` / `StringLen` plan-only fixture
- no len surface widening
- no helper-symbol inference from raw MIR
- no perf keeper claim

## Acceptance

```bash
bash tools/build_hako_llvmc_ffi.sh
NYASH_LLVM_ROUTE_TRACE=1 target/release/ny-llvmc \
  --in apps/tests/mir_shape_guard/lowering_plan_map_len_directabi_min_v1.mir.json \
  --out /tmp/p75_lowering_plan_map_len_directabi.o
NYASH_LLVM_ROUTE_TRACE=1 bash tools/smokes/v2/profiles/integration/phase29x/derust/phase29x_backend_owner_daily_runtime_data_map_size_min.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
