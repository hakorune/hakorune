---
Status: Landed implementation card
Date: 2026-04-24
Scope: Quiet workspace/all-target Rust warnings without changing runtime semantics.
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - docs/development/current/main/phases/phase-291x/291x-126-rust-warning-quieting-card.md
---

# 291x-127 Workspace Warning Quieting Card

## Goal

Extend the previous root library warning cleanup to the broader workspace
all-target check.

This is a BoxShape / hygiene card. It does not add a new CoreBox row, route
shape, parser rule, runtime behavior, environment variable, or fallback.

## Contract

- `cargo check --workspace --all-targets` should emit no Rust compiler warnings.
- `RUSTFLAGS="-D warnings" cargo check --workspace --all-targets` should pass.
- Suppression must stay local to the staged seam that owns the warning.
- Do not add crate-wide `#![allow(dead_code)]`.
- Delete true legacy/commented-out leftovers instead of suppressing them.

## Quieted Buckets

- `hakorune_mir_builder` MetadataContext tests now pin the RegionId type via a
  test helper instead of relying on ambiguous inference.
- plugin legacy v1 leftovers were removed from the active TypeBox-only code
  paths where they were only comments or unused constants.
- `nyash_tlv` no longer reports the normal Rust-stub build mode as a Cargo
  warning.
- K2 value-codec borrowed-alias / string-slot helpers keep local staged-seam
  `allow` annotations with phase reasons.
- Linux all-target builds no longer compile the Windows notepad example's GUI
  state as unused code.

## Non-Goals

- changing plugin ABI behavior
- enabling new value-codec routes
- deleting staged borrowed-alias or string-slot route vocabulary
- globally suppressing future warnings
- changing dev-gate perf artifact policy

## Acceptance

```bash
cargo check --workspace --all-targets
RUSTFLAGS="-D warnings" cargo check --workspace --all-targets
cargo fmt -- --check
cargo test -p hakorune-mir-builder metadata_context
cargo test -p nyash_kernel value_codec -- --test-threads=1
cargo test -p nyash-filebox-plugin
cargo test -p nyash-fixture-plugin
bash tools/checks/current_state_pointer_guard.sh
tools/checks/dev_gate.sh quick
git diff --check
```
