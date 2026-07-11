---
Status: Landed
Date: 2026-04-28
Scope: remove redundant Scan* aliases from scan_loop_segments and switch the remaining block recipe alias surface to primary names
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/recipes/scan_loop_segments.rs
  - src/mir/builder/control_flow/recipes/loop_scan_methods_block_v0.rs
---

# 291x-592: Scan Alias Dedup

## Goal

Trim redundant shared-vocabulary aliases now that the block recipe surface can
point directly at the primary scan-loop segment names.

This is BoxShape-only cleanup. It does not change segment behavior or any route
logic.

## Evidence

After 291x-589 through 291x-591, the shared aliases
`scan_loop_segments::{ScanNestedLoopRecipe, ScanSegment}` only existed to feed
`recipes/loop_scan_methods_block_v0.rs`.

That block recipe module can reference the primary SSOT names directly:

- `NestedLoopRecipe`
- `LoopScanSegment<LinearBlockRecipe>`

## Boundaries

- Remove only the redundant shared aliases.
- Keep the block recipe module's local alias surface stable for its existing
  plan callers.
- Do not change any plan-side imports from `recipes::loop_scan_methods_block_v0`.

## Acceptance

- `scan_loop_segments.rs` no longer defines `ScanNestedLoopRecipe` or
  `ScanSegment`.
- `recipes/loop_scan_methods_block_v0.rs` points at the primary shared-vocabulary
  names.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `cargo check --release --bin hakorune` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Reduced the shared scan-loop vocab to one canonical name set.
- Kept the block recipe module stable while removing an unnecessary alias layer
  from the shared owner.

## Verification

```bash
rg -n "ScanNestedLoopRecipe|ScanSegment<|type ScanSegment|type ScanNestedLoopRecipe" src/mir/builder/control_flow -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo fmt -- --check
cargo check --release --bin hakorune
git diff --check
```
