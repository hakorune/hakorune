---
Status: Landed
Date: 2026-04-28
Scope: close out P5b keep-plan compatibility shelf
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/cleanup/policies/p5b_escape_derived_policy.rs
  - src/mir/builder/control_flow/plan/mod.rs
---

# 291x-569: P5b Keep-Plan Shelf Closeout

## Goal

Close the remaining `plan/policies/keep_plan/` compatibility shelf.

After read-digits and trim shelves were pruned, the only remaining keep-plan
file was a P5b re-export that also carried local tests. Move those tests to
the cleanup owner and delete the compatibility module tree.

## Cleaner Boundary

```text
cleanup/policies/p5b_escape_derived_policy.rs
  owns P5b policy implementation and tests

plan/
  does not expose a route-policy compatibility shelf
```

## Boundaries

- BoxShape/test-ownership cleanup only.
- Do not change P5b classification semantics.
- Do not change strict-mode env semantics.
- Do not add a replacement re-export.

## Acceptance

- No `plan::policies`, `keep_plan`, or `super::policies` path remains under
  `src/mir/builder/control_flow`.
- `cargo test --release p5b --lib` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Moved P5b compatibility-shelf tests to the cleanup policy owner.
- Aligned moved tests with the current escape recognizer contract
  (`pre_delta` is total escape-step delta for no-else shapes).
- Deleted `plan/policies/keep_plan/` tracked files.
- Deleted `plan/policies/mod.rs`.
- Removed the `plan::policies` module declaration from `plan/mod.rs`.

## Verification

```bash
rg -n "plan::policies|keep_plan|super::policies" src/mir/builder/control_flow -g'*.rs'
cargo test --release p5b --lib
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
```
