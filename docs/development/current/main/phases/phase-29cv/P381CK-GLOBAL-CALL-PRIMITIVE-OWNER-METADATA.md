# P381CK Global Call Primitive Owner Metadata

Date: 2026-05-05
Scope: consume `definition_owner` for active primitive global-call definition ownership.

## Context

P381CH made `definition_owner` the MIR-owned same-module definition-set fact.
P381CI removed unused retired-capsule direct-view predicates, but the active
numeric leaf and generic-i64 C predicates still reconstructed ownership from
proof/target-shape/return-shape strings.

Those predicates were not capsule-retirement blockers, but they were still
duplicate ownership truth in C.

## Change

Replaced the remaining active primitive ownership checks with metadata helpers:

- `lowering_plan_global_call_view_uses_leaf_i64_definition`
- `lowering_plan_global_call_view_uses_generic_i64_or_leaf_definition`

Then rewired:

- transitive definition-set planning
- direct global-call emission availability checks
- module-generic prepass type marking for leaf calls

The old `is_direct_leaf_i64` and `is_direct_generic_i64` proof/shape predicates
were deleted.

Void-sentinel no-dst handling remains on its return-contract helper because it
is a call-result policy, not a definition-owner policy.

## Verification

Commands:

```bash
rg -n "lowering_plan_global_call_view_is_direct_(leaf_i64|generic_i64)" \
  lang/c-abi/shims
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381ck_stage1_cli_env.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381ch_stage1_cli_env_rust.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381ck_stage1_cli_env.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ck_stage1_cli_env.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381ck_emit_program.out \
  /tmp/hakorune_p381ck_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381ck_emit_program.out \
  /tmp/hakorune_p381ck_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ck_stage1_cli_env.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381ck_emit_mir.out \
  /tmp/hakorune_p381ck_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381ck_emit_mir.out \
  /tmp/hakorune_p381ck_emit_mir.err
```

Observed:

- no remaining `is_direct_leaf_i64` / `is_direct_generic_i64` C predicates
- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Definition ownership for both retired capsules and active primitive global-call
paths is now read from MIR-owned `definition_owner` metadata. C retains only
the narrow call-result helper needed for void-sentinel no-result emission.
