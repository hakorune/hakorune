# 291x-789 PHI Query Wrapper Retire Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/mir/phi_query.rs`
- `docs/development/current/main/CURRENT_STATE.toml`

## Why

After `291x-784` removed the first dead `phi_query` facade, one narrow wrapper
still remained:

- `infer_phi_base_query`

Repository search showed it was only used by in-file tests. Runtime callers
already use the anchored SSOT path:

- `infer_phi_base_query_with_anchors`

## Decision

Retire the remaining wrapper and move tests to the anchored query path directly.

## Landed

- Removed `infer_phi_base_query`.
- Updated `phi_query` tests to build the value-def map and call
  `infer_phi_base_query_with_anchors` directly.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

`phi_query` no longer carries a local dead-code hold. The remaining MIR
structural vocabulary queue now mostly consists of active holds (`cond_profile`,
`hints`) rather than dead wrapper shelves.

## Proof

- `rg -n "infer_phi_base_query\\(" src tests -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
