---
Status: Done
Decision: accepted
Date: 2026-05-05
Scope: move Rust global-call downstream readers from temporary shape checks toward MIR-owned return contracts
Related:
  - docs/development/current/main/phases/phase-29cv/P381BE-UNIFORM-BODY-EMITTER-CONTRACT-INVENTORY.md
  - docs/development/current/main/phases/phase-29cv/P381BF-VOID-LOGGING-DIRECT-CONTRACT-SHRINK.md
  - src/mir/global_call_route_plan/model.rs
---

# P381BG: Global-Call Return Contract Readers

## Problem

P381BF moved C-side void logging consumers to read the
`void_sentinel_i64_zero` + `scalar_i64` contract, but Rust downstream readers
still repeat temporary `GlobalCallTargetShape` branches when they only need the
call result contract.

That keeps capsule names visible in places that should care about value class,
not about the source-owner capsule that proved the value class.

## Decision

Add a small Rust-side return-contract vocabulary for global-call targets and
use it from value-class consumers.

This card is BoxShape only:

- no new accepted MIR shape
- no new `GlobalCallTargetShape`
- no changed JSON strings
- no deletion of `GenericStringVoidLoggingBody`

Implemented:

- added `GlobalCallReturnContract`
- centralized `return_shape` and `value_demand` JSON strings through that
  contract
- moved downstream value-class readers in global-call route analysis to read
  the return contract instead of repeating temporary capsule shape branches
- left proof, target shape, and classifier diagnostics unchanged

## Acceptance

```bash
cargo test --release void_logging -- --nocapture
bash tools/checks/stage0_shape_inventory_guard.sh
bash tools/checks/current_state_pointer_guard.sh
git diff --check
```

## Result

Done:

- Rust downstream readers no longer need to know that void logging is proven by
  `GenericStringVoidLoggingBody` when they only need `void_sentinel_i64_zero`
- `GenericStringVoidLoggingBody` remains in the shape inventory as the
  classifier/proof token for this card

Next:

1. decide whether proof/shape JSON compatibility still requires the temporary
   shape name
2. if compatibility is preserved through another field, remove the Rust variant
   in a separate card
