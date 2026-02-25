# CoreLoopComposer v0/v1 boundary SSOT

This document fixes the responsibility boundary between CoreLoopComposer v0 and v1.

## Scope

- v0 handles loops where `value_join_needed == false`.
- v1 handles loops where `value_join_needed == true`.
- The selection is made at the composer entrypoints (not in the router).

## Gates (current minimal contract)

- v0: `value_join_needed == false`, `skeleton_kind == Loop`, return-only exits, no cleanup.
- v1: `value_join_needed == true`, `skeleton_kind == Loop`, return-only exits, no cleanup.

## Behavior

- If the gate fails, v0/v1 returns `Ok(None)` and the caller falls back to legacy paths.
- In strict/dev shadow adopt, a `None` result is treated as fail-fast with a clear error.

## Notes

- Pattern6 (ScanWithInit) currently has no v1 composition yet; v1 must return `None`.
- Pattern2/3/5/7 value-join paths are intended to live in v1.
