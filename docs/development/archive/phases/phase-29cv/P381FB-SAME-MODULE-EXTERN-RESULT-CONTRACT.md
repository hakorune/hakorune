# P381FB Same-Module Extern Result Contract

Date: 2026-05-06
Scope: replace source-owner-specific same-module extern prepass branches with MIR-owned return-contract helpers.

## Context

After P381FA, `mir_call_need_policy` and `mir_call_shell` were already
table-driven, but the same-module uniform function emitter still had one narrow
source-owner ladder:

```text
same_module_function_prepass_extern_call_view(...)
```

It recognized these Stage1/parser-adjacent extern routes by name:

- `stage1_emit_program_json`
- `stage1_emit_mir_from_source`
- `stage1_emit_mir_from_program_json`

along with `env.get`, `env.set`, and hostbridge trap handling.

That was still a BoxShape leak: the prepass was learning result publication from
specific extern owners instead of consuming the MIR-owned extern return contract.

## Change

Added LoweringPlan metadata helpers in:

```text
lang/c-abi/shims/hako_llvmc_ffi_lowering_plan_metadata.inc
```

- `lowering_plan_extern_call_view_result_origin_kind(...)`
- `lowering_plan_extern_call_view_returns_scalar_i64(...)`

`same_module_function_prepass_extern_call_view(...)` now consumes those helpers
instead of checking per-owner extern names.

The resulting contract is:

- `return_shape=string_handle|string_handle_or_null` and
  `value_demand=runtime_i64_or_handle` publish string origin
- `return_shape=scalar_i64` and `value_demand=runtime_i64` publish plain i64 type

No behavior changed. This only moves publication truth to MIR metadata.

## Verification

Commands:

```bash
bash tools/build_hako_llvmc_ffi.sh
cargo test -q parser_program_json
cargo test -q stage1_emit_program_json_extern_route
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

The selected parser handoff body no longer needs Stage0 name-based extern result
publication for the Stage1 emit helpers. Remaining parser handoff work is now
the actual body-lowering contract, not a repeated prepass owner list.
