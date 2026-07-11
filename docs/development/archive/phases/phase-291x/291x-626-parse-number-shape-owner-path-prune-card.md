---
Status: Landed
Date: 2026-04-28
Scope: remove parse-number shape re-export from ast_feature_extractor facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/facts/route_shape_recognizers/parse_number.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-626: Parse-Number Shape Owner Path Prune

## Goal

Remove `detect_parse_number_shape` from the `ast_feature_extractor`
compatibility facade and use the parse-number route-shape owner directly.

This is BoxShape-only cleanup. It does not change parse-number detection,
canonicalizer observation, route-entry exports, or accepted loop shapes.

## Evidence

`detect_parse_number_shape` is implemented in
`facts::route_shape_recognizers::parse_number`. After `291x-625`, read-digits
already uses this owner path.

The remaining facade use for parse-number was the `route_entry` re-export used
by the builder/canonicalizer export chain. That outer export can stay while its
implementation path points to the owner module.

## Boundaries

- Remove only the parse-number re-export from `ast_feature_extractor`.
- Keep `route_entry::detect_parse_number_shape` exported.
- Update canonicalizer wording for this shape.
- Do not change detector implementation or matching behavior.
- Do not move parse-string, continue, or skip-whitespace in this card.

## Acceptance

- No `ast_feature_extractor::detect_parse_number_shape` users remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reduced the `ast_feature_extractor` facade by one more route-shape detector.
- Preserved the route-entry and builder export chain for canonicalizer callers.

## Verification

```bash
rg -n "ast_feature_extractor::detect_parse_number_shape|pub use route_shape_recognizers::parse_number::detect_parse_number_shape" src/mir -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
