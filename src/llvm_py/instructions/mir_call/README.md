# mir_call module layout

This directory is the modular lowering route for `mir_call` in LLVM Python backend.

## Entry

- Dispatcher: `src/llvm_py/instructions/mir_call/__init__.py`
- Callee routes:
  - `global_call.py`
  - `method_call.py`
  - `constructor_call.py`
  - `closure_call.py`
  - `value_call.py`
  - `extern_call.py`

## Shared SSOT

- `arg_resolver.py`
  - `resolve_call_arg(...)` is the single argument-resolution policy for call routes.
  - `make_call_arg_resolver(...)` binds route-local `_resolve_arg` closures to the same policy.
  - Contract:
    1. local vmap first (same-block SSA),
    2. `resolve_i64_strict(..., hot_scope="call")` fallback,
    3. no throw from helper (unresolved -> `None`).

- `string_console_method_call.py`
  - owns the shared `substring/indexOf/lastIndexOf/log` route order.
  - `method_call.py` and `mir_call_legacy.py` consume it so string/console lowering
    does not drift while length/size specialization remains owner-local to the modern route.

- `direct_box_method.py`
  - owns known-box direct lowering only.
  - MirBuilder direct routes are delegated to `instructions/mir_builder_direct.py`.
  - direct miss now fail-fasts; this directory no longer keeps a Python-side
    `by_name` compat emitter.

- `builders/closure_split_contract.py`
  - owns closure capture classification only.
  - route-local closure lowering reads this contract so empty-env vs capture-env
    classification does not drift while env scalarization and thin-entry
    specialization stay deferred.

This keeps call hot-trace counters (`resolve_*_call`) consistent across routes.

## Print marshalling utility

- `print_marshal.py` provides `PrintArgMarshallerBox`.
- It is a utility for print argument conversion (`box.from_i64 -> to_i8p_h` path).
- It accepts optional resolver maps (`vmap/preds/block_end_values/bb_map`) so callers can
  stay on the same strict call-resolution contract as route modules.

## Legacy relation

- `src/llvm_py/instructions/mir_call_legacy.py` remains as compatibility copy.
- Legacy `_resolve_arg` paths are also created via `make_call_arg_resolver(...)` so route-local drift is minimized.
