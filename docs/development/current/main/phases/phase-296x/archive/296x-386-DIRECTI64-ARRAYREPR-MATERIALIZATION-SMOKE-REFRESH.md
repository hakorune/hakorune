---
Status: Landed
Date: 2026-05-30
Scope: prove the rebased DirectI64 ArrayRepr path still preserves public ArrayBox birth, DirectArray birth, materialization snapshot, and selected-method direct lowering.
Blocker: DIRECTI64-ARRAYREPR-POST-REBASE-PERF-OWNER-REFRESH-296X-001
Related:
  - docs/development/current/main/phases/phase-296x/296x-385-DIRECTI64-ARRAYREPR-LOWERING-CONSUMER-REBASE.md
  - docs/development/current/main/phases/phase-296x/296x-384-DIRECTI64-ARRAYREPR-PRODUCER-IMPLEMENTATION.md
  - docs/development/current/main/phases/phase-296x/296x-383-DIRECTI64-ARRAYREPR-PRODUCER-CONTRACT.md
  - docs/development/current/main/phases/phase-296x/296x-381-DIRECTARRAY-FAMILY-NEXT-ORDER-TASKBOARD.md
  - src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
  - src/llvm_py/tests/test_collection_method_call.py
  - tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_materialization_smoke_refresh_guard.sh
---

# 296x-386 DirectI64 ArrayRepr Materialization Smoke Refresh

## Purpose

Smoke the rebased `ArrayRepr::DirectI64` path after the consumer rebase.

This row proves the explicit fact path still preserves the public facade and
materialization boundaries:

- `nyash.array.birth_h` stays public ArrayBox birth
- `nyash.array.direct_i64.birth_h` still produces DirectArrayI64
- explicit ArrayRepr facts still drive the selected-method direct lowering
- public materialization/snapshot smoke stays green

## Contract

```text
output_contract=direct-i64-arrayrepr-materialization-smoke-refresh-v0
input_contract=direct-i64-arrayrepr-lowering-consumer-rebase-v0
selected_boundary=direct_i64_arrayrepr_materialization_smoke_refresh
next_diagnostic=direct_i64_arrayrepr_post_rebase_perf_owner_refresh
selected_next=direct_i64_arrayrepr_post_rebase_perf_owner_refresh
public_arraybox_birth_smoke=ok
direct_array_birth_smoke=ok
direct_array_materialization_snapshot_smoke=ok
selected_method_direct_lowering_smoke=ok
public_arraybox_birth_unchanged=1
public_arraybox_handle_reinterpretation_allowed=0
silent_fallback_allowed=0
implementation_open=0
optimization_open=0
winner_claim=0
replacement_active=0
hook_installed=0
global_allocator=0
summary=ok
```

## Acceptance

- public ArrayBox birth smoke stays green
- DirectArray birth smoke stays green
- materialization snapshot smoke stays green
- selected-method direct lowering smoke stays green
- no perf claim is made in this row
- no new DirectArray member is introduced

The row is now landed because the rebased explicit fact path remains green.
The next current row is the post-rebase perf owner refresh.

## Forbidden

- no perf owner refresh in this row
- no winner claim
- no helper micro-optimization
- no public handle reinterpretation

## Commands

```bash
python3 -m unittest src/llvm_py/tests/test_direct_array_i64_constructor_lowering.py
python3 -m unittest src/llvm_py/tests/test_collection_method_call.py
```

## Guard

```bash
bash tools/checks/k2_wide_phase296x_direct_i64_arrayrepr_materialization_smoke_refresh_guard.sh
```
