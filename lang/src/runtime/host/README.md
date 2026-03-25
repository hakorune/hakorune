# Runtime Host Facade

Responsibility:
- Provide a single host call entry for `.hako` runtime/vm code.
- Own category routing (`runtime|loader|process|fs|net|time`) and payload normalization.
- Pair with `lang/src/runtime/kernel/` (kernel behavior belongs there).

Non-goals:
- No runtime/plugin semantics here.
- No multi-entry host routing from vm/runtime callers.

Contract:
- Callers must use `HostFacadeBox.call(kind, selector, payload)`.
- `runtime` category handles env/console style calls.
- `loader` category is an explicit cold dynamic lane for provider/codegen/box bridge calls.
