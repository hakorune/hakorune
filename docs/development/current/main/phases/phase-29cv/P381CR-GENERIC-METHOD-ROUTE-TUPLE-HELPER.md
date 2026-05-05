# P381CR Generic Method Route Tuple Helper

Date: 2026-05-05
Scope: share generic-method LoweringPlan route tuple matching across Stage0 C shims.

## Context

Stage0 generic-method consumers repeated the same route tuple comparison in
separate files:

- route id
- core op
- route kind
- lowering tier

That tuple is the common contract for route policy, emit-kind selection, need
classification, and set-route selection. Keeping the comparison duplicated made
future route vocabulary edits depend on several equivalent-but-separate
matchers.

## Change

Added `lowering_plan_generic_method_view_matches_route_tuple` in
`hako_llvmc_ffi_lowering_plan_metadata.inc`.

The following consumers now call the shared helper:

- `hako_llvmc_ffi_mir_call_route_policy.inc`
- `hako_llvmc_ffi_generic_method_match.inc`
- `hako_llvmc_ffi_mir_call_need_policy.inc`

Exact consumers still keep their stricter local checks for `symbol`,
`route_proof`, and value shape after the shared tuple match.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cr_generic_tuple.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cr_generic_tuple.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cr_generic_tuple.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cr_emit_program.out \
  /tmp/hakorune_p381cr_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cr_emit_program.out \
  /tmp/hakorune_p381cr_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cr_generic_tuple.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cr_emit_mir.out \
  /tmp/hakorune_p381cr_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cr_emit_mir.out \
  /tmp/hakorune_p381cr_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

The generic-method route tuple comparison now has one owner. Route-specific
consumers keep only their extra policy checks.
