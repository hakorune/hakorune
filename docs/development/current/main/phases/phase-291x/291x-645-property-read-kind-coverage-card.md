---
Status: Landed
Date: 2026-04-28
Scope: cover MIR property reads for all unified member property kinds
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/tests/mir_unified_members_property_read.rs
---

# 291x-645: Property Read Kind Coverage

## Goal

Pin MIR property-read lowering for all unified member property getter families.

This is test-only BoxShape cleanup. It does not change syntax, registry logic,
or MIR lowering.

## Evidence

The MIR property registry supports three getter families:

```text
computed   -> __get_<name>
once       -> __get_once_<name>
birth_once -> __get_birth_<name>
```

The existing MIR regression only covered computed `get`. Parser tests covered
synthetic emission for all three, but not MIR read lowering/registration.

## Decision

Extend the MIR property-read regression to cover computed, once, and
birth_once reads.

Each case asserts:

- one receiver `NewBox`;
- one getter call for the expected synthetic getter;
- no plain `FieldGet` for the source property name.

## Acceptance

```bash
cargo fmt
cargo test mir_unified_members_property_read --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- Kept the original computed property read receiver-reuse regression.
- Added once and birth_once property read cases.
- Shared the assertion helper so each case checks one receiver `NewBox`, one
  expected synthetic getter call, and no source-property `FieldGet`.
