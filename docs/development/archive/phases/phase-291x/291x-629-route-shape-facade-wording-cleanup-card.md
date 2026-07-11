---
Status: Landed
Date: 2026-04-28
Scope: reconcile route-shape facade wording after owner-path pruning
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/facts/ast_feature_extractor.rs
  - src/mir/builder/control_flow/joinir/route_entry/mod.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-629: Route-Shape Facade Wording Cleanup

## Goal

Remove stale comments that still described `ast_feature_extractor` as a
route-shape detector facade after `291x-624` through `291x-628` moved the shape
exports to their owner modules.

This is docs/comment-only BoxShape cleanup. It does not change code behavior,
exports, routing, canonicalizer observation, or accepted loop shapes.

## Evidence

The code now keeps aggregate `LoopFeatures` extraction in
`facts::ast_feature_extractor`, while shape-specific detectors are exported from
their recognizer owners through `joinir::route_entry`.

Remaining comments still said:

- `ast_feature_extractor` was a facade for `route_shape_recognizers`
- `route_entry` had an `ast_feature_extractor` thin wrapper
- `loop_canonicalizer` delegated to `ast_feature_extractor`

## Boundaries

- Update comments only.
- Do not change any imports, exports, detector code, or route behavior.
- Keep historical phase labels where they still orient the reader.

## Acceptance

- `cargo fmt -- --check` passes.
- `cargo check --release --bin hakorune -q` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `git diff --check` passes.

## Result

- Reworded current comments to identify aggregate extraction and route-shape
  recognizer owners separately.
- Removed stale facade/backward-compatibility wording from current code comments.

## Verification

```bash
cargo fmt -- --check
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
