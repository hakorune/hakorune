# compound-assignment-surface-proof

Status: Active
Scope: C199 compound assignment surface.

This proof app fixes compound assignment as source sugar for the existing
assignment form:

- local variable targets
- field targets
- index targets
- `+=`, `-=`, `*=`, and `/=`

Run:

```bash
bash tools/checks/k2_wide_compound_assignment_surface_guard.sh
```
