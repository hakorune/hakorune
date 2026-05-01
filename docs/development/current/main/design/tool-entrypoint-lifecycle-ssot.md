---
Status: Accepted
Decision: accepted
Date: 2026-05-01
Scope: root tool entrypoint protection, archive, and delete lifecycle.
Related:
  - docs/development/current/main/phases/phase-29cv/README.md
  - docs/development/current/main/phases/phase-29cv/P52-ROOT-HELPER-PROTECTED-CATEGORY-POLICY.md
  - tools/ROOT_SURFACE.md
  - tools/archive/manual-smokes/README.md
  - tools/archive/manual-tools/README.md
---

# Tool Entrypoint Lifecycle SSOT

## Goal

Keep the repository root and `tools/` root thin without deleting real user,
platform, build, CI, or compatibility entrypoints by accident.

This policy is for shell/PowerShell/Python helper entrypoints. It does not
change compiler behavior.

The current root helper inventory lives in `tools/ROOT_SURFACE.md`.

## Archive/Delete Rule

A root helper is an archive/delete candidate only when all of these are true:

- no active refs from current docs, tools, src, lang, Makefile, or root README
- no current PASS gate owns it
- no compat capsule owns it
- no protected category owns it

The protected-category check is required because some platform/build entrypoints
are intentionally weakly referenced.

## Protected Categories

These categories are not archived just because `rg` shows weak refs:

- current smoke gates
- CI helpers
- build helpers
- platform-specific helpers
- compat capsules
- generator, manifest, or codegen tools
- release or packaging helpers
- docs pointer guards

Weakly referenced protected helpers should have an owner pointer in docs or in a
nearby README. Missing owner text is a yellow flag, not an immediate delete.

## Lifecycle

Use this progression for cleanup:

1. root entrypoint
2. archive or compat capsule
3. delete

Do not jump from root entrypoint to delete unless the file is already proven
unowned and non-executable evidence is not useful.

## Delete Criteria

Archived helpers become delete candidates after either 30-60 days or two cleanup
batches, and only when all of these stay true:

- no active refs from current docs, tools, src, lang, Makefile, or root README
- no current PASS gate owns it
- no compat capsule README owns it with a reproduction command
- the archive README lists it as a delete candidate or covered by the delete
  policy
- `tools/checks/current_state_pointer_guard.sh` passes
- `git diff --check` passes

Deletion should be small and reviewable. Prefer one archive bucket or one
helper family per commit.

## Root Weak-Ref Reading

For phase-29cv cleanup, keep this default reading:

- keep CI/golden chain helpers while `tools/core_ci.sh` owns them
- keep platform build helpers until their platform owner is documented or
  retired
- capsule-classify backend canaries before moving them
- archive old one-shot migration helpers once docs point at the archived path
