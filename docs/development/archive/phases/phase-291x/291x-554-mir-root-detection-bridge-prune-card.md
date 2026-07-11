---
Status: Landed
Date: 2026-04-28
Scope: Prune crate-internal loop-canonicalizer detection aliases from the MIR root
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/design/mir-root-facade-contract-ssot.md
  - docs/tools/check-scripts-index.md
  - src/mir/mod.rs
  - src/mir/builder.rs
  - src/mir/loop_canonicalizer/route_shape_recognizer.rs
  - tools/checks/mir_root_import_hygiene_guard.sh
---

# 291x-554: MIR Root Detection Bridge Prune

## Goal

Remove the remaining crate-internal loop-canonicalizer detection bridge from
`src/mir/mod.rs`.

The MIR root facade should not be a convenience barrel, even for crate-internal
aliases. `src/mir/builder.rs` already owns the bridge from builder/control-flow
facts to consumers, so the loop-canonicalizer wrapper can import that owner path
directly.

## Inventory

Removed root aliases:

- `detect_skip_whitespace_shape`
- `detect_read_digits_loop_true_shape`
- `detect_continue_shape`
- `detect_parse_number_shape`
- `detect_parse_string_shape`
- `detect_escape_skip_shape`

Migrated caller:

- `src/mir/loop_canonicalizer/route_shape_recognizer.rs`

Guarded regrowth:

- `tools/checks/mir_root_import_hygiene_guard.sh`

## Cleaner Boundary

```text
src/mir/builder.rs
  owns the crate-internal detection bridge

src/mir/loop_canonicalizer/route_shape_recognizer.rs
  imports detection helpers from crate::mir::builder

src/mir/mod.rs
  stays public facade-oriented; no internal detect_* aliases
```

## Boundaries

- BoxShape-only.
- Do not change route detection behavior.
- Do not change the public MIR root export allowlist.
- Do not move detection implementation owners.
- Do not touch CoreMethodContract/CoreOp or `.inc` lanes.

## Acceptance

- No `crate::mir::detect_*` callers remain.
- No `pub(crate) use builder::detect_*` aliases remain in `src/mir/mod.rs`.
- `bash tools/checks/mir_root_import_hygiene_guard.sh` passes.
- `bash tools/checks/mir_root_facade_guard.sh` passes.
- `bash tools/checks/current_state_pointer_guard.sh` passes.
- `bash tools/checks/core_method_contract_inc_no_growth_guard.sh` passes.
- `cargo check -q` passes.
- `cargo fmt -- --check` passes.
- `git diff --check` passes.

## Result

- Repointed the loop-canonicalizer route-shape wrapper to
  `crate::mir::builder`.
- Removed six crate-internal aliases from the MIR root.
- Extended the MIR root import hygiene guard so the bridge cannot regrow.

## Verification

```bash
rg -n "crate::mir::detect_|pub\\(crate\\) use builder::detect" src/mir -g'*.rs'
bash tools/checks/mir_root_import_hygiene_guard.sh
bash tools/checks/mir_root_facade_guard.sh
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
cargo check -q
cargo fmt -- --check
git diff --check
```
