---
Status: Complete
Date: 2026-05-12
Scope: VM reference-executor boundary for exact `usize` semantic rows.
Related:
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - docs/development/current/main/phases/phase-294x/README.md
  - docs/development/current/main/phases/phase-294x/294x-90-usize-semantics-taskboard.md
---

# 294x-09a VM Reference Executor Boundary

## Purpose

Fix the VM role before 294x starts adding exact `usize` reference execution.

The Rust VM is not returning as the product/mainline backend owner. In 294x it
is only the semantic reference executor for MIR-owned facts and contracts.
Unsupported non-VM backends must fail fast until they can lower the same
semantics.

## Decision

```text
VM is not a product owner.
VM is a semantic reference executor.
```

Japanese mirror:

```text
VMは本線実装者ではない。
VMは意味論の参照実行器。
```

## VM Row Gate

A VM row may land only if all are true:

- a MIR-owned fact, policy, or contract already exists;
- the VM only executes that MIR-owned semantic contract;
- unsupported non-VM backend routes fail fast or have a visible lowering row;
- hako_alloc live field migration is not included in the same row;
- the next backend/lowering row is visible in the taskboard.

## Forbidden Reading

Do not treat VM green as hako_alloc/mimalloc green.

```text
NG:
  VM-only exact usize ops count as complete product support
  EXE/backend silently falls back to i64
  hako_alloc migrates live fields before backend/storage support is real
```

## Non-Goals

- VM exact `usize` implementation;
- backend lowering;
- typed-object exact numeric storage;
- hako_alloc field migration;
- changing VM retirement/keep decisions outside this phase.

## Verification

- docs-only row;
- `bash tools/checks/current_state_pointer_guard.sh`;
- `git diff --check`.
