---
Status: Landed
Date: 2026-04-28
Scope: next compiler-cleanliness lane selection after MIR root detection bridge prune
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-554-mir-root-detection-bridge-prune-card.md
  - src/mir/loop_route_detection/kind.rs
  - tools/checks/route_detector_legacy_surface_guard.sh
---

# 291x-555: Next Lane Selection

## Goal

Choose the next small compiler-cleanliness lane after the MIR root detection
bridge prune landed.

This card is lane selection only. No behavior changed.

## Candidate Review

| Candidate | Current shape | Decision |
| --- | --- | --- |
| `LoopPatternKind` legacy alias | live code has only the alias definition; callers already use `LoopRouteKind` | select next |
| `builder_calls::CallTarget` compatibility path | multiple builder callsites still depend on the compatibility path | defer; needs a focused builder import migration |
| route-shape wrapper names in `loop_canonicalizer` | public wrapper API still uses `try_extract_*`; active tests cover it | defer; larger API decision |
| Stage-A/runtime compat lane | runner/selfhost compatibility policy | defer; not same layer |
| CoreMethodContract -> CoreOp / LoweringPlan | semantic contract lane | defer; not BoxShape cleanup |

## Decision

Select **`LoopPatternKind` legacy alias prune** as the next lane.

Reason:

- `LoopRouteKind` is the current route vocabulary.
- `LoopPatternKind` is no longer used by live Rust code or tests.
- Historical/private docs can retain old wording as archives.
- The route detector legacy guard is the correct no-regrowth location.

## Next Card

Create `291x-556-loop-pattern-kind-alias-prune` before editing code.

Planned change:

```text
src/mir/loop_route_detection/kind.rs
  remove LoopPatternKind alias

tools/checks/route_detector_legacy_surface_guard.sh
  reject live-code LoopPatternKind reintroduction
```

## Acceptance

```bash
rg -n "\\bLoopPatternKind\\b" src tests -g'*.rs'
bash tools/checks/route_detector_legacy_surface_guard.sh
bash tools/checks/current_state_pointer_guard.sh
cargo check -q
git diff --check
```
