---
Status: Landed
Date: 2026-04-28
Scope: guard unified-member parser test environment variables
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/tests/helpers/env.rs
  - src/tests/parser_unified_members_get.rs
  - src/tests/parser_unified_members_property_emit.rs
  - src/tests/parser_weak_field_contract.rs
  - src/tests/parser_birth_once_cycle.rs
---

# 291x-651: Unified Member Test Env Guard

## Goal

Stop unified-member parser tests from leaking `NYASH_ENABLE_UNIFIED_MEMBERS`
across the test process.

This is test-only BoxShape cleanup. It does not change parser or compiler
behavior.

## Evidence

Several parser regressions used direct global mutation:

```rust
std::env::set_var("NYASH_ENABLE_UNIFIED_MEMBERS", "1");
```

without restoring the previous value. That can make parser behavior
order-dependent when tests share the same process.

## Decision

Add a shared test ENV helper that serializes scoped environment mutation and
restores the previous value on drop.

Use it in the unified-member parser tests that need
`NYASH_ENABLE_UNIFIED_MEMBERS=1`.

## Boundaries

- Do not change production ENV access.
- Do not broaden this card to unrelated MIR/router ENV tests.
- Do not change parser assertions.

## Acceptance

```bash
cargo fmt
cargo test parser_unified_members_get --lib
cargo test parser_unified_members_property_emit --lib
cargo test parser_weak_field_contract --lib
cargo test parser_birth_once_cycle --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Added `tests::helpers::env::with_env_var(...)`.
- Converted unified-member parser tests to scoped, locked ENV mutation.
