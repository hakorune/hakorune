# P381CQ MIR Call Extern Need Contract Cleanup

Date: 2026-05-05
Scope: centralize the common extern-call need-policy contract in Stage0.

## Context

P381CP centralized the extern-call LoweringPlan validators in the MIR call
shell. The adjacent need policy still repeated the same shared extern route
contract in each table row:

- `emit_kind=runtime_call`
- `proof=extern_registry`
- `tier=ColdRuntime`

Those are not route-specific facts. Keeping them in every row widened the
surface that future route additions must keep in sync.

## Change

`LoweringPlanExternNeedRule` now carries only route-specific vocabulary:

- route id
- core op
- route kind
- runtime symbol
- need kind

The shared matcher checks the common extern-call contract once, including
`route_proof=extern_registry`. This keeps the need table focused on the
route-to-need mapping and makes the extern registry proof contract explicit at
the policy boundary.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/ny-llvmc \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit obj \
  --out /tmp/hakorune_p381cq_extern_need.o
bash tools/ny_mir_builder.sh \
  --in /tmp/hakorune_p381cp_stage1_cli_env_probe.mir.json \
  --emit exe \
  -o /tmp/hakorune_p381cq_extern_need.exe
```

Runtime sanity:

```bash
stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cq_extern_need.exe emit-program \
  apps/tests/mir-branch-ret/main.hako "$source_text" 1 0 \
  /tmp/hakorune_p381cq_emit_program.out \
  /tmp/hakorune_p381cq_emit_program.err
stage1_contract_validate_emit_output \
  emit-program /tmp/hakorune_p381cq_emit_program.out \
  /tmp/hakorune_p381cq_emit_program.err

stage1_contract_run_bin_with_env \
  /tmp/hakorune_p381cq_extern_need.exe emit-mir \
  apps/tests/mir-branch-ret/main.hako "$source_text" 0 1 \
  /tmp/hakorune_p381cq_emit_mir.out \
  /tmp/hakorune_p381cq_emit_mir.err
stage1_contract_validate_emit_output \
  emit-mir /tmp/hakorune_p381cq_emit_mir.out \
  /tmp/hakorune_p381cq_emit_mir.err
```

Observed:

- C shim build passed
- OBJ generation passed
- EXE generation passed
- `emit-program` env run: rc=0, validation rc=0
- `emit-mir` env run: rc=0, validation rc=0

## Result

Extern-call need-policy rows now describe only the route-specific mapping. The
common runtime-call extern registry contract lives in one matcher.
