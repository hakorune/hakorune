---
Status: accepted
Decision: accepted
Date: 2026-03-21
Scope: Rust `--emit-mir-json` route must serialize the entry function first so boundary/pure-first EXE smokes do not compile helper functions as `functions[0]`.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/10-Now.md
  - docs/development/current/main/design/de-rust-lane-map-ssot.md
  - docs/development/current/main/phases/phase-29bq/29bq-90-selfhost-checklist.md
  - docs/development/current/main/phases/phase-29bq/29bq-91-mirbuilder-migration-progress-checklist.md
  - src/runner/mir_json_emit/mod.rs
  - tools/smokes/v2/profiles/integration/joinir/phase29bq_emit_mir_entry_order_ternary_basic_vm.sh
  - .github/workflows/fast-smoke.yml
---

# 29bq-116 Emit-MIR Entry Order Blocker

## Problem

- `apps/tests/ternary_basic.hako` was green on VM and in `--dump-mir`, but the fast CI EXE smoke failed.
- `--emit-mir-json` serialized helper functions before `main`, so `functions[0]` became `Main.equals/1`.
- boundary/pure-first keep paths still inspect `functions[0]` in a few places, so the EXE smoke compiled the wrong function and returned `1`.

## Contract

1. Rust `--emit-mir-json` must serialize the entry function first.
2. For daily app fixtures, the entry function is `main` (`metadata.is_entry_point=true`).
3. helper/static methods may follow, but they must not occupy `functions[0]` when `main` exists.
4. The blocker pin is:
   - `tools/smokes/v2/profiles/integration/joinir/phase29bq_emit_mir_entry_order_ternary_basic_vm.sh`

## Acceptance

- `jq '.functions[0].name' <emit-mir-json-output>` returns `"main"` for `apps/tests/ternary_basic.hako`.
- `tools/smokes/v2/profiles/integration/joinir/phase29bq_emit_mir_entry_order_ternary_basic_vm.sh` passes.
- if a new exact blocker appears after `functions[0]` becomes `main`, reopen it under a new phase instead of widening `29bq-116`.

## Notes

- This is a lane B compiler-pipeline blocker, not a lane C vm-hako blocker.
- The MIR builder itself was correct; the failure was in MIR JSON externalization contract.
- Once `functions[0]` became `main`, the next exact blocker was `29bq-117` (`ArrayBox.birth()` unsupported in the llvmlite harness keep lane).
