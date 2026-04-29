# 291x-750 CleanupKindFacts Variant Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/builder/control_flow/plan/facts/feature_facts.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

Worker inventory marked `CleanupKindFacts::{Break,Continue}` as delete
candidates. `Return` is still constructed in the canonicalization test surface,
but `Break` and `Continue` were only future vocabulary in the enum definition.

Keeping unused cleanup variants made the cleanup facts contract look broader
than the current producer/consumer path.

## Decision

Remove only the unconstructed `Break` and `Continue` variants.

Do not remove `CleanupKindFacts::Return` in this card. It remains a separate
test-surface/structural decision because current normalizer tests still build
that cleanup fact.

## Landed

- Removed `CleanupKindFacts::Break`.
- Removed `CleanupKindFacts::Continue`.
- Removed the now-unneeded variant-level `#[allow(dead_code)]` marker.

## Remaining Queue Impact

The cleanup facts queue now contains only the structural `Return` question:

- keep it as a test/dev observation slot
- or remove the cleanup fact slot after replacing old coreloop v1 tests

## Proof

- `rg -n "CleanupKindFacts::Break|CleanupKindFacts::Continue" src tests docs/development/current/main -g '*.rs' -g '*.md'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
- `cargo test --lib --no-run`
