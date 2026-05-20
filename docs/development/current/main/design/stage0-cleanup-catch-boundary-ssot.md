---
Status: SSOT
Date: 2026-05-21
Scope: Stage0 cleanup/catch boundary, parser acceptance, MIR-builder cleanup lowering, and JoinIR strict stop lines.
Related:
  - docs/guides/exception-handling.md
  - docs/guides/exceptions-stage3.md
  - src/parser/statements/exceptions.rs
  - src/mir/builder/control_flow/exception/try_catch.rs
  - src/mir/builder/control_flow/joinir/control_tree_capability_guard.rs
  - tools/checks/k2_wide_stage0_cleanup_catch_boundary_guard.sh
---

# Stage0 Cleanup/Catch Boundary

## Decision

Stage0 stabilizes deterministic `cleanup` behavior. It does not open a full
exception system.

Canonical Stage0 wording:

```text
cleanup:
  supported deterministic finalization boundary

catch:
  parser/AST/MIR carrier for compatibility and future exception lanes

throw:
  reserved/prohibited in the source surface

try:
  legacy compatibility spelling only; postfix cleanup/catch is canonical
```

## Responsibility Split

Parser:

- accepts postfix `cleanup` / `catch` under the Stage3 surface
- accepts legacy `try { ... } catch ... cleanup ...` only through the
  compatibility route
- rejects `finally` in favor of `cleanup`
- rejects `throw` with a freeze-style diagnostic

MIR builder:

- lowers `TryCatch` for cleanup execution
- defers protected-section `return` until cleanup has run
- keeps `return` / `throw` from cleanup rejected by default

JoinIR strict:

- does not support `TryCatch` yet
- must fail fast with a hint that points to the MIR-builder cleanup route
- must not imply that Stage0 has full exception semantics

## Stop Lines

The following remain closed until a future accepted phase opens them:

- no user-surface `throw`
- no full exception object model
- no typed catch dispatch semantics
- no backend exception ABI or stack unwinding
- no JoinIR strict lowering of `TryCatch`
- no silent fallback from unsupported `TryCatch` routes
- no `finally` spelling
- no cross-function exception/Result direct ABI change

## Compatibility Notes

Legacy `try` exists to keep old fixtures readable while they are migrated.
New docs and examples should prefer postfix cleanup:

```hako
work() cleanup {
    release()
}
```

`catch` may appear in parser tests and compatibility examples because the AST
carrier exists, but Stage0 should describe it as a reserved future boundary
unless the row explicitly owns exception semantics.

## Acceptance

The guard for this SSOT is:

```bash
bash tools/checks/k2_wide_stage0_cleanup_catch_boundary_guard.sh
```

It fixes the docs vocabulary, parser freeze tags, and JoinIR strict hints so
Stage0 cleanup cannot drift into an undocumented exception lane.
