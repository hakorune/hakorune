# Phase 253x — joinAll first-failure surface

Status: LANDED
Date: 2026-04-13
Scope: `TaskGroupBox.joinAll(timeout_ms)` only

## Goal

- align `joinAll()` with the same first-failure latch used by explicit scope exit
- preserve the first failure as a box payload instead of only a rendered string
- keep timeout and aggregate reporting out of this cut

## Landed slices

1. pin `joinAll()` as `ResultBox::Ok(void)` / `ResultBox::Err(first_failure_payload)`
2. preserve first-failure owner state as a box payload
3. keep timeout folded as no-new-error in this cut
4. add focused `joinAll()` regressions

## Still out

- aggregate / multi-failure reporting
- dedicated timeout payload for `joinAll()`
- implicit root-scope join surface
