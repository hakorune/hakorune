---
Status: Landed
Date: 2026-04-27
Scope: normalized-shadow public surface inventory
Related:
  - src/mir/control_tree/normalized_shadow/mod.rs
  - src/mir/control_tree/normalized_shadow/common/mod.rs
  - docs/development/current/main/phases/phase-291x/291x-423-normalized-shadow-anf-status-wording-cleanup-card.md
---

# 291x-424: Normalized-Shadow Public Surface Inventory

## Goal

Pick the next small compiler-cleanliness seam after normalized-shadow ANF status
wording cleanup.

This is a BoxShape inventory. No behavior changed.

## Findings

The normalized-shadow root module still re-exports several internal contract
and helper types:

```rust
pub use contracts::{CapabilityCheckResult, UnsupportedCapability};
pub use env_layout::EnvLayout;
pub use exit_reconnector::ExitReconnectorBox;
pub use parity_contract::{MismatchKind, ShadowParityResult};
```

Callers outside `normalized_shadow` do not use these facade re-exports. Current
external callers use only:

```rust
normalized_shadow::StepTreeNormalizedShadowLowererBox
normalized_shadow::env_layout::EnvLayout
```

The broad `common` module remains a separate boundary because public ANF
contract types still expose `KnownIntrinsic` from `common::expr_lowering_contract`.
Changing `common` visibility should be done only after the ANF public contract
surface is inventoried separately.

## Decision

Clean the root facade only:

- keep `pub use builder::StepTreeNormalizedShadowLowererBox`
- remove unused root re-exports for contracts, env layout, exit reconnector, and
  parity result types
- narrow obviously internal root modules only when `cargo check --tests`
  confirms no public-interface leak

Do not change:

- route order
- accepted StepTree shapes
- fail-fast tags
- `StepTreeNormalizedShadowLowererBox` execution path
- `normalized_shadow::env_layout::EnvLayout` module path
- `common` visibility

## Next Cleanup

`291x-425`: normalized-shadow public surface prune.

Acceptance:

```bash
cargo check --tests
bash tools/checks/current_state_pointer_guard.sh
git diff --check
rg -n "pub use (contracts|env_layout|exit_reconnector|parity_contract)" \
  src/mir/control_tree/normalized_shadow/mod.rs
rg -n "normalized_shadow::(CapabilityCheckResult|UnsupportedCapability|EnvLayout|ExitReconnectorBox|MismatchKind|ShadowParityResult)" \
  src -g '*.rs'
```

The final `rg` commands should produce no output.
