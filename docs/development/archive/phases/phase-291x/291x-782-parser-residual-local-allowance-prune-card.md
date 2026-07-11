# 291x-782 Parser Residual Local Allowance Prune Card

Status: Landed
Date: 2026-04-30
Lane: phase-291x CoreBox surface contract cleanup

## Scope

- `src/parser/sugar.rs`
- `src/parser/expr/ternary.rs`
- `docs/development/current/main/CURRENT_STATE.toml`
- `docs/development/current/main/phases/phase-291x/README.md`

## Why

After 291x-781 removed the expression/common helper shelves, a broad parser
scan still found two local `#[allow(dead_code)]` items:

- `make_eq_null` in `src/parser/sugar.rs`
- `is_sugar_enabled` in `src/parser/expr/ternary.rs`

Both were unused helper leftovers, not structural vocabulary.

## Decision

Delete the unused helpers and leave parser structural follow-up out of this
card. This closes the parser dead-code allowance slice found by the worker
inventory and final broad parser scan.

## Landed

- Removed unused `make_eq_null`.
- Removed unused ternary `is_sugar_enabled`.
- Advanced `CURRENT_STATE.toml` to this card.

## Remaining Queue Impact

Parser cleanup from this pass is closed. Remaining cleanup is MIR structural
vocabulary / owner-seam inventory.

## Proof

- `rg -n "allow\\(dead_code\\)" src/parser -g '*.rs'`
- `bash tools/checks/current_state_pointer_guard.sh`
- `cargo fmt --check`
- `cargo test --lib --no-run`
- `git diff --check`
