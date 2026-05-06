# 293x-002 Binary Trees Real App

- Status: Landed
- Lane: `phase-293x real-app bringup`
- Scope: add a fast VM-sized CLBG-style binary tree app that stresses recursive
  allocation, short-lived object churn, and recursive checksums.

## Changes

- Added `apps/binary-trees/main.hako`.
- Added app-local `apps/binary-trees/test.sh`.
- Added `tools/smokes/v2/profiles/integration/apps/binary_trees_vm.sh`.
- Added binary-trees to the `real-apps` integration suite.
- Updated `apps/README.md` and the phase-293x taskboard.

## Contract

- Stretch tree depth `7` checks to `-1`.
- Long-lived tree depth `6` checks to `-1`.
- Depth `4` short-lived run uses `64` iterations and checks to `-128`.
- Depth `6` short-lived run uses `16` iterations and checks to `-32`.

## Verification

```bash
apps/binary-trees/test.sh
tools/smokes/v2/profiles/integration/apps/binary_trees_vm.sh
tools/smokes/v2/run.sh --profile integration --suite real-apps --skip-preflight
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
