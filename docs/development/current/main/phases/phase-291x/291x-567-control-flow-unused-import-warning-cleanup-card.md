---
Status: Landed
Date: 2026-04-28
Scope: control-flow unused import warning cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/verify/verifier/debug_helpers/mod.rs
  - src/mir/builder/control_flow/verify/verifier/tests.rs
  - src/mir/builder/control_flow/plan/loop_break_steps/emit_joinir_helpers.rs
---

# 291x-567: Control-Flow Unused Import Warning Cleanup

## Goal

Remove the unused import warnings surfaced by release build/test during the
read-digits keep-plan shelf prune.

This is warning hygiene only. No verifier, JoinIR, or loop-break behavior
changes.

## Cleaner Boundary

```text
debug-only helpers/tests
  import debug-only helpers only under cfg(debug_assertions)

release build
  sees only imports used by release code
```

## Boundaries

- BoxShape/warning hygiene only.
- Do not change debug assertion bodies.
- Do not change loop-break exit binding logic.
- Do not silence warnings with broad `allow(unused_imports)`.

## Acceptance

- `cargo check --release --bin hakorune` no longer reports the targeted 2 lib warnings.
- `cargo test --release read_digits_loop_true_policy_returns_break_when_true_and_allowlist --lib`
  no longer reports the targeted 4 lib-test warnings.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Gated `cond_profile_from_scan_shapes` behind `cfg(debug_assertions)`.
- Gated verifier debug test imports behind `cfg(debug_assertions)`.
- Gated `BTreeSet` import in loop-break JoinIR helpers behind
  `cfg(debug_assertions)`.

## Verification

```bash
cargo check --release --bin hakorune
cargo test --release read_digits_loop_true_policy_returns_break_when_true_and_allowlist --lib
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
git diff --check
```
