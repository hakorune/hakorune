# P381GT JoinIR Lowering Test Helper Gating

Date: 2026-05-06
Scope: gate JoinIR lowering helpers that are only used by unit tests.

## Context

After P381GS, `cargo check --bin hakorune` still reported normal-build dead-code
warnings for two JoinIR lowering helper surfaces:

- `condition_var_extractor`
- private `TypeHintPolicy` phase-family accessors

Both are still covered by unit tests, but neither is used by the normal compiler
path.

## Change

- Gated `condition_var_extractor` and its legacy facade re-export with
  `#[cfg(test)]`.
- Gated the private P1/P2/P3A/P3B type-hint family helpers with `#[cfg(test)]`.
- Kept production policy entry points (`is_target`, `is_p3c_target`,
  `extract_phi_type_hint`) unchanged.

## Result

Observed `cargo check --bin hakorune` warning count:

```text
before: 32 warnings
after:  28 warnings
```

This is test-surface cleanup only. It does not change JoinIR lowering policy or
compiler acceptance shapes.

## Validation

```bash
cargo check --bin hakorune
cargo test -q condition_var_extractor --lib
cargo test -q condition_to_joinir --lib
cargo test -q type_hint_policy --lib
```
