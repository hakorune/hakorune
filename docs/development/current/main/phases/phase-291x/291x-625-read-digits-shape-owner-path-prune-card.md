---
Status: Landed
Date: 2026-04-28
Scope: remove read-digits shape re-export from ast_feature_extractor facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/facts/route_shape_recognizers/parse_number.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/builder/control_flow/cleanup/policies/loop_true_read_digits_policy.rs
---

# 291x-625: Read-Digits Shape Owner Path Prune

## Goal

Remove `detect_read_digits_loop_true_shape` from the `ast_feature_extractor`
compatibility facade and use the parse-number route-shape owner directly.

This is BoxShape-only cleanup. It does not change read-digits detection,
loop-true policy routing, canonicalizer exports, or accepted loop shapes.

## Evidence

`detect_read_digits_loop_true_shape` is implemented in
`facts::route_shape_recognizers::parse_number`. The `ast_feature_extractor`
module only re-exported it.

The live facade users were:

- `joinir/route_entry/mod.rs`, to keep the builder/canonicalizer export chain
- `cleanup/policies/loop_true_read_digits_policy.rs`

Both can import from the owner path while preserving their external surfaces.

## Boundaries

- Remove only the read-digits re-export from `ast_feature_extractor`.
- Keep `route_entry::detect_read_digits_loop_true_shape` exported.
- Do not change parse-number detector implementation.
- Do not change cleanup policy behavior.
- Do not move the broader parse-number facade in this card.

## Acceptance

- No `ast_feature_extractor::detect_read_digits_loop_true_shape` users remain.
- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reduced the `ast_feature_extractor` facade by one route-shape detector.
- Preserved the route-entry and builder export chain for canonicalizer callers.

## Verification

```bash
rg -n "ast_feature_extractor::detect_read_digits_loop_true_shape|detect_read_digits_loop_true_shape" src/mir/builder/control_flow/facts/ast_feature_extractor.rs src/mir/builder/control_flow src/mir/loop_canonicalizer -g'*.rs'
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
