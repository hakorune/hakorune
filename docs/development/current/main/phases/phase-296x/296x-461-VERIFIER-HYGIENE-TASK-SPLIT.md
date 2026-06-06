---
Status: Active
Date: 2026-06-06
Scope: BoxShape cleanup inventory for MIR verifier/common test helpers.
Related:
  - docs/development/current/main/design/compiler-expressivity-first-policy.md
  - docs/development/current/main/phases/phase-296x/296x-460-VERIFIED-TABLE-ACCESS-PROOF-DECISION.md
---

# 296x-461 Verifier Hygiene Task Split

## Decision

This is BoxShape cleanup. Do not mix it with FastMemory TableIndex proof
BoxCount work.

Immediate low-risk slice:

```text
VERIFIER-HYGIENE-001:
  add MirInstruction::extern_name()
  migrate duplicated verifier extern-name helpers
```

Parked until after the next FastMemory proof slice:

```text
VERIFIER-HYGIENE-002:
  narrow verifier instruction walker helper

VERIFIER-HYGIENE-003:
  private verifier test_support helpers

VERIFIER-HYGIENE-004:
  instruction unit test ownership cleanup
```

## Walker Boundary

A shared walker is allowed only as a small helper. Do not introduce a visitor
trait yet.

Required shape:

```rust
pub(crate) enum BlockOrder {
    Storage,
    SortedId,
}

pub(crate) enum InstructionScope {
    BodyOnly,
    BodyAndTerminator,
}

pub(crate) struct VerificationSite<'a> {
    pub block: BasicBlockId,
    pub instruction_index: usize,
    pub inst: &'a MirInstruction,
}
```

Risk boundaries:

```text
awaits:
  has body-only neighbor checks; do not silently include terminators

barrier_context:
  needs same-block neighbor windows; keep block-local shape

fastmem:
  has side-table region metadata and multi-pass checks; only use helper for
  simple instruction-site walks

cfg:
  mostly CFG/PHI semantics; do not force through flat instruction walker
```

## extern_name Utility

Accepted:

```text
MirInstruction::extern_name() -> Option<&str>
```

Owner:

```text
src/mir/instruction/methods.rs
```

Initial migration target:

```text
src/mir/verification/awaits.rs
```

Later migration target:

```text
src/mir/verification/barrier.rs
```

## Test Helper Boundary

Use a verifier-private support module only:

```text
src/mir/verification/test_support.rs
```

Initial helpers:

```text
function_with_instructions(...)
error_text(...)
```

Do not delete or shrink `tests/mir_instruction_unit.rs` in the same slice. That
is a separate test-ownership decision.

## Acceptance

```bash
cargo test -q verification::awaits --lib
cargo test -q test_call_instruction_extern_name --lib
bash tools/checks/current_state_pointer_guard.sh
```

No behavior changes are allowed.
