# P381GR JoinIR Test Surface Import Gating

Date: 2026-05-06
Scope: shrink normal-build JoinIR warning surface without changing behavior.

## Context

`cargo check --bin hakorune` still reported unused import/re-export warnings for
JoinIR helper surfaces that are only used by unit tests:

- `condition_lowerer::lower_condition_to_joinir`
- AST lowerer `HashSet` facade
- JoinIR VM bridge conversion facade imports

The same sweep exposed one broad `join_ir_vm_bridge` test that enabled debug
logging before Ring0 was initialized.

## Change

- Gated test-only imports/re-exports with `#[cfg(test)]`.
- Left normal runtime paths and public lowering behavior unchanged.
- Initialized Ring0 inside the debug-only select handler test before enabling
  debug logging.

## Result

The normal compiler build no longer carries those test-only import surfaces.
Observed `cargo check --bin hakorune` warning count:

```text
before: 41 warnings
after:  33 warnings
```

This is source/test-surface cleanup only. It does not add or remove compiler
acceptance shapes.

## Validation

```bash
cargo check --bin hakorune
cargo test -q condition_lowerer --lib
cargo test -q join_ir_vm_bridge --lib
```
