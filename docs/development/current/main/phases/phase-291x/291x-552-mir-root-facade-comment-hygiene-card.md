---
Status: Landed
Date: 2026-04-27
Scope: Clean MIR root facade comments after guard closeout
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - src/mir/mod.rs
  - tools/checks/mir_root_facade_guard.sh
  - tools/checks/mir_root_import_hygiene_guard.sh
---

# 291x-552: MIR Root Facade Comment Hygiene

## Goal

Remove stale wording from `src/mir/mod.rs` after the root facade export and
import guards landed.

The old comments described root exports as "easy access" and left one
crate-internal loop-canonicalizer bridge separated from its comment. That shape
invites future root-surface regrowth by making the root look like a convenience
barrel instead of a controlled facade.

## Cleaner Boundary

```text
MIR root facade exports
  core MIR surfaces and refresh orchestration only

crate-internal detection bridge
  loop-canonicalizer shape-detection aliases only

semantic metadata vocabulary
  owner modules only
```

## Boundaries

- BoxShape-only.
- Do not change MIR behavior.
- Do not change public root export symbols.
- Do not update the root facade allowlist.
- Do not rewrite loop-canonicalizer ownership in this card.

## Acceptance

- `src/mir/mod.rs` comments point at the root facade SSOT and allowlist guard.
- Crate-internal detection aliases are grouped under one bridge comment.
- Public facade refresh exports are separated from crate-internal aliases.
- `bash tools/checks/mir_root_facade_guard.sh` passes.
- `bash tools/checks/mir_root_import_hygiene_guard.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Replaced "easy access" wording with explicit root facade contract wording.
- Grouped the crate-internal loop-canonicalizer detection aliases together.
- Added a public facade exports comment that limits the root to core surfaces
  and refresh orchestration.

## Verification

```bash
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/mir_root_import_hygiene_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
