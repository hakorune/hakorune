---
Status: Complete
Date: 2026-05-12
Scope: M172 proof-app readability cleanup before M173 implementation resumes.
Related:
  - docs/development/current/main/design/mimalloc-hako-port-implementation-plan-ssot.md
  - apps/mimalloc-page-map-release-proof/main.hako
  - tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
---

# 293x-182 M172 Proof Check Cleanup

## Goal

Make the M172 proof app readable before the next allocator algorithm row.

M172 already proved the page-map-backed release seam. This card does not change
that seam. It only replaces the proof app's long `&&` summary condition with an
app-local `ProofCheck` helper so each invariant has a label and a single
observer call.

## Accepted Order

1. Clean the M172 proof app with an app-local helper.
2. Keep `C198 check block surface` as a future language row after `C197`
   hardens ordinary `&&` / `||` condition usage.
3. Resume `M173 pre-realloc release invariant freeze`.
4. Continue `M174-M190` in the existing mimalloc roadmap order.

## Stop Line

This card does not add parser syntax, `check` blocks, compound assignment,
guard syntax, realloc, aligned allocation, huge allocation, secure-list
hardening, provider activation, hook install, process allocator replacement, or
`.inc` allocator-name matching.

The helper is proof-local. It must not become an allocator runtime owner.

## Proof

```bash
bash tools/checks/k2_wide_mimalloc_page_map_release_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
