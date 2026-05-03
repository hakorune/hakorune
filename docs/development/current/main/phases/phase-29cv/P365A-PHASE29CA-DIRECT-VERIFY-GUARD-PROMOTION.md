---
Status: Completed
Decision: accepted
Date: 2026-05-04
Scope: promote phase29ca direct-verify dominance canary from tools/dev to tools/checks
Related:
  - tools/checks/phase29ca_direct_verify_dominance_block_canary.sh
  - tools/dev/README.md
  - docs/tools/check-scripts-index.md
---

# P365A: Phase29ca Direct Verify Guard Promotion

## Intent

Move the phase29ca direct-verify dominance canary out of `tools/dev`.

The script is not an interactive developer helper. It is a release-binary guard
that keeps a former direct-verify dominance/Phi blocker and loop progression
regression from returning.

## Boundary

Allowed:

- move the canary to `tools/checks`
- keep its legacy filename for discoverability
- update docs that pointed at the old `tools/dev` path
- keep it out of quick gate because it requires a built release binary

Not allowed:

- change the expected `emit_rc=0` / `run_rc=4` contract
- change the fixture or binary selection behavior
- archive the canary

## Acceptance

```bash
bash tools/checks/phase29ca_direct_verify_dominance_block_canary.sh
bash tools/checks/tools_dev_surface_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
