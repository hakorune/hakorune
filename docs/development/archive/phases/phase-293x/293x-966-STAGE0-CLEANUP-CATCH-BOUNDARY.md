---
Status: Landed
Date: 2026-05-21
Row: STAGE0-CLEANUP-CATCH-BOUNDARY
Validation: docs/static guard + parser Stage3 tests
Related:
  - docs/development/current/main/design/stage0-cleanup-catch-boundary-ssot.md
  - docs/guides/exception-handling.md
  - docs/guides/exceptions-stage3.md
  - tools/checks/k2_wide_stage0_cleanup_catch_boundary_guard.sh
---

# STAGE0-CLEANUP-CATCH-BOUNDARY

## Purpose

Cut a sidecar cleanup phase for the Stage0 exception surface so the active
MIMAP lane does not inherit an ambiguous half-exception story.

## Decision

Stage0 owns deterministic `cleanup`. `catch` is a parser/AST/MIR carrier for
compatibility and future lanes. `throw` remains reserved/prohibited. Legacy
`try` remains compatibility-only; postfix cleanup/catch is canonical.

## Scope

- Add the Stage0 cleanup/catch boundary SSOT.
- Make the public exception guides state the Stage0 boundary explicitly.
- Update JoinIR strict hints so unsupported `TryCatch` points to the
  MIR-builder cleanup route instead of implying full exception support.
- Add a static guard for docs/source drift.

## Stop Lines

- no user-surface `throw`
- no full exception object model
- no typed catch dispatch semantics
- no backend exception ABI
- no JoinIR strict `TryCatch` lowering
- no provider/backend behavior
- no MIMAP current blocker change

## Validation

```bash
bash tools/checks/k2_wide_stage0_cleanup_catch_boundary_guard.sh
cargo test --test parser_stage3
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
