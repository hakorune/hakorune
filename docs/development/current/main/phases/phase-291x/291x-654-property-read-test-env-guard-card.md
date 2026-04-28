---
Status: Landed
Date: 2026-04-28
Scope: reuse shared env guard in MIR property read tests
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/tests/mir_unified_members_property_read.rs
  - src/tests/helpers/env.rs
---

# 291x-654: Property Read Test Env Guard

## Goal

Use the shared test ENV helper for MIR property-read tests.

This is test-only BoxShape cleanup. It does not change parser, MIR lowering, or
property behavior.

## Evidence

After unified-member parser tests moved to `tests::helpers::env::with_env_var`,
`mir_unified_members_property_read.rs` still carried a local `EnvGuard` for the
same `NYASH_ENABLE_UNIFIED_MEMBERS` flag.

That duplicated a test-only pattern and left one more local way to mutate
process-global test ENV.

## Decision

Replace the local MIR property-read `EnvGuard` with the shared scoped helper.

## Boundaries

- Do not broaden this card to unrelated router tests.
- Do not change MIR property-read assertions.
- Do not change production ENV access.

## Acceptance

```bash
cargo fmt
cargo test mir_unified_members_property_read --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Removed the local MIR property-read `EnvGuard`.
- Reused `tests::helpers::env::with_env_var(...)` for unified-member test ENV.
