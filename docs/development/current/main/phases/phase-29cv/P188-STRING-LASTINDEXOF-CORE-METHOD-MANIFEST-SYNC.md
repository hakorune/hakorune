---
Status: Accepted
Decision: accepted
Date: 2026-05-02
Scope: phase-29cv P188, StringLastIndexOf CoreMethod manifest sync
Related:
  - docs/development/current/main/phases/phase-29cv/P185-STRING-LASTINDEXOF-DIRECTABI-CONSUME.md
  - lang/src/runtime/meta/core_method_contract_box.hako
  - lang/src/runtime/meta/generated/core_method_contract_manifest.json
  - src/mir/core_method_op.rs
---

# P188: StringLastIndexOf CoreMethod Manifest Sync

## Problem

P185 added MIR carrier vocabulary for `StringLastIndexOf`, but the `.hako`
CoreMethod contract manifest still stopped at `StringIndexOf`.

That leaves this SSOT test red:

```text
mir::core_method_op::tests::manifest_core_ops_are_known_by_mir_carrier
```

The next blocker needs another string method route, so CoreMethod vocabulary
must be clean before adding more rows.

## Decision

Sync the `.hako` CoreMethod contract and generated manifest with the already
landed MIR carrier:

```text
StringBox.lastIndexOf/1
core_op=StringLastIndexOf
lowering_tier=warm_direct_abi
cold_lowering=nyash.string.lastIndexOf_hh
```

This is a contract/manifest cleanup only. It does not add a new runtime route;
the `lastIndexOf` route was already landed in P185.

## Acceptance

```bash
cargo test -q manifest_core_ops_are_known_by_mir_carrier --lib
cargo test -q lastindexof --lib
cargo fmt --check
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
