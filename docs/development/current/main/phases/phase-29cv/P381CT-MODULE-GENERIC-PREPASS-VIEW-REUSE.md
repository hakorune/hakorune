# P381CT Module Generic Prepass View Reuse

Date: 2026-05-05
Scope: reuse LoweringPlan views inside the module-generic prepass.

## Context

The module-generic prepass read the same lowering-plan entry repeatedly while
walking a single call instruction. Generic-method predicates, extern-call
predicates, and global-call predicates each re-read their view before checking
the next branch.

This was behaviorally redundant and made the prepass harder to audit because
the data source looked like many separate entry points.

## Change

For each call instruction, the prepass now reads the available views once:

- generic-method view
- extern-call view
- global-call view

The existing predicate chain then uses `has_*_view` booleans and the cached view
structures. Route-specific predicates and result-origin logic are unchanged.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381ct_prepass_view.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381ct_prepass_view.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ct_prepass_view.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381ct_emit_program.out \
  /tmp/hakorune_p381ct_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381ct_emit_program.out \
  /tmp/hakorune_p381ct_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381ct_prepass_view.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381ct_emit_mir.out \
  /tmp/hakorune_p381ct_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381ct_emit_mir.out \
  /tmp/hakorune_p381ct_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The prepass now has one lowering-plan read point per view type for each call
instruction. The predicate chain remains explicit, but the entry ownership is
clearer.
