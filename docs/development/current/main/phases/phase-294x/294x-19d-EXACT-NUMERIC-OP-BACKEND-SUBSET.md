---
Status: Complete
Date: 2026-05-12
Scope: Python LLVM lowering for the exact numeric operation route subset needed
  before production hako_alloc field migration.
Related:
  - docs/development/current/main/phases/phase-294x/294x-19c-EXACT-FIELD-ABI-BACKEND-CONSUMPTION.md
  - docs/development/current/main/design/usize-semantic-foundation-ssot.md
  - src/mir/exact_numeric_backend_capability.rs
  - src/runner/mir_json_emit/metadata.rs
  - src/llvm_py/instructions/binop.py
  - src/llvm_py/instructions/compare.py
---

# 294x-19d Exact Numeric Op Backend Subset

## Decision

After Python LLVM can consume exact typed-object fields, it may consume the
MIR-owned exact numeric operation route facts that production `hako_alloc`
field migration needs.

This row opens only the explicit route-fact path. Generic integer lowering
remains the current `i64` lane, and production `hako_alloc` fields remain
`i64` until a later migration row changes them by field group.

## Contract

- MIR JSON emits per-function exact numeric operation route facts.
- Python LLVM loads those facts into resolver-owned maps keyed by destination
  value id.
- Exact `add` / `sub` / `mul` routes lower as checked arithmetic and trap on
  overflow or target-range failure.
- Exact compare routes lower with signed or unsigned predicates from the
  declared exact numeric type.
- Exact logical right-shift routes lower as `lshr` and trap when the shift
  count is outside the exact type width.
- Unsupported non-VM/Python backends keep route facts fail-fast.

## Non-Goals

- No production `hako_alloc` field migration.
- No division, modulo, bitwise, or wrapping arithmetic vocabulary.
- No WASM exact numeric op lowering.
- No source syntax changes.

## Landed

- MIR JSON now emits exact numeric binary-op, compare, and shift route facts.
- Python LLVM loads route facts into resolver-owned maps keyed by destination
  value id.
- Exact `add` / `sub` / `mul` routes lower through checked LLVM overflow
  intrinsics and trap on overflow or target-range failure.
- Exact compare routes use signed or unsigned predicates from the declared
  exact numeric type.
- Exact logical right-shift routes lower to `lshr` and trap on out-of-range
  shift counts.
- The backend capability gate now accepts exact numeric operation route facts
  for the Python LLVM helper path and PyVM reference executor only.

Still closed after this row:

- production `hako_alloc` field migration;
- division, modulo, bitwise, and wrapping exact numeric ops;
- WASM exact numeric op lowering.

## Acceptance

```bash
cargo test -q --lib mir_json_exact_numeric_routes
python3 -m unittest src.llvm_py.tests.test_exact_numeric_ops
cargo test -q --lib exact_numeric_backend_capability
cargo check --release --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
```
