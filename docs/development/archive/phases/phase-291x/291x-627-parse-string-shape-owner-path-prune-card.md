---
Status: Landed
Date: 2026-04-28
Scope: remove parse-string family shape re-exports from ast_feature_extractor facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/facts/route_shape_recognizers/parse_string.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-627: Parse-String Shape Owner Path Prune

## Goal

Remove the parse-string family shape detectors from the `ast_feature_extractor`
compatibility facade and use the parse-string route-shape owner directly.

This covers `detect_parse_string_shape` and `detect_continue_shape`, which are
both implemented by `facts::route_shape_recognizers::parse_string`.

This is BoxShape-only cleanup. It does not change parse-string detection,
continue-shape detection, canonicalizer observation, route-entry exports, or
accepted loop shapes.

## Evidence

The route-entry exports were the live consumers of the facade path for this
family. The outer builder/canonicalizer export chain can stay unchanged while
`route_entry` imports from the owner module.

## Boundaries

- Remove only parse-string family detector re-exports from `ast_feature_extractor`.
- Keep `route_entry::detect_parse_string_shape` and
  `route_entry::detect_continue_shape` exported.
- Update canonicalizer wording for these shapes.
- Do not change detector implementations or matching behavior.
- Do not move skip-whitespace in this card.

## Acceptance

- No `ast_feature_extractor::detect_parse_string_shape` users remain.
- No `ast_feature_extractor::detect_continue_shape` users remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reduced the `ast_feature_extractor` facade by the parse-string route-shape
  family.
- Preserved the route-entry and builder export chain for canonicalizer callers.

## Verification

```bash
rg -n "ast_feature_extractor::detect_(parse_string|continue)_shape|pub use route_shape_recognizers::parse_string" src/mir -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
