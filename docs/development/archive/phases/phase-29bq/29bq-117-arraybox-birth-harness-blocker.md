---
Status: accepted
Decision: accepted
Date: 2026-03-21
Scope: llvmlite harness must accept `ArrayBox.birth()` as the no-op initializer marker that follows `newbox ArrayBox` on the fast EXE path.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - src/llvm_py/instructions/mir_call/method_call.py
  - src/llvm_py/tests/test_method_call_collection_birth.py
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh
  - .github/workflows/fast-smoke.yml
---

# 29bq-117 ArrayBox.birth Harness Blocker

## Problem

- After `29bq-116` serialized `main` first, fast-smoke advanced to the real next blocker.
- `ny-llvmc --driver harness` failed on `apps/tests/ternary_basic.hako` with:
  - `Unsupported MIR method call: box='ArrayBox' method='birth'`
- The failing entry-prologue shape was:
  1. `newbox ArrayBox`
  2. `copy`
  3. `mir_call Method(ArrayBox.birth)`
  4. normal compare / branch / phi / ret

## Contract

1. llvmlite harness lowers `ArrayBox.birth()` as the initializer no-op that follows `newbox ArrayBox`.
2. This phase stays narrow:
   - do not widen unrelated collection methods in the same slice
   - do not reopen parser / mirbuilder lowering for this blocker
3. The exact blocker pin is:
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh`

## Acceptance

- `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_call_collection_birth` passes.
- `bash tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh` passes.
- fast-smoke EXE trio is green again:
  - `ternary_basic -> 10`
  - `ternary_nested -> 50`
  - `peek_expr_block -> 1`

## Notes

- This is a lane B compiler-pipeline blocker, not a lane C vm-hako blocker.
- `ArrayBox.birth()` here is not a new semantics owner; it is the compat initializer marker in the harness keep lane.
