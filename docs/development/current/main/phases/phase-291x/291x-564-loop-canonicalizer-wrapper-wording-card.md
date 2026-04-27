---
Status: Landed
Date: 2026-04-28
Scope: Refresh loop-canonicalizer route-shape wrapper wording
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-564: Loop-Canonicalizer Wrapper Wording

## Goal

Replace stale "backward compatibility" wording in the loop-canonicalizer
route-shape wrapper with its current adapter contract.

The wrapper is live: canonicalizer callsites use its tuple-shaped API while
the detector facts are owned by builder/control-flow. The comment should say
that directly instead of making the wrapper look like legacy residue.

## Cleaner Boundary

```text
builder/control-flow facts
  own route-shape detector facts

loop_canonicalizer/route_shape_recognizer
  adapts detector facts to canonicalizer tuple-shaped API
```

## Boundaries

- BoxShape/docs-in-code only.
- Do not change canonicalizer behavior.
- Do not rename wrapper functions.
- Do not move detector implementations.

## Acceptance

- No `backward compatibility` wording remains under `src/mir/loop_canonicalizer`.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Updated the skip-whitespace wrapper comment to describe the adapter role and
  builder/control-flow owner path.
- Kept behavior unchanged.

## Verification

```bash
rg -n "backward compatibility" src/mir/loop_canonicalizer -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
