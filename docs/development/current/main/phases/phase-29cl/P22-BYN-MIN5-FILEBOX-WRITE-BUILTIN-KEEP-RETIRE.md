---
Status: Closed Task Pack
Decision: accepted
Date: 2026-03-26
Scope: `P21` の broadened compat keep/archive cleanup first slice として、built-in `FileBox.write` keep を retired し、FileBox family の kernel `by_name` residue を空にする。
Related:
  - docs/development/current/main/phases/phase-29cl/README.md
  - docs/development/current/main/phases/phase-29cl/P1-BY-NAME-CUTOVER-ORDER.md
  - docs/development/current/main/phases/phase-29cl/P15-BYN-MIN5-FILEBOX-BUILTIN-KEEP-INVENTORY.md
  - docs/development/current/main/phases/phase-29cl/P21-BYN-MIN5-HARD-RETIRE-EXECUTION-PACK.md
  - crates/nyash_kernel/src/plugin/invoke/by_name.rs
  - crates/nyash_kernel/src/tests.rs
  - src/llvm_py/tests/test_method_fallback_tail.py
  - tools/checks/phase29cl_by_name_mainline_guard.sh
  - tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh
---

# P22: BYN-min5 FileBox Write Builtin Keep Retire

## Purpose

- retire the last built-in `FileBox` keep method from `plugin/invoke/by_name.rs`
- keep this as a narrow kernel-side residue slice; do not reopen Python compat helper work
- make the FileBox family read as fully off `by_name`

## Current Truth

1. `FileBox.write` was already retired from the Python-side compat helper before this slice.
2. `plugin/invoke/by_name.rs` was still carrying `write` as the last built-in `FileBox` keep branch.
3. no current daily caller-proof requires `FileBox.write` to remain on built-in `by_name`.
4. `FileBox` execution family is now direct-route for `open`, `read`, `readBytes`, and `close`.
5. after this slice, no built-in `FileBox` keep remains in kernel `by_name`.

## Acceptance

1. `PYTHONPATH=src/llvm_py:. python3 -m unittest src.llvm_py.tests.test_method_fallback_tail src.llvm_py.tests.test_boxcall_plugin_invoke_args`
2. `cargo test -p nyash_kernel filebox_ -- --nocapture`
3. `bash tools/checks/phase29cl_by_name_mainline_guard.sh`
4. `bash tools/checks/phase29cl_by_name_surrogate_archive_guard.sh`
5. `bash tools/smokes/v2/profiles/integration/apps/phase29cl_by_name_lock_vm.sh`
6. `bash tools/smokes/v2/profiles/integration/apps/phase29ck_vmhako_llvm_backend_runtime_proof.sh`

## Next Exact Front

1. broader compat keep/archive cleanup beyond the FileBox family
