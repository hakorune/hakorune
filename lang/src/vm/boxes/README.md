# VM Boxes — Shared Helpers (Guard)

Responsibility
- Pure helpers used by engines (op handlers, scanners, compare, JSON frag/cursor).

Allowed
- Import from `lang/src/shared/*` and other boxes/*.

Forbidden
- Engine orchestration (no main run loop, no dispatch)
- Direct I/O or plugin/ABI calls (engines should own those)

Notes
- Mini‑VM's minimal executor currently lives here (`mir_vm_min.hako`). It may
  move under `engines/mini/` later; keep it box‑pure (no I/O) until then.

