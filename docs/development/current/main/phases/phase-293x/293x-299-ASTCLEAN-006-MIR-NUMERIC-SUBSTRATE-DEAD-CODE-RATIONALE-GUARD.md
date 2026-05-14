# 293x-299 ASTCLEAN-006 MIR numeric substrate dead_code rationale guard

Status: complete

## Decision

Decision: accepted.

`src/mir/numeric_substrate.rs` intentionally contains staged exact-numeric substrate vocabulary. Its `#[allow(dead_code)]` attributes are acceptable only when each line carries a row/rationale comment that explains the future consumer.

## Scope

- Guard `numeric_substrate.rs` against bare `#[allow(dead_code)]` attributes.
- Keep exact numeric substrate APIs intact.
- Treat numeric substrate as intentional staging, not as a bulk deletion target.

## Non-goals

- No numeric substrate behavior change.
- No exact numeric API deletion.
- No backend/runtime exact numeric lowering change.

## Guard

- `tools/checks/k2_wide_astclean_numeric_substrate_dead_code_rationale_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_numeric_substrate_dead_code_rationale_guard.sh` passed locally.
