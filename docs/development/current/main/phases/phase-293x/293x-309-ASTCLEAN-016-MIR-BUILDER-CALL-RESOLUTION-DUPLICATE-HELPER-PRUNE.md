# 293x-309 ASTCLEAN-016 MIR builder call resolution duplicate helper prune

Status: complete

## Decision

Decision: accepted.

`src/mir/builder/call_resolution.rs` keeps global/external/suggestion helpers used by call build paths. Shadowed-method and self-recursion warning helpers are owned by `calls/method_resolution.rs`, so duplicate test-only helpers in `call_resolution.rs` should be removed.

## Scope

- Remove duplicate `is_commonly_shadowed_method` helper from `call_resolution.rs`.
- Remove duplicate `generate_self_recursion_warning` helper from `call_resolution.rs`.
- Remove the tests that only covered those duplicate helpers.
- Keep live global/external/suggestion helpers intact.

## Non-goals

- No call resolution behavior change.
- No method-resolution warning behavior change.
- No call build path change.

## Guard

- `tools/checks/k2_wide_astclean_call_resolution_duplicate_helper_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_call_resolution_duplicate_helper_guard.sh` passed locally.
