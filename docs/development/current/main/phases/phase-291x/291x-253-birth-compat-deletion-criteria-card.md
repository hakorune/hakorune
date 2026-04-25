---
Status: Landed
Date: 2026-04-25
Scope: Fix the deletion criteria for the remaining generic-method `birth` compatibility row without pruning it yet.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-249-constructor-birth-compatibility-contract-card.md
  - docs/development/current/main/phases/phase-291x/291x-250-constructor-birth-carrier-design-card.md
  - docs/development/current/main/phases/phase-291x/291x-251-constructor-birth-owner-shape-decision-card.md
  - docs/development/current/main/phases/phase-291x/291x-252-constructor-birth-marker-helper-card.md
  - src/mir/builder/calls/emit.rs
  - src/mir/builder/decls.rs
  - lang/c-abi/shims/hako_llvmc_ffi_generic_method_policy.inc
  - tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh
---

# 291x-253 Birth Compat Deletion Criteria Card

## Goal

Make the remaining generic-method `birth` compatibility row auditable before
any prune attempt.

This is a BoxShape cleanup. It does not add a new accepted shape and does not
remove the `.inc` `birth` classifier row.

## Change

The `main(args)` bootstrap `ArrayBox` allocation now uses the same constructor
birth-marker helper as array/map literals:

```text
NewBox(ArrayBox)
  -> emit_constructor_birth_marker(...)
  -> push(script args)
```

This removes one more inline `birth()` construction site from
[`src/mir/builder/decls.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/decls.rs)
and keeps the transitional marker emission owned by
[`src/mir/builder/calls/emit.rs`](/home/tomoaki/git/hakorune-selfhost/src/mir/builder/calls/emit.rs).

The archived `main(args)` capability smoke now resolves its shared helper from
the integration vm-hako caps library through the repo root, matching the already
repaired archived smokes.

## Keep Boundary

Do not prune the generic-method `birth` row in this card.

The row still protects metadata-absent or compatibility surfaces that can
present `ArrayBox.birth()` / `MapBox.birth()` as a normal method call:

- llvmlite harness compatibility for explicit `ArrayBox.birth()` after
  `newbox ArrayBox`
- `main(args)` MIR-shape compatibility that still observes `NewBox -> birth`
- dev verifier / normalizer assumptions around the transitional
  `NewBox -> birth` invariant
- env-guarded builtin constructor fallback in the general constructor path

The general constructor fallback in `builder_build.rs` remains out of scope for
this card because it mixes user-box constructor dispatch with builtin fallback
compatibility.

## Deletion Criteria

A future prune card may remove the `birth` row only after all of these are true:

- all compiler-owned constructor marker emissions flow through
  `emit_constructor_birth_marker(...)` or a successor constructor contract
- metadata-absent boundary MIR JSON no longer relies on method-name
  classification for `ArrayBox.birth()` / `MapBox.birth()`
- the llvmlite harness compatibility smoke has either moved to constructor
  metadata or been intentionally retired with a replacement contract
- `main(args)` bootstrap and literal construction smokes still prove the
  initialization path without depending on `.inc` method-name classification
- the env-guarded builtin fallback path is either removed or covered by a
  non-method-name constructor contract

## Result

The remaining `birth` mirror is now explicitly classified as compatibility
debt, not an active semantic owner. The next safe step is a focused prune probe:
temporarily remove only the `birth` row, run the pinned smokes, and either land
the prune or document the exact failing metadata-absent boundary.

## Acceptance

```bash
cargo check -q
bash tools/checks/current_state_pointer_guard.sh
bash tools/checks/core_method_contract_inc_no_growth_guard.sh
bash tools/smokes/v2/profiles/integration/joinir/phase29bq_harness_arraybox_birth_ternary_basic_vm.sh
bash tools/smokes/v2/profiles/archive/vm_hako_caps/args/args_vm.sh
git diff --check
```
