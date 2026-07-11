---
Status: Landed
Date: 2026-04-28
Scope: remove escape-shape re-export from ast_feature_extractor facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/facts/escape_shape_recognizer.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/builder/control_flow/cleanup/policies/p5b_escape_derived_policy.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-624: Escape Shape Owner Path Prune

## Goal

Remove `detect_escape_skip_shape` from the `ast_feature_extractor` compatibility
facade and point live callers at the escape-shape owner.

This is BoxShape-only cleanup. It does not change route-shape matching, cleanup
policy decisions, canonicalizer observation, or JoinIR route exports.

## Evidence

`detect_escape_skip_shape` is owned by
`facts::escape_shape_recognizer`. The `ast_feature_extractor` module only
re-exported it as a facade entry.

The direct live caller through the old facade was the P5b cleanup policy. The
JoinIR route-entry export can use the owner path while preserving the public
route-entry surface for the canonicalizer and builder root exports.

## Boundaries

- Remove only the escape-shape re-export from `ast_feature_extractor`.
- Keep `route_entry::detect_escape_skip_shape` exported.
- Update P5b cleanup policy to import from `escape_shape_recognizer`.
- Update stale canonicalizer wording for this shape.
- Do not change detector implementation or matching behavior.

## Acceptance

- No `ast_feature_extractor::detect_escape_skip_shape` users remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reduced the `ast_feature_extractor` facade by one owner-moved shape detector.
- Preserved downstream `route_entry` / `joinir` / builder exports.

## Verification

```bash
rg -n "ast_feature_extractor::detect_escape_skip_shape|pub use super::escape_shape_recognizer::detect_escape_skip_shape" src/mir -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
