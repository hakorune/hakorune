# P381GU JoinIR Return Collector Test Surface Gating

Date: 2026-05-06
Scope: gate the unused JoinIR return collector module to test builds.

## Context

`return_collector` is a Phase 284 P1 return-detection SSOT, but the current
normal compiler path does not import it. Its active coverage is the local unit
test suite.

## Change

- Gated `return_collector` with `#[cfg(test)]` at the JoinIR lowering module
  boundary.
- Left the module and its tests intact for future reactivation.

## Result

Observed `cargo check --bin hakorune` warning count:

```text
before: 28 warnings
after:  24 warnings
```

This is test-surface cleanup only. It does not change loop lowering behavior or
compiler acceptance shapes.

## Validation

```bash
cargo check --bin hakorune
cargo test -q return_collector --lib
```
