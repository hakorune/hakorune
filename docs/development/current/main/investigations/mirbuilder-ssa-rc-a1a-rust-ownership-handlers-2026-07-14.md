---
Status: Closed
Date: 2026-07-14
Decision: SSA-RC-A1a — Rust semantic-oracle handlers only
Parent: mirbuilder-dprime-binding-ssa-final-form-task-2026-07-14.md
Next: SSA-RC-V0 Ownership SSA verifier and forwarding
---

# SSA-RC-A1a Rust Ownership Handler Evidence

## Outcome

The Rust MIR interpreter now executes the passive Ownership SSA vocabulary
without reusing the legacy alias-group lifecycle path.

```text
CopyOwned:
  exact BoxRef source
  undefined destination register
  Arc clone into the destination

DestroyOwned:
  exact BoxRef register
  take only the named register
  never scan or clear same-object aliases
```

Non-BoxRef operands and an already-defined `CopyOwned` destination fail with a
typed/contract error before the ownership write. The existing function-frame
transaction remains the error-restoration owner.

## Verification

```text
cargo test -q --features vm-reference ownership_contract_tests -- --nocapture
  5/5 green

cargo test -q mir::contracts::backend_core_ops::tests -- --nocapture
  21/21 green

bash tools/checks/resolved_region_flow_authority_guard.sh
  ownership production profile 17/17 green
  authority guard green

cargo build --release --bin hakorune
  green (existing warnings only)

bash tools/checks/dev_gate.sh quick
  PASS 66/66
```

All new or modified source/check files remain below 800 lines.

## Non-claims

```text
production canonical ownership callers = 0
Owned Phi / Return forwarding = not active
parameter/result/call ABI ownership = not active
LLVM/Wasm/Hako interpreter ownership handlers = 0
legacy ReleaseStrong retirement = not started by this row
```

SSA-RC-V0 must seal path-sensitive ownership classification and forwarding
before any Owned Phi, Return, or ABI behavior can become production authority.
