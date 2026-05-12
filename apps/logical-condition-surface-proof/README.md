# logical-condition-surface-proof

Status: Active
Scope: C197 logical condition surface hardening.

This proof app fixes the ordinary boolean-condition surface:

- parenthesized multiline `&&` / `||` conditions
- leading logical operators on continuation lines
- `if` and `loop` condition usage
- RHS short-circuit preservation for assignment-expression side effects

It intentionally does not use proof `check` blocks. `check` is a separate eager
proof-list surface and must not be treated as an alias for ordinary
short-circuit conditions.

Run:

```bash
bash tools/checks/k2_wide_logical_condition_surface_guard.sh
```
