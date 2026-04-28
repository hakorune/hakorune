---
Status: Landed
Date: 2026-04-29
Scope: sync verified PortSig SSOT wording with ownership cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/verified-recipe-port-sig-ssot.md
---

# 291x-690: Verified PortSig SSOT Wording Sync

## Goal

Keep the PortSig design SSOT aligned with the current verification ownership.

This is docs-only BoxShape cleanup. It must not change code behavior.

## Evidence

After 291x-687, PortSig obligation verification is owned by
`recipe_tree::verified`, but the design SSOT still said Parts checks PortSig
obligations.

## Decision

Update wording so the SSOT says:

- RecipeTree verified constructs and enforces PortSig obligations.
- Lower/Parts receive `VerifiedRecipeBlock` and wire mechanically.

## Boundaries

- Do not change PortSig rules.
- Do not change strict/dev(+planner_required) scope.
- Do not change code.

## Acceptance

```bash
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- PortSig ownership wording matches the current implementation.
