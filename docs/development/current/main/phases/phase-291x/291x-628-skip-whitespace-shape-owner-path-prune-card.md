---
Status: Landed
Date: 2026-04-28
Scope: remove skip-whitespace shape re-export and now-empty route_entry facade shell
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/facts/route_shape_recognizers/skip_whitespace.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-628: Skip-Whitespace Shape Owner Path Prune

## Goal

Remove `detect_skip_whitespace_shape` from the `ast_feature_extractor`
compatibility facade and use the skip-whitespace route-shape owner directly.

This is BoxShape-only cleanup. It does not change skip-whitespace or trim-shape
detection, canonicalizer observation, route-entry exports, or accepted loop
shapes.

## Evidence

`detect_skip_whitespace_shape` is implemented in
`facts::route_shape_recognizers::skip_whitespace`.

After `291x-624` through `291x-627`, this was the final route-shape detector
being re-exported through `ast_feature_extractor` for `route_entry`.

## Boundaries

- Remove only the skip-whitespace detector re-export from `ast_feature_extractor`.
- Keep `route_entry::detect_skip_whitespace_shape` exported.
- Update canonicalizer wording for this shape.
- Do not change detector implementation or matching behavior.
- Remove the now-unused `route_entry::ast_feature_extractor` wildcard shell.

## Acceptance

- No `ast_feature_extractor::detect_skip_whitespace_shape` users remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed the last route-shape detector re-export from `ast_feature_extractor`.
- Removed the now-unused `route_entry::ast_feature_extractor` wildcard shell.
- Preserved the route-entry and builder export chain for canonicalizer callers.

## Verification

```bash
rg -n "ast_feature_extractor::detect_skip_whitespace_shape|pub use route_shape_recognizers::skip_whitespace" src/mir -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
