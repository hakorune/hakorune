---
Status: Landed
Date: 2026-04-28
Scope: align DigitPos reject-message tests
Related:
  - docs/development/current/main/CURRENT_STATE.toml
  - src/mir/loop_route_detection/support/body_local/digitpos.rs
---

# 291x-661: DigitPos Reject Message Test Contract

## Goal

Align stale DigitPos promoter tests with the current reject message contract.

This is test-contract cleanup. It must not change DigitPos detection,
promotion, carrier construction, or route behavior.

## Evidence

`cargo test body_local --lib` exposed three stale assertions in
`DigitPosPromoter` tests. The implementation returns:

```text
No A-4 DigitPos route shape detected (indexOf not found or not cascading)
```

The tests still expected the old substring:

```text
DigitPos pattern
```

Single-test reproduction showed the failure is independent of the
`BodyLocalRoute` facade prune.

## Decision

Update the three stale assertions to check the current stable reject substring:

```text
A-4 DigitPos route shape
```

## Boundaries

- Do not change `DigitPosPromoter::try_promote`.
- Do not change `DigitPosDetector`.
- Do not change carrier names or promotion decisions.
- Keep this separate from BodyLocalRoute facade cleanup.

## Acceptance

```bash
cargo fmt
cargo test body_local --lib
cargo check --release --bin hakorune -q
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

- DigitPos reject-message tests now assert the current route-shape reject
  contract.
- `cargo test body_local --lib` is green.
- DigitPos runtime behavior is unchanged.
