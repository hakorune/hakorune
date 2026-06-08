---
Status: Done
Date: 2026-06-08
Scope: OUTBOX-0.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/workstreams/mimalloc-current.md
  - docs/development/current/main/phases/phase-296x/296x-639-MIM-PORT-FMEM-140-SELFHOST-SURFACE-PREFLIGHT-TASK-ORDER.md
  - src/mir/builder/exprs.rs
  - src/mir/builder/stmts/variable_stmt.rs
  - src/mir/function/metadata.rs
  - src/tests/mir_outbox_contract.rs
  - src/tests/parser_outbox_contract.rs
---

# 296x-640 OUTBOX-0 Narrow Lowering Landing

## Purpose

Record the narrow outbox lowering slice that closes the parser-accepted /
MIR-missing gap without opening a richer ownership checker.

## Implementation

```text
outbox declaration:
  materialize each binding as a Void-typed local
  reuse the shared local-statement shell
  record binding names in function.metadata.outbox_bindings

parser contract:
  duplicate outbox bindings remain fail-fast
  rich move/state tracking remains closed
```

## Report / Check

```text
outbox_lowering=1
outbox_binding_count>0
outbox_init_expr_supported=0
outbox_transfer_return_metadata=1
outbox_rich_move_checker=0
```

## Verification

```bash
cargo test -q parser_outbox_contract --lib
cargo test -q mir_outbox_contract --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Landed

```text
The outbox source surface now lowers to a narrow Void-binding transfer
marker, and the parser contract keeps duplicate outbox bindings fail-fast.
```

## Closeout

```text
next: SELFHOST-SURFACE-000 selfhost surface check
```
