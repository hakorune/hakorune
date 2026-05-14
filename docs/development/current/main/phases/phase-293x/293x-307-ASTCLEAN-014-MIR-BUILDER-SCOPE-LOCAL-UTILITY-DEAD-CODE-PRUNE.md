# 293x-307 ASTCLEAN-014 MIR builder scope/local utility dead_code prune

Status: complete

## Decision

Decision: accepted.

MIR builder scope/local utilities should not keep test-only helper methods or stale allowances on live helper wrappers. Active fields and wrappers stay direct; unused wrappers are deleted.

## Scope

- Remove unused loop-header/loop-exit helper methods from `scope_context.rs`.
- Keep the loop stack state itself and update the local unit test to inspect the stack directly.
- Remove stale `dead_code` allowances from live LocalSSA wrappers.
- Delete the unused `local_cmp_operand` wrapper.

## Non-goals

- No MIR builder behavior change.
- No LocalSSA owner change.
- No loop lowering or context lifecycle change.

## Guard

- `tools/checks/k2_wide_astclean_mir_builder_scope_local_guard.sh`

## Local guard

- `bash tools/checks/k2_wide_astclean_mir_builder_scope_local_guard.sh` passed locally.
