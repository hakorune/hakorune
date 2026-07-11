# P381GK MIR Call Extern Emit Rule SSOT

Date: 2026-05-06
Scope: remove duplicated extern emit route rows from the Stage0 MIR call shell.

## Context

P381CP centralized the common extern-call LoweringPlan view contract, but
`hako_llvmc_ffi_mir_call_shell.inc` still kept each full
`MirCallExternEmitRule` row twice:

- once in the active emit rule table
- once in each named compatibility validator

The named validators are public local seams and must stay stable, but the row
payload should have one owner.

## Change

`lang/c-abi/shims/hako_llvmc_ffi_mir_call_shell.inc` now exposes the extern
emit table through one local table helper. Both active emit lookup and the six
named validators read from that table.

Kept stable validator names:

- `lowering_plan_env_get_view_is_valid`
- `lowering_plan_env_set_view_is_valid`
- `lowering_plan_hostbridge_extern_invoke_view_is_valid`
- `lowering_plan_stage1_emit_program_json_view_is_valid`
- `lowering_plan_stage1_emit_mir_from_source_view_is_valid`
- `lowering_plan_stage1_emit_mir_from_program_json_view_is_valid`

The validators still require their own route id, so a named validator cannot
accept a different extern route by accident.

## Result

Extern emit policy now has one local SSOT for:

- route id
- core op
- route kind
- runtime symbol
- return shape
- value demand
- arity
- value/result side conditions
- trace consumer
- trap/call behavior

This is BoxShape cleanup only. It does not add a Stage0 accepted shape or widen
any route.

## Validation

```bash
bash tools/build_hako_llvmc_ffi.sh
target/release/hakorune --emit-mir-json /tmp/hakorune_p381gk_stage1_cli_env.mir.json lang/src/runner/stage1_cli_env.hako
target/release/ny-llvmc --in apps/tests/mir_shape_guard/lowering_plan_env_get_coldruntime_min_v1.mir.json --emit obj --out /tmp/hakorune_p381gk_env_get.o
bash tools/checks/stage0_shape_inventory_guard.sh
```

Hygiene:

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Boundary Probe

The full current `stage1_cli_env.hako` MIR still stops in the backend recipe
when passed directly to `ny-llvmc` / `tools/ny_mir_builder.sh`:

```text
unsupported pure shape for current backend recipe
```

That is the current backend recipe boundary, not part of this helper-dedup
contract.
