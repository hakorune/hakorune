---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after loop-canonicalizer wording cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-564-loop-canonicalizer-wrapper-wording-card.md
  - src/mir/builder/control_flow/plan/policies/keep_plan/loop_true_read_digits_policy.rs
---

# 291x-565: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after loop-canonicalizer wrapper
wording cleanup landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `keep_plan/loop_true_read_digits_policy.rs` | re-export-only shelf; live router already imports cleanup owner directly | select next |
| `keep_plan/trim_policy.rs` | re-export-only shelf, but `trim_loop_lowering.rs` still imports `PolicyDecision` through `super::policies` | defer until after read-digits prune |
| `keep_plan/p5b_escape_derived_policy.rs` | compat file still owns tests | defer; decide test ownership first |
| `plan/features/mod.rs` flattened route re-exports | live API used by normalizer/composer | defer; larger API cleanup |
| `plan/recipe_tree/mod.rs` builder re-exports | live internal root API | defer; wording or API-card only |
| `joinir/merge/rewriter/mod.rs` parent re-export | `merge_and_rewrite` still lives in parent module | defer; extraction card |

## Decision

Select **read-digits keep-plan compatibility shelf prune** as the next lane.

Reason:

- `loop_true_read_digits_policy.rs` under `keep_plan/` only re-exports the
  cleanup-owned policy.
- Runtime/planner callers already use
  `control_flow::cleanup::policies::loop_true_read_digits_policy` directly.
- The compat file has no local tests, unlike the P5b shelf.
- The cleanup is BoxShape-only and should not change route semantics.

## Next Card

Create `291x-566-read-digits-keep-plan-shelf-prune-card` before editing code.

Planned change:

```text
src/mir/builder/control_flow/plan/policies/keep_plan/loop_true_read_digits_policy.rs
  delete re-export-only compatibility file

src/mir/builder/control_flow/plan/policies/keep_plan/mod.rs
  remove loop_true_read_digits_policy module declaration

src/mir/builder/control_flow/plan/policies/keep_plan/README.md
  mark read-digits shelf retired; cleanup owner remains the only path
```

## Acceptance

```bash
rg -n "plan::policies::.*loop_true_read_digits|keep_plan::loop_true_read_digits|super::policies::loop_true_read_digits" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check --release --bin hakorune
cargo fmt -- --check
git diff --check
bash tools/smokes/v2/profiles/integration/apps/archive/phase104_read_digits_vm.sh
```
