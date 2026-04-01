---
Status: SSOT
Decision: provisional
Date: 2026-04-01
Scope: investigate delete readiness for the remaining explicit legacy/compat callers rooted at `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`, including the compiled-stage1 surrogate caller.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/phases/phase-29x/README.md
  - docs/development/current/main/phases/phase-29x/29x-96-backend-owner-legacy-ledger-ssot.md
  - docs/development/current/main/phases/phase-29x/29x-97-compare-bridge-retirement-prep-ssot.md
  - docs/development/current/main/design/backend-owner-cutover-ssot.md
---

# 29x-98 Legacy Route Retirement Investigation

## Rule

- delete-ready remains none until the caller inventory reaches zero.
- no new daily caller may be added to `emit_object_from_mir_json(...)`.
- investigation is caller-by-caller; do not reopen compare bridge daily ownership.
- the helper stays archive-later while any explicit legacy/compat caller remains.

## Keep

- `src/host_providers/llvm_codegen.rs::emit_object_from_mir_json(...)`
- `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs`
- `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs`
- `src/runtime/plugin_loader_v2/enabled/extern_functions.rs`
- `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` (archive-later surrogate caller)

## Current Caller Inventory

The current caller inventory is three keep lanes plus one archive-later surrogate caller.

| Caller | Bucket | Note |
| --- | --- | --- |
| `src/backend/mir_interpreter/handlers/extern_provider/hostbridge.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/backend/mir_interpreter/handlers/extern_provider/loader_cold.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `src/runtime/plugin_loader_v2/enabled/extern_functions.rs` | keep | explicit legacy/compat caller; keep until a replacement daily route exists |
| `crates/nyash_kernel/src/plugin/module_string_dispatch/llvm_backend_surrogate.rs` | archive-later | compiled-stage1 surrogate caller; keeps the helper alive but is not a daily route |

## Investigation TODO

1. confirm the caller inventory stays at exactly these four surfaces.
2. keep the legacy helper archive-later until the caller set reaches zero.
3. when the caller set reaches zero, delete `emit_object_from_mir_json(...)` and collapse the phase docs.

## Delete Condition

- all explicit callers have moved to root-first or daily route surfaces.
- compare/archive proof coverage is preserved in archive assets.
- `emit_object_from_mir_json(...)` no longer appears in code-side caller inventory.

## Non-Goals

- no new compare bridge.
- no reopening of file-based `mir_json_file_to_object(...)`.
- no daily caller growth.
