---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after builder context comment hygiene
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-562-builder-context-comment-hygiene-card.md
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
---

# 291x-563: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after builder context comment
hygiene landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| loop-canonicalizer route-shape wrapper stale compatibility wording | wrappers are live canonicalizer adapters, but one comment still says "backward compatibility" | select next |
| route-shape wrapper API cleanup | function tuple API is live and needs a larger API decision | defer |
| loop-route detection support comments | mostly owner-path guidance, not stale enough for a focused card | defer |
| direct `MirBuilder` context-field compatibility comments | struct fields are intentionally exposed to builder submodules | defer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; not BoxShape cleanup |

## Decision

Select **loop-canonicalizer route-shape wrapper wording cleanup** as the next
lane.

Reason:

- `route_shape_recognizer.rs` is now an adapter over builder-owned detector
  facts.
- Calling it "backward compatibility" makes it look temporary and legacy-owned.
- The cleanup is comment-only and keeps the live adapter API unchanged.

## Next Card

Create `291x-564-loop-canonicalizer-wrapper-wording-card` before editing code.

Planned change:

```text
src/mir/loop_canonicalizer/route_shape_recognizer.rs
  backward compatibility wording -> adapter/owner-path wording
```

## Acceptance

```bash
rg -n "backward compatibility" src/mir/loop_canonicalizer -g'*.rs'
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
