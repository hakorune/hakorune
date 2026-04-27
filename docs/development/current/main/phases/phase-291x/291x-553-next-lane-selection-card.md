---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after MIR root facade guard closeout
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-552-mir-root-facade-comment-hygiene-card.md
  - src/mir/mod.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
  - src/mir/builder.rs
---

# 291x-553: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after the MIR root facade
export/import guards landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| MIR root crate-internal detection bridge | `src/mir/mod.rs` still re-exports loop-canonicalizer `detect_*` helpers via `pub(crate) use`; only `route_shape_recognizer` uses the root path | select next |
| `builder_calls::CallTarget` compatibility path | several builder callers still use `builder_calls::CallTarget`; likely a multi-file builder import cleanup | defer; larger surface |
| `LoopPatternKind` legacy alias | no active caller found, but belongs to loop-route detector API cleanup | defer behind root closeout |
| Stage-A/runtime compat lane | runner/selfhost compatibility policy, not CoreBox/MIR root cleanup | defer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; do not mix with BoxShape cleanup |

## Decision

Select **MIR root crate-internal detection bridge prune** as the next lane.

Reason:

- It is the smallest remaining root facade leak.
- The public root export allowlist is already fixed, but internal root aliases
  still make `src/mir/mod.rs` look like a convenience barrel.
- `src/mir/builder.rs` already owns the `detect_*` bridge to control-flow
  facts; the caller can import that owner path directly.
- The change is BoxShape-only and should not alter route detection behavior.

## Next Card

Create `291x-554-mir-root-detection-bridge-prune` before editing code.

Planned change:

```text
src/mir/loop_canonicalizer/route_shape_recognizer.rs
  crate::mir::detect_* -> crate::mir::builder::detect_*

src/mir/mod.rs
  remove pub(crate) use builder::detect_* aliases
```

## Acceptance

```bash
rg -n "crate::mir::detect_|pub\\(crate\\) use builder::detect" src/mir -g'*.rs'
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/mir_root_import_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
git diff --check
```
