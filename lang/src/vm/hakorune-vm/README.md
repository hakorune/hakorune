# Hakorune VM (nyvm) — Engine Guard

Responsibility
- Engine orchestration and instruction dispatch for nyvm.
- Control‑flow handlers (branch/jump/phi) and value routing.

Allowed Imports
- `lang/src/vm/boxes/*` (helpers)
- `lang/src/shared/*` (string ops, JSON scan, etc.)

Forbidden
- Parser/Resolver/Emitter pipelines
- Direct plugin/ABI wiring (use kernel/extern facades)

Migration Note
- Final layout will be `lang/src/vm/engines/hakorune/` with module aliases.

