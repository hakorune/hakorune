ring0 — Kernel Layer (Facade)

Purpose
- Document the kernel responsibilities (runner/VM/host-bridge/fail-fast).
- Act as a conceptual anchor without moving existing files immediately.

Policy
- No dependencies on ring2 (plugins).
- Keep logic minimal; orchestration and fail-fast only.

Notes
- Physical moves are deferred to avoid large diffs. ring0 exists as a documentation and guard point first.

