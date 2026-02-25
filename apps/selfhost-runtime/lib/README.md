# Selfhost Runtime Lib (SSOT)

Purpose: keep selfhost-only utilities in one place so the compiler team can
extend the selfhost pipeline without adding new language features.

Scope:
- Selfhost runtime helpers only (no language spec changes).
- Prefer utility boxes over syntax changes.
- ASCII-only and minimal dependencies.

Files:
- string_cursor.hako: stateful cursor for string scanning (peek/next/skip_ws/read_digits).
- ast_extractors.hako: structural extraction helpers for JSON-like AST nodes.

Entry:
- apps/selfhost-runtime/selfhost_prelude.hako (prefer `using apps.selfhost_runtime.prelude`)

Notes:
- Add new helpers here first; promote to apps/lib or apps/std only after
  repeated use and a documented decision.
