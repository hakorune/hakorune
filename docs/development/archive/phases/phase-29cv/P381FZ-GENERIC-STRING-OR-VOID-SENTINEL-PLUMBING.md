# P381FZ Generic String-Or-Void Sentinel Plumbing

Date: 2026-05-06
Scope: reduce same-module body-emitter duplication for void/null sentinel constants without changing accepted shapes.

## Problem

The generic string-or-void sentinel route already carries its public contract
through MIR-owned facts:

- `proof=typed_global_call_generic_string_or_void_sentinel`
- `return_shape=string_handle_or_null`
- `definition_owner=uniform_mir`

The remaining cleanup is body-emitter plumbing, not another owner promotion or
target-shape change. In the Stage0 same-module body emitter, the prepass and
emit paths both knew how to materialize a void/null sentinel constant as the
same ABI value:

```text
put_const(dst, 0)
set_type(dst, T_I64)
```

That duplicated the representation contract in two local branches.

## Change

Add one local helper in `hako_llvmc_ffi_same_module_function_emit.inc`:

```text
same_module_function_publish_void_sentinel_const(dst)
```

Both prepass const handling and emit const handling now call that helper after
`same_module_function_read_void_sentinel_const(ins)` accepts the constant.

## Boundary

Allowed:

- local helper extraction only
- no route acceptance change
- no new LoweringPlan proof or target shape
- no parser/body owner promotion

Not allowed:

- broad generic string analyzer refactor
- C fallback by proof name
- changing the sentinel ABI representation

## Verification

```bash
cargo test -q mir::global_call_route_plan::tests::void_sentinel
cargo test -q runner::mir_json_emit::tests::global_call_routes::void_sentinel
bash tools/build_hako_llvmc_ffi.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/stage0_shape_inventory_guard.sh
git diff --check
```
