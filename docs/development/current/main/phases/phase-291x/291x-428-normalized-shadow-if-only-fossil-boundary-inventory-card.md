---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow if-only fossil boundary inventory
Related:
  - src/mir/control_tree/normalized_shadow/entry/README.md
  - src/mir/control_tree/normalized_shadow/entry/if_only.rs
  - src/mir/control_tree/normalized_shadow/builder.rs
  - docs/development/current/main/phases/phase-291x/291x-427-normalized-shadow-suffix-router-owner-move-card.md
---

# 291x-428: Normalized-Shadow If-Only Fossil Boundary Inventory

## Goal

Pick the next small compiler-cleanliness seam after normalized-shadow suffix
router owner move.

This is a BoxShape inventory. No behavior changed.

## Findings

`entry/if_only.rs` is the Phase 123-128 baseline route. It intentionally keeps
early prototype semantics:

- minimal compare parsing
- compare LHS emitted as placeholder `0`
- branches verified as `Return(Integer literal)`
- then branch emitted as the simplified branch body
- unsupported Phase 123-128 shapes decline with existing `[phase123/*]`,
  `[phase124/*]`, `[phase125/*]`, or `[phase128/*]` tags

Newer normalized-shadow routes already run before this baseline:

```text
PostIfPostKBuilderBox
IfAsLastJoinKLowererBox
Phase 123-128 if_only baseline
```

The risk is not current behavior. The risk is that future cleanup treats the
baseline placeholders as accidental bugs and silently changes route semantics.

## Decision

Add a fossil-boundary note to the entry README, the if-only entry module, and
the builder call site.

Do not change:

- compare LHS placeholder
- then-branch-only simplified emission
- `[phase123/*]` / `[phase125/*]` decline tags
- route priority
- accepted StepTree shapes

## Next Cleanup

`291x-429`: normalized-shadow if-only fossil boundary note.

Acceptance:

```bash
cargo check --bin hakorune
bash tools/checks/current_state_pointer_guard.sh
git diff --check
bash tools/smokes/v2/profiles/integration/apps/archive/phase123_if_only_normalized_semantics_vm.sh
bash tools/smokes/v2/profiles/integration/apps/archive/phase128_if_only_partial_assign_normalized_vm.sh
```
