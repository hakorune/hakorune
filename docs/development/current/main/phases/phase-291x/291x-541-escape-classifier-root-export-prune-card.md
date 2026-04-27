---
Status: Landed
Date: 2026-04-27
Scope: Prune escape classifier helper from the MIR root facade
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/development/current/main/phases/phase-291x/291x-529-escape-barrier-root-export-prune-card.md
  - src/mir/mod.rs
  - src/mir/escape_barrier.rs
---

# 291x-541: Escape Classifier Root Export Prune

## Goal

Finish the escape-barrier root export cleanup by keeping the
`classify_escape_uses` helper on its owner module instead of the MIR root
facade.

291x-529 removed the escape-barrier vocabulary from the root but kept this
helper as a temporary entry point. After the MIR root facade contract was
written, the cleaner boundary is explicit owner-module imports.

## Inventory

Removed root export:

- `classify_escape_uses`

Migrated consumers:

- `src/mir/passes/dce/local_fields.rs`
- `src/mir/passes/escape.rs`

## Cleaner Boundary

```text
mir::escape_barrier
  owns escape-use classification helpers and vocabulary

mir root
  does not re-export escape classifier helper vocabulary
```

## Boundaries

- BoxShape-only.
- Do not change escape-use classification behavior.
- Do not change DCE or escape pass behavior.
- Do not change barrier insertion/removal semantics.

## Acceptance

- MIR root no longer re-exports `classify_escape_uses`.
- Consumers use `mir::escape_barrier::classify_escape_uses`.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `git diff --check` passes.

## Result

- Removed the last escape-barrier helper from the MIR root export surface.
- Preserved escape classifier behavior.

## Verification

```bash
cargo check -q
cargo fmt -- --check
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
git diff --check
```
