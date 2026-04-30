# 291x-785 If-Else PHI Helper Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/facts/route_shape_recognizers/if_else_phi.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory on the extractor/detector family split the module into:

- active route-shape vocabulary: `detect_if_else_phi_in_body`
- dead helper shelf: `detect_if_in_body`

Repository evidence showed no `src/` or `tests/` callers for
`detect_if_in_body`, while `detect_if_else_phi_in_body` remains live via
`ast_feature_extractor.rs`.

## Decision

Delete only the zero-use `detect_if_in_body` helper. Keep the active if-else
detector untouched.

## Landed

- Removed the unused `detect_if_in_body` helper and its local dead-code hold.
- Left the live if-else detector path unchanged.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

The extractor/detector family is narrower after this card:

- `if_else_phi` keeps only its live detector surface
- `loop_simple_while` still needs vocabulary review because
  `validate_loop_condition_structure` is reused live
- LocalSSA finalizer seams remain a separate decision

## Proof

- `rg -n "detect_if_in_body|detect_if_else_phi_in_body" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
