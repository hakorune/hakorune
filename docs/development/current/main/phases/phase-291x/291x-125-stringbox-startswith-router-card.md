---
Status: Landed implementation card
Date: 2026-04-24
Scope: Promote `StringBox.startsWith(prefix)` into the phase-291x catalog-backed value path.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/README.md
  - docs/development/current/main/phases/phase-291x/291x-90-corebox-surface-catalog-design-brief.md
  - docs/development/current/main/phases/phase-291x/291x-91-stringbox-surface-task-board.md
  - docs/development/current/main/phases/phase-291x/291x-92-corebox-surface-inventory-ledger.md
  - docs/development/current/main/phases/phase-291x/291x-96-corebox-router-unified-value-path-card.md
---

# 291x-125 StringBox startsWith Router Card

## Goal

Close one deferred StringBox surface row:

```text
StringBox.startsWith(prefix) -> Bool
```

This is a StringBox-only BoxCount cleanup. It must not promote `endsWith`,
`split`, `charAt`, `equals`, MapBox rows, or std sugar policy.

## Contract

- canonical method: `startsWith`
- aliases: none
- arity: `1`
- slot: `310`
- effect: `Read`
- return: `Value` / Bool
- empty prefix returns `true`
- receiver-local duplicate argument normalization stays at the method boundary

Examples:

```text
"banana".startsWith("ban") -> true
"banana".startsWith("nan") -> false
"banana".startsWith("")    -> true
```

## Required Edges

- Rust catalog row in `src/boxes/basic/string_surface_catalog.rs`
- router Unified allowlist for `startsWith/1`
- type annotation returns `MirType::Bool`
- Rust VM slot dispatch consumes the catalog row
- `.hako` VM-facing `StringCoreBox` delegates to `StringSearchKernelBox.starts_with(...)`
- phase-291x StringBox smoke pins output and no-stub drift
- dedicated vm-hako smoke pins source-level Bool publication

## Non-Goals

- `endsWith`
- `split`
- `charAt`
- `equals`
- std module import-policy cleanup
- `.inc` codegen thinning

## Landing Snapshot

- Rust catalog row: `StringMethodId::StartsWith`
- Rust slot: `310`
- Rust VM boundary: `startsWith/1` uses the catalog-backed surface invocation
- `.hako` owner: `StringSearchKernelBox.starts_with(...)`
- smoke:
  `tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_startswith_vm.sh`

## Acceptance

```bash
cargo test string_surface_catalog --lib
cargo test router --lib
cargo test corebox_surface_aliases_use_catalog_return_type --lib
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_surface_catalog_vm.sh
bash tools/smokes/v2/profiles/integration/apps/phase291x_stringbox_startswith_vm.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```
