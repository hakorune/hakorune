# check-block-surface-proof

Status: Active
Scope: C198 check block surface.

This proof app fixes `check "name" { "label": expr }` as an eager proof-list
expression:

- every item is evaluated left-to-right
- later items still run after an earlier failure
- the result is scalar `1` for all-pass and `0` for any failure
- labels are source-level proof metadata only in this row

Run:

```bash
bash tools/checks/k2_wide_check_block_surface_guard.sh
```
