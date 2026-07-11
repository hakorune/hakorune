# 291x-770 PHI Metrics Env Accessor Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/config/env/joinir_dev.rs`
- `docs/development/current/main/design/environment-variables-inventory-ssot.md`
- `CURRENT_STATE.toml`

## Why

`phi_metrics_enabled()` was a dead accessor for `NYASH_PHI_METRICS` and carried
the remaining config-side `#[allow(dead_code)]`.

Current environment-variable inventory still listed `NYASH_PHI_METRICS` as a
low-priority documentation candidate, but there was no active implementation
caller and no reference docs entry.

## Decision

Remove the dead accessor instead of documenting a no-op metric toggle. Drop the
current inventory rows so the environment-variable SSOT no longer asks future
cleanup to preserve the retired toggle.

The historical Phase 84 archive remains untouched as archive evidence.

## Landed

- Removed `phi_metrics_enabled()`.
- Removed the stale `phi_fallback_disabled()` no-op accessor and Phase 82
  diagnostic comments from the active env module.
- Removed `NYASH_PHI_METRICS` from current env inventory tables and low-priority
  action items.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

This closes the config-side `#[allow(dead_code)]` item in `joinir_dev.rs`.

## Proof

- `cargo test --lib --no-run`
- `bash tools/checks/current_state_pointer_guard.sh`
- `git diff --check`
