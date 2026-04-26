---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow public surface prune
Related:
  - src/mir/control_tree/normalized_shadow/mod.rs
  - src/mir/control_tree/normalized_shadow/common/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-424-normalized-shadow-public-surface-inventory-card.md
---

# 291x-425: Normalized-Shadow Public Surface Prune

## Goal

Prune unused normalized-shadow root facade exports.

This is a BoxShape cleanup. No route behavior changed.

## Change

Removed unused root re-exports:

```rust
pub use contracts::{CapabilityCheckResult, UnsupportedCapability};
pub use env_layout::EnvLayout;
pub use exit_reconnector::ExitReconnectorBox;
pub use parity_contract::{MismatchKind, ShadowParityResult};
```

Kept root module visibility unchanged for now:

```rust
contracts
exit_reconnector
parity_contract
```

During verification, narrowing these modules to `pub(crate)` produced
dead-code warnings for legacy parity/exit helpers. Warning growth is not a
keeper for this cleanup, so this card only removes the unused facade
re-exports.

The public normalized-shadow facade remains:

```rust
pub use builder::StepTreeNormalizedShadowLowererBox;
pub mod env_layout;
```

## Preserved Behavior

- Route order is unchanged.
- Accepted StepTree shapes are unchanged.
- Fail-fast/debug tags are unchanged.
- `StepTreeNormalizedShadowLowererBox` remains the root entry point.
- `normalized_shadow::env_layout::EnvLayout` remains available by module path.
- `common` visibility is unchanged.
- `contracts`, `exit_reconnector`, and `parity_contract` module visibility is
  unchanged.

## Verification

```bash
cargo check --tests
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "pub use (contracts|env_layout|exit_reconnector|parity_contract)" \
  src/mir/control_tree/normalized_shadow/mod.rs
rg -n "normalized_shadow::(CapabilityCheckResult|UnsupportedCapability|EnvLayout|ExitReconnectorBox|MismatchKind|ShadowParityResult)" \
  src -g '*.rs'
```

## Next Cleanup

Inventory the next compiler-cleanliness seam after normalized-shadow public
surface prune.
