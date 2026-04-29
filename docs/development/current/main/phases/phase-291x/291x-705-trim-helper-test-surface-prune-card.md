---
Status: Landed
Date: 2026-04-29
Scope: trim helper cleanup
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/builder/control_flow/plan/trim_lowerer.rs
  - src/mir/builder/control_flow/plan/trim_validator.rs
---

# 291x-705: Trim Helper Test-Surface Prune

## Why

Worker-backed inventory plus direct owner-path checks showed that the trim helper
types were no longer used by production lowering:

- `TrimLowerer`
- `TrimValidator`

The remaining references were their own self-tests plus comment-only mentions in
future-facing trim/ConditionOnly notes, so keeping the helper shelf live in lib
builds only added dead-code noise.

## Changes

- gated `TrimLowerer` and its supporting imports behind `#[cfg(test)]`
- gated `TrimValidator` and its supporting imports behind `#[cfg(test)]`
- preserved the existing trim helper self-tests unchanged

## Result

- `cargo build --release` warning count moved from **12** to **8**
- trim helper utilities remain covered by dedicated tests while lib builds keep
  only the live trim planning surface

## Proof

```bash
cargo build --release
cargo test --release --lib test_add_to_condition_env -- --nocapture
cargo test --release --lib test_extract_substring_args_valid -- --nocapture
```
