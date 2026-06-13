---
Status: Landed
Date: 2026-06-13
Scope: compiler cleanliness navigation and dead-dust closeout after 291x-792.
Related:
  - CURRENT_TASK.md
  - docs/development/current/main/phases/phase-291x/291x-792-compiler-cleanliness-clean-enough-closeout-card.md
  - docs/development/current/main/design/compiler-pipeline-thinning-ssot.md
  - docs/development/current/main/design/joinir-target-lowerer-thinning-ssot.md
  - docs/development/current/main/design/loop-update-analyzer-thinning-ssot.md
  - docs/development/current/main/design/loop-body-local-init-thinning-ssot.md
  - docs/development/current/main/design/inline-boundary-builder-thinning-ssot.md
  - docs/development/current/main/design/generic-case-a-trim-thinning-ssot.md
  - docs/development/current/main/design/user-method-policy-thinning-ssot.md
---

# 291x-793: Compiler Cleanliness Navigation / Dead-Dust Closeout

## Goal

Make the completed compiler cleanup wave discoverable from restart/navigation
docs, remove two confirmed dead helper surfaces, and clear one stale README
marker.

This is the final polish card after `291x-792`. Do not continue code thinning
from this card.

## Changes

```text
navigation:
  compiler-pipeline-thinning-ssot.md links the child cleanup SSOTs
  CURRENT_TASK.md records that the cleanup lane existed and closed

dead code:
  MirVerifier::get_errors removed
  MirVerifier::clear_errors removed
  build_simple_loop removed from src/mir/loop_api.rs

small docs dust:
  removed trailing *** marker from src/mir/join_ir/lowering/README.md
```

## Stop-Line

This closes the cleanup lane follow-up requested after `291x-792`.

```text
compiler_cleanup_status=closed_after_nav_dead_dust
continue_code_thinning=0
next_default_lane=CURRENT_STATE.toml active_lane
```

Future cleanup must create a new focused card with an owner family, a
BoxShape-only statement, forbidden behavior changes, and targeted proof
commands.

## Proof

```bash
! rg -n "get_errors|clear_errors|build_simple_loop" src tests crates -g '*.rs' -g '*.md'
cargo check -q --lib
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
