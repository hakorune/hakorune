# Stage-3 Exceptions Guide

Status: Compatibility/historical note.

The canonical current guide is:

- `docs/guides/exception-handling.md`
- Stage0 cleanup boundary SSOT:
  `docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md`

## Current Boundary

Stage0 stabilizes deterministic `cleanup`. It does not open a full exception
system.

```text
cleanup:
  supported deterministic finalization boundary

catch:
  parser/AST/MIR carrier for compatibility and future exception lanes

throw:
  reserved/prohibited in the source surface

try:
  legacy compatibility spelling only
```

New examples should use postfix cleanup/catch. Legacy `try { ... } catch ...
cleanup ...` may still parse in compatibility profiles, but it is not the
canonical language surface.

## What Still Exists

- parser support for catch parameter shapes:
  - `catch (Type e)`
  - `catch (e)`
  - `catch ()`
- parser rejection of `finally`; use `cleanup`
- parser rejection of user-surface `throw`
- MIR-builder cleanup lowering, including protected-section return deferral
- JoinIR strict fail-fast for `TryCatch`

## Stop Lines

- no user-surface `throw`
- no typed catch dispatch semantics
- no exception object model
- no backend exception ABI or stack unwinding
- no JoinIR strict `TryCatch` lowering
- no silent fallback from unsupported exception routes

## Historical Notes

Older text described a Result-mode throw/catch bridge and backend exception
plans. Treat that as historical design inventory. Any future exception lane
must re-open the design with an explicit reference doc decision and fail-fast
contracts before implementation.
