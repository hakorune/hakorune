# 293x-029 M0 NUMERIC-SHIFT-SEMANTICS-LOCK

Status: Landed
Date: 2026-05-08

## Decision

The current `>>` operator is accepted as signed i64 arithmetic right shift in
the live dynamic integer lane.

This is a semantics lock, not a new numeric substrate feature. It prevents
allocator substrate work from assuming unsigned/logical shift behavior before a
dedicated logical-shift surface exists.

## Scope

Accepted in this card:

- MIR `BinaryOp::Shr` keeps the existing dynamic `Integer(i64)` arithmetic
  right-shift meaning.
- LLVM Python lowering emits `ashr` for `>>`.
- VM execution of `-8 >> 1` returns `-4`.
- No parser widening is required because no new syntax is introduced.

Deferred:

- logical right shift syntax or capability call
- exact-width unsigned value semantics
- literal suffixes such as `1u64` or `64usize`
- wrapping arithmetic syntax
- checked arithmetic syntax
- MIR JSON exact-width numeric op tags

## Owner Boundary

- Language docs own the user-visible meaning of current `>>`.
- MIR owns the current `BinaryOp::Shr` opcode spelling.
- VM and LLVM lower the opcode as signed i64 arithmetic shift.
- Future unsigned/logical rows must add an explicit new route or opcode and
  must not reinterpret existing `>>` silently.

## Gates

```bash
cargo test -q mir_numeric_shift_semantics --lib
PYTHONPATH=src/llvm_py:. python3 -m unittest src/llvm_py/tests/test_binop_numeric_tail.py
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Next

Continue M0 with an explicit wrapping/checked arithmetic decision, or move to
M1 raw layout vocabulary if the allocator substrate lane only needs the current
signed i64 arithmetic lock.
