# 293x-298 ASTCLEAN-005 MIR TypeRegistry dead_code allow prune

Status: complete

## Decision

Decision: accepted.

The first dead-code allowance prune pilot is scoped to `src/mir/builder/type_registry.rs`. Used items should not carry `#[allow(dead_code)]`; genuinely staged/debug-only items may keep the attribute only with an explicit row reason.

## Scope

- Remove obsolete `#[allow(dead_code)]` from used TypeRegistry entries.
- Remove the unused `get_type` getter instead of leaving a warning-producing API stub.
- Keep staged/debug-only TypeRegistry allowances with `ASTCLEAN-005` reason comments.
- Preserve TypeRegistry behavior and tracing behavior.

## Non-goals

- No broad MIR cleanup.
- No TypeRegistry API deletion.
- No trace env var or logging behavior change.

## Guard

- `tools/checks/k2_wide_astclean_mir_typeregistry_dead_code_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_mir_typeregistry_dead_code_guard.sh` passed locally.
