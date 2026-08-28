# handlers/calls — Call Handling Layer

Purpose: isolate call handling by callee kind for the MIR interpreter.

- In scope: routing `handle_call`, per‑callee executors, and terminal rejection
  of missing `Callee` inputs.
- Out of scope: arithmetic/box handlers, memory ops, extern provider registry.

Do not import sibling handler modules directly from here (keep boundaries tight).
Use `super::*` only and call through the interpreter methods.

Files
- `mod.rs`: entry point and callee routing
- `global.rs`: structural global targets (Callee::Global) and the separate
  legacy Method(None) compatibility entry point
- `method.rs`: instance/static method calls (Callee::Method)
- `externs.rs`: extern calls (Callee::Extern)

Removal status (Phase 2 complete)
- Unified callee path is the only selected execution path. The old
  `None -> func` register load and module by-name lookup are retired in
  R5b-B0; `callee=None` fails before either lookup with
  `call-missing-callee: typed Callee required`.
- `func` and `Option<Callee>` remain in the instruction shape only until the
  R6 atomic schema cutover; they are not runtime target authority here.

Extern SSOT
- `externs.rs` is the runtime SSOT for provider dispatch. Provider names must
  arrive as `Callee::Extern`; the structural Global consumer does not repair a
  provider call from text.
- The separate compatibility entry point may normalize an arity suffix before
  dispatch while its Method(None) route remains under retirement.

Structural Global consumer
- `Callee::Global` is consumed by its `CanonicalGlobalTargetV1` variant. The
  supported `Builtin(Print)` route requires exactly `print/1`; same-module
  targets use their sealed arity for one-way function-table lookup. Missing or
  unsupported structural entries fail without name parsing, arity completion,
  prefix matching, or retry. Provider routing belongs to `Callee::Extern`.

Layer Guard
- Scope: call routing only (Global/Method/Extern and missing-Callee
  isolation). Do not import provider registries or runtime plugins directly
  from here.
- Use helpers under `super::super::utils::*` for shared concerns (e.g., `normalize_arity_suffix`).
