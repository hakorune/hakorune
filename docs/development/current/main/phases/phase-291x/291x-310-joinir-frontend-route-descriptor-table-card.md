---
Status: Landed
Date: 2026-04-26
Scope: JoinIR frontend route descriptor table split
Related:
  - src/mir/join_ir/frontend/ast_lowerer/route.rs
  - docs/development/current/main/phases/phase-291x/291x-309-joinir-residual-name-policy-inventory-card.md
---

# 291x-310: JoinIR Frontend Route Descriptor Table

## Goal

Unify AST frontend route keys and dev gates into one descriptor table.

This is behavior-preserving BoxShape cleanup.

## Change

Added local route descriptors:

```text
FunctionRouteDesc { name, route, gate }
FunctionRouteGate::{Always, NestedIfDev, ReadQuotedDev}
FUNCTION_ROUTES
```

Removed the split local policy arrays:

```text
TABLE
NESTED_IF_KEYS
READ_QUOTED_KEYS
```

`resolve_function_route(...)` now reads one table row, then either returns the
route or emits the same dev-gate error as before.

## Preserved Behavior

Always-enabled routes remain:

```text
test
local
_read_value_from_pair
simple
```

Dev-gated routes remain:

```text
nested_if_merge -> HAKO_JOINIR_NESTED_IF=1
read_quoted     -> HAKO_JOINIR_READ_QUOTED=1
```

Unsupported and retired keys still fail through the unsupported-function path.

## Non-Goals

- No accepted route expansion.
- No accepted route deletion.
- No env gate behavior change.
- No Case-A loop fallback dispatch cleanup.
- No bridge target or type-hint policy change.

## Validation

```bash
cargo test -q current_program_json_route_keys_resolve_to_expected_routes
cargo test -q read_quoted_dev_keys_fail_fast_without_env
cargo test -q nested_if_dev_keys_fail_fast_without_env
cargo test -q joinir_frontend_
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
